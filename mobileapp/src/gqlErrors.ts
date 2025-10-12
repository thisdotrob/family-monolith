import { ApolloError } from '@apollo/client';

export const getErrorCodes = (error: ApolloError | Error | any): string[] => {
  const codes: string[] = [];
  if (!error) return codes;
  const graphQLErrors = (error as any).graphQLErrors as Array<any> | undefined;
  if (Array.isArray(graphQLErrors)) {
    for (const e of graphQLErrors) {
      const code = e?.extensions?.code;
      if (typeof code === 'string') codes.push(code);
      // Some backends may send error arrays (legacy); normalize if needed
      if (Array.isArray(e)) {
        for (const val of e) if (typeof val === 'string') codes.push(val);
      }
    }
  }
  // Also check top-level extensions if present
  const extCode = (error as any)?.extensions?.code;
  if (typeof extCode === 'string') codes.push(extCode);
  return Array.from(new Set(codes));
};

export const hasCode = (error: ApolloError | Error | any, code: string) =>
  getErrorCodes(error).includes(code);

export const isOfflineError = (error: ApolloError | Error | any) => {
  const networkError: any = (error as any).networkError;
  if (!networkError) return false;
  const msg = String(networkError.message || '');
  // Common Apollo/Fetch offline patterns
  if (msg.includes('Network request failed')) return true;
  if (msg.includes('Failed to fetch')) return true;
  // React Native fetch sometimes surfaces as TypeError: Network request failed
  if ((networkError as any).status === 0) return true;
  return false;
};
