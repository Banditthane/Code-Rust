# Lesson 04: Rust Database Integration

## 1. Database Integration คืออะไร?

Database Integration คือการเชื่อมต่อ Rust Application เข้ากับ Database เพื่อให้โปรแกรมสามารถจัดเก็บ ค้นหา และแก้ไขข้อมูลได้

ตัวอย่าง:

```text
Client
   |
   v
Axum API
   |
   v
Service Layer
   |
   v
Database Layer
   |
   v
SQLite / PostgreSQL
```

## 2. SQLx กับ Rust

SQLx เป็น Async SQL Toolkit สำหรับ Rust

ความสามารถ:

- เชื่อมต่อ Database
- สร้าง Connection Pool
- Execute SQL
- Fetch Query Results
- ทำ Transactions
- ใช้ Compile-time Checked Queries ในรูปแบบที่กำหนด

SQLx ใช้ Rust Types ร่วมกับ Database Types เพื่อช่วยตรวจสอบข้อมูล

## 3. ติดตั้ง SQLx

ตัวอย่างสำหรับ SQLite:

```powershell
cargo add sqlx --features runtime-tokio,sqlite
cargo add tokio --features full
```

ตัวอย่างสำหรับ PostgreSQL:

```powershell
cargo add sqlx --features runtime-tokio,postgres
cargo add tokio --features full
```

หากใช้ SQLx กับ Serde:

```powershell
cargo add serde --features derive
cargo add serde_json
```

หมายเหตุ: Feature ของ SQLx ต้องเลือกให้ตรงกับ Database Driver และ Runtime ที่ใช้

## 4. Database Model

สร้าง Rust Struct สำหรับแทนข้อมูลใน Database

ตัวอย่าง:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    email: String,
}
```

Model ควรสะท้อนข้อมูลที่ Application ต้องใช้ แต่ไม่จำเป็นต้องเหมือน Database Schema ทุก Field เสมอไป

## 5. Query กับ Struct

SQLx รองรับการอ่านข้อมูลเข้าสู่ Struct ผ่าน `query_as`

ตัวอย่างแนวคิด:

```rust
let users = sqlx::query_as::<_, User>(
    "SELECT id, name, email FROM users"
)
.fetch_all(&pool)
.await?;
```

ข้อกำหนด:

- Struct ต้องรองรับการแปลงจาก Query Result
- Field ต้องตรงกับ Columns ที่ Query คืนมา
- ชนิดข้อมูลต้องเข้ากันได้
- ต้องจัดการ Error จาก Query

สำหรับการ Mapping ด้วย `query_as` แบบนี้ Struct ต้อง Implement `FromRow` ที่เหมาะสม ซึ่งสามารถใช้ Macro หรือ Derive ตามรูปแบบที่เลือกได้

## 6. FromRow

สามารถใช้ `FromRow` สำหรับ Mapping Database Row ไปเป็น Struct

ตัวอย่าง:

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}
```

ต้องเปิด SQLx Feature ที่รองรับ Macro ตามการใช้งาน เช่น `macros` หากใช้ Derive Macro

## 7. Database CRUD

ตัวอย่าง CRUD สำหรับ User:

### Create

```sql
INSERT INTO users (name, email)
VALUES (?, ?);
```

### Read

```sql
SELECT id, name, email
FROM users
WHERE id = ?;
```

### Update

```sql
UPDATE users
SET name = ?, email = ?
WHERE id = ?;
```

### Delete

```sql
DELETE FROM users
WHERE id = ?;
```

ควรใช้ Bind Parameters ทุกครั้งที่รับค่าจาก Input

## 8. Async Database Operations

SQLx ทำงานแบบ Async

ตัวอย่าง:

```rust
async fn get_users(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, email FROM users"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
```

ข้อดี:

- สามารถรอ Database I/O โดยไม่ Block Thread ในรูปแบบ Synchronous
- ทำงานร่วมกับ Tokio ได้
- รองรับการทำงานพร้อมกันหลาย Requests

## 9. Connection Pool

ใช้ Pool เพื่อจัดการ Database Connections

ตัวอย่าง:

```rust
use sqlx::sqlite::SqlitePoolOptions;

let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .connect("sqlite:app.db")
    .await?;
```

สามารถส่ง Pool ไปยัง Service หรือ Axum Application State ได้

ไม่ควรสร้าง Database Connection ใหม่ทุกครั้งที่มี HTTP Request โดยไม่จำเป็น

