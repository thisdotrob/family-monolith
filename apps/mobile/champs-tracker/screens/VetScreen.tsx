import { View, StyleSheet } from 'react-native';
import { Text } from 'react-native-paper';

const VetScreen = () => {
  return (
    <View style={styles.container}>
      <Text variant="headlineMedium" style={styles.title}>
        🏥 Vet Visit
      </Text>
      <Text variant="bodyLarge" style={styles.placeholder}>
        Vet visit tracking form will be implemented here.
      </Text>
      <Text variant="bodyMedium" style={styles.fields}>
        Fields: Date & Time, Reason, Weight, Treatments, Cost, Notes
      </Text>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
    backgroundColor: '#f5f5f5',
  },
  title: {
    textAlign: 'center',
    marginBottom: 20,
    color: '#333',
  },
  placeholder: {
    textAlign: 'center',
    marginBottom: 20,
    color: '#666',
  },
  fields: {
    textAlign: 'center',
    color: '#999',
    fontStyle: 'italic',
  },
});

export default VetScreen;