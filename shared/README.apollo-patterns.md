# Apollo Client Usage Patterns

This document outlines common patterns for using Apollo Client in this project, including timezone handling and authentication contexts.

## Authentication Patterns

### Authenticated Operations (Default)

Most GraphQL operations require authentication. The Apollo client automatically includes the JWT token in the Authorization header for these requests.

```tsx
import { useQuery, useMutation } from '@apollo/client';

// Standard authenticated query
const { data, loading, error } = useQuery(TASKS_QUERY, {
  variables: { projectId: '123' }
});

// Standard authenticated mutation
const [createTask] = useMutation(CREATE_TASK_MUTATION);
```

### Unauthenticated Operations

Some operations like login and refresh token should **not** include authentication headers. Use the `context: { unauthenticated: true }` pattern for these:

```tsx
import { useMutation } from '@apollo/client';
import { LOGIN_MUTATION, REFRESH_TOKEN_MUTATION } from '@shared';

// Login mutation (unauthenticated)
const [login] = useMutation(LOGIN_MUTATION, {
  context: { unauthenticated: true }
});

// Refresh token mutation (unauthenticated)
const [refreshToken] = useMutation(REFRESH_TOKEN_MUTATION, {
  context: { unauthenticated: true }
});
```

**Important**: The `unauthenticated: true` context prevents the auth header link from adding the Authorization header to the request.

## Timezone Patterns

Many queries and mutations require timezone information for server-side calculations (overdue status, grouping, etc.).

### Basic Timezone Usage

```tsx
import { useQuery } from '@apollo/client';
import { useTimezone, withTimezone } from '@shared';

function TasksList({ projectId }: { projectId: string }) {
  const timezone = useTimezone();
  
  const { data, loading, error } = useQuery(TASKS_QUERY, {
    variables: withTimezone({ projectId }, timezone)
  });
  
  // Or manually:
  const { data: historyData } = useQuery(HISTORY_QUERY, {
    variables: {
      projectId,
      statuses: ['done'],
      timezone // Add timezone directly
    }
  });
}
```

### Timezone-Required Operations

These GraphQL operations require timezone variables:

- `tasks` query - for `isOverdue` and `bucket` derivation
- `history` query - for proper date grouping
- `createTask` mutation - when validating scheduled/deadline times
- `updateTask` mutation - when validating scheduled/deadline times
- `createRecurringSeries` mutation - when validating first occurrence timing

### useTimezone Hook

The `useTimezone` hook provides the device timezone:

```tsx
import { useTimezone } from '@shared';

function MyComponent() {
  const timezone = useTimezone(); // Returns IANA timezone string (e.g., "America/New_York")
  
  // Hook returns immediately with sync fallback, then updates if needed
  // On mobile: tries expo-localization first, falls back to Intl API
  // On web: uses Intl API directly
  
  return <div>Current timezone: {timezone}</div>;
}
```

## Error Handling Patterns

### GraphQL Errors

GraphQL errors include standardized `extensions.code` values:

```tsx
import { useQuery } from '@apollo/client';

const { data, loading, error } = useQuery(SOME_QUERY);

if (error) {
  error.graphQLErrors.forEach(({ extensions }) => {
    switch (extensions?.code) {
      case 'VALIDATION_FAILED':
        // Handle validation errors
        break;
      case 'PERMISSION_DENIED':
        // Handle permission errors
        break;
      case 'NOT_FOUND':
        // Handle not found errors
        break;
      case 'CONFLICT_STALE_WRITE':
        // Handle stale write conflicts
        break;
      case 'RATE_LIMITED':
        // Handle rate limiting
        break;
      default:
        // Handle other errors
    }
  });
}
```

### Automatic Token Refresh

The Apollo client automatically handles token refresh on authentication errors. The error link detects `TOKEN_EXPIRED` errors and:

1. Sets `isAuthenticating` to `true`
2. Calls the refresh token mutation with `context: { unauthenticated: true }`
3. Updates stored tokens on success
4. Retries the original operation
5. Logs out the user if refresh fails

This happens transparently - components don't need special handling for token expiration.

## Best Practices

1. **Always use timezone** for queries that involve dates/times
2. **Use `unauthenticated: true`** only for login/refresh operations
3. **Handle GraphQL error codes** appropriately in UI
4. **Use `withTimezone` helper** for cleaner variable construction
5. **Trust automatic token refresh** - don't manually handle token expiration
6. **Initialize with `useTimezone`** early in components that need it

## Examples

### Complete Task List Component

```tsx
import { useQuery } from '@apollo/client';
import { useTimezone, withTimezone } from '@shared';

function TasksList({ projectId }: { projectId: string }) {
  const timezone = useTimezone();
  
  const { data, loading, error, refetch } = useQuery(TASKS_QUERY, {
    variables: withTimezone({
      projectId,
      statuses: ['todo'],
      offset: 0,
      limit: 20
    }, timezone),
    pollInterval: 10000, // Poll every 10 seconds
  });

  if (loading) return <div>Loading...</div>;
  
  if (error) {
    const staleWrite = error.graphQLErrors.find(e => e.extensions?.code === 'CONFLICT_STALE_WRITE');
    if (staleWrite) {
      return <div>Data changed, <button onClick={() => refetch()}>refresh</button></div>;
    }
    return <div>Error: {error.message}</div>;
  }

  return (
    <div>
      {data?.tasks.items.map(task => (
        <TaskItem key={task.id} task={task} />
      ))}
    </div>
  );
}
```

### Login Form

```tsx
import { useMutation } from '@apollo/client';
import { useAuth } from '@shared';

function LoginForm() {
  const { saveTokens } = useAuth();
  
  const [login, { loading, error }] = useMutation(LOGIN_MUTATION, {
    context: { unauthenticated: true } // Important!
  });

  const handleSubmit = async (username: string, password: string) => {
    try {
      const { data } = await login({
        variables: { input: { username, password } }
      });
      
      if (data.login.success) {
        await saveTokens(data.login.token, data.login.refreshToken);
      }
    } catch (err) {
      // Handle login errors
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      {/* Form fields */}
    </form>
  );
}
```