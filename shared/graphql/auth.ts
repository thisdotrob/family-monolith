import { gql } from '@apollo/client';

export const REFRESH_TOKEN_MUTATION = gql`
  mutation RefreshToken($refreshToken: String!) {
    refreshToken(input: { refreshToken: $refreshToken }) {
      success
      token
      refreshToken
      errors
    }
  }
`;
