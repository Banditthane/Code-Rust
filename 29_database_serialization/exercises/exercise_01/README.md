# Exercise 01: SQLite Basics

## ระดับ

Easy

## Objective

ฝึกสร้าง SQLite Database เชื่อมต่อจาก Rust และจัดการ Table เบื้องต้น

## Problem Statement

สร้างโปรแกรมจัดการข้อมูลสินค้า (Product Database)

โปรแกรมต้องสร้าง Database File ชื่อ:

```text
products.db
```

และสร้าง Table ชื่อ:

```text
products
```

กำหนด Columns:

| Column | Type    | Constraint  |
| ------ | ------- | ----------- |
| id     | INTEGER | PRIMARY KEY |
| name   | TEXT    | NOT NULL    |
| price  | REAL    | NOT NULL    |
| stock  | INTEGER | NOT NULL    |

## Requirements

### Part 1: Database Connection

1. ใช้ SQLx
2. เชื่อมต่อ SQLite
3. สร้าง Database File หากยังไม่มี
4. แสดงสถานะการเชื่อมต่อ
5. จัดการ Connection Error

### Part 2: Create Table

สร้าง Table `products`

ต้องมี Columns:

- id
- name
- price
- stock

กำหนด Primary Key และ NOT NULL ตามที่ระบุ

### Part 3: Insert Products

เพิ่มสินค้าอย่างน้อย 3 รายการ:

| Name     | Price | Stock |
| -------- | ----: | ----: |
| Keyboard |  1200 |    10 |
| Mouse    |   500 |    20 |
| Monitor  |  4500 |     5 |

ใช้ Parameterized Query ในการ Insert

### Part 4: Read Products

อ่านข้อมูลสินค้าทั้งหมดและแสดงผล

ข้อมูลที่ต้องแสดง:

- ID
- Name
- Price
- Stock

### Part 5: Database Verification

ต้องสามารถตรวจสอบได้ว่า:

- Database File ถูกสร้างแล้ว
- Table ถูกสร้างแล้ว
- มีข้อมูลสินค้าอย่างน้อย 3 รายการ
- สามารถปิดและเปิดโปรแกรมใหม่โดยข้อมูลยังอยู่

## Constraints

- ต้องใช้ Rust
- ต้องใช้ SQLite
- ต้องใช้ SQLx
- ต้องใช้ Tokio
- ต้องใช้ Parameterized Queries
- ต้องใช้ `Result` จัดการ Error
- ต้องไม่ใช้ String Concatenation สร้าง SQL จาก User Input
- ต้องไม่ใช้ `unwrap()` กับ Database Operations โดยไม่มีการจัดการ
- ต้องไม่สร้างข้อมูลซ้ำทุกครั้งที่รันโดยไม่มีการกำหนดพฤติกรรมที่ชัดเจน

## Expected Output

```text
SQLite Basics

Connecting to database...
Database connected!

Creating products table...
Table created!

Inserting products...
Inserted: Keyboard
Inserted: Mouse
Inserted: Monitor

Product List
--------------------------------
ID: 1
Name: Keyboard
Price: 1200
Stock: 10
--------------------------------
ID: 2
Name: Mouse
Price: 500
Stock: 20
--------------------------------
ID: 3
Name: Monitor
Price: 4500
Stock: 5
--------------------------------

Database operation completed.
```

หมายเหตุ: ID อาจแตกต่างจากตัวอย่าง ขึ้นอยู่กับข้อมูลเดิมใน Database

## Hints

ติดตั้ง Dependencies:

```powershell
cargo add sqlx --features runtime-tokio,sqlite
cargo add tokio --features full
```

แนวทาง:

- ใช้ `SqlitePoolOptions`
- ใช้ `connect()` หรือ Connection Options ที่รองรับการสร้างไฟล์
- ใช้ `sqlx::query()` สำหรับ Execute SQL
- ใช้ `.bind()` สำหรับส่ง Parameters
- ใช้ `.execute()` สำหรับคำสั่งสร้างหรือแก้ไขข้อมูล
- ใช้ `.fetch_all()` สำหรับอ่านข้อมูลหลายรายการ

## Completion Checklist

- [ ] สร้าง Cargo Project ได้
- [ ] ติดตั้ง SQLx ได้
- [ ] เชื่อมต่อ SQLite ได้
- [ ] สร้าง Database File ได้
- [ ] สร้าง Table ได้
- [ ] กำหนด Primary Key ได้
- [ ] Insert Products ได้
- [ ] Read Products ได้
- [ ] ใช้ Bind Parameters ได้
- [ ] จัดการ Database Error ได้
- [ ] ปิดและเปิดโปรแกรมใหม่แล้วข้อมูลยังอยู่

## Run

```powershell
cd 29_database_serialization/exercises/exercise_01/sqlite_basics

cargo check
cargo run
```
