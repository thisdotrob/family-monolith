import { useCallback, useState } from 'react';
import { Dialog, Portal, Button, Text } from 'react-native-paper';
import type { ApolloError } from '@apollo/client';

export type UseStaleWriteRetryOptions = {
  refetch: () => Promise<any>;
  showMessage?: (message: string) => void;
};

export const extractGraphqlErrorCodes = (error: ApolloError): string[] => {
  try {
    return (error.graphQLErrors || []).map((e: any) => e?.extensions?.code).filter(Boolean);
  } catch {
    return [];
  }
};

export const isOfflineError = (error: ApolloError): boolean => {
  // Apollo links typically put network errors here
  // Treat "Failed to fetch", TypeError network failure, or no network status as offline
  const msg = String(error?.networkError || error?.message || '').toLowerCase();
  return (
    !!error.networkError ||
    msg.includes('failed to fetch') ||
    msg.includes('network request failed') ||
    msg.includes('network error') ||
    msg.includes('offline')
  );
};

export const useStaleWriteRetry = ({ refetch, showMessage }: UseStaleWriteRetryOptions) => {
  const [retryVisible, setRetryVisible] = useState(false);
  const [pendingRetry, setPendingRetry] = useState<null | (() => Promise<any>)>(null);

  const handleApolloError = useCallback(
    async (error: ApolloError, retryFn?: () => Promise<any>) => {
      const codes = extractGraphqlErrorCodes(error);

      if (codes.includes('CONFLICT_STALE_WRITE')) {
        try {
          await refetch();
        } catch {
          // still continue to offer retry; refetch failure could be transient
        }
        if (retryFn) {
          setPendingRetry(() => retryFn);
          setRetryVisible(true);
        }
        return true; // handled
      }

      if (codes.includes('PERMISSION_DENIED')) {
        showMessage?.('You do not have permission to perform this action.');
        return true;
      }

      if (isOfflineError(error)) {
        showMessage?.('You appear to be offline. Please try again when back online.');
        return true;
      }

      return false; // not handled, caller can show generic message
    },
    [refetch, showMessage],
  );

  const ConflictRetryDialog = useCallback(
    () => (
      <Portal>
        <Dialog visible={retryVisible} onDismiss={() => setRetryVisible(false)}>
          <Dialog.Title>Item changed</Dialog.Title>
          <Dialog.Content>
            <Text>
              This item has been updated elsewhere. We refreshed the latest data. Would you like to
              retry your action?
            </Text>
          </Dialog.Content>
          <Dialog.Actions>
            <Button onPress={() => setRetryVisible(false)}>Cancel</Button>
            <Button
              mode="contained"
              onPress={async () => {
                const fn = pendingRetry;
                setRetryVisible(false);
                setPendingRetry(null);
                if (fn) {
                  try {
                    await fn();
                  } catch {
                    // caller should surface any follow-up errors
                  }
                }
              }}
            >
              Retry
            </Button>
          </Dialog.Actions>
        </Dialog>
      </Portal>
    ),
    [retryVisible, pendingRetry],
  );

  return { handleApolloError, ConflictRetryDialog } as const;
};
