import { AuthProvider } from '@shared/contexts/AuthContext';
import App from '../src/App';
import LocalStorage from '../src/LocalStorage';
import { LastSelectionProvider } from '../src/contexts/LastSelectionContext';

export default function RootLayout() {
  return (
    <AuthProvider storage={LocalStorage}>
      <LastSelectionProvider>
        <App />
      </LastSelectionProvider>
    </AuthProvider>
  );
}
