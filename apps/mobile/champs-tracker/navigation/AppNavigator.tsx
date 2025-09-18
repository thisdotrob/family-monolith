import { createStackNavigator } from '@react-navigation/stack';
import HomeScreen from '../screens/HomeScreen';
import BathroomScreen from '../screens/BathroomScreen';
import EatingScreen from '../screens/EatingScreen';
import OutdoorScreen from '../screens/OutdoorScreen';
import VetScreen from '../screens/VetScreen';
import MedicationScreen from '../screens/MedicationScreen';
import PlayScreen from '../screens/PlayScreen';
import HighlightsScreen from '../screens/HighlightsScreen';
import type { RootStackParamList } from './types';

const Stack = createStackNavigator<RootStackParamList>();

const AppNavigator = () => {
  return (
    <Stack.Navigator
      initialRouteName="Home"
      screenOptions={{
        headerStyle: {
          backgroundColor: '#6200EE',
        },
        headerTintColor: '#fff',
        headerTitleStyle: {
          fontWeight: 'bold',
        },
      }}
    >
      <Stack.Screen 
        name="Home" 
        component={HomeScreen} 
        options={{ title: '🐱 Champs Tracker' }}
      />
      <Stack.Screen 
        name="Bathroom" 
        component={BathroomScreen} 
        options={{ title: '💩 Bathroom Activity' }}
      />
      <Stack.Screen 
        name="Eating" 
        component={EatingScreen} 
        options={{ title: '🍽️ Eating Activity' }}
      />
      <Stack.Screen 
        name="Outdoor" 
        component={OutdoorScreen} 
        options={{ title: '🌳 Outdoor Activity' }}
      />
      <Stack.Screen 
        name="Vet" 
        component={VetScreen} 
        options={{ title: '🏥 Vet Visit' }}
      />
      <Stack.Screen 
        name="Medication" 
        component={MedicationScreen} 
        options={{ title: '💊 Medication' }}
      />
      <Stack.Screen 
        name="Play" 
        component={PlayScreen} 
        options={{ title: '🎾 Play Activity' }}
      />
      <Stack.Screen 
        name="Highlights" 
        component={HighlightsScreen} 
        options={{ title: '⭐ Highlights' }}
      />
    </Stack.Navigator>
  );
};

export default AppNavigator;