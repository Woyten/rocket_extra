# Additional macros for rocket.rs

## Derive `FromRequest`

```rust
use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
struct BookingService {
    db_conn: DbConnection,
    queue_conn: QueueConnection,
    user: AuthUser,
}
```

The errors of the individual fields will be converted via the `From` trait to a common target error type. The target error type defaults to `()` but can be overridden manually:

```rust
use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
#[error_type = "MyError"]
struct BookingService {
    db_conn: DbConnection,
    queue_conn: QueueConnection,
    user: AuthUser,
}
```