import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import App from './App';
import { AuthProvider } from './contexts/AuthContext';
import '@testing-library/jest-dom/vitest';

describe('App', () => {
  it('renders login page when not authenticated', () => {
    render(
      <AuthProvider>
        <App />
      </AuthProvider>
    );
    expect(screen.getByText('Login')).toBeInTheDocument();
  });
});
