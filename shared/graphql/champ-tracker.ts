import { gql } from '@apollo/client';

export const CREATE_BATHROOM_ACTIVITY = gql`
  mutation CreateBathroomActivity($input: BathroomActivityInput!) {
    createBathroomActivity(input: $input) {
      id
      timestamp
      consistency
      observations
      litterChanged
      createdAt
    }
  }
`;

export const GET_BATHROOM_ACTIVITIES = gql`
  query GetBathroomActivities($limit: Int, $offset: Int) {
    champTracker {
      bathroomActivities(limit: $limit, offset: $offset) {
        id
        userId
        timestamp
        consistency
        observations
        litterChanged
        createdAt
      }
    }
  }
`;