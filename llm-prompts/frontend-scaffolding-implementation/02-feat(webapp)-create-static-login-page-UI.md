You are an expert software engineer. Building on the previous step, your task is to create the static user interface for the login page of the web application.

**Commit Title:** `feat(webapp): create static login page UI`

## 1. Create the Login Page Component

Create a new directory `src/pages` and a new file `src/pages/LoginPage.tsx`. This component will contain the login form.

**`src/pages/LoginPage.tsx`:**
```tsx
import React from 'react';

const LoginPage = () => {
  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center">
      <div className="bg-white p-8 rounded-lg shadow-md w-full max-w-md">
        <h1 className="text-2xl font-bold mb-6 text-center">Login</h1>
        
        {/* This div will be used for success/failure messages */}
        <div className="mb-4"></div>

        <form>
          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="username">
              Username
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="username"
              type="text"
              placeholder="Username"
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
            />
          </div>
          <div className="flex items-center justify-between">
            <button
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="button"
            >
              Sign In
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default LoginPage;
```

## 2. Render the Login Page

Update `src/App.tsx` to render the newly created `LoginPage`.

**`src/App.tsx`:**
```tsx
import LoginPage from './pages/LoginPage';

function App() {
  return (
    <LoginPage />
  );
}

export default App;
```

## 3. Verification

Run the development server (`npm run dev`). The browser should now display a centered login form with "Username" and "Password" fields and a "Sign In" button. The styling should be applied correctly via Tailwind CSS.
