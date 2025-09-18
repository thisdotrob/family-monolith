# Champs Tracker - Technical Specification

## Project Overview

**Project Name:** Champs Tracker  
**Purpose:** React Native mobile application for tracking various aspects of a pet cat's (Champagne) daily activities and health  
**Repository Location:** `apps/mobile/champs-tracker/`  
**Target Platform:** React Native (iOS/Android)  

## Architecture Overview

### Technology Stack
- **Frontend:** React Native with Expo 53
- **Navigation:** React Navigation
- **State Management:** Apollo Client with GraphQL
- **Authentication:** Existing JWT-based auth system
- **Backend:** Existing Rust GraphQL API (extend with new mutations/queries)
- **Database:** SQLite via sqlx (extend existing schema)
- **Image Handling:** React Native image picker with backend upload

### Integration Points
- Extends existing monorepo structure following established patterns
- Uses shared authentication context from `shared/contexts/AuthContext.tsx`
- Leverages existing Apollo Client setup from `shared/apollo/createApolloClient.ts`
- Integrates with existing GraphQL schema in `server/src/graphql/`

## Functional Requirements

### Core Features

#### 1. Home Screen
- **Description:** Simple dashboard with activity type selection
- **UI Elements:**
  - Grid of icon buttons for each activity type
  - Visual icons with emojis (e.g., 💩 for bathroom, 🍽️ for eating)
  - Clean, intuitive layout optimized for quick access
- **Navigation:** Gateway to all feature screens

#### 2. Bathroom Habit Tracking
- **Data Fields:**
  - `timestamp` (DateTime, required)
  - `consistency` (String, optional)
  - `observations` (Text, optional)
  - `litter_changed` (Boolean, required)
  - `photo` (Image upload, optional)
- **User Flow:** Tap bathroom icon → Fill form → Submit
- **Validation:** Minimal - timestamp required, all others optional

#### 3. Eating Habit Tracking
- **Data Fields:**
  - `timestamp` (DateTime, required)
  - `quantity_eaten` (String, required)
  - `leftovers_thrown_away` (String, optional)
  - `food_type` (String, required)
- **User Flow:** Tap eating icon → Fill form → Submit
- **Special Considerations:** Support for multiple food types/brands

#### 4. Outdoor Time Tracking
- **Data Fields:**
  - `start_timestamp` (DateTime, required)
  - `duration` (Integer minutes, required)
  - `activity_type` (String enum: "pram", "walking", "free_roam", etc.)
  - `behavior` (Text, optional)
  - `location` (String, optional)
  - `photo` (Image upload, optional)
- **User Flow:** Log after activity completion
- **Input Method:** Manual entry (not real-time tracking)

#### 5. Vet Visit Tracking
- **Data Fields:**
  - `date_time` (DateTime, required)
  - `reason` (String, required)
  - `weight` (Float with unit, optional)
  - `treatments_procedures` (Text, optional)
  - `cost` (Decimal with currency, optional)
  - `notes` (Text, optional)
- **User Flow:** Comprehensive form for medical record keeping
- **Data Retention:** Long-term medical history preservation

#### 6. Medication Tracking
- **Data Fields:**
  - `name` (String, required)
  - `dosage` (String, required)
  - `start_date` (Date, required)
  - `end_date` (Date, optional)
  - `reason` (String, required)
  - `notes` (Text, optional - for side effects, etc.)
- **Dose Logging:**
  - `dose_timestamp` (DateTime, required)
  - `medication_id` (Foreign key, required)
  - `notes` (Text, optional)
- **User Flow:** Create medication → Log individual doses → Track completion

#### 7. Play Time Tracking
- **Data Fields:**
  - `start_time` (DateTime, required)
  - `duration` (Integer minutes, required)
  - `play_type` (String, required)
  - `participants` (String, required - who was involved)
  - `location` (String, optional)
  - `photo` (Image upload, optional)
  - `notes` (Text, optional)
- **User Flow:** Post-activity logging with rich detail capture

#### 8. Daily Highlights
- **Data Fields:**
  - `timestamp` (DateTime, required)
  - `content` (Text, optional)
  - `photo` (Image upload, optional)
  - `tags` (Array of strings: "funny", "milestone", "concerning", etc.)
- **User Flow:** Quick capture of notable moments
- **Content Types:** Text, photo, or both

#### 9. Photo Gallery
- **Features:**
  - Central view of all uploaded photos
  - Organized by date and category
  - Tap photo to view originating entry
  - Filter by activity type
  - Full-screen image viewing
- **Integration:** Links back to source entries bidirectionally

## Technical Requirements

### Database Schema Extensions

```sql
-- Bathroom activities
CREATE TABLE bathroom_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    consistency TEXT,
    observations TEXT,
    litter_changed BOOLEAN NOT NULL,
    photo_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Eating activities
CREATE TABLE eating_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    quantity_eaten TEXT NOT NULL,
    leftovers_thrown_away TEXT,
    food_type TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Outdoor activities
CREATE TABLE outdoor_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    start_timestamp DATETIME NOT NULL,
    duration_minutes INTEGER NOT NULL,
    activity_type TEXT NOT NULL,
    behavior TEXT,
    location TEXT,
    photo_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Vet visits
CREATE TABLE vet_visits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    date_time DATETIME NOT NULL,
    reason TEXT NOT NULL,
    weight_kg REAL,
    treatments_procedures TEXT,
    cost_amount REAL,
    cost_currency TEXT DEFAULT 'USD',
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Medications
CREATE TABLE medications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    dosage TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    reason TEXT NOT NULL,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Medication doses
CREATE TABLE medication_doses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    medication_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    dose_timestamp DATETIME NOT NULL,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (medication_id) REFERENCES medications(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Play activities
CREATE TABLE play_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    duration_minutes INTEGER NOT NULL,
    play_type TEXT NOT NULL,
    participants TEXT NOT NULL,
    location TEXT,
    photo_url TEXT,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Daily highlights
CREATE TABLE daily_highlights (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    content TEXT,
    photo_url TEXT,
    tags TEXT, -- JSON array of strings
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Photo metadata
CREATE TABLE cat_photos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    original_filename TEXT,
    activity_type TEXT NOT NULL,
    activity_id INTEGER NOT NULL,
    uploaded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    file_size_bytes INTEGER,
    mime_type TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### GraphQL Schema Extensions

```graphql
# Input Types
input BathroomActivityInput {
  timestamp: DateTime!
  consistency: String
  observations: String
  litterChanged: Boolean!
  photo: Upload
}

input EatingActivityInput {
  timestamp: DateTime!
  quantityEaten: String!
  leftoversThrowAway: String
  foodType: String!
}

input OutdoorActivityInput {
  startTimestamp: DateTime!
  durationMinutes: Int!
  activityType: String!
  behavior: String
  location: String
  photo: Upload
}

input VetVisitInput {
  dateTime: DateTime!
  reason: String!
  weightKg: Float
  treatmentsProcedures: String
  costAmount: Float
  costCurrency: String
  notes: String
}

input MedicationInput {
  name: String!
  dosage: String!
  startDate: Date!
  endDate: Date
  reason: String!
  notes: String
}

input MedicationDoseInput {
  medicationId: ID!
  doseTimestamp: DateTime!
  notes: String
}

input PlayActivityInput {
  startTime: DateTime!
  durationMinutes: Int!
  playType: String!
  participants: String!
  location: String
  photo: Upload
  notes: String
}

input DailyHighlightInput {
  timestamp: DateTime!
  content: String
  photo: Upload
  tags: [String!]!
}

# Object Types
type BathroomActivity {
  id: ID!
  userId: ID!
  user: User!
  timestamp: DateTime!
  consistency: String
  observations: String
  litterChanged: Boolean!
  photoUrl: String
  createdAt: DateTime!
}

type EatingActivity {
  id: ID!
  userId: ID!
  user: User!
  timestamp: DateTime!
  quantityEaten: String!
  leftoversThrowAway: String
  foodType: String!
  createdAt: DateTime!
}

type OutdoorActivity {
  id: ID!
  userId: ID!
  user: User!
  startTimestamp: DateTime!
  durationMinutes: Int!
  activityType: String!
  behavior: String
  location: String
  photoUrl: String
  createdAt: DateTime!
}

type VetVisit {
  id: ID!
  userId: ID!
  user: User!
  dateTime: DateTime!
  reason: String!
  weightKg: Float
  treatmentsProcedures: String
  costAmount: Float
  costCurrency: String
  notes: String
  createdAt: DateTime!
}

type Medication {
  id: ID!
  userId: ID!
  user: User!
  name: String!
  dosage: String!
  startDate: Date!
  endDate: Date
  reason: String!
  notes: String
  doses: [MedicationDose!]!
  createdAt: DateTime!
}

type MedicationDose {
  id: ID!
  medicationId: ID!
  medication: Medication!
  userId: ID!
  user: User!
  doseTimestamp: DateTime!
  notes: String
  createdAt: DateTime!
}

type PlayActivity {
  id: ID!
  userId: ID!
  user: User!
  startTime: DateTime!
  durationMinutes: Int!
  playType: String!
  participants: String!
  location: String
  photoUrl: String
  notes: String
  createdAt: DateTime!
}

type DailyHighlight {
  id: ID!
  userId: ID!
  user: User!
  timestamp: DateTime!
  content: String
  photoUrl: String
  tags: [String!]!
  createdAt: DateTime!
}

type CatPhoto {
  id: ID!
  userId: ID!
  user: User!
  filePath: String!
  originalFilename: String
  activityType: String!
  activityId: ID!
  uploadedAt: DateTime!
  fileSizeBytes: Int
  mimeType: String
}

# Mutations
extend type Mutation {
  createBathroomActivity(input: BathroomActivityInput!): BathroomActivity!
  createEatingActivity(input: EatingActivityInput!): EatingActivity!
  createOutdoorActivity(input: OutdoorActivityInput!): OutdoorActivity!
  createVetVisit(input: VetVisitInput!): VetVisit!
  createMedication(input: MedicationInput!): Medication!
  logMedicationDose(input: MedicationDoseInput!): MedicationDose!
  createPlayActivity(input: PlayActivityInput!): PlayActivity!
  createDailyHighlight(input: DailyHighlightInput!): DailyHighlight!
}

# Queries
extend type Query {
  bathroomActivities(limit: Int, offset: Int): [BathroomActivity!]!
  eatingActivities(limit: Int, offset: Int): [EatingActivity!]!
  outdoorActivities(limit: Int, offset: Int): [OutdoorActivity!]!
  vetVisits(limit: Int, offset: Int): [VetVisit!]!
  medications(activeOnly: Boolean): [Medication!]!
  medicationDoses(medicationId: ID, limit: Int, offset: Int): [MedicationDose!]!
  playActivities(limit: Int, offset: Int): [PlayActivity!]!
  dailyHighlights(limit: Int, offset: Int): [DailyHighlight!]!
  catPhotos(activityType: String, limit: Int, offset: Int): [CatPhoto!]!
}
```

### File Upload Handling

#### Backend Implementation
- Extend existing static file serving in `server/src/server/`
- Create `/uploads/cat-photos/` directory structure
- Generate unique filenames with timestamps
- Support common image formats (JPEG, PNG, HEIC)
- Implement file size limits (e.g., 10MB max)
- Return secure URLs for uploaded files

#### Frontend Implementation
- Use `expo-image-picker` for camera/gallery access
- Implement compression before upload to manage bandwidth
- Show upload progress indicators
- Handle upload failures gracefully
- Cache uploaded images locally for performance

## User Interface Requirements

### Design Principles
- **Simplicity:** Quick logging is the primary goal
- **Visual Clarity:** Clear icons and minimal text
- **Accessibility:** Support for screen readers and large text
- **Performance:** Fast loading and responsive interactions

### Screen Specifications

#### Home Screen
```
┌─────────────────────────────────┐
│ 🐱 Champs Tracker              │
├─────────────────────────────────┤
│  💩        🍽️        🌳        │
│Bathroom   Eating   Outdoor     │
│                                │
│  🏥        💊        🎾        │
│ Vet      Medicine   Play       │
│                                │
│  ⭐        📸                  │
│Highlights Photos              │
└─────────────────────────────────┘
```

#### Activity Entry Forms
- **Header:** Activity type with back navigation
- **Form Fields:** Clean, labeled inputs matching data requirements
- **Photo Upload:** Camera/gallery picker with preview
- **Submit Button:** Prominent, confirms successful entry
- **Validation:** Real-time feedback for required fields

#### Photo Gallery
- **Grid Layout:** Thumbnail grid with infinite scroll
- **Filter Options:** By activity type and date range
- **Detail View:** Full-screen with entry context link
- **Performance:** Lazy loading and image optimization

### Navigation Structure
```
Home Screen
├── Bathroom Entry Form
├── Eating Entry Form
├── Outdoor Entry Form
├── Vet Visit Form
├── Medication Management
│   ├── Create Medication
│   ├── Active Medications List
│   └── Log Dose Form
├── Play Activity Form
├── Daily Highlight Form
└── Photo Gallery
    └── Photo Detail View
