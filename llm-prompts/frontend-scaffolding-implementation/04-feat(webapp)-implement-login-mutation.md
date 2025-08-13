You are an expert software engineer. Your task is to implement the login functionality by connecting the UI to the GraphQL API using the `useMutation` hook.

**Commit Title:** `feat(webapp): implement login mutation`

## 1. Define the GraphQL Mutation

Create a new directory `src/graphql` and a file `src/graphql/mutations.ts` to store the `login` mutation.

**`src/graphql/mutations.ts`:**
```ts
import { gql } from '@apollo/client';

export const LOGIN_MUTATION = gql`
  mutation Login($username: String!, $password: String!) {
    login(input: { username: $username, password: $password }) {
      success
      token
      refreshToken
      errors
    }
  }
`;
```

## 2. Update the Login Page to Handle State and Mutations

Modify `src/pages/LoginPage.tsx` to make it a stateful, controlled component that uses the `useMutation` hook to perform the login.

**`src/pages/LoginPage.tsx`:**
```tsx
import React, { useState } from 'react';
import { useMutation } from '@apollo/client';
import { LOGIN_MUTATION } from '../graphql/mutations';

interface MessageState {
  text: string;
  type: 'success' | 'error' | '';
}

const LoginPage = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [message, setMessage] = useState<MessageState>({ text: '', type: '' });

  const [login, { loading }] = useMutation(LOGIN_MUTATION, {
    onCompleted: (data) => {
      if (data.login.success) {
        setMessage({ text: 'Login successful!', type: 'success' });
        // Token handling will be added in a future step
      } else {
        const errorMessage = data.login.errors?.join(', ') || 'An unknown error occurred.';
        setMessage({ text: `Login failed: ${errorMessage}`, type: 'error' });
      }
    },
    onError: (error) => {
      setMessage({ text: `An error occurred: ${error.message}`, type: 'error' });
    },
  });

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    setMessage({ text: '', type: '' });
    login({ variables: { username, password } });
  };

  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center">
      <div className="bg-white p-8 rounded-lg shadow-md w-full max-w-md">
        <h1 className="text-2xl font-bold mb-6 text-center">Login</h1>

        {message.text && (
          <div
            className={`mb-4 p-4 rounded ${
              message.type === 'success' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'
            }`}
          >
            {message.text}
          </div>
        )}

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="username">
              Username
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="username"
              type="text"
              placeholder="Username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
          </div>
          <div className="mb-6">
            <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="password">
              Password
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
              id="password"
              type="password"
              placeholder="******************"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
          </div>
          <div className="flex items-center justify-between">
            <button
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:bg-blue-300"
              type="submit"
              disabled={loading}
            >
              {loading ? 'Signing In...' : 'Sign In'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default LoginPage;
```

## 3. Verification

To verify this step, you will need to manually test the login functionality.

1.  Run the application (`npm run dev`).
2.  **Test Failure**: Enter incorrect credentials and click "Sign In". The form should display a red error message, like "Login failed: Invalid credentials".
3.  **Test Success**: Enter correct credentials (you may need to add a user to the database manually as per `AUTH.md`). Click "Sign In". The form should display a green success message: "Login successful!".
4.  **Test Loading State**: While the mutation is in progress, the "Sign In" button should be disabled and its text should change to "Signing In...". You can simulate a slow network connection in your browser's developer tools to see this more clearly.
