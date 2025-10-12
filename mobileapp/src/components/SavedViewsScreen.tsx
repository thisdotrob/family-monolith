import React, { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { Text, Appbar, Button } from 'react-native-paper';
import TagManager from './TagManager';

interface SavedViewsScreenProps {
  onNavigate?: (screen: string) => void;
}

const SavedViewsScreen: React.FC<SavedViewsScreenProps> = ({ onNavigate }) => {
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
        <Appbar.Content title="Saved Views" />
        <Appbar.Action icon="tag" onPress={openTagManager} />
        <Appbar.Action
          icon="dots-vertical"
          onPress={() => console.log('More options menu (stub)')}
        />
      </Appbar.Header>

      <View style={styles.content}>
        <Text variant="headlineSmall" style={styles.title}>
          Saved Views Screen
        </Text>
        <Text variant="bodyLarge" style={styles.description}>
          This is a placeholder for the Saved Views screen. According to the spec, the Tag Manager
          should be accessible from the “header overflow” menu, but for convenience it’s also
          accessible via the tag icon.
        </Text>

        <View style={styles.buttons}>
          <Button mode="outlined" onPress={openTagManager} icon="tag">
            Open Tag Manager
          </Button>

          {onNavigate && (
            <>
              <Button
                mode="outlined"
                onPress={() => onNavigate('Tasks')}
                icon="format-list-checkbox"
              >
                Go to Tasks
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

export default SavedViewsScreen;
