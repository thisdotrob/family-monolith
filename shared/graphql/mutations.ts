import { gql } from '@apollo/client';

export const LOGIN_MUTATION = gql`
  mutation Login($username: String!, $password: String!) {
    login(input: { username: $username, password: $password }) {
      success
      token
      refreshToken
      errors
    }
  }
`;

export const CREATE_TAG_MUTATION = gql`
  mutation CreateTag($name: String!) {
    createTag(name: $name) {
      id
      name
      createdAt
      updatedAt
    }
  }
`;

export const RENAME_TAG_MUTATION = gql`
  mutation RenameTag($tagId: String!, $newName: String!) {
    renameTag(tagId: $tagId, newName: $newName) {
      id
      name
      createdAt
      updatedAt
    }
  }
`;

export const DELETE_TAG_MUTATION = gql`
  mutation DeleteTag($tagId: String!) {
    deleteTag(tagId: $tagId)
  }
`;
