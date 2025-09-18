import { useState } from 'react';
import { View, StyleSheet, ScrollView, Alert } from 'react-native';
import { Text, TextInput, Button, Switch, Card } from 'react-native-paper';
import { useMutation } from '@apollo/client';
import { CREATE_BATHROOM_ACTIVITY, GET_BATHROOM_ACTIVITIES } from '@shared/graphql/champ-tracker';
import BathroomActivityList from '../components/BathroomActivityList';

const BathroomScreen = () => {
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({
    timestamp: new Date(),
    consistency: '',
    observations: '',
    litterChanged: false,
  });

  const [createBathroomActivity, { loading }] = useMutation(CREATE_BATHROOM_ACTIVITY, {
    refetchQueries: [{ query: GET_BATHROOM_ACTIVITIES, variables: { limit: 20, offset: 0 } }],
    onCompleted: () => {
      Alert.alert('Success', 'Bathroom activity logged successfully!');
      setFormData({
        timestamp: new Date(),
        consistency: '',
        observations: '',
        litterChanged: false,
      });
      setShowForm(false);
    },
    onError: (error) => {
      Alert.alert('Error', `Failed to log activity: ${error.message}`);
    },
  });

  const handleSubmit = async () => {
    try {
      await createBathroomActivity({
        variables: {
          input: {
            timestamp: formData.timestamp.toISOString(),
            consistency: formData.consistency.trim() || null,
            observations: formData.observations.trim() || null,
            litterChanged: formData.litterChanged,
          },
        },
      });
    } catch (error) {
      console.error('Submit error:', error);
    }
  };

  const updateTimestamp = () => {
    // For now, just update to current time when pressed
    // TODO: Implement proper date/time picker in future enhancement
    setFormData({ ...formData, timestamp: new Date() });
  };

  if (showForm) {
    return (
      <ScrollView style={styles.container} contentContainerStyle={styles.content}>
        <Card style={styles.formCard}>
          <Card.Title title="💩 Log Bathroom Activity" />
          <Card.Content>
            <View style={styles.formGroup}>
              <Text variant="labelLarge" style={styles.label}>Date & Time</Text>
              <Button
                mode="outlined"
                onPress={updateTimestamp}
                style={styles.timestampButton}
              >
                {formData.timestamp.toLocaleDateString()} {formData.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
              </Button>
              <Text variant="bodySmall" style={styles.timestampHelp}>
                Tap to update to current time
              </Text>
            </View>

            <View style={styles.formGroup}>
              <TextInput
                label="Consistency (optional)"
                value={formData.consistency}
                onChangeText={(text) => setFormData({ ...formData, consistency: text })}
                mode="outlined"
                placeholder="e.g., normal, soft, hard"
              />
            </View>

            <View style={styles.formGroup}>
              <TextInput
                label="Observations (optional)"
                value={formData.observations}
                onChangeText={(text) => setFormData({ ...formData, observations: text })}
                mode="outlined"
                multiline
                numberOfLines={3}
                placeholder="Any notable observations..."
              />
            </View>

            <View style={styles.formGroup}>
              <View style={styles.switchContainer}>
                <Text variant="bodyLarge">Litter Changed</Text>
                <Switch
                  value={formData.litterChanged}
                  onValueChange={(value) => setFormData({ ...formData, litterChanged: value })}
                />
              </View>
            </View>
          </Card.Content>
          
          <Card.Actions style={styles.actions}>
            <Button
              mode="outlined"
              onPress={() => setShowForm(false)}
              disabled={loading}
            >
              Cancel
            </Button>
            <Button
              mode="contained"
              onPress={handleSubmit}
              loading={loading}
              disabled={loading}
            >
              Log Activity
            </Button>
          </Card.Actions>
        </Card>

      </ScrollView>
    );
  }

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Button
          mode="contained"
          onPress={() => setShowForm(true)}
          icon="plus"
          style={styles.addButton}
        >
          Log New Activity
        </Button>
      </View>
      
      <BathroomActivityList />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  content: {
    padding: 16,
  },
  header: {
    padding: 16,
    paddingBottom: 8,
  },
  addButton: {
    alignSelf: 'flex-start',
  },
  formCard: {
    marginBottom: 16,
  },
  formGroup: {
    marginBottom: 16,
  },
  label: {
    marginBottom: 8,
  },
  timestampButton: {
    marginBottom: 4,
  },
  timestampHelp: {
    color: '#666',
    fontStyle: 'italic',
  },
  switchContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingVertical: 8,
  },
  actions: {
    justifyContent: 'flex-end',
    gap: 8,
  },
});

export default BathroomScreen;