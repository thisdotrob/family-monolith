import React from 'react';
import { render, screen, act } from '@testing-library/react';
import { AuthProvider, useAuth } from './AuthContext';
import { describe, it, expect } from 'vitest';

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
  it('should initialize with isRefreshingToken as false', () => {
    render(
      <AuthProvider>
        <TestComponent />
      </AuthProvider>
    );
    expect(screen.getByTestId('is-refreshing')).toHaveTextContent('false');
  });

  it('should allow updating isRefreshingToken', () => {
    render(
      <AuthProvider>
        <TestComponent />
      </AuthProvider>
    );

    act(() => {
      screen.getByText('Refresh').click();
    });

    expect(screen.getByTestId('is-refreshing')).toHaveTextContent('true');
  });
});
