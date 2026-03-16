# Process Flow Diagrams

## 1. User Authentication Flow

```mermaid
flowchart TD
    A[User Visits FileVault] -->|Unauthenticated| B[Redirect to Login Page]
    B --> C["User enters<br/>Username & Password"]
    C --> D{Form Validation<br/>Passed?}
    D -->|No| E["Show Error:<br/>Invalid Input"]
    E --> C
    D -->|Yes| F["Query: Find User<br/>by Username"]
    F --> G{User<br/>Exists?}
    G -->|No| H["Show Error:<br/>Invalid Credentials"]
    H --> C
    G -->|Yes| I["Compare Password<br/>with Hash"]
    I --> J{Password<br/>Match?}
    J -->|No| H
    J -->|Yes| K["Create Session<br/>Token"]
    K --> L["Set HTTP-Only<br/>Cookie"]
    L --> M["Redirect to<br/>Dashboard"]
    M --> N[Dashboard Loaded<br/>Authenticated]
    
    style A fill:#e1f5ff
    style N fill:#c8e6c9
    style E fill:#ffccbc
    style H fill:#ffccbc
```

## 2. File Upload Flow

```mermaid
flowchart TD
    A[User Initiates Upload] --> B["Select 1+ Files<br/>from Device"]
    B --> C["Drag & Drop<br/>or Click Upload"]
    C --> D{Multiple<br/>Files?}
    D -->|Yes| E["Queue Files<br/>for Upload"]
    D -->|No| F["Single File<br/>Processing"]
    E --> G["Validate Each<br/>File<br/>- Size ≤ 100MB<br/>- Check MIME"]
    F --> G
    G --> H{All Files<br/>Valid?}
    H -->|No| I["Show Error:<br/>File Too Large<br/>or Invalid"]
    I --> J[End]
    H -->|Yes| K["Show Upload<br/>Progress Bar"]
    K --> L["Upload Max 5<br/>Concurrent Files"]
    L --> M["API: POST /upload<br/>multipart/form-data"]
    M --> N["Server Validates<br/>User Auth<br/>Folder Exists<br/>Storage Quota"]
    N --> O{Server<br/>Validation<br/>Passed?}
    O -->|No| P["Return Error<br/>400/409/413"]
    P --> Q["Show Error<br/>to User"]
    Q --> J
    O -->|Yes| R["Save File to<br/>Disk<br/>/uploads/user_id/file_id/name"]
    R --> S["Create File<br/>Record in DB<br/>INSERT INTO files"]
    S --> T{DB Insert<br/>Success?}
    T -->|No| U["Rollback:<br/>Delete from Disk"]
    U --> P
    T -->|Yes| V["Update<br/>storage_used_bytes"]
    V --> W["Return Success<br/>201 Created"]
    W --> X["Update UI:<br/>Show New File<br/>in List"]
    X --> Y[Upload Complete]
    
    style A fill:#e1f5ff
    style Y fill:#c8e6c9
    style I fill:#ffccbc
    style Q fill:#ffccbc
    style P fill:#ffccbc
```

## 3. File Management Operations

```mermaid
flowchart TD
    A["User on Dashboard<br/>Viewing Files & Folders"] --> B{User Action?}
    
    B -->|Download| C["Click Download Icon"]
    C --> D["API: GET /files/id/download"]
    D --> E["Stream File Content<br/>from Disk"]
    E --> F["Set Content-Type<br/>Header"]
    F --> G["Set Content-Disposition<br/>Attachment"]
    G --> H["Browser Downloads<br/>File"]
    
    B -->|Delete File| I["Click Delete Icon"]
    I --> J["Show Confirmation<br/>Dialog"]
    J --> K{Confirm<br/>Delete?}
    K -->|Cancel| L[End]
    K -->|Confirm| M["API: DELETE /files/id"]
    M --> N["Update files.deleted_at<br/>timestamp"]
    N --> O["Update storage_used_bytes"]
    O --> P["Return 204 No Content"]
    P --> Q["Refresh File List<br/>Remove from UI"]
    Q --> L
    
    B -->|Create Folder| R["Click New Folder"]
    R --> S["Show Modal Dialog"]
    S --> T["User Enters<br/>Folder Name"]
    T --> U{"Name<br/>Valid?"}
    U -->|No| V["Show Error:<br/>Name Required"]
    V --> T
    U -->|Yes| W["API: POST /folders"]
    W --> X["Validate Parent<br/>Folder Exists<br/>User Auth"]
    X --> Y["INSERT INTO folders"]
    Y --> Z["Return 201 Created"]
    Z --> AA["Add Folder to UI<br/>Refresh List"]
    AA --> L
    
    B -->|Navigate| AB["Click on Folder"]
    AB --> AC["API: GET /folders/id"]
    AC --> AD["Query: SELECT Files<br/>WHERE folder_id = id"]
    AD --> AE["Query: SELECT Folders<br/>WHERE parent_id = id"]
    AE --> AF["Render Template<br/>Breadcrumb + List"]
    AF --> AG["Display Folder<br/>Contents"]
    AG --> L
    
    style A fill:#e1f5ff
    style L fill:#c8e6c9
    style V fill:#ffccbc
```

