import React from 'react';
import { render, screen } from '@testing-library/react';
import { AuthProvider } from '../contexts/AuthContext';
import LoadingOverlay from './LoadingOverlay';
import { describe, it, expect, vi } from 'vitest';
import { ApolloProvider } from '@apollo/client';
import client from '../api/apollo';

// Mock useAuth
vi.mock('../contexts/AuthContext', async () => {
  const actual = await vi.importActual('../contexts/AuthContext');
  return {
    ...actual,
    useAuth: vi.fn(),
  };
});

const { useAuth } = await import('../contexts/AuthContext');

describe('LoadingOverlay', () => {
  it('should not be visible when isRefreshingToken is false', () => {
    vi.mocked(useAuth).mockReturnValue({
      isRefreshingToken: false,
      token: null,
      setIsRefreshingToken: () => {},
      saveTokens: () => {},
      logout: () => {},
    });

    render(
      <ApolloProvider client={client}>
        <AuthProvider>
          <LoadingOverlay />
        </AuthProvider>
      </ApolloProvider>
    );
    expect(screen.queryByTestId('loading-overlay')).not.toBeInTheDocument();
  });

  it('should be visible when isRefreshingToken is true', () => {
    vi.mocked(useAuth).mockReturnValue({
      isRefreshingToken: true,
      token: null,
      setIsRefreshingToken: () => {},
      saveTokens: () => {},
      logout: () => {},
    });

    render(
      <ApolloProvider client={client}>
        <AuthProvider>
          <LoadingOverlay />
        </AuthProvider>
      </ApolloProvider>
    );
    const overlay = screen.getByTestId('loading-overlay');
    expect(overlay).toBeInTheDocument();
    expect(overlay).toHaveTextContent('Refreshing JWT, hold tight...');
  });
});
