import { gql } from '@apollo/client';

export const GET_USER_QUERY = gql`
  query GetUser {
    user {
      id
      name
    }
  }
`;

export const ME_QUERY = gql`
  query Me {
    me {
      username
      firstName
    }
  }
`;