## 4. MVP User Journey

```mermaid
flowchart TD
    A["👤 New User<br/>Day 1"] --> B["Visit filevault.com"]
    B --> C["Land on Login<br/>Page"]
    C --> D{Has<br/>Account?}
    D -->|No| E["(Post-MVP:<br/>Signup)"]
    E --> F["Create Account"]
    D -->|Yes| G["Enter Credentials"]
    F --> H["Log In"]
    G --> H
    H --> I["✅ Log In Success<br/>Day 1"]
    I --> J["See Dashboard<br/>Empty Folder"]
    J --> K["📤 Upload 1st File<br/>Day 1"]
    K --> L["Select File<br/>from Device"]
    L --> M["File Uploaded"]
    M --> N["✅ File Visible<br/>in List<br/>Day 1"]
    N --> O["📁 Create Folder<br/>Day 2"]
    O --> P["Type Folder Name<br/>Documents"]
    P --> Q["Folder Created"]
    Q --> R["✅ Navigate<br/>into Folder<br/>Day 2"]
    R --> S["Folder Empty"]
    S --> T["📤 Upload to<br/>Folder<br/>Day 2"]
    T --> U["Upload More Files"]
    U --> V["✅ Folder<br/>Contains Files<br/>Day 2"]
    V --> W["📥 Download File<br/>Day 3"]
    W --> X["Click Download"]
    X --> Y["File Downloaded<br/>to Device"]
    Y --> Z["✅ File on<br/>Local Device<br/>Day 3"]
    Z --> AA["🗑️ Delete File<br/>Day 4"]
    AA --> AB["Confirm Delete"]
    AB --> AC["File Removed"]
    AC --> AD["✅ File Gone<br/>from List<br/>Day 4"]
    AD --> AE["MVP Complete ✓"]
    
    style A fill:#e3f2fd
    style I fill:#c8e6c9
    style N fill:#c8e6c9
    style R fill:#c8e6c9
    style V fill:#c8e6c9
    style Z fill:#c8e6c9
    style AD fill:#c8e6c9
    style AE fill:#a5d6a7
```

## 5. API Request/Response Cycle

```mermaid
sequenceDiagram
    participant User as User<br/>Browser
    participant Server as Axum<br/>Server
    participant DB as SQLite<br/>Database
    participant Disk as Disk<br/>Storage
    
    User->>Server: POST /upload<br/>(multipart)
    activate Server
    Server->>Server: Authenticate User<br/>Session Check
    Server->>DB: Verify Folder Exists
    activate DB
    DB-->>Server: Folder ID Valid
    deactivate DB
    Server->>Server: Validate File<br/>Size, MIME Type
    Server->>Disk: Save File to<br/>/uploads/user_id/...
    activate Disk
    Disk-->>Server: File Saved
    deactivate Disk
    Server->>DB: INSERT INTO files<br/>(id, user_id,<br/>folder_id, name,<br/>path, size, mime)
    activate DB
    DB-->>Server: File Record Created
    deactivate DB
    Server->>DB: UPDATE users<br/>SET storage_used_bytes += size
    activate DB
    DB-->>Server: Quota Updated
    deactivate DB
    Server-->>User: 201 Created<br/>{ file_id, name,<br/>size, timestamp }
    deactivate Server
    User->>User: Refresh UI<br/>Show New File<br/>in List

    User->>Server: GET /folders/123
    activate Server
    Server->>Server: Authenticate User
    Server->>DB: SELECT Files<br/>WHERE folder_id = 123<br/>AND deleted_at IS NULL
    activate DB
    DB-->>Server: [file1, file2, ...]
    deactivate DB
    Server->>DB: SELECT Folders<br/>WHERE parent_id = 123<br/>AND deleted_at IS NULL
    activate DB
    DB-->>Server: [folder1, folder2, ...]
    deactivate DB
    Server-->>User: 200 OK<br/>{ folders[], files[],<br/>breadcrumb[], quota }
    deactivate Server
    User->>User: Render HTML<br/>Dashboard Template

    User->>Server: DELETE /files/456
    activate Server
    Server->>Server: Authenticate User
    Server->>DB: UPDATE files<br/>SET deleted_at = NOW()<br/>WHERE id = 456
    activate DB
    DB-->>Server: 1 Row Updated
    deactivate DB
    Server->>DB: SELECT size FROM files<br/>WHERE id = 456
    activate DB
    DB-->>Server: size = 5000000
    deactivate DB
    Server->>DB: UPDATE users<br/>SET storage_used_bytes -= 5000000
    activate DB
    DB-->>Server: Quota Updated
    deactivate DB
    Server-->>User: 204 No Content
    deactivate Server
    User->>User: Remove File<br/>from UI
```

## 6. System Architecture Diagram

```mermaid
flowchart LR
    subgraph Client["🖥️ Client Layer"]
        Browser["Web Browser"]
        Session["Session Storage"]
    end
    
    subgraph Network["🌐 Network"]
        HTTPS["HTTPS TLS 1.3"]
    end
    
    subgraph App["🚀 Application Layer"]
        Router["Axum Router"]
        Auth["Auth Middleware"]
        Upload["Upload Handler"]
        Files["Files Handler"]
        Folders["Folders Handler"]
    end
    
    subgraph Data["💾 Data Layer"]
        DB["SQLite Database"]
        Cache["Query Cache"]
    end
    
    subgraph Storage["📦 Storage Layer"]
        Disk["/uploads Directory"]
    end
    
    Browser -->|HTTP Request<br/>JSON/FormData| HTTPS
    HTTPS --> Router
    Router --> Auth
    Auth --> Upload
    Auth --> Files
    Auth --> Folders
    
    Upload --> DB
    Files --> DB
    Folders --> DB
    
    DB --> Cache
    Cache --> Files
    Cache --> Folders
    
    Upload --> Disk
    Files --> Disk
    
    Router -->|HTTP Response<br/>JSON/HTML| HTTPS
    HTTPS -->|Render<br/>Display| Browser
    
    Session -->|Authenticate| HTTPS
    
    style Client fill:#e3f2fd
    style Network fill:#fff3e0
    style App fill:#f3e5f5
    style Data fill:#e8f5e9
    style Storage fill:#fce4ec
```

## 7. Folder Hierarchy Example

```mermaid
flowchart TD
    Root["📂 My Files<br/>user_id = abc123<br/>parent = NULL"]
    
    Projects["📁 Projects<br/>parent = abc123"]
    Documents["📁 Documents<br/>parent = abc123"]
    Archive["📁 Archive<br/>parent = abc123"]
    
    Q1["📁 Q1 2026<br/>parent = Projects"]
    Q2["📁 Q2 2026<br/>parent = Projects"]
    
    Budget["📄 budget.xlsx<br/>folder = Projects"]
    Plan["📄 roadmap.md<br/>folder = Q1"]
    Report["📄 report.pdf<br/>folder = Q1"]
    Resume["📄 resume.pdf<br/>folder = Documents"]
    
    Root --> Projects
    Root --> Documents
    Root --> Archive
    
    Projects --> Q1
    Projects --> Q2
    Projects --> Budget
    
    Q1 --> Plan
    Q1 --> Report
    
    Documents --> Resume
    
    style Root fill:#bbdefb
    style Projects fill:#c8e6c9
    style Q1 fill:#fff9c4
    style Plan fill:#f8bbd0
    style Report fill:#f8bbd0
```

## 8. MVP Development Timeline

```mermaid
gantt
    title FileVault MVP Development Timeline (1-2 weeks)
    dateFormat YYYY-MM-DD
    
    section Backend
    Database Setup           :db, 2026-03-16, 1d
    Auth (Session)          :auth, after db, 1d
    File Upload Handler     :upload, after auth, 2d
    File List Handler       :list, after auth, 1d
    Folder Handlers         :folders, after auth, 2d
    Download Handler        :download, after upload, 1d
    Integration Testing     :test, after download, 1d
    
    section Frontend
    Templates Setup         :tmpl, 2026-03-16, 1d
    Dashboard UI            :dash, after tmpl, 1d
    Upload Form             :form, after tmpl, 1d
    Responsive Design       :resp, after form, 2d
    File List Binding       :bind, after list, 1d
    Error Handling          :error, after bind, 1d
    
    section Deployment
    Local Testing           :local, after test, 1d
    Performance Testing     :perf, after local, 1d
    Bug Fixes               :bugs, after perf, 1d
    Launch MVP              :launch, after bugs, 0d
```

## Key Process Insights

### Upload Safety
- **Atomic Operations**: File written to disk AND database record created together
- **Rollback**: If DB fails, file is deleted from disk
- **Concurrency**: Max 5 uploads per user to prevent overload

### File Deletion
- **Soft Delete**: Mark `deleted_at` instead of hard delete (audit trail)
- **Quota Release**: Free storage immediately
- **Recovery**: Can restore deleted files within 90 days (Post-MVP)

### Folder Navigation
- **Breadcrumb**: Show path: "My Files > Projects > Q1 2026"
- **Parent-Child**: Self-referencing relationship supports unlimited nesting
- **Root Handling**: NULL parent_id represents root level

### Authentication
- **Session-Based**: HTTP-only cookies prevent XSS access
- **Token Expiry**: 30 days of inactivity
- **HTTPS Mandatory**: TLS 1.3 encrypts all traffic

### Performance Optimizations
- **Caching**: Cache folder listings and user quota
- **Pagination**: Show 50 items per page
- **Indexing**: Composite indexes on (user_id, parent/folder_id, deleted_at)
- **Async I/O**: Tokio handles concurrent uploads efficiently
