import React from 'react';
import { render, screen, act } from '@testing-library/react';
import { AuthProvider, useAuth } from './AuthContext';
import { describe, it, expect, afterEach } from 'vitest';
import { isRefreshingTokenVar } from '../api/state';
import { ApolloProvider } from '@apollo/client';
import client from '../api/apollo';

const TestComponent = () => {
  const { isRefreshingToken, setIsRefreshingToken } = useAuth();
  return (
    <div>
      <div data-testid="is-refreshing">{isRefreshingToken.toString()}</div>
      <button onClick={() => setIsRefreshingToken(true)}>Refresh</button>
    </div>
  );
};

describe('AuthProvider', () => {
  afterEach(() => {
    act(() => {
      isRefreshingTokenVar(false); // Reset the state after each test
    });
  });

  it('should initialize with isRefreshingToken as false', () => {
    render(
      <ApolloProvider client={client}>
        <AuthProvider>
          <TestComponent />
        </AuthProvider>
      </ApolloProvider>
    );
    expect(screen.getByTestId('is-refreshing')).toHaveTextContent('false');
  });

  it('should allow updating isRefreshingToken', () => {
    render(
      <ApolloProvider client={client}>
        <AuthProvider>
          <TestComponent />
        </AuthProvider>
      </ApolloProvider>
    );

    act(() => {
      screen.getByText('Refresh').click();
    });

    expect(screen.getByTestId('is-refreshing')).toHaveTextContent('true');
  });
});
