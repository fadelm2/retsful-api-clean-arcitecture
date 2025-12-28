# Axum Clean Architecture – JWT Auth API

Project ini adalah contoh **RESTful API User Management** menggunakan **Rust** dengan pendekatan **Clean Architecture**.

Fokus utama:
- Struktur kode rapi & scalable
- Pemisahan tanggung jawab yang jelas
- JWT Authentication (Register & Login)
- Request validation yang benar
- Siap dikembangkan ke production

---

## 🧱 Tech Stack

- **Rust**
- **Axum** – Web framework
- **SQLx** – PostgreSQL async ORM
- **PostgreSQL** – Database
- **JWT (jsonwebtoken)** – Authentication
- **bcrypt** – Password hashing
- **validator** – Request validation
- **chrono** – Date & time
- **tracing** – Logging
- **tokio** – Async runtime

---

## 📂 Project Structure (Clean Architecture)

```text
src/
├── main.rs                # Application entry point
├── app.rs                 # Dependency Injection & bootstrap
├── domain/                # Core business layer
│   ├── entity/            # Domain entities
│   └── repository/        # Repository interfaces
├── usecase/               # Business logic
├── infrastructure/        # External implementations
│   ├── db/                # Database connection
│   ├── auth/              # JWT service
│   └── repository/        # SQLx repository implementation
├── delivery/              # HTTP layer
│   └── http/
│       ├── handler/       # HTTP handlers (Axum)
│       └── router.rs      # API routes