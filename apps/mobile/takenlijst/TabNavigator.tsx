import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { NavigationContainer } from '@react-navigation/native';
import { useTheme } from 'react-native-paper';
import { MaterialCommunityIcons } from '@expo/vector-icons';

import ProjectsScreen from './screens/ProjectsScreen';
import TasksScreen from './screens/TasksScreen';
import SavedViewsScreen from './screens/SavedViewsScreen';
import HistoryScreen from './screens/HistoryScreen';

const Tab = createBottomTabNavigator();

const TabNavigator = () => {
  const theme = useTheme();

  return (
    <NavigationContainer>
      <Tab.Navigator
        screenOptions={({ route }: { route: any }) => ({
          tabBarIcon: ({ focused, color, size }: { focused: boolean; color: string; size: number }) => {
            let iconName: keyof typeof MaterialCommunityIcons.glyphMap;

            switch (route.name) {
              case 'Projects':
                iconName = focused ? 'folder' : 'folder-outline';
                break;
              case 'Tasks':
                iconName = focused ? 'format-list-checkbox' : 'format-list-checkbox';
                break;
              case 'SavedViews':
                iconName = focused ? 'eye' : 'eye-outline';
                break;
              case 'History':
                iconName = focused ? 'history' : 'history';
                break;
              default:
                iconName = 'circle';
            }

            return <MaterialCommunityIcons name={iconName} size={size} color={color} />;
          },
          tabBarActiveTintColor: theme.colors.primary,
          tabBarInactiveTintColor: theme.colors.onSurfaceVariant,
          tabBarStyle: {
            backgroundColor: theme.colors.surface,
            borderTopColor: theme.colors.outline,
          },
          headerStyle: {
            backgroundColor: theme.colors.surface,
          },
          headerTintColor: theme.colors.onSurface,
          headerTitleStyle: {
            fontWeight: 'bold',
          },
        })}
        initialRouteName="Projects"
      >
        <Tab.Screen 
          name="Projects" 
          component={ProjectsScreen}
          options={{
            title: 'Projects',
            headerTitle: 'Family Takenlijst - Projects',
          }}
        />
        <Tab.Screen 
          name="Tasks" 
          component={TasksScreen}
          options={{
            title: 'Tasks',
            headerTitle: 'Family Takenlijst - Tasks',
          }}
        />
        <Tab.Screen 
          name="SavedViews" 
          component={SavedViewsScreen}
          options={{
            title: 'Saved Views',
            headerTitle: 'Family Takenlijst - Saved Views',
          }}
        />
        <Tab.Screen 
          name="History" 
          component={HistoryScreen}
          options={{
            title: 'History',
            headerTitle: 'Family Takenlijst - History',
          }}
        />
      </Tab.Navigator>
    </NavigationContainer>
  );
};

export default TabNavigator;