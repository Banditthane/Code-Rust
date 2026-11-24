# Exercise 02: Database CRUD

## ระดับ

Easy-Medium

## Objective

ฝึกสร้างระบบจัดการข้อมูลสินค้าโดยใช้ SQLx และ SQLite พร้อมทำ CRUD Operations ให้ครบถ้วน

## Problem Statement

พัฒนาระบบ Product Management จาก Exercise 01

ระบบต้องสามารถ:

1. เพิ่มสินค้า
2. แสดงสินค้าทั้งหมด
3. ค้นหาสินค้าตาม ID
4. แก้ไขข้อมูลสินค้า
5. ลบสินค้า
6. ตรวจสอบข้อมูลก่อนบันทึก
7. จัดการกรณีไม่พบสินค้า

## Database Schema

สร้าง Table:

```sql
CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    stock INTEGER NOT NULL
);
```

## Requirements

### Part 1: Product Model

สร้าง Struct:

```text
Product
```

กำหนด Fields:

- id
- name
- price
- stock

เลือก Rust Types ให้เหมาะสมกับ Database Columns

### Part 2: Create Product

สร้าง Function สำหรับเพิ่มสินค้า

ต้องตรวจสอบ:

- Name ต้องไม่ว่าง
- Price ต้องไม่ติดลบ
- Stock ต้องไม่ติดลบ

ใช้ Parameterized Query

### Part 3: Read Products

สร้าง Function:

```text
get_all_products()
```

อ่านสินค้าทั้งหมดและคืนค่าเป็น Collection

สร้าง Function:

```text
get_product_by_id()
```

ค้นหาสินค้าตาม ID

กรณีไม่พบสินค้า ต้องสามารถแยกจากกรณี Database Error ได้

### Part 4: Update Product

สร้าง Function:

```text
update_product()
```

สามารถแก้ไข:

- Name
- Price
- Stock

ต้องตรวจสอบว่ามีสินค้าตาม ID ก่อนหรือไม่ หรือใช้ Rows Affected เพื่อตรวจสอบผลการ Update

### Part 5: Delete Product

สร้าง Function:

```text
delete_product()
```

ลบสินค้าตาม ID

ต้องตรวจสอบผลการลบและจัดการกรณีไม่พบข้อมูล

### Part 6: Search Products

เพิ่มความสามารถค้นหาสินค้าด้วยชื่อ

ตัวอย่าง:

```text
Search: key
```

ผลลัพธ์ควรแสดงสินค้าที่มีคำว่า `key` อยู่ในชื่อ

ใช้ Parameterized Query และพิจารณาการ Escape Wildcard หากต้องการให้ `%` และ `_` เป็นตัวอักษรธรรมดา

### Part 7: Sorting

เพิ่มความสามารถเรียงลำดับ:

- Price Ascending
- Price Descending
- Name Ascending
- Stock Descending

ข้อควรระวัง: ชื่อ Column หรือ SQL Keywords ไม่สามารถส่งเป็น Value Parameter ได้โดยตรง ต้องเลือกจากรายการที่อนุญาตเท่านั้น

## Constraints

- ต้องใช้ SQLite
- ต้องใช้ SQLx
- ต้องมี Product Struct
- ต้องทำ CRUD ครบ
- ต้องใช้ Parameterized Queries
- ต้องตรวจสอบ Input
- ต้องจัดการ Not Found
- ต้องจัดการ Database Error
- ต้องไม่ใช้ SQL String ที่ประกอบจาก User Input โดยตรง
- ต้องแยก Database Logic ออกจากการแสดงผล
- ต้องไม่ลบหรือแก้ไขข้อมูลทั้งหมดโดยไม่ได้ตั้งใจ

## Expected Output

```text
Product Management System

1. Create Product
2. List Products
3. Find Product
4. Update Product
5. Delete Product
6. Search Products
7. Sort Products
0. Exit

Select: 1

Product Name: Keyboard
Price: 1200
Stock: 10

Product created successfully!
```

ตัวอย่างการค้นหา:

```text
Select: 6

Search keyword: key

Search Results
--------------------------------
ID: 1
Name: Keyboard
Price: 1200
Stock: 10
--------------------------------
```

ตัวอย่างกรณีไม่พบ:

```text
Select: 3

Product ID: 999

Product not found.
```

## Hints

- ใช้ `SqlitePool`
- ใช้ `sqlx::query_as()` สำหรับ Mapping Rows ไปยัง Struct
- ใช้ `FromRow` หรือวิธี Mapping ที่เหมาะสม
- ใช้ `fetch_optional()` สำหรับค้นหาหนึ่งรายการที่อาจไม่มีข้อมูล
- ใช้ `rows_affected()` ตรวจสอบการ Update และ Delete
- ใช้ `Result` แยก Database Error
- แยก Validation Error ออกจาก Database Error

## Completion Checklist

- [ ] สร้าง Product Model ได้
- [ ] Create Product ได้
- [ ] Read All Products ได้
- [ ] Find Product By ID ได้
- [ ] Update Product ได้
- [ ] Delete Product ได้
- [ ] Search Products ได้
- [ ] Sort Products ได้
- [ ] ตรวจสอบ Name ได้
- [ ] ตรวจสอบ Price ได้
- [ ] ตรวจสอบ Stock ได้
- [ ] จัดการ Not Found ได้
- [ ] จัดการ Database Error ได้
- [ ] ใช้ Parameterized Queries ทุกจุดที่รับ Input
- [ ] แยก Database Logic จาก User Interface ได้

## Run

```powershell
cd 29_database_serialization/exercises/exercise_02/database_crud

cargo check
cargo run
```
