import { View, StyleSheet } from 'react-native';
import { ActivityIndicator } from 'react-native-paper';
import { useQuery } from '@apollo/client';
import { ME_QUERY } from '@shared/graphql/queries';
import TabNavigator from './TabNavigator';

const HomePage = () => {
  const { loading, error } = useQuery(ME_QUERY);

  if (loading) {
    return <ActivityIndicator animating={true} style={styles.centered} />;
  }

  if (error) {
    return (
      <View style={styles.container}>
        <ActivityIndicator animating={true} style={styles.centered} />
      </View>
    );
  }

  // Once authenticated and user data is loaded, show the tab navigator
  return <TabNavigator />;
};

const styles = StyleSheet.create({
  centered: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
});

export default HomePage;
