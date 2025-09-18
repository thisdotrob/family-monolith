import { NavigationContainer } from '@react-navigation/native';
import AppNavigator from './navigation/AppNavigator';

const HomePage = () => {
  return (
    <NavigationContainer>
      <AppNavigator />
    </NavigationContainer>
  );
};

export default HomePage;