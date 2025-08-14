import { ApolloLink, execute, Observable, from } from '@apollo/client';
import { onError } from '@apollo/client/link/error';
import { GraphQLError } from 'graphql';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { GET_USER_QUERY } from '../graphql/queries';
import { REFRESH_TOKEN_MUTATION } from '../graphql/mutations';

// Mock the modules
vi.mock('../auth', () => ({
  saveTokens: vi.fn(),
  logout: vi.fn(),
}));
vi.mock('./state', () => ({
  isRefreshingTokenVar: vi.fn(),
}));

import { saveTokens, logout } from '../auth';
import { isRefreshingTokenVar } from './state';

describe('Apollo Error Link', () => {
  afterEach(() => {
    vi.clearAllMocks();
  });

  it('should refresh token and retry the request', (done) => {
    const tokenExpiredError = new GraphQLError('Token has expired', {
      extensions: { code: 'TOKEN_EXPIRED' },
    });

    const successfulQueryResponse = { data: { user: { id: '1', name: 'Test User' } } };
    const refreshTokenResponse = {
      data: {
        refreshToken: {
          success: true,
          token: 'new-token',
          refreshToken: 'new-refresh-token',
          errors: [],
        },
      },
    };

    let callCount = 0;

    const mockLink = new ApolloLink((operation) => {
      callCount++;
      if (callCount === 1) {
        expect(operation.operationName).toBe('GetUser');
        return new Observable((observer) => {
          observer.next({ errors: [tokenExpiredError] });
          observer.complete();
        });
      }
      if (callCount === 2) {
        expect(operation.operationName).toBe('RefreshToken');
        // Check that the refreshing state is true when we make the refresh call
        expect(isRefreshingTokenVar).toHaveBeenCalledWith(true);
        return new Observable((observer) => {
          observer.next(refreshTokenResponse);
          observer.complete();
        });
      }
      if (callCount === 3) {
        expect(operation.operationName).toBe('GetUser');
        return new Observable((observer) => {
          observer.next(successfulQueryResponse);
          observer.complete();
        });
      }
      return new Observable((observer) => {
        observer.error(new Error('Unexpected call'));
        observer.complete();
      });
    });

    const errorLink = onError(({ graphQLErrors, forward, operation }) => {
      if (graphQLErrors) {
        for (const err of graphQLErrors) {
          if (err.extensions.code === 'TOKEN_EXPIRED') {
            isRefreshingTokenVar(true);
            execute(mockLink, { query: REFRESH_TOKEN_MUTATION }).subscribe({
              next: (response) => {
                const { token, refreshToken } = response.data.refreshToken;
                saveTokens(null, token, refreshToken);
                isRefreshingTokenVar(false);
                forward(operation);
              },
              error: () => {
                logout(null);
                isRefreshingTokenVar(false);
              },
            });
            return;
          }
        }
      }
    });

    const link = from([errorLink, mockLink]);

    execute(link, { query: GET_USER_QUERY }).subscribe({
      next: (result) => {
        if (result.data) {
          expect(result).toEqual(successfulQueryResponse);
          expect(callCount).toBe(3);
          expect(saveTokens).toHaveBeenCalledWith(null, 'new-token', 'new-refresh-token');
          // Check that the refreshing state was set to false
          expect(isRefreshingTokenVar).toHaveBeenLastCalledWith(false);
          done();
        }
      },
    });
  });
});
