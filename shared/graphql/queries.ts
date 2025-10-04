import { gql } from '@apollo/client';

// GraphQL Enums (must match server definitions)
export const TaskStatus = {
  TODO: 'todo' as const,
  DONE: 'done' as const,
  ABANDONED: 'abandoned' as const,
} as const;

export const TaskBucket = {
  OVERDUE: 'Overdue' as const,
  TODAY: 'Today' as const,
  TOMORROW: 'Tomorrow' as const,
  UPCOMING: 'Upcoming' as const,
  NO_DATE: 'NoDate' as const,
} as const;

export type TaskStatus = typeof TaskStatus[keyof typeof TaskStatus];
export type TaskBucket = typeof TaskBucket[keyof typeof TaskBucket];

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

export const TASKS_QUERY = gql`
  query Tasks(
    $projectId: String!
    $timezone: String!
    $statuses: [TaskStatus!]
    $assignee: String
    $includeUnassigned: Boolean
    $assignedToMe: Boolean
    $tagIds: [String!]
    $search: String
    $offset: Int
    $limit: Int
  ) {
    tasks(
      projectId: $projectId
      timezone: $timezone
      statuses: $statuses
      assignee: $assignee
      includeUnassigned: $includeUnassigned
      assignedToMe: $assignedToMe
      tagIds: $tagIds
      search: $search
      offset: $offset
      limit: $limit
    ) {
      items {
        id
        projectId
        authorId
        assigneeId
        seriesId
        title
        description
        status
        scheduledDate
        scheduledTimeMinutes
        deadlineDate
        deadlineTimeMinutes
        completedAt
        completedBy
        abandonedAt
        abandonedBy
        createdAt
        updatedAt
        isOverdue
        bucket
      }
      totalCount
    }
  }
`;

export const HISTORY_QUERY = gql`
  query History(
    $statuses: [TaskStatus!]!
    $timezone: String!
    $projectId: String
    $tagIds: [String!]
    $completerId: String
    $fromDate: String
    $toDate: String
    $offset: Int
    $limit: Int
  ) {
    history(
      statuses: $statuses
      timezone: $timezone
      projectId: $projectId
      tagIds: $tagIds
      completerId: $completerId
      fromDate: $fromDate
      toDate: $toDate
      offset: $offset
      limit: $limit
    ) {
      items {
        id
        projectId
        authorId
        assigneeId
        seriesId
        title
        description
        status
        scheduledDate
        scheduledTimeMinutes
        deadlineDate
        deadlineTimeMinutes
        completedAt
        completedBy
        abandonedAt
        abandonedBy
        createdAt
        updatedAt
        isOverdue
        bucket
      }
      totalCount
    }
  }
`;
