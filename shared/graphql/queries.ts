import { gql } from '@apollo/client';

export const ME_QUERY = gql`
  query Me {
    me {
      username
      firstName
    }
  }
`;

export const TAGS_QUERY = gql`
  query Tags($offset: Int, $limit: Int) {
    tags(offset: $offset, limit: $limit) {
      id
      name
      createdAt
      updatedAt
    }
  }
`;
