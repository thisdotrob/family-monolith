import { View, StyleSheet } from 'react-native';
import { Text, Button } from 'react-native-paper';
import { useAuth } from '@shared/contexts/AuthContext';
import { useApolloClient, useQuery } from '@apollo/client';
import { useTimezone, withTimezone } from '@shared/time';
import { TASKS_QUERY, TaskStatus } from '@shared/graphql/queries';

const TasksScreen = () => {
  const { logout } = useAuth();
  const client = useApolloClient();
  const timezone = useTimezone();

  // Example usage of timezone with tasks query (will fail until backend history query is implemented)
  // This demonstrates the pattern for when the full tasks list is implemented
  const { data: _tasksData, loading: _tasksLoading, error: _tasksError } = useQuery(
    TASKS_QUERY,
    {
      variables: withTimezone(
        {
          projectId: 'example-project-id', // This would come from selected project context
          statuses: [TaskStatus.TODO],
          offset: 0,
          limit: 20,
        },
        timezone,
      ),
      skip: true, // Skip for now since we don't have a real project selected
      errorPolicy: 'all', // Don't crash on GraphQL errors during development
    },
  );

  const logoutOnPress = async () => {
    try {
      await logout();
      client.clearStore();
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <View style={styles.container}>
      <Text variant="headlineMedium" style={styles.title}>
        Tasks
      </Text>
      <Text variant="bodyLarge" style={styles.description}>
        Task list for the selected project will be shown here.
      </Text>
      <Text variant="bodyMedium" style={styles.placeholder}>
        ✅ This is a placeholder screen for the Tasks tab.
      </Text>
      <Text variant="bodySmall" style={styles.timezoneInfo}>
        🌍 Device timezone: {timezone}
      </Text>
      <Text variant="bodySmall" style={styles.timezoneInfo}>
        📡 Tasks query ready with timezone variable
      </Text>
      
      <Button mode="contained" onPress={logoutOnPress} style={styles.button} icon="logout">
        Logout
      </Button>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  title: {
    marginBottom: 16,
    textAlign: 'center',
  },
  description: {
    marginBottom: 12,
    textAlign: 'center',
  },
  placeholder: {
    marginBottom: 20,
    textAlign: 'center',
    fontStyle: 'italic',
  },
  timezoneInfo: {
    marginBottom: 8,
    textAlign: 'center',
    fontStyle: 'italic',
    opacity: 0.7,
  },
  button: {
    marginTop: 20,
  },
});

export default TasksScreen;