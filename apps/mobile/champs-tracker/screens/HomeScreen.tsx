import { View, StyleSheet, ScrollView } from 'react-native';
import { Text, Button, ActivityIndicator } from 'react-native-paper';
import { useNavigation } from '@react-navigation/native';
import type { StackNavigationProp } from '@react-navigation/stack';
import { useApolloClient, useQuery } from '@apollo/client';
import { useAuth } from '@shared/contexts/AuthContext';
import { ME_QUERY } from '@shared/graphql/queries';
import ActivityButton from '../components/ActivityButton';
import type { RootStackParamList } from '../navigation/types';

type HomeScreenNavigationProp = StackNavigationProp<RootStackParamList, 'Home'>;

const HomeScreen = () => {
  const navigation = useNavigation<HomeScreenNavigationProp>();
  const { logout } = useAuth();
  const client = useApolloClient();

  const { data, loading, error } = useQuery(ME_QUERY);

  if (loading) {
    return <ActivityIndicator animating={true} style={styles.centered} />;
  }

  if (error) {
    return (
      <View style={styles.container}>
        <Text style={styles.centered}>Error: {error.message}</Text>
      </View>
    );
  }

  const logoutOnPress = async () => {
    try {
      await logout();
      client.clearStore();
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <ScrollView style={styles.container} contentContainerStyle={styles.content}>
      <View style={styles.header}>
        <Text variant="headlineMedium" style={styles.welcome}>
          Welcome, {data?.me?.firstName || 'User'}!
        </Text>
        <Text variant="bodyMedium" style={styles.subtitle}>
          Track Champagne's daily activities
        </Text>
      </View>

      <View style={styles.activitiesContainer}>
        <View style={styles.activityRow}>
          <ActivityButton
            icon="💩"
            title="Bathroom"
            onPress={() => navigation.navigate('Bathroom')}
          />
          <ActivityButton
            icon="🍽️"
            title="Eating"
            onPress={() => navigation.navigate('Eating')}
          />
          <ActivityButton
            icon="🌳"
            title="Outdoor"
            onPress={() => navigation.navigate('Outdoor')}
          />
        </View>

        <View style={styles.activityRow}>
          <ActivityButton
            icon="🏥"
            title="Vet Visits"
            onPress={() => navigation.navigate('Vet')}
          />
          <ActivityButton
            icon="💊"
            title="Medicine"
            onPress={() => navigation.navigate('Medication')}
          />
          <ActivityButton
            icon="🎾"
            title="Play"
            onPress={() => navigation.navigate('Play')}
          />
        </View>

        <View style={styles.activityRow}>
          <ActivityButton
            icon="⭐"
            title="Highlights"
            onPress={() => navigation.navigate('Highlights')}
          />
        </View>
      </View>

      <View style={styles.footer}>
        <Button 
          mode="outlined" 
          onPress={logoutOnPress} 
          style={styles.logoutButton}
          icon="logout"
        >
          Logout
        </Button>
      </View>
    </ScrollView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  content: {
    flexGrow: 1,
    padding: 20,
  },
  centered: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  header: {
    alignItems: 'center',
    marginBottom: 30,
  },
  welcome: {
    textAlign: 'center',
    color: '#333',
    marginBottom: 8,
  },
  subtitle: {
    textAlign: 'center',
    color: '#666',
  },
  activitiesContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  activityRow: {
    flexDirection: 'row',
    justifyContent: 'center',
    marginBottom: 16,
  },
  footer: {
    marginTop: 30,
    alignItems: 'center',
  },
  logoutButton: {
    minWidth: 120,
  },
});

export default HomeScreen;