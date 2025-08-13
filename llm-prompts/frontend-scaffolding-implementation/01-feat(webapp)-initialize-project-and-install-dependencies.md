You are an expert software engineer. Your task is to implement the first step in building our Family Monolith web application.

**Commit Title:** `feat(webapp): initialize project and install dependencies`

## 1. Project Initialization

First, create a new Vite project named `webapp` using the React and TypeScript template.

```bash
npm create vite@latest webapp -- --template react-ts
```

Navigate into the new directory:

```bash
cd webapp
```

## 2. Install Dependencies

Install the necessary dependencies for styling with Tailwind CSS and for making GraphQL requests with Apollo Client.

```bash
npm install tailwindcss postcss autoprefixer
npx tailwindcss init -p
npm install @apollo/client graphql
```

## 3. Configure Tailwind CSS

Configure Tailwind CSS by updating `tailwind.config.js` and `src/index.css`.

**`tailwind.config.js`:**
Replace the empty `content` array with this:
```javascript
content: [
  "./index.html",
  "./src/**/*.{js,ts,jsx,tsx}",
],
```

**`src/index.css`:**
Replace the entire file content with the Tailwind CSS directives:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

## 4. Clean Up Boilerplate

Remove the boilerplate code from the default Vite template to create a clean slate.

**`src/App.tsx`:**
Replace the entire file content with a simple, empty component:
```tsx
function App() {
  return (
    <div>
      {/* App content will go here */}
    </div>
  )
}

export default App
```

**`src/App.css`:**
Delete this file.

**`src/assets/react.svg`:**
Delete this file.

## 5. Verification

After completing these steps, run the development server:

```bash
npm run dev
```

Open your browser to the specified local address. You should see a completely blank white page. Verify that there are no errors in the browser's developer console. This confirms that the project is set up correctly and all dependencies are in place.
