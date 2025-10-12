import React, { useState } from 'react';
import { View, StyleSheet, ScrollView } from 'react-native';
import {
  Text,
  List,
  IconButton,
  TextInput,
  Button,
  Dialog,
  Portal,
  ActivityIndicator,
  Snackbar,
} from 'react-native-paper';
import { useQuery, useMutation } from '@apollo/client';
import { TAGS_QUERY } from '@shared/graphql/queries';
import {
  CREATE_TAG_MUTATION,
  RENAME_TAG_MUTATION,
  DELETE_TAG_MUTATION,
} from '@shared/graphql/mutations';

interface Tag {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
}

interface TagManagerProps {
  visible: boolean;
  onDismiss: () => void;
}

// Normalize tag name (trim, collapse spaces, strip leading #)
const normalizeTagName = (name: string): string => {
  return name
    .trim()
    .replace(/\s+/g, ' ') // collapse multiple spaces
    .replace(/^#+/, ''); // strip leading #
};

const TagManager: React.FC<TagManagerProps> = ({ visible, onDismiss }) => {
  const [newTagName, setNewTagName] = useState('');
  const [editingTag, setEditingTag] = useState<Tag | null>(null);
  const [editTagName, setEditTagName] = useState('');
  const [deleteConfirmTag, setDeleteConfirmTag] = useState<Tag | null>(null);
  const [snackbarMessage, setSnackbarMessage] = useState('');
  const [snackbarVisible, setSnackbarVisible] = useState(false);

  const { data, loading, error, refetch } = useQuery(TAGS_QUERY, {
    variables: { offset: 0, limit: 200 },
    skip: !visible,
  });

  const [createTag, { loading: createLoading }] = useMutation(CREATE_TAG_MUTATION, {
    onCompleted: () => {
      setNewTagName('');
      refetch();
      showSnackbar('Tag created successfully');
    },
    onError: (error) => {
      showSnackbar(`Error creating tag: ${error.message}`);
    },
  });

  const [renameTag, { loading: renameLoading }] = useMutation(RENAME_TAG_MUTATION, {
    onCompleted: () => {
      setEditingTag(null);
      setEditTagName('');
      refetch();
      showSnackbar('Tag renamed successfully');
    },
    onError: (error) => {
      showSnackbar(`Error renaming tag: ${error.message}`);
    },
  });

  const [deleteTag, { loading: deleteLoading }] = useMutation(DELETE_TAG_MUTATION, {
    onCompleted: () => {
      setDeleteConfirmTag(null);
      refetch();
      showSnackbar('Tag deleted successfully');
    },
    onError: (error) => {
      showSnackbar(`Error deleting tag: ${error.message}`);
    },
  });

  const showSnackbar = (message: string) => {
    setSnackbarMessage(message);
    setSnackbarVisible(true);
  };

  const handleCreateTag = () => {
    const normalized = normalizeTagName(newTagName);
    if (!normalized) {
      showSnackbar('Tag name cannot be empty');
      return;
    }
    createTag({ variables: { name: normalized } });
  };

  const handleEditTag = (tag: Tag) => {
    setEditingTag(tag);
    setEditTagName(tag.name);
  };

  const handleRenameTag = () => {
    if (!editingTag) return;
    const normalized = normalizeTagName(editTagName);
    if (!normalized) {
      showSnackbar('Tag name cannot be empty');
      return;
    }
    renameTag({ variables: { tagId: editingTag.id, newName: normalized } });
  };

  const handleDeleteTag = (tag: Tag) => {
    setDeleteConfirmTag(tag);
  };

  const confirmDeleteTag = () => {
    if (!deleteConfirmTag) return;
    deleteTag({ variables: { tagId: deleteConfirmTag.id } });
  };

  if (loading && !data) {
    return (
      <Portal>
        <Dialog visible={visible} onDismiss={onDismiss}>
          <Dialog.Content>
            <View style={styles.loadingContainer}>
              <ActivityIndicator animating={true} />
              <Text style={styles.loadingText}>Loading tags...</Text>
            </View>
          </Dialog.Content>
        </Dialog>
      </Portal>
    );
  }

  if (error) {
    return (
      <Portal>
        <Dialog visible={visible} onDismiss={onDismiss}>
          <Dialog.Title>Error</Dialog.Title>
          <Dialog.Content>
            <Text>Error loading tags: {error.message}</Text>
          </Dialog.Content>
          <Dialog.Actions>
            <Button onPress={onDismiss}>Close</Button>
          </Dialog.Actions>
        </Dialog>
      </Portal>
    );
  }

  const tags: Tag[] = data?.tags || [];

  return (
    <Portal>
      <Dialog visible={visible} onDismiss={onDismiss} style={styles.dialog}>
        <Dialog.Title>Tag Manager</Dialog.Title>
        <Dialog.Content>
          {/* Create new tag */}
          <View style={styles.createSection}>
            <Text variant="titleSmall" style={styles.sectionTitle}>
              Create New Tag
            </Text>
            <View style={styles.createRow}>
              <TextInput
                mode="outlined"
                label="Tag name"
                value={newTagName}
                onChangeText={setNewTagName}
                style={styles.createInput}
                onSubmitEditing={handleCreateTag}
                disabled={createLoading}
              />
              <Button
                mode="contained"
                onPress={handleCreateTag}
                disabled={createLoading || !newTagName.trim()}
                loading={createLoading}
                style={styles.createButton}
              >
                Add
              </Button>
            </View>
          </View>

          {/* Tags list */}
          <ScrollView style={styles.tagsList}>
            <Text variant="titleSmall" style={styles.sectionTitle}>
              Existing Tags ({tags.length})
            </Text>
            {tags.length === 0 ? (
              <Text style={styles.emptyText}>No tags yet. Create one above!</Text>
            ) : (
              tags.map((tag) => (
                <List.Item
                  key={tag.id}
                  title={tag.name}
                  left={(props) => <List.Icon {...props} icon="tag" />}
                  right={() => (
                    <View style={styles.tagActions}>
                      <IconButton
                        icon="pencil"
                        size={20}
                        onPress={() => handleEditTag(tag)}
                        disabled={renameLoading || deleteLoading}
                      />
                      <IconButton
                        icon="delete"
                        size={20}
                        onPress={() => handleDeleteTag(tag)}
                        disabled={renameLoading || deleteLoading}
                      />
                    </View>
                  )}
                  style={styles.tagItem}
                />
              ))
            )}
          </ScrollView>
        </Dialog.Content>
        <Dialog.Actions>
          <Button onPress={onDismiss}>Close</Button>
        </Dialog.Actions>
      </Dialog>

      {/* Edit tag dialog */}
      <Dialog visible={!!editingTag} onDismiss={() => setEditingTag(null)}>
        <Dialog.Title>Rename Tag</Dialog.Title>
        <Dialog.Content>
          <TextInput
            mode="outlined"
            label="Tag name"
            value={editTagName}
            onChangeText={setEditTagName}
            onSubmitEditing={handleRenameTag}
            disabled={renameLoading}
          />
        </Dialog.Content>
        <Dialog.Actions>
          <Button onPress={() => setEditingTag(null)} disabled={renameLoading}>
            Cancel
          </Button>
          <Button
            mode="contained"
            onPress={handleRenameTag}
            disabled={renameLoading || !editTagName.trim()}
            loading={renameLoading}
          >
            Rename
          </Button>
        </Dialog.Actions>
      </Dialog>

      {/* Delete confirmation dialog */}
      <Dialog visible={!!deleteConfirmTag} onDismiss={() => setDeleteConfirmTag(null)}>
        <Dialog.Title>Delete Tag</Dialog.Title>
        <Dialog.Content>
          <Text>
            Are you sure you want to delete the tag “{deleteConfirmTag?.name}”?
            {'\n\n'}
            Note: If this tag is being used by any tasks, deletion will be prevented.
          </Text>
        </Dialog.Content>
        <Dialog.Actions>
          <Button onPress={() => setDeleteConfirmTag(null)} disabled={deleteLoading}>
            Cancel
          </Button>
          <Button
            mode="contained"
            onPress={confirmDeleteTag}
            disabled={deleteLoading}
            loading={deleteLoading}
            buttonColor="#d32f2f"
          >
            Delete
          </Button>
        </Dialog.Actions>
      </Dialog>

      {/* Snackbar for feedback */}
      <Snackbar
        visible={snackbarVisible}
        onDismiss={() => setSnackbarVisible(false)}
        duration={3000}
      >
        {snackbarMessage}
      </Snackbar>
    </Portal>
  );
};

const styles = StyleSheet.create({
  dialog: {
    maxHeight: '80%',
  },
  loadingContainer: {
    alignItems: 'center',
    padding: 20,
  },
  loadingText: {
    marginTop: 10,
  },
  createSection: {
    marginBottom: 20,
  },
  sectionTitle: {
    marginBottom: 10,
    fontWeight: 'bold',
  },
  createRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 10,
  },
  createInput: {
    flex: 1,
  },
  createButton: {
    minWidth: 80,
  },
  tagsList: {
    maxHeight: 300,
  },
  emptyText: {
    textAlign: 'center',
    fontStyle: 'italic',
    padding: 20,
  },
  tagItem: {
    paddingVertical: 4,
  },
  tagActions: {
    flexDirection: 'row',
    alignItems: 'center',
  },
});

export default TagManager;
