import { View, StyleSheet, FlatList, RefreshControl } from 'react-native';
import { Text, Card, Chip } from 'react-native-paper';
import { useQuery } from '@apollo/client';
import { GET_BATHROOM_ACTIVITIES } from '@shared/graphql/champ-tracker';

interface BathroomActivity {
  id: string;
  userId: string;
  timestamp: string;
  consistency?: string;
  observations?: string;
  litterChanged: boolean;
  createdAt: string;
}

interface BathroomActivitiesData {
  champTracker: {
    bathroomActivities: BathroomActivity[];
  };
}

const BathroomActivityList = () => {
  const { data, loading, error, refetch } = useQuery<BathroomActivitiesData>(
    GET_BATHROOM_ACTIVITIES,
    {
      variables: { limit: 20, offset: 0 },
      fetchPolicy: 'cache-and-network',
    }
  );

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const renderActivity = ({ item }: { item: BathroomActivity }) => (
    <Card style={styles.card} mode="outlined">
      <Card.Content>
        <View style={styles.header}>
          <Text variant="titleMedium">💩 Bathroom Activity</Text>
          <Text variant="bodySmall" style={styles.timestamp}>
            {formatDate(item.timestamp)}
          </Text>
        </View>
        
        <View style={styles.details}>
          <View style={styles.row}>
            <Text variant="bodyMedium" style={styles.label}>Litter Changed:</Text>
            <Chip 
              mode="outlined" 
              compact 
              style={[styles.chip, item.litterChanged ? styles.chipYes : styles.chipNo]}
            >
              {item.litterChanged ? 'Yes' : 'No'}
            </Chip>
          </View>
          
          {item.consistency && (
            <View style={styles.row}>
              <Text variant="bodyMedium" style={styles.label}>Consistency:</Text>
              <Text variant="bodyMedium">{item.consistency}</Text>
            </View>
          )}
          
          {item.observations && (
            <View style={styles.observationsRow}>
              <Text variant="bodyMedium" style={styles.label}>Observations:</Text>
              <Text variant="bodyMedium" style={styles.observations}>{item.observations}</Text>
            </View>
          )}
        </View>
        
        <Text variant="bodySmall" style={styles.metadata}>
          Logged by User {item.userId} • {formatDate(item.createdAt)}
        </Text>
      </Card.Content>
    </Card>
  );

  if (error) {
    return (
      <View style={styles.centered}>
        <Text variant="bodyLarge" style={styles.error}>
          Error loading activities: {error.message}
        </Text>
      </View>
    );
  }

  const activities = data?.champTracker?.bathroomActivities || [];

  return (
    <View style={styles.container}>
      <FlatList
        data={activities}
        renderItem={renderActivity}
        keyExtractor={(item) => item.id}
        contentContainerStyle={styles.listContent}
        refreshControl={
          <RefreshControl refreshing={loading} onRefresh={refetch} />
        }
        ListEmptyComponent={
          <View style={styles.centered}>
            <Text variant="bodyLarge" style={styles.emptyText}>
              No bathroom activities logged yet
            </Text>
          </View>
        }
      />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  listContent: {
    padding: 16,
    flexGrow: 1,
  },
  card: {
    marginBottom: 12,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 12,
  },
  timestamp: {
    color: '#666',
  },
  details: {
    marginBottom: 12,
  },
  row: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 8,
  },
  observationsRow: {
    marginBottom: 8,
  },
  label: {
    fontWeight: '600',
    marginRight: 8,
    minWidth: 80,
  },
  chip: {
    height: 28,
  },
  chipYes: {
    backgroundColor: '#e8f5e8',
  },
  chipNo: {
    backgroundColor: '#fef2f2',
  },
  observations: {
    flex: 1,
    marginTop: 4,
  },
  metadata: {
    color: '#999',
    fontStyle: 'italic',
  },
  centered: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  error: {
    color: '#d32f2f',
    textAlign: 'center',
  },
  emptyText: {
    color: '#666',
    textAlign: 'center',
  },
});

export default BathroomActivityList;