# Developer Guide for Multi-Platform Monorepo

This is a full-stack monorepo supporting multiple web apps, mobile apps, and a Rust GraphQL backend. The project enables building and deploying multiple apps from a single codebase.

## Project Overview

- **Purpose**: Multi-app platform with shared authentication and GraphQL API
- **Architecture**: Monorepo with web apps, mobile apps, shared components, and Rust backend
- **Languages**: TypeScript/React (frontend), Rust (backend)
- **Key Features**: Multi-app support, shared authentication, local distribution for mobile

## Key Technologies

### Frontend Stack
- **Web**: React 19, Vite, TailwindCSS v4, Apollo Client
- **Mobile**: React Native, Expo 53, React Navigation, Apollo Client
- **Shared**: Apollo Client, GraphQL, TypeScript
- **Testing**: Vitest (web), Jest (mobile)

### Backend Stack
- **Language**: Rust (2024 edition)
- **Framework**: Axum web server
- **Database**: SQLite with sqlx
- **GraphQL**: async-graphql
- **Authentication**: JWT tokens with refresh tokens

## Project Structure

### Root Level
- `package.json` - Root package with lint-staged and Husky
- `tsconfig.base.json` - Shared TypeScript configuration
- `.prettierrc.json` - Code formatting rules
- `.husky/pre-commit` - Pre-commit hooks for formatting

### Core Directories
- `webapp/` - Web application using Vite + React
- `mobileapp/` - Mobile application using Expo + React Native
- `server/` - Rust backend with GraphQL API
- `shared/` - Shared TypeScript code (Apollo client, GraphQL types)
- `apps/web/` - Individual web app modules
- `apps/mobile/` - Individual mobile app modules

## Multi-App System

### Web Apps
- Built with `VITE_APP_ID=<appId>` environment variable
- Apps located in `apps/web/<appId>/`
- Each app exports a default React component from `index.ts`
- Built to `webapp/dist/<appId>/` and served at `/:appId`
- Dynamic app selection via `webapp/src/appSelection.tsx`

### Mobile Apps  
- App selection hardcoded in `mobileapp/src/selectMobileApp.ts`
- Currently supports single app: `placeholder`
- Apps located in `apps/mobile/<appId>/`

## Development Workflows

### Starting Development
```bash
# Web app development
cd webapp
VITE_APP_ID=placeholder npm run dev

# Mobile app development  
cd mobileapp
npx expo start

# Backend development
cd server
cargo run --bin dev
```

### Building and Deployment

#### Web Apps
```bash
cd webapp
VITE_APP_ID=<appId> npm run build
# Deploys to webapp/dist/<appId>/
```

#### Mobile Apps (Android)
```bash
cd mobileapp
npm run build:android:placeholder
# Creates app-placeholder.apk for self-hosting
```

#### Backend
```bash
cd server
cargo build --release
```

## Code Standards and Conventions

### Formatting and Linting
- **Prettier**: Single quotes, trailing commas, 100 char width
- **Pre-commit hooks**: Format check for web, mobile, and Rust code
- **TypeScript**: Strict mode enabled with comprehensive type checking

### File Naming
- Use kebab-case for file names
- React components use PascalCase
- Temporary files for development: prefix with `tmp_rovodev_`

### Import Aliases
- `@shared` - Points to shared directory
- `@apps-web` - Points to apps/web directory  
- `@apps-mobile` - Points to apps/mobile directory

## Authentication System

### JWT Implementation
- Access tokens with expiration
- Refresh token rotation
- Apollo Client automatic token refresh
- Shared auth context between web and mobile

### Key Files
- `server/src/auth/` - Backend auth logic
- `shared/contexts/AuthContext.tsx` - Frontend auth context
- `shared/apollo/createApolloClient.ts` - Apollo setup with auth

## Database and Migrations

### SQLite Database
- Located at path defined in `server/src/config.rs`
- Migrations in `server/migrations/`
- Uses sqlx for compile-time checked queries

### Running Migrations
```bash
cd server
sqlx migrate run
```

## Deployment

### Web App Deployment
- Build per app with `VITE_APP_ID`
- Copy `dist/<appId>/` to server's `static/<appId>/`
- Served at `https://domain.com/<appId>`

### Mobile App Distribution
- **Android**: Local APK build + self-hosted download links
- **iOS**: Local build + Xcode sideloading to family devices
- See `mobileapp/BUILD_LOCAL_DISTRIBUTION.md` for details

### Backend Deployment
- SystemD service configuration in `server/deploy/`
- Docker support available
- Serves static files and GraphQL API

## Important Configuration Files

### Web App Config
- `webapp/vite.config.ts` - Vite configuration with multi-app support
- `webapp/tailwind.config.cjs` - TailwindCSS configuration

### Mobile App Config  
- `mobileapp/app.config.ts` - Expo configuration
- `mobileapp/eas.json` - EAS Build configuration

### Backend Config
- `server/Cargo.toml` - Rust dependencies
- `server/src/config.rs` - Application configuration

## Development Best Practices

### Code Organization
- Keep app-specific code in `apps/` directories
- Share common logic via `shared/` directory
- Use TypeScript strict mode for type safety

### Testing
- Web: Use Vitest for component and integration tests
- Mobile: Use Jest with React Native Testing Library
- Backend: Use Rust's built-in testing framework

### GraphQL Development
- Schema defined in `server/src/graphql/`
- Shared queries/mutations in `shared/graphql/`
- Use Apollo Client DevTools for debugging

### Multi-App Development
- Always set `VITE_APP_ID` when building web apps
- Create new apps by adding directories to `apps/web/` or `apps/mobile/`
- Update app selection logic when adding new mobile apps

## Troubleshooting

### Common Issues
- Missing `VITE_APP_ID`: Set environment variable before building
- Apollo Client errors: Check GraphQL endpoint and authentication
- Mobile build failures: Ensure Expo CLI and EAS CLI are updated

### Debug Tools
- Vite dev server with hot reload
- Expo development client
- Apollo Client DevTools
- Rust tracing for backend logging

## Documentation References
- `server/ARCHITECTURE.MD` - Backend architecture details
- `server/AUTH.md` - Authentication implementation
- `webapp/MULTI_APP.md` - Web multi-app system
- `mobileapp/BUILD_LOCAL_DISTRIBUTION.md` - Mobile deployment guide