# Exercise 04: Database Challenge

## ระดับ

Challenge

## Objective

ประยุกต์ใช้ Rust, SQLx, SQLite, Serde และ Database Transactions เพื่อสร้างระบบจัดการข้อมูลที่มีโครงสร้างใกล้เคียงกับ Application จริง

## Problem Statement

สร้างระบบ **Library Management System**

ระบบต้องจัดการ:

- Books
- Members
- Borrow Records

สามารถเพิ่ม อ่าน แก้ไข และลบข้อมูล พร้อมจัดการการยืมและคืนหนังสือ

## Part 1: Project Architecture

ออกแบบโครงสร้าง Project โดยแยกหน้าที่ของแต่ละส่วน

ตัวอย่าง:

```text
database_challenge/
├── Cargo.toml
├── README.md
├── migrations/
│   ├── 0001_create_books.sql
│   ├── 0002_create_members.sql
│   └── 0003_create_borrow_records.sql
└── src/
    ├── main.rs
    ├── config.rs
    ├── database.rs
    ├── models/
    │   ├── mod.rs
    │   ├── book.rs
    │   ├── member.rs
    │   └── borrow_record.rs
    ├── repositories/
    │   ├── mod.rs
    │   ├── book_repository.rs
    │   ├── member_repository.rs
    │   └── borrow_repository.rs
    ├── services/
    │   ├── mod.rs
    │   ├── book_service.rs
    │   └── borrow_service.rs
    └── errors.rs
```

สามารถปรับโครงสร้างตามความเหมาะสม

## Part 2: Database Schema

สร้าง Tables อย่างน้อย 3 ตาราง

### Books

| Column           | Type    | Constraint  |
| ---------------- | ------- | ----------- |
| id               | INTEGER | PRIMARY KEY |
| title            | TEXT    | NOT NULL    |
| author           | TEXT    | NOT NULL    |
| isbn             | TEXT    | UNIQUE      |
| total_copies     | INTEGER | NOT NULL    |
| available_copies | INTEGER | NOT NULL    |

### Members

| Column | Type    | Constraint  |
| ------ | ------- | ----------- |
| id     | INTEGER | PRIMARY KEY |
| name   | TEXT    | NOT NULL    |
| email  | TEXT    | UNIQUE      |
| active | INTEGER | NOT NULL    |

### Borrow Records

| Column      | Type    | Constraint  |
| ----------- | ------- | ----------- |
| id          | INTEGER | PRIMARY KEY |
| book_id     | INTEGER | FOREIGN KEY |
| member_id   | INTEGER | FOREIGN KEY |
| borrowed_at | TEXT    | NOT NULL    |
| returned_at | TEXT    | NULL        |

กำหนด Foreign Key ให้สัมพันธ์กับ Books และ Members

ต้องออกแบบข้อกำหนดเพิ่มเติมให้เหมาะสม เช่น:

- จำนวนสำเนาหนังสือต้องไม่ติดลบ
- จำนวนสำเนาที่พร้อมยืมต้องไม่เกินจำนวนทั้งหมด
- Member ต้องมีสถานะ Active ก่อนยืมหนังสือ
- การคืนหนังสือต้องอ้างอิง Borrow Record ที่ยังไม่ถูกคืน

## Part 3: Book Management

สร้างความสามารถ:

1. เพิ่มหนังสือ
2. อ่านหนังสือทั้งหมด
3. ค้นหาหนังสือตาม ID
4. ค้นหาตามชื่อหนังสือ
5. แก้ไขข้อมูลหนังสือ
6. ลบหนังสือ

ต้องตรวจสอบ:

- Title ต้องไม่ว่าง
- Author ต้องไม่ว่าง
- ISBN ต้องไม่ซ้ำ
- Total Copies ต้องไม่ติดลบ
- Available Copies ต้องไม่ติดลบ
- Available Copies ต้องไม่เกิน Total Copies

ห้ามลบหนังสือที่ยังมี Borrow Record ที่ยังไม่คืนโดยไม่มีกฎจัดการที่ชัดเจน

## Part 4: Member Management

สร้างความสามารถ:

1. เพิ่ม Member
2. อ่าน Members
3. ค้นหา Member ตาม ID
4. แก้ไขข้อมูล Member
5. เปลี่ยนสถานะ Active
6. ตรวจสอบ Email ซ้ำ

ต้องไม่อนุญาตให้ Member ที่ไม่ Active ยืมหนังสือ

## Part 5: Borrow Book

สร้าง Function:

```text
borrow_book()
```

ขั้นตอนการทำงาน:

1. ตรวจสอบ Member
2. ตรวจสอบสถานะ Active
3. ตรวจสอบ Book
4. ตรวจสอบ Available Copies
5. สร้าง Borrow Record
6. ลด Available Copies ลง 1
7. Commit Transaction

ทุกขั้นตอนที่เกี่ยวข้องกับการยืมต้องอยู่ภายใต้ Transaction เดียวกัน

หากขั้นตอนใดล้มเหลว ต้อง Rollback การเปลี่ยนแปลงที่เกิดขึ้นใน Transaction นั้น

## Part 6: Return Book

สร้าง Function:

```text
return_book()
```

ขั้นตอน:

1. ค้นหา Borrow Record
2. ตรวจสอบว่ามี Record อยู่จริง
3. ตรวจสอบว่ายังไม่ได้คืน
4. กำหนด Returned At
5. เพิ่ม Available Copies กลับ 1
6. Commit Transaction

ต้องป้องกันการคืนหนังสือรายการเดิมซ้ำ

## Part 7: Database Transactions

ต้องใช้ Transactions ใน Operations ที่ต้องเปลี่ยนแปลงข้อมูลหลายรายการร่วมกัน เช่น:

- Borrow Book
- Return Book

ข้อกำหนด:

- ต้อง Commit เมื่อทุกขั้นตอนสำเร็จ
- ต้อง Rollback เมื่อเกิด Error
- ต้องป้องกันจำนวนหนังสือผิดพลาด
- ต้องจัดการ Concurrent Requests ตามความเหมาะสม
- ต้องไม่ทำให้ Available Copies ติดลบ

## Part 8: Serialization

ใช้ Serde สำหรับ:

- Serialize Book เป็น JSON
- Serialize Member เป็น JSON
- Serialize Borrow Record เป็น JSON
- Deserialize Input จาก JSON
- สร้าง Response Model สำหรับแสดงผล

สามารถแยก Database Model ออกจาก API Response Model ได้

ตัวอย่าง JSON:

```json
{
  "id": 1,
  "title": "Rust Programming",
  "author": "Example Author",
  "isbn": "9780000000001",
  "total_copies": 5,
  "available_copies": 3
}
```

## Part 9: Error Handling

สร้าง Application Errors เช่น:

- BookNotFound
- MemberNotFound
- MemberInactive
- BookUnavailable
- DuplicateISBN
- DuplicateEmail
- AlreadyReturned
- DatabaseError
- InvalidInput

ต้องแยก Business Errors ออกจาก Database Errors

ไม่ควรส่ง Internal Database Details ให้ผู้ใช้งานโดยตรง

## Part 10: Testing

เขียน Tests อย่างน้อย:

### Book Tests

- Create Book
- Get Book
- Update Book
- Delete Book
- Search Book
- Duplicate ISBN

### Member Tests

- Create Member
- Get Member
- Update Member
- Duplicate Email
- Inactive Member

### Borrow Tests

- Borrow Available Book
- Borrow Unavailable Book
- Borrow as Inactive Member
- Return Book
- Return Already Returned Book
- Borrow and Return Updates Available Copies Correctly

### Transaction Tests

- Rollback When Borrow Fails
- Rollback When Return Fails
- Ensure Available Copies Never Become Negative

## Constraints

- ต้องใช้ Rust
- ต้องใช้ SQLite
- ต้องใช้ SQLx
- ต้องใช้ Tokio
- ต้องใช้ Serde
- ต้องมีอย่างน้อย 3 Tables
- ต้องมี Primary Keys
- ต้องมี Foreign Keys
- ต้องมี Unique Constraints ตามความเหมาะสม
- ต้องทำ CRUD
- ต้องใช้ Parameterized Queries
- ต้องใช้ Transactions สำหรับ Borrow และ Return
- ต้องตรวจสอบ Input
- ต้องจัดการ Application Errors
- ต้องแยก Repository และ Service
- ต้องมี Unit Tests
- ต้องมี Integration Tests
- ห้ามใช้ SQL String Concatenation กับ User Input
- ห้ามใช้ `unwrap()` ใน Database Operations โดยไม่มีเหตุผล
- ต้องป้องกันการยืมและคืนซ้ำ
- ต้องป้องกัน Available Copies ติดลบ
- ต้องจัดการ Concurrent Access อย่างเหมาะสม

## Expected Output

```text
Library Management System

1. Add Book
2. List Books
3. Search Book
4. Add Member
5. List Members
6. Borrow Book
7. Return Book
8. Borrow History
0. Exit

Select: 1

Book Title: Rust Programming
Author: Example Author
ISBN: 9780000000001
Total Copies: 5

Book created successfully!
```

ตัวอย่างการยืม:

```text
Select: 6

Member ID: 1
Book ID: 1

Checking member...
Member is active.

Checking book...
Book is available.

Creating borrow record...
Updating available copies...

Book borrowed successfully!
```

ตัวอย่างการคืน:

```text
Select: 7

Borrow Record ID: 1

Checking borrow record...
Book returned successfully!

Available copies updated.
```

ตัวอย่างเมื่อไม่มีหนังสือ:

```text
Select: 6

Member ID: 1
Book ID: 999

Error: Book not found.
```

## Hints

Dependencies ที่อาจใช้:

```powershell
cargo add sqlx --features runtime-tokio,sqlite
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
cargo add chrono --features serde
```

แนวทางเพิ่มเติม:

- ใช้ `SqlitePool` สำหรับ Connection Pool
- ใช้ `query_as()` สำหรับ Mapping Rows
- ใช้ `FromRow` สำหรับ Database Models
- ใช้ `Transaction` สำหรับ Borrow และ Return
- ใช้ `fetch_optional()` สำหรับการค้นหาที่อาจไม่พบข้อมูล
- ใช้ `rows_affected()` ตรวจสอบผลการ Update
- ใช้ Serde สำหรับ JSON
- ใช้ Migration จัดการ Schema
- ใช้ Temporary Database หรือ Database แยกสำหรับ Tests

ข้อควรระวังด้าน SQLite:

- ต้องเปิด Foreign Key Enforcement สำหรับทุก Connection ที่ใช้
- SQLite มีข้อจำกัดด้าน Concurrent Writes จึงควรออกแบบ Transaction ให้สั้น
- ต้องตรวจสอบจำนวนสำเนาหนังสือภายใน Transaction เพื่อป้องกันการยืมเกินจำนวน
- ควรใช้ Constraints ใน Database ช่วยรักษาความถูกต้องของข้อมูลด้วย

## Completion Checklist

### Database

- [ ] สร้าง Books Table
- [ ] สร้าง Members Table
- [ ] สร้าง Borrow Records Table
- [ ] กำหนด Primary Keys
- [ ] กำหนด Foreign Keys
- [ ] กำหนด Unique Constraints
- [ ] ใช้ Migrations

### Book Management

- [ ] Create Book
- [ ] Read Books
- [ ] Search Book
- [ ] Update Book
- [ ] Delete Book

### Member Management

- [ ] Create Member
- [ ] Read Members
- [ ] Update Member
- [ ] ตรวจสอบ Email ซ้ำ
- [ ] จัดการ Active Status

### Borrow and Return

- [ ] Borrow Book
- [ ] Return Book
- [ ] ตรวจสอบ Available Copies
- [ ] ป้องกันการยืมเกินจำนวน
- [ ] ป้องกันการคืนซ้ำ
- [ ] ใช้ Transactions

### Serialization

- [ ] Serialize Book เป็น JSON
- [ ] Serialize Member เป็น JSON
- [ ] Serialize Borrow Record เป็น JSON
- [ ] Deserialize JSON ได้

### Quality

- [ ] แยก Repository
- [ ] แยก Service
- [ ] จัดการ Application Errors
- [ ] Unit Tests
- [ ] Integration Tests
- [ ] ทดสอบ Transaction Rollback
- [ ] ตรวจสอบ Concurrent Access

## Run

```powershell
cd 29_database_serialization/exercises/exercise_04/database_challenge

cargo check
cargo test
cargo run
```

## Final Challenge

เมื่อระบบพื้นฐานทำงานได้แล้ว ให้เพิ่ม:

1. REST API ด้วย Axum
2. PostgreSQL แทน SQLite
3. Pagination
4. Search และ Filtering
5. Database Migration Versioning
6. Database Backup
7. Docker Support
8. API Documentation
9. Authentication และ Authorization

**หลักสำคัญ:** ระบบ Database ที่ดีต้องรักษาความถูกต้องของข้อมูล แม้เกิด Error หรือมีหลาย Requests ทำงานพร้อมกัน
