# Lesson 02: Database Operations

## 1. Database Connection

Rust Application ต้องเชื่อมต่อ Database ก่อนจึงจะอ่านหรือเขียนข้อมูลได้

แนวทางเชื่อมต่อมีหลายแบบ:

- SQLite: เชื่อมต่อกับ Database File
- PostgreSQL: เชื่อมต่อกับ Database Server
- Connection Pool: จัดการ Connection หลายรายการเพื่อนำกลับมาใช้ซ้ำ

ในบทนี้จะใช้ SQLx เป็นตัวอย่างในการเชื่อมต่อ Database

## 2. SQLx คืออะไร?

SQLx เป็น Rust Library สำหรับทำงานกับ SQL Database แบบ Async

รองรับ Database หลายประเภท เช่น:

- PostgreSQL
- MySQL
- SQLite

คุณสมบัติ:

- Async Database Operations
- Parameterized Queries
- Connection Pool
- Transactions
- Compile-time Checked Queries ในรูปแบบที่กำหนด
- รองรับ Tokio Runtime

## 3. ติดตั้ง SQLx สำหรับ SQLite

เพิ่ม Dependencies:

```powershell
cargo add sqlx --features runtime-tokio,sqlite
cargo add tokio --features full
```

ตัวอย่าง `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
```

หมายเหตุ: เวอร์ชันที่ติดตั้งจริงอาจใหม่กว่าตัวอย่าง ควรตรวจสอบ `Cargo.toml` หลังติดตั้ง

## 4. Connect to SQLite

SQLx ใช้ `SqlitePool` สำหรับจัดการ Connection Pool

แนวคิด:

```rust
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:app.db")
        .await?;

    println!("Database connected");

    pool.close().await;

    Ok(())
}
```

ตัวอย่างนี้เชื่อมต่อกับ Database File ที่ชื่อ `app.db`

หากต้องการสร้างไฟล์ Database อัตโนมัติเมื่อยังไม่มี สามารถใช้ SQLite Connection Options เช่น `create_if_missing(true)` ได้

## 5. CRUD Operations

CRUD เป็นคำย่อของ:

| Operation | ความหมาย    | SQL    |
| --------- | ----------- | ------ |
| Create    | เพิ่มข้อมูล | INSERT |
| Read      | อ่านข้อมูล  | SELECT |
| Update    | แก้ไขข้อมูล | UPDATE |
| Delete    | ลบข้อมูล    | DELETE |

CRUD เป็นพื้นฐานของ Database Application ส่วนใหญ่

## 6. CREATE — INSERT

เพิ่มข้อมูล:

```sql
INSERT INTO users (name, email)
VALUES ('Alice', 'alice@example.com');
```

เมื่อใช้ SQLx ควรใช้ Bind Parameters แทนการต่อ String

แนวคิด:

```rust
sqlx::query(
    "INSERT INTO users (name, email) VALUES (?, ?)"
)
.bind("Alice")
.bind("alice@example.com")
.execute(&pool)
.await?;
```

สำหรับ SQLite ใช้ `?` เป็น Placeholder ได้

## 7. READ — SELECT

อ่านข้อมูล:

```sql
SELECT id, name, email
FROM users;
```

ใช้ SQLx:

```rust
let rows = sqlx::query(
    "SELECT id, name, email FROM users"
)
.fetch_all(&pool)
.await?;
```

`fetch_all()` จะคืนค่ารายการ Rows ทั้งหมดที่ Query ได้

## 8. UPDATE

แก้ไขข้อมูล:

```sql
UPDATE users
SET name = 'Alice Smith'
WHERE id = 1;
```

ใช้ SQLx:

```rust
sqlx::query(
    "UPDATE users SET name = ? WHERE id = ?"
)
.bind("Alice Smith")
.bind(1)
.execute(&pool)
.await?;
```

ควรตรวจสอบจำนวน Rows ที่ถูกแก้ไขผ่านผลลัพธ์ของ Execute

## 9. DELETE

ลบข้อมูล:

```sql
DELETE FROM users
WHERE id = 1;
```

ใช้ SQLx:

```rust
sqlx::query(
    "DELETE FROM users WHERE id = ?"
)
.bind(1)
.execute(&pool)
.await?;
```

ควรตรวจสอบ Rows Affected เพื่อแยกกรณีลบสำเร็จและไม่พบข้อมูล

## 10. Parameterized Queries

Parameterized Query ใช้ Placeholder สำหรับค่าที่ต้องส่งเข้า SQL

ตัวอย่างที่ไม่ควรทำ:

```rust
let sql = format!(
    "SELECT * FROM users WHERE name = '{}'",
    user_input
);
```

การต่อ String จาก User Input อาจเปิดช่องให้ SQL Injection

ควรทำ:

```rust
sqlx::query(
    "SELECT * FROM users WHERE name = ?"
)
.bind(user_input)
.fetch_all(&pool)
.await?;
```

ข้อดี:

- แยก SQL Structure ออกจาก Parameter Values
- ลดความเสี่ยง SQL Injection
- จัดการชนิดข้อมูลได้เหมาะสมกว่า
- อ่านและดูแล Query ได้ง่ายกว่า

## 11. Fetch Methods

SQLx มีวิธีดึงข้อมูลหลายแบบ:

| Method             | หน้าที่                    |
| ------------------ | -------------------------- |
| `fetch_one()`      | ต้องได้หนึ่ง Row           |
| `fetch_optional()` | ได้ศูนย์หรือหนึ่ง Row      |
| `fetch_all()`      | ได้ Rows ทั้งหมด           |
| `fetch()`          | คืนค่าเป็น Stream ของ Rows |

ควรเลือกให้เหมาะกับผลลัพธ์ที่คาดหวัง

ตัวอย่าง:

- ค้นหา User ตาม ID: `fetch_optional()`
- อ่าน User ทั้งหมด: `fetch_all()`
- Query ที่ต้องมีผลลัพธ์หนึ่งรายการ: `fetch_one()`

## 12. Transactions

Transaction ใช้รวมหลาย Database Operations ให้ทำงานเป็นหน่วยเดียว

หลักการสำคัญ:

- BEGIN: เริ่ม Transaction
- COMMIT: ยืนยันการเปลี่ยนแปลง
- ROLLBACK: ยกเลิกการเปลี่ยนแปลง

ตัวอย่างสถานการณ์:

การโอนเงินระหว่างบัญชี:

1. หักเงินจากบัญชีต้นทาง
2. เพิ่มเงินให้บัญชีปลายทาง
3. บันทึกประวัติการโอน

หากขั้นตอนใดล้มเหลว ต้องไม่ปล่อยให้ข้อมูลอยู่ในสถานะไม่สอดคล้องกัน

ตัวอย่างแนวคิด SQLx:

```rust
let mut transaction = pool.begin().await?;

// Execute database operations using transaction

transaction.commit().await?;
```

หาก Transaction ถูก Drop ก่อน Commit โดยทั่วไป SQLx จะพยายาม Rollback เมื่อ Connection ถูกคืนสู่ Pool

## 13. ACID Properties

Transaction มักอธิบายด้วย ACID:

| คุณสมบัติ   | ความหมาย                                               |
| ----------- | ------------------------------------------------------ |
| Atomicity   | ทำทั้งหมดหรือไม่ทำเลย                                  |
| Consistency | รักษากฎความถูกต้องของข้อมูล                            |
| Isolation   | ควบคุมผลกระทบระหว่าง Transactions                      |
| Durability  | ข้อมูลที่ Commit แล้วคงอยู่ตามการรับประกันของ Database |

## 14. Connection Pool

Connection Pool คือกลุ่ม Database Connections ที่ Application นำกลับมาใช้ซ้ำ

ประโยชน์:

- ลดต้นทุนการเปิด Connection ใหม่
- รองรับ Request หลายรายการ
- จำกัดจำนวน Connections
- จัดการ Connection Lifecycle

ควรกำหนดขนาด Pool ให้เหมาะสมกับ Database และรูปแบบการใช้งาน

## 15. Database Migration

Migration ใช้จัดการการเปลี่ยนแปลง Schema ตามลำดับ

ตัวอย่าง:

```text
migrations/
├── 0001_create_users.sql
├── 0002_create_products.sql
└── 0003_create_orders.sql
```

ตัวอย่าง Migration:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);
```

SQLx CLI สามารถช่วยจัดการ Migration ได้

ติดตั้ง:

```powershell
cargo install sqlx-cli --no-default-features --features sqlite
```

สร้าง Migration:

```powershell
sqlx migrate add create_users
```

รัน Migration:

```powershell
sqlx migrate run
```

## 16. สรุป

| หัวข้อ              | สิ่งที่ต้องเข้าใจ             |
| ------------------- | ----------------------------- |
| Connection          | เชื่อมต่อ Database            |
| Pool                | จัดการ Connections            |
| CRUD                | เพิ่ม อ่าน แก้ไข ลบ           |
| Parameterized Query | ส่งค่าผ่าน Parameters         |
| Fetch               | อ่าน Query Results            |
| Transaction         | รวม Operations เป็นหน่วยเดียว |
| ACID                | คุณสมบัติของ Transactions     |
| Migration           | จัดการ Schema Changes         |

## Completion Checklist

- [ ] เชื่อมต่อ SQLite ด้วย SQLx ได้
- [ ] ใช้ Connection Pool ได้
- [ ] ทำ INSERT ได้
- [ ] ทำ SELECT ได้
- [ ] ทำ UPDATE ได้
- [ ] ทำ DELETE ได้
- [ ] ใช้ Bind Parameters ได้
- [ ] เข้าใจ SQL Injection
- [ ] ใช้ `fetch_one()` ได้
- [ ] ใช้ `fetch_optional()` ได้
- [ ] ใช้ `fetch_all()` ได้
- [ ] เข้าใจ Transactions
- [ ] เข้าใจ ACID
- [ ] เข้าใจ Connection Pool
- [ ] เข้าใจ Database Migration
