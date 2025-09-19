import { View, StyleSheet, FlatList, RefreshControl } from 'react-native';
import { Text, Card, Chip } from 'react-native-paper';
import { useQuery } from '@apollo/client';
import { GET_EATING_ACTIVITIES } from '@shared/graphql/champ-tracker';

interface EatingActivity {
  id: string;
  userId: string;
  timestamp: string;
  quantityEaten: string;
  leftoversThrownAway?: string;
  foodType: string;
  createdAt: string;
}

interface EatingActivitiesData {
  champTracker: {
    eatingActivities: EatingActivity[];
  };
}

const EatingActivityList = () => {
  const { data, loading, error, refetch } = useQuery<EatingActivitiesData>(
    GET_EATING_ACTIVITIES,
    {
      variables: { limit: 20, offset: 0 },
      fetchPolicy: 'cache-and-network',
    }
  );

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const renderActivity = ({ item }: { item: EatingActivity }) => (
    <Card style={styles.card} mode="outlined">
      <Card.Content>
        <View style={styles.header}>
          <Text variant="titleMedium">🍽️ Eating Activity</Text>
          <Text variant="bodySmall" style={styles.timestamp}>
            {formatDate(item.timestamp)}
          </Text>
        </View>
        
        <View style={styles.details}>
          <View style={styles.row}>
            <Text variant="bodyMedium" style={styles.label}>Quantity Eaten:</Text>
            <Chip mode="outlined" compact style={styles.chip}>
              {item.quantityEaten}
            </Chip>
          </View>
          
          <View style={styles.row}>
            <Text variant="bodyMedium" style={styles.label}>Food Type:</Text>
            <Text variant="bodyMedium" style={styles.value}>{item.foodType}</Text>
          </View>
          
          {item.leftoversThrownAway && (
            <View style={styles.row}>
              <Text variant="bodyMedium" style={styles.label}>Leftovers:</Text>
              <Text variant="bodyMedium" style={styles.value}>{item.leftoversThrownAway}</Text>
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

  const activities = data?.champTracker?.eatingActivities || [];

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
              No eating activities logged yet
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
  label: {
    fontWeight: '600',
    marginRight: 8,
    minWidth: 100,
  },
  value: {
    flex: 1,
  },
  chip: {
    height: 28,
    backgroundColor: '#f0f4ff',
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

export default EatingActivityList;