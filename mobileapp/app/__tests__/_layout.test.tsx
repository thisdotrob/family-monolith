import React from 'react';
import { jest, describe, it } from '@jest/globals';
import { render, waitFor } from '@testing-library/react-native';
import RootLayout from '../_layout';

// Mock AsyncStorage
jest.mock('@react-native-async-storage/async-storage', () => {
  const asyncStorageMock = {
    getItem: jest.fn<(...args: any[]) => Promise<string | null>>().mockResolvedValue(null),
    setItem: jest.fn<(...args: any[]) => Promise<void>>().mockResolvedValue(),
    removeItem: jest.fn<(...args: any[]) => Promise<void>>().mockResolvedValue(),
  };
  return asyncStorageMock;
});

// Mock react-native-paper
jest.mock('react-native-paper', () => {
  const RealModule = jest.requireActual<typeof import('react-native-paper')>('react-native-paper');
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
