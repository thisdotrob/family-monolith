# Project Specification: Family Monolith Frontend Applications

## 1. Overview

This document outlines the requirements for creating two frontend applications: a React web application and a React Native mobile application. Both applications will serve as clients for the existing Family Monolith backend, providing a user interface for family members to log in.

The initial phase of development will focus on creating a single login page for each application.

## 2. Core Technologies

| Area                  | Web Application                               | Mobile Application         |
| --------------------- | --------------------------------------------- | -------------------------- |
| **Framework**         | React                                         | React Native (with Expo)   |
| **Language**          | TypeScript                                    | TypeScript                 |
| **Build Tool**        | Vite                                          | Expo CLI                   |
| **UI/Styling**        | Tailwind CSS                                  | React Native Paper         |
| **API Client**        | Apollo Client                                 | Apollo Client              |
| **State Management**  | React Context API (`AuthContext`)             | React Context API (`AuthContext`) |
| **Token Storage**     | `localStorage`                                | `AsyncStorage`             |

## 3. Backend API

-   **Endpoint (Web)**: `/graphql` (relative path)
-   **Endpoint (Mobile)**: `https://blobfishapp.duckdns.org/graphql` (absolute path)
-   **Authentication**: See `AUTH.md` for details on the JWT and refresh token flow.
-   **Key Mutations**:
    -   `login(input: { username, password })`
    -   `refreshToken(input: { refreshToken })`
    -   `logout(input: { refreshToken })`

## 4. Web Application (`webapp`)

### 4.1. Project Setup

-   A new Vite project will be created in the `monolith-frontend/webapp` directory.
-   The project will be initialized using the `react-ts` template.

### 4.2. Login Page

-   **URL**: The root of the application (`/`).
-   **Components**:
    -   A text input for `username`.
    -   A text input for `password` (type `password`).
    -   A "Login" button.
-   **Behavior**:
    1.  The user enters their username and password and clicks "Login".
    2.  The application sends a `login` mutation to the GraphQL API.
    3.  **On Success**:
        -   A success message "Login successful!" is displayed in green text above the login form.
        -   The received `token` and `refreshToken` are stored in `localStorage`.
    4.  **On Failure**:
        -   An error message "Login failed. Please check your username and password." is displayed in red text above the login form.

### 4.3. Authentication Handling

-   An `AuthContext` will be created to manage authentication state (`token`, user info) throughout the application.
-   An Apollo Client instance will be configured to handle all GraphQL communication.
-   An Apollo Link will be implemented to automatically add the `Authorization: Bearer <token>` header to all authenticated requests.

### 4.4. Token Refresh Logic

-   An Apollo Link will be created to handle token expiration.
-   If an API request fails with an authentication error, the client will automatically use the stored `refreshToken` to call the `refreshToken` mutation.
-   While the token refresh is in progress, the UI will be blocked by a loading indicator, and a message "Refreshing session..." will be displayed.
-   If the refresh is successful, the new tokens will be stored, and the original failed request will be retried automatically.
-   If the refresh fails, the user will be logged out, and all stored tokens will be cleared.

## 5. Mobile Application (`mobileapp`)

### 5.1. Project Setup

-   A new Expo (React Native) project will be created in a `monolith-frontend/mobileapp` directory.

### 5.2. Login Page

-   The UI and behavior will mirror the web application exactly, using components from React Native Paper for a Material Design look and feel.
-   **Components**:
    -   A `TextInput` for `username`.
    -   A `TextInput` for `password` (secure text entry).
    -   A `Button` for "Login".
-   **Behavior**:
    -   Identical to the web application's login flow.
    -   Success/failure messages will be displayed as described for the web app.

### 5.3. Authentication Handling

-   An `AuthContext` will be used, similar to the web app.
-   The `token` and `refreshToken` will be stored using `AsyncStorage`.
-   An Apollo Client instance will be configured for all API communication.

### 5.4. Token Refresh Logic

-   The token refresh implementation will be identical to the web application's logic, ensuring a consistent and robust authentication experience.

## 6. Deployment Documentation

A `DEPLOYMENT.md` file will be created with instructions for both applications.

### 6.1. Web Application Deployment

-   **Bundling**: Instructions on how to run `npm run build` (or the equivalent Vite command) to generate the production assets.
-   **Serving**: A clear note explaining that the contents of the generated `dist/` directory must be manually copied into the backend server's `static/` directory to be served.

### 6.2. Mobile Application Deployment (TestFlight)

-   **Prerequisites**: List necessary accounts (Apple Developer Program) and tools (Expo CLI, Transporter).
-   **Configuration**: Steps to configure the `app.json` file with the correct `bundleIdentifier` for iOS.
-   **Building**: How to run `eas build --platform ios` to create a build on Expo's servers.
-   **Submission**: A step-by-step guide on how to download the build artifact and upload it to App Store Connect using the Transporter app.
-   **TestFlight**: Instructions on how to add internal testers (family members) in App Store Connect to grant them access to the build.

## 7. Testing Plan

-   **Unit Tests**:
    -   Use Jest and React Testing Library.
    -   Test individual components (e.g., rendering of the login form).
    -   Test utility functions (e.g., token storage/retrieval).
-   **Integration Tests**:
    -   Test the complete login flow.
    -   Mock the Apollo Client to simulate:
        -   Successful login.
        -   Failed login (invalid credentials).
        -   API error during login.
        -   Automatic token refresh flow.
