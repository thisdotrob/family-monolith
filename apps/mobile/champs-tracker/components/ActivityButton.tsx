import React from 'react';
import { TouchableOpacity, View, Text, StyleSheet } from 'react-native';

interface ActivityButtonProps {
  icon: string;
  title: string;
  onPress: () => void;
  color?: string;
}

const ActivityButton: React.FC<ActivityButtonProps> = ({ 
  icon, 
  title, 
  onPress, 
  color = '#6200EE' 
}) => {
  return (
    <TouchableOpacity
      style={[styles.button, { borderColor: color }]}
      onPress={onPress}
      activeOpacity={0.7}
    >
      <View style={styles.content}>
        <Text style={styles.icon}>{icon}</Text>
        <Text style={styles.title}>{title}</Text>
      </View>
    </TouchableOpacity>
  );
};

const styles = StyleSheet.create({
  button: {
    width: 100,
    height: 100,
    borderRadius: 12,
    borderWidth: 2,
    backgroundColor: '#fff',
    margin: 8,
    justifyContent: 'center',
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 3.84,
    elevation: 5,
  },
  content: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  icon: {
    fontSize: 32,
    marginBottom: 4,
  },
  title: {
    fontSize: 12,
    fontWeight: '600',
    textAlign: 'center',
    color: '#333',
  },
});

export default ActivityButton;