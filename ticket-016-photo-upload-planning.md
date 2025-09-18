# Ticket 016: Photo Upload System Planning and Infrastructure

## Overview
Plan and design the photo upload system for the champs-tracker app. This ticket establishes the foundation for photo uploads across all activity types and creates a roadmap for future implementation.

## Acceptance Criteria
- [ ] Photo upload architecture documented and planned
- [ ] Backend infrastructure requirements identified
- [ ] Frontend integration points defined
- [ ] Database schema for photo metadata designed
- [ ] File storage strategy determined
- [ ] Security considerations documented
- [ ] Implementation roadmap created for future tickets

## Planning Areas

### 1. Backend Infrastructure Requirements

#### File Storage Strategy
- **Local File Storage**: Store uploaded photos in `server/static/uploads/cat-photos/`
- **File Naming**: Generate unique filenames using UUID + timestamp
- **Directory Structure**: Organize by year/month for scalability
- **File Types**: Support JPEG, PNG, HEIC formats
- **Size Limits**: Maximum 10MB per photo with automatic compression

#### Database Schema for Photo Metadata
```sql
CREATE TABLE cat_photos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    original_filename TEXT,
    activity_type TEXT NOT NULL, -- 'bathroom', 'eating', 'outdoor', etc.
    activity_id INTEGER NOT NULL, -- Foreign key to specific activity
    uploaded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    file_size_bytes INTEGER,
    mime_type TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

#### GraphQL Schema Extensions
```rust
#[derive(SimpleObject)]
pub struct CatPhoto {
    pub id: i32,
    pub user_id: i32,
    pub file_path: String,
    pub original_filename: Option<String>,
    pub activity_type: String,
    pub activity_id: i32,
    pub uploaded_at: DateTime<Utc>,
    pub file_size_bytes: Option<i32>,
    pub mime_type: Option<String>,
}

// Add photo_url fields to existing activity types
// Update input types to include Upload scalar for file uploads
```

### 2. Frontend Integration Points

#### Image Picker Integration
- **Library**: Use `expo-image-picker` for camera/gallery access
- **Permissions**: Handle camera and media library permissions
- **Compression**: Implement client-side compression before upload
- **Preview**: Show image preview before submission

#### Upload Process Flow
1. User selects/takes photo during activity entry
2. Image compressed and prepared for upload
3. Form submission includes both activity data and photo
4. Progress indicator during upload
5. Success confirmation with photo thumbnail

#### Photo Gallery Integration
- **Central Gallery**: Browse all photos across activity types
- **Activity Linking**: Tap photo to view originating activity entry
- **Filtering**: Filter by activity type and date range
- **Performance**: Lazy loading and thumbnail optimization

### 3. Security and Performance Considerations

#### Security Measures
- **File Validation**: Verify file types and reject malicious uploads
- **Access Control**: Ensure users can only access their own photos
- **Secure URLs**: Generate time-limited URLs for photo access
- **Storage Isolation**: Separate photos by user context

#### Performance Optimizations
- **Thumbnail Generation**: Create smaller thumbnails for gallery views
- **Caching**: Implement proper caching headers for photo delivery
- **Cleanup**: Remove orphaned files from failed uploads
- **Compression**: Automatic server-side compression if needed

### 4. Implementation Roadmap

#### Phase 1: Backend Photo Infrastructure (Future Ticket)
- Implement file upload handling in Rust backend
- Create photo metadata database table and migration
- Add GraphQL mutations for photo upload
- Implement secure file serving endpoints

#### Phase 2: Frontend Photo Capture (Future Ticket)
- Integrate expo-image-picker in activity forms
- Implement photo compression and upload
- Add progress indicators and error handling
- Update all activity forms to support optional photos

#### Phase 3: Photo Gallery Implementation (Future Ticket)
- Create central photo gallery screen
- Implement photo browsing and filtering
- Add photo-to-activity linking functionality
- Optimize for performance and user experience

#### Phase 4: Enhancement and Polish (Future Ticket)
- Add bulk photo operations
- Implement photo editing capabilities
- Add photo sharing functionality
- Performance monitoring and optimization

### 5. Technical Dependencies

#### New Backend Dependencies
```toml
# Add to server/Cargo.toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
uuid = { version = "1.0", features = ["v4"] }
image = "0.24"  # For image processing and compression
mime = "0.3"    # For MIME type detection
```

#### New Frontend Dependencies
```json
{
  "expo-image-picker": "^15.x.x",
  "expo-media-library": "^16.x.x",
  "react-native-image-resizer": "^3.x.x"
}
```

### 6. Data Flow Design

#### Upload Flow
```
User Activity Entry Form
    ↓ (optional photo selection)
Image Picker → Image Compression → Form Submission
    ↓
GraphQL Mutation (activity + photo)
    ↓
Backend Processing (save activity + upload photo)
    ↓
Database Storage (activity record + photo metadata)
    ↓ 
Response with photo URL
```

#### Gallery Flow
```
Photo Gallery Screen
    ↓
GraphQL Query (all user photos)
    ↓
Thumbnail Display
    ↓ (user taps photo)
Full Photo View + Activity Link
    ↓ (user taps activity link)
Navigate to Original Activity Entry
```

## Files to Create/Modify (Future Implementation)
- `server/migrations/YYYYMMDD_create_cat_photos.sql`
- `server/src/upload/` (new module for file handling)
- `server/src/graphql/champ_tracker.rs` (extend with photo fields)
- `shared/graphql/champ-tracker.ts` (add photo operations)
- `apps/mobile/champs-tracker/components/PhotoPicker.tsx`
- `apps/mobile/champs-tracker/components/PhotoGallery.tsx`
- `apps/mobile/champs-tracker/screens/PhotoGalleryScreen.tsx`

## Success Criteria for Future Implementation
- [ ] Photos upload successfully from all activity forms
- [ ] Photo gallery displays all uploaded photos
- [ ] Photos link back to their originating activities
- [ ] Upload progress and error handling works reliably
- [ ] Performance remains good with 100+ photos
- [ ] Security prevents unauthorized photo access

## Estimated Effort for Future Implementation
- **Phase 1 (Backend)**: Medium - File upload infrastructure
- **Phase 2 (Frontend Integration)**: Medium - Image picker integration
- **Phase 3 (Gallery)**: Small - Display and navigation
- **Phase 4 (Enhancements)**: Small - Polish and optimization

## Notes
This ticket serves as the planning foundation for photo upload functionality. The actual implementation will be tackled in future tickets after the core activity tracking features are complete and stable.

## Dependencies
- All activity tracking features should be implemented first (tickets 001-015)
- Backend infrastructure must be stable
- Mobile app navigation and forms should be working well

## Current Status
Planning complete - ready for future implementation when core features are stable.