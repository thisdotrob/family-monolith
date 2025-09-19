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

export const CREATE_EATING_ACTIVITY = gql`
  mutation CreateEatingActivity($input: EatingActivityInput!) {
    createEatingActivity(input: $input) {
      id
      timestamp
      quantityEaten
      leftoversThrownAway
      foodType
      createdAt
    }
  }
`;

export const GET_EATING_ACTIVITIES = gql`
  query GetEatingActivities($limit: Int, $offset: Int) {
    champTracker {
      eatingActivities(limit: $limit, offset: $offset) {
        id
        userId
        timestamp
        quantityEaten
        leftoversThrownAway
        foodType
        createdAt
      }
    }
  }
`;