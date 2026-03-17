# FileVault: Agent Customization Instructions

## Project Overview

**FileVault** is a cloud-based file storage and synchronization platform built with Rust. It provides users with a simple, responsive web interface for managing personal files with folder-based organization, inspired by Google Drive and OneDrive.

- **Framework**: Axum (Rust web framework)
- **Database**: SQLite with SQLx migrations
- **Templating**: Askama
- **Architecture**: Feature-based module organization
- **Target**: MVP (upload, organize, download files)

## Technology Stack

```
Core Dependencies:
- axum 0.8.8 - Web framework with macros
- axum-extra - Cookie handling (private & signed)
- sqlx 0.8.6 - SQLite with async runtime (tokio-rustls)
- askama 0.15.4 - Template engine
- tokio 1.50.0 - Async runtime
- uuid, chrono, serde, anyhow
```

## Project Structure

```
file-management-system/
├── src/
│   ├── main.rs                 # Server initialization, routing setup
│   ├── features/
│   │   ├── auth/               # Authentication (extractor, handler, model, pages)
│   │   ├── files/              # File listing/management (handler, pages)
│   │   └── upload/             # File upload (handler, partial templates)
│   └── shared/
│       ├── context.rs          # AppContext (db_pool, session key)
│       └── templates/          # Base templates (base.html.askama, dashboard.html.askama)
├── migrations/                 # SQLx migrations (users, folders, files)
├── docs/
│   ├── product-requirement-document.md  # User stories, acceptance criteria
│   ├── entity-relationship-diagram.md   # Database schema
│   └── process-flow.md         # User workflows
├── Cargo.toml                  # Dependencies and metadata
├── askama.toml                 # Askama configuration
└── .cargo/config.toml          # DATABASE_URL env variable
```

## Key Architecture Decisions

### Database Layer
- **Migrations**: Located in `migrations/` using SQLx declarative macros
- **Schema**: users, folders, files tables with soft-delete support (deleted_at)
- **Foreign Keys**: Cascading deletes for user-owned resources
- **Storage Tracking**: Users have quota_bytes and used_bytes for managing limits

### Feature Organization
Each feature module (auth, files, upload) is self-contained with:
- `handler.rs` - Route handlers and router definition
- `model.rs` - Data models/domain types
- `request.rs` - Request parsing and validation
- `pages/` - Page handlers for HTML templates
- `template/` - Feature-specific partial templates

### Routing Pattern
Routes are nested by feature in main.rs:
```rust
router
  .route("/", get(file_page))
  .nest("/auth", auth::handler::router())
```

### Session & Cookies
- `axum-extra` provides signed/private cookie extraction
- `Key::generate()` creates per-instance session keys
- Available in request extraction via `PrivateCookieJar` or `SignedCookieJar`

### Templating
- **Engine**: Askama (compile-time safety)
- **Base Layout**: `src/shared/templates/base.html.askama`
- **Configuration**: `askama.toml` - template directory settings
- **Pattern**: Handlers return `impl IntoResponse` (usually template instances)

## Build & Run Commands

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build

# Run (watches DATABASE_URL from .cargo/config.toml)
cargo run

# Server starts on 127.0.0.1:8080

# Test (when test suite exists)
cargo test

# Format & lint
cargo fmt
cargo clippy
```

## Development Workflow

### Running Migrations
Migrations run automatically on app startup:
```rust
sqlx::migrate!("./migrations").run(&db_pool).await?;
```

To create a new migration:
```bash
sqlx migrate add -r <name>
# Creates migrations/<timestamp>_<name>.up.sql and .down.sql
```

### Adding a New Endpoint

1. **Add handler** in appropriate feature module (e.g., `src/features/files/handler.rs`)
2. **Register route** in feature's router function
3. **Create template** in `src/features/{feature}/template/` if needed
4. **Update router nesting** in main.rs if adding new feature

### Adding a New Template
1. Create `.html.askama` file in `src/shared/templates/` or feature-specific `template/`
2. Implement `IntoResponse` manually or use Askama auto-derive
3. Pass data via template context struct that derives `Template`

## Database Schema

**users** - Storage accounts
- id (PK), username (unique), password_hash
- storage_quota_bytes (default 10 GB), storage_used_bytes
- created_at, updated_at, deleted_at (soft delete)

**folders** - Hierarchical folder structure
- id (PK), user_id (FK), name, parent_folder_id (self-join)
- created_at, updated_at, deleted_at
- Supports unlimited nesting via parent_folder_id

**files** - Stored files
- id (PK), user_id (FK), folder_id (nullable FK), name
- storage_path, size_bytes, mime_type
- created_at, updated_at, deleted_at

## Common Development Tasks

### Database Query Pattern
Use SQLx `query_as!` or `query!` for compile-time checked queries:
```rust
let user = sqlx::query_as::<_, User>(
  "SELECT * FROM users WHERE id = ?"
)
  .bind(user_id)
  .fetch_one(&pool)
  .await?;
```

### Template Rendering
Create a struct with `Template` derive:
```rust
#[derive(Template)]
#[template(path = "dashboard.html.askama")]
pub struct DashboardPage { ... }

// In handler
Ok(DashboardPage { ... })
```

### Error Handling
Use `anyhow::Result<T>` for endpoint errors:
```rust
pub async fn my_handler() -> anyhow::Result<impl IntoResponse> { ... }
```

### Request Extraction
Common extractors:
- `State<AppContext>` - App state with db_pool
- `Path<Id>` - URL path parameters
- `Query<Params>` - Query string
- `Json<T>` - JSON body
- `PrivateCookieJar` - Session cookies (from axum-extra)

## Common Pitfalls & Best Practices

1. **Database Pool**: Always use `&state.db_pool` for async operations
2. **Soft Deletes**: Remember to filter `WHERE deleted_at IS NULL` in queries
3. **User Isolation**: Always filter by `user_id` when fetching resources
4. **Storage Quota**: Update `storage_used_bytes` when files are added/deleted
5. **Path Safety**: Validate folder_id ownership before allowing access
6. **Migrations**: Always create `.down.sql` files for reversibility
7. **Template Escaping**: Askama auto-escapes by default—use `|safe` carefully

## File-to-Feature Mapping

- `src/main.rs` - Server setup, router registration
- `src/features/auth/` - User authentication, login/signup
- `src/features/files/` - File browsing, folder navigation
- `src/features/upload/` - File upload logic, multipart handling
- `src/shared/context.rs` - AppContext definition
- `src/shared/templates/` - Global layout templates
- `migrations/` - Database versioning
- `docs/` - Product specs and diagrams

## Next Steps in Development

Based on the PRD, features are prioritized as:
1. **P0 (MVP Critical)**: File upload, folder organization, file download
2. **P1 (Release 1)**: File search, sorting, batch operations
3. **P2 (Release 2)**: Sharing, versioning, notifications

Current implementation should focus on core file operations with proper user isolation and storage quota enforcement.

---

**Last Updated**: 2026-03-17  
**Target Rust Version**: 1.75+
