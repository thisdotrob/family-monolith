import { View, StyleSheet } from 'react-native';
import { Text, Button } from 'react-native-paper';
import { useAuth } from '@shared/contexts/AuthContext';
import { useApolloClient, useQuery } from '@apollo/client';
import { useTimezone, withTimezone } from '@shared/time';
import { HISTORY_QUERY, TaskStatus } from '@shared/graphql/queries';

const HistoryScreen = () => {
  const { logout } = useAuth();
  const client = useApolloClient();
  const timezone = useTimezone();

  // Example usage of timezone with history query (will fail until backend history query is implemented)
  // This demonstrates the pattern for when the full history tab is implemented
  const { data: _historyData, loading: _historyLoading, error: _historyError } = useQuery(
    HISTORY_QUERY,
    {
      variables: withTimezone(
        {
          statuses: [TaskStatus.DONE], // Default to showing completed tasks
          // Default to last 7 days - these would be computed based on timezone
          fromDate: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
          toDate: new Date().toISOString().split('T')[0],
          offset: 0,
          limit: 20,
        },
        timezone,
      ),
      skip: true, // Skip for now since backend history query isn't implemented yet
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
        History
      </Text>
      <Text variant="bodyLarge" style={styles.description}>
        Toggle between Done and Abandoned tasks will be available here.
      </Text>
      <Text variant="bodyMedium" style={styles.placeholder}>
        📜 This is a placeholder screen for the History tab.
      </Text>
      <Text variant="bodySmall" style={styles.timezoneInfo}>
        🌍 Device timezone: {timezone}
      </Text>
      <Text variant="bodySmall" style={styles.timezoneInfo}>
        📡 History query ready with timezone variable
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

export default HistoryScreen;