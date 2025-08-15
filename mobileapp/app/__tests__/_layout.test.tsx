import React from 'react';
import { render, waitFor } from '@testing-library/react-native';
import RootLayout from '../_layout';

// Mock AsyncStorage
jest.mock('@react-native-async-storage/async-storage', () => ({
  getItem: jest.fn().mockResolvedValue(null),
  setItem: jest.fn().mockResolvedValue(null),
  removeItem: jest.fn().mockResolvedValue(null),
}));

// Mock react-native-paper
jest.mock('react-native-paper', () => {
  const RealModule = jest.requireActual('react-native-paper');
  return {
    ...RealModule,
    PaperProvider: ({ children }: { children: React.ReactNode }) => <>{children}</>,
  };
});

describe('RootLayout', () => {
  it('renders without crashing', async () => {
    const { getByTestId } = render(<RootLayout />);
    await waitFor(() => getByTestId('username-input'));
  });
});
