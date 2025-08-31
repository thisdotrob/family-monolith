import { View, Modal, StyleSheet } from 'react-native';
import { ActivityIndicator, Text } from 'react-native-paper';

const GlobalLoading = () => {
  return (
    <Modal transparent={true} animationType="fade">
      <View style={styles.container}>
        <View style={styles.content}>
          <ActivityIndicator animating={true} size="large" />
          <Text style={styles.text}>Authenticating...</Text>
        </View>
      </View>
    </Modal>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
  },
  content: {
    backgroundColor: 'white',
    padding: 20,
    borderRadius: 10,
    alignItems: 'center',
  },
  text: {
    marginTop: 16,
    fontSize: 16,
  },
});

export default GlobalLoading;