```

## Error Handling Strategy

### Network & API Errors
- **GraphQL Errors:** Display user-friendly messages for validation failures
- **Network Timeout:** Retry mechanism with exponential backoff
- **Offline Support:** Queue mutations for when connection is restored
- **Authentication Errors:** Redirect to login with context preservation

### File Upload Errors
- **Size Limits:** Compress images automatically or show size warning
- **Format Errors:** Accept only supported image formats with clear messaging
- **Storage Errors:** Graceful degradation when upload fails but entry succeeds
- **Progress Indicators:** Show upload status and allow cancellation

### Form Validation
- **Required Fields:** Real-time validation with clear error states
- **Data Format:** Validate timestamps, numeric inputs, etc.
- **User Feedback:** Toast notifications for successful entries
- **Draft Saving:** Preserve form data if user navigates away

### Data Integrity
- **Concurrent Updates:** Handle multiple users editing simultaneously
- **Sync Conflicts:** Last-write-wins with user notification
- **Backup Strategy:** Regular database backups on server
- **Data Recovery:** Soft delete for accidental removals

## Performance Requirements

### Loading Time Targets
- **App Launch:** < 2 seconds to home screen
- **Form Submission:** < 1 second for successful entry
- **Photo Upload:** Progress indicator for uploads > 3 seconds
- **Gallery Loading:** Thumbnail grid loads in < 1 second

### Memory Management
- **Image Handling:** Automatic compression and cleanup
- **Data Caching:** Apollo Client cache optimization
- **Background Processing:** Efficient image processing
- **Memory Leaks:** Proper cleanup of event listeners and timers

### Offline Capabilities
- **Read Access:** Cache recent entries for offline viewing
- **Write Queue:** Store pending mutations for sync when online
- **Conflict Resolution:** Merge strategies for offline changes
- **Sync Indicators:** Clear status of data synchronization

## Security Requirements

### Authentication Integration
- **JWT Tokens:** Use existing token refresh mechanism
- **Authorization:** Verify user access to cat data
- **Session Management:** Handle token expiration gracefully
- **Multi-User:** Support family/caregiver access to same cat data

### Data Privacy
- **User Association:** All entries tied to creating user
- **Access Control:** Users can see all entries but know who created what
- **Data Encryption:** HTTPS for all API communications
- **Local Storage:** Secure storage for cached authentication tokens

### File Security
- **Upload Validation:** Verify file types and scan for malicious content
- **Access URLs:** Generate secure, time-limited URLs for photo access
- **Storage Isolation:** Separate uploaded files by user/activity context
- **Cleanup:** Remove orphaned files from failed uploads

## Testing Strategy

### Unit Testing
- **GraphQL Resolvers:** Test all mutations and queries with valid/invalid inputs
- **Database Operations:** Test CRUD operations and data integrity
- **Authentication:** Test JWT integration and user context
- **File Upload:** Test upload, validation, and cleanup processes

### Integration Testing
- **API Endpoints:** Test complete request/response cycles
- **Database Migrations:** Verify schema changes work correctly
- **Authentication Flow:** Test login/logout/token refresh
- **File Storage:** Test upload/retrieve/delete workflows

### Frontend Testing (React Native)
- **Component Testing:** Test all form components and navigation
- **User Interaction:** Test form submission, image picker, validation
- **Apollo Integration:** Test GraphQL queries/mutations with mock data
- **Navigation:** Test all screen transitions and back button behavior

### End-to-End Testing
- **User Workflows:** Test complete activity logging flows
- **Photo Upload:** Test camera/gallery integration and upload process
- **Offline/Online:** Test app behavior during connectivity changes
- **Multi-User:** Test concurrent access and data synchronization

### Performance Testing
- **Image Upload:** Test various file sizes and formats
- **Data Loading:** Test with large datasets (hundreds of entries)
- **Memory Usage:** Monitor for memory leaks during extended use
- **Network Conditions:** Test on slow/unreliable connections

### Device Testing
- **iOS Devices:** Test on multiple iOS versions and device sizes
- **Android Devices:** Test on various Android versions and manufacturers
- **Camera Integration:** Test camera permissions and functionality
- **Storage Permissions:** Test photo gallery access permissions

## Deployment Strategy

### Development Environment
- **Local Development:** Use existing `npx expo start` workflow
- **Backend Integration:** Connect to local Rust GraphQL server
- **Database Setup:** Extend existing SQLite schema with migrations
- **Hot Reload:** Leverage Expo development client for rapid iteration

### Build Process
- **App Configuration:** Update `mobileapp/src/selectMobileApp.ts` to include `champs-tracker`
- **Environment Variables:** Configure API endpoints for different environments
- **Asset Optimization:** Optimize app icons, splash screens, and bundled assets
- **Build Commands:** Extend existing npm scripts for new app builds

### Distribution
- **Android APK:** Follow existing local distribution pattern in `BUILD_LOCAL_DISTRIBUTION.md`
- **iOS Sideloading:** Use Xcode for family device installation
- **App Store:** Future consideration for broader distribution
- **Version Management:** Semantic versioning aligned with monorepo practices

## Dependencies and Libraries

### New Dependencies Required
```json
{
  "expo-image-picker": "^15.x.x",
  "expo-media-library": "^16.x.x", 
  "@react-native-async-storage/async-storage": "^1.x.x",
  "react-native-toast-message": "^2.x.x",
  "react-native-super-grid": "^5.x.x"
}
```

### Backend Dependencies
```toml
[dependencies]
# Add to existing Cargo.toml
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
image = "0.24"
```

## Development Timeline

### Phase 1: Backend Foundation (Week 1)
- Database schema creation and migrations
- GraphQL schema extensions
- Basic CRUD mutations and queries
- File upload infrastructure
- Authentication integration testing

### Phase 2: Core Mobile App (Week 2)
- App structure and navigation setup
- Home screen with activity buttons
- Basic form components for each activity type
- Apollo Client integration
- Authentication flow integration

### Phase 3: Feature Implementation (Week 3-4)
- Complete all activity entry forms
- Image picker and upload functionality
- Form validation and error handling
- Data persistence and offline queuing
- Photo gallery implementation

### Phase 4: Polish and Testing (Week 5)
- UI/UX refinements
- Performance optimization
- Comprehensive testing
- Documentation completion
- Deployment preparation

### Phase 5: Deployment (Week 6)
- Build configuration and optimization
- Local distribution setup
- User acceptance testing
- Production deployment
- Monitoring and maintenance setup

## Maintenance and Future Enhancements

### Monitoring Requirements
- **Error Tracking:** Implement crash reporting and error monitoring
- **Performance Metrics:** Monitor app startup time and API response times
- **Usage Analytics:** Track feature usage to guide future development
- **Health Checks:** Monitor backend API availability and database performance

### Potential Future Features
- **Data Visualization:** Charts and trends for cat health insights
- **Reminders:** Notifications for medication doses and vet appointments
- **Export Capabilities:** PDF reports for vet visits
- **Multiple Cats:** Support for tracking multiple pets
- **Web Interface:** Companion web app for desktop access
- **Advanced Analytics:** AI-powered health insights and pattern recognition

### Scalability Considerations
- **Database Optimization:** Indexing strategies for performance at scale
- **File Storage:** CDN integration for photo delivery
- **API Rate Limiting:** Protect against abuse and ensure fair usage
- **Caching Strategy:** Redis integration for improved response times
- **Horizontal Scaling:** Database partitioning and read replicas

## Success Criteria

### Functional Success
- ✅ All seven activity types can be logged successfully
- ✅ Photos upload and display correctly in gallery
- ✅ Multi-user access works with proper attribution
- ✅ Offline functionality queues entries for later sync
- ✅ Authentication integrates seamlessly with existing system

### Performance Success
- ✅ App launches in under 2 seconds
- ✅ Form submissions complete in under 1 second
- ✅ Photo uploads show progress and complete reliably
- ✅ Gallery loads quickly with smooth scrolling
- ✅ No memory leaks during extended usage

### User Experience Success
- ✅ Home screen provides intuitive access to all features
- ✅ Forms are simple and quick to complete
- ✅ Error messages are clear and actionable
- ✅ App works reliably without requiring technical knowledge
- ✅ Data persists correctly across app restarts and device changes

---

## Getting Started

1. **Review existing monorepo structure** in `apps/mobile/placeholder/` for patterns
2. **Set up development environment** following existing mobile app setup
3. **Create database migrations** for new tables in `server/migrations/`
4. **Implement GraphQL schema** extensions in `server/src/graphql/`
5. **Create new mobile app** structure in `apps/mobile/champs-tracker/`
6. **Follow existing patterns** for authentication, Apollo Client, and navigation

This specification provides a complete roadmap for implementing the Champs Tracker app within the existing monorepo architecture while maintaining consistency with established patterns and practices.