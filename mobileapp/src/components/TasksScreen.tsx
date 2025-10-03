import React, { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { Text, Appbar, Button } from 'react-native-paper';
import TagManager from './TagManager';

interface TasksScreenProps {
  onNavigate?: (screen: string) => void;
}

const TasksScreen: React.FC<TasksScreenProps> = ({ onNavigate }) => {
  const [tagManagerVisible, setTagManagerVisible] = useState(false);

  const openTagManager = () => {
    setTagManagerVisible(true);
  };

  const closeTagManager = () => {
    setTagManagerVisible(false);
  };

  return (
    <View style={styles.container}>
      <Appbar.Header>
        <Appbar.Content title="Tasks" />
        <Appbar.Action icon="tag" onPress={openTagManager} />
        <Appbar.Action
          icon="dots-vertical"
          onPress={() => console.log('More options menu (stub)')}
        />
      </Appbar.Header>

      <View style={styles.content}>
        <Text variant="headlineSmall" style={styles.title}>
          Tasks Screen
        </Text>
        <Text variant="bodyLarge" style={styles.description}>
          This is a placeholder for the Tasks screen. The Tag Manager can be accessed via the tag
          icon in the header.
        </Text>

        <View style={styles.buttons}>
          <Button mode="outlined" onPress={openTagManager} icon="tag">
            Open Tag Manager
          </Button>

          {onNavigate && (
            <>
              <Button mode="outlined" onPress={() => onNavigate('SavedViews')} icon="bookmark">
                Go to Saved Views
              </Button>
              <Button mode="outlined" onPress={() => onNavigate('Home')} icon="home">
                Back to Home
              </Button>
            </>
          )}
        </View>
      </View>

      <TagManager visible={tagManagerVisible} onDismiss={closeTagManager} />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  content: {
    flex: 1,
    padding: 20,
  },
  title: {
    textAlign: 'center',
    marginBottom: 16,
  },
  description: {
    textAlign: 'center',
    marginBottom: 24,
    lineHeight: 22,
  },
  buttons: {
    gap: 12,
  },
});

export default TasksScreen;