## 10. Database Transactions

ใช้ Transaction เมื่อต้องทำหลาย Queries ที่ควรสำเร็จหรือ Rollback ร่วมกัน

ตัวอย่างแนวคิด:

```rust
let mut tx = pool.begin().await?;

// Execute related database operations

tx.commit().await?;
```

ควรออกแบบให้ Transaction มีขอบเขตสั้นเท่าที่จำเป็น เพื่อลดเวลาที่ถือ Lock และลดการแย่งใช้ Database Resources

## 11. SQLx Migrations

Migration ใช้จัดการ Schema Changes

ตัวอย่าง:

```text
migrations/
├── 0001_create_users.sql
├── 0002_create_products.sql
└── 0003_create_orders.sql
```

การใช้ Migration ช่วยให้:

- สร้าง Schema ซ้ำได้อย่างเป็นระบบ
- ติดตามการเปลี่ยนแปลง
- ทำงานร่วมกันในทีมได้
- เตรียม Database ในแต่ละ Environment ได้

ควรตรวจสอบ Migration ก่อนนำไปใช้กับข้อมูลจริง

## 12. Environment Configuration

ไม่ควรเขียน Database Credentials ลง Source Code

ตัวอย่าง Environment Variables:

```text
DATABASE_URL
APP_HOST
APP_PORT
```

ตัวอย่าง Connection String:

SQLite:

```text
sqlite:app.db
```

PostgreSQL:

```text
postgres://user:password@localhost/database
```

Connection String ที่มี Credentials ต้องเก็บเป็น Secret และห้าม Commit ลง Git

## 13. Database Errors

ตัวอย่าง Error ที่ควรจัดการ:

- Connection Failed
- Query Failed
- Constraint Violation
- Unique Constraint Violation
- Foreign Key Violation
- Record Not Found
- Transaction Failed
- Migration Failed

แนวทาง:

- แยก Database Error จาก Business Error
- ไม่ส่ง Internal Database Error ให้ Client โดยตรง
- Log รายละเอียดที่จำเป็นโดยไม่เปิดเผย Secret
- แปลง Error เป็น Application Error ที่เหมาะสม

## 14. Axum กับ SQLx

สามารถใช้ Axum ร่วมกับ SQLx เพื่อสร้าง REST API ที่อ่านและเขียน Database ได้

โครงสร้างแนวคิด:

```text
HTTP Request
     |
     v
Axum Router
     |
     v
Handler
     |
     v
Service
     |
     v
SQLx Repository
     |
     v
Database
```

แนวทางแบ่งหน้าที่:

| ส่วน       | หน้าที่                     |
| ---------- | --------------------------- |
| Router     | กำหนด Endpoint              |
| Handler    | รับ Request และส่ง Response |
| Service    | Business Logic              |
| Repository | Database Queries            |
| Model      | โครงสร้างข้อมูล             |
| Error      | จัดการข้อผิดพลาด            |

## 15. สรุป

- SQLx ใช้เชื่อมต่อ SQL Database
- Connection Pool ช่วยจัดการ Connections
- `query_as` ใช้ Mapping Query Results เป็น Struct
- `FromRow` ช่วยแปลง Database Row เป็น Rust Struct
- Async Queries ทำงานร่วมกับ Tokio
- Transactions ใช้จัดการหลาย Operations ร่วมกัน
- Migrations ใช้จัดการ Schema Changes
- Environment Variables ใช้เก็บ Configuration
- Axum และ SQLx สามารถทำงานร่วมกันเพื่อสร้าง Database-backed API

## Completion Checklist

- [ ] เข้าใจ SQLx
- [ ] ติดตั้ง SQLx ได้
- [ ] เชื่อมต่อ SQLite ได้
- [ ] เชื่อมต่อ PostgreSQL ได้ในระดับพื้นฐาน
- [ ] สร้าง Database Model ได้
- [ ] เข้าใจ `FromRow`
- [ ] ใช้ `query_as` ได้
- [ ] ทำ CRUD ผ่าน SQLx ได้
- [ ] ใช้ Async Queries ได้
- [ ] ใช้ Connection Pool ได้
- [ ] ใช้ Transactions ได้
- [ ] เข้าใจ Migrations
- [ ] ใช้ Environment Variables ได้
- [ ] จัดการ Database Errors ได้
- [ ] เข้าใจการทำงานร่วมกันของ Axum และ SQLx
