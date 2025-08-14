import { ApolloClient } from '@apollo/client';

export const saveTokens = (client: ApolloClient<any>, token: string, refreshToken: string) => {
  localStorage.setItem('token', token);
  localStorage.setItem('refreshToken', refreshToken);
  client.resetStore();
};

export const logout = (client: ApolloClient<any>) => {
  localStorage.removeItem('token');
  localStorage.removeItem('refreshToken');
  client.resetStore();
};
