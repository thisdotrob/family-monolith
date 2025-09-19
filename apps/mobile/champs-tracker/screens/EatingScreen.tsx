import { useState } from 'react';
import { View, StyleSheet, ScrollView, Alert } from 'react-native';
import { Text, TextInput, Button, Card } from 'react-native-paper';
import { useMutation } from '@apollo/client';
import { CREATE_EATING_ACTIVITY, GET_EATING_ACTIVITIES } from '@shared/graphql/champ-tracker';
import EatingActivityList from '../components/EatingActivityList';

const EatingScreen = () => {
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({
    timestamp: new Date(),
    quantityEaten: '',
    leftoversThrowAway: '',
    foodType: '',
  });

  const [createEatingActivity, { loading }] = useMutation(CREATE_EATING_ACTIVITY, {
    refetchQueries: [{ query: GET_EATING_ACTIVITIES, variables: { limit: 20, offset: 0 } }],
    onCompleted: () => {
      Alert.alert('Success', 'Eating activity logged successfully!');
      setFormData({
        timestamp: new Date(),
        quantityEaten: '',
        leftoversThrowAway: '',
        foodType: '',
      });
      setShowForm(false);
    },
    onError: (error) => {
      Alert.alert('Error', `Failed to log activity: ${error.message}`);
    },
  });

  const handleSubmit = async () => {
    if (!formData.quantityEaten.trim() || !formData.foodType.trim()) {
      Alert.alert('Validation Error', 'Please fill in quantity eaten and food type');
      return;
    }

    try {
      await createEatingActivity({
        variables: {
          input: {
            timestamp: formData.timestamp.toISOString(),
            quantityEaten: formData.quantityEaten.trim(),
            leftoversThrownAway: formData.leftoversThrowAway.trim() || null,
            foodType: formData.foodType.trim(),
          },
        },
      });
    } catch (error) {
      console.error('Submit error:', error);
    }
  };

  const updateTimestamp = () => {
    setFormData({ ...formData, timestamp: new Date() });
  };

  if (showForm) {
    return (
      <ScrollView style={styles.container} contentContainerStyle={styles.content}>
        <Card style={styles.formCard}>
          <Card.Title title="🍽️ Log Eating Activity" />
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
                label="Quantity Eaten *"
                value={formData.quantityEaten}
                onChangeText={(text) => setFormData({ ...formData, quantityEaten: text })}
                mode="outlined"
                placeholder="e.g., Full bowl, Half portion, Small amount"
              />
            </View>

            <View style={styles.formGroup}>
              <TextInput
                label="Food Type *"
                value={formData.foodType}
                onChangeText={(text) => setFormData({ ...formData, foodType: text })}
                mode="outlined"
                placeholder="e.g., Wet food - chicken, Dry kibble, Treats"
              />
            </View>

            <View style={styles.formGroup}>
              <TextInput
                label="Leftovers Thrown Away (optional)"
                value={formData.leftoversThrowAway}
                onChangeText={(text) => setFormData({ ...formData, leftoversThrowAway: text })}
                mode="outlined"
                placeholder="e.g., None, Small amount, Half bowl"
              />
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
      
      <EatingActivityList />
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
  actions: {
    justifyContent: 'flex-end',
    gap: 8,
  },
});

export default EatingScreen;