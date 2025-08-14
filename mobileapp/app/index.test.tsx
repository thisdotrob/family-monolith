import React from 'react';
import { render, screen } from '@testing-library/react-native';
import LoginPage from './index';

// Since this is a unit test for the page, we can render it directly.
// We don't need to mock the router.

describe('Login Page', () => {
  it('renders the login form correctly', () => {
    // Render the component
    render(<LoginPage />);

    // Check for the main title
    expect(screen.getByText('Login')).toBeVisible();

    // Check for the input fields by their accessibility labels
    expect(screen.getByLabelText('Username')).toBeVisible();
    expect(screen.getByLabelText('Password')).toBeVisible();

    // Check for the sign-in button
    expect(screen.getByRole('button', { name: /sign in/i })).toBeVisible();
  });
});
