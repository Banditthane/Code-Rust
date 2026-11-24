# Exercise 02: Backend and Data Project

## ระดับ

Easy-Medium

## Objective

ฝึกสร้าง Backend Application ที่เชื่อมต่อ Database และให้บริการผ่าน REST API

## Problem Statement

พัฒนา **Inventory Management API**

ระบบจัดการสินค้าในคลัง โดยผู้ใช้สามารถ:

- เพิ่มสินค้า
- แสดงรายการสินค้า
- ดูรายละเอียดสินค้า
- แก้ไขข้อมูลสินค้า
- ลบสินค้า
- ค้นหาสินค้า
- ปรับจำนวนสินค้าในคลัง

## Requirements

### Part 1: Project Setup

สร้าง Rust Backend Project

เลือกใช้:

- Axum
- Tokio
- Serde
- SQLx
- SQLite หรือ PostgreSQL

### Part 2: Product Model

สร้าง Product Model โดยมี Fields อย่างน้อย:

| Field       | Type ตัวอย่าง                     |
| ----------- | --------------------------------- |
| id          | i64                               |
| name        | String                            |
| description | Option<String>                    |
| price       | Decimal หรือชนิดข้อมูลที่เหมาะสม  |
| quantity    | i64                               |
| created_at  | DateTime หรือชนิดข้อมูลที่เหมาะสม |

ข้อควรระวัง: ไม่ควรใช้ Floating Point เป็นตัวแทนเงินในระบบการเงินจริง ควรเลือก Decimal Type หรือเก็บเป็นจำนวนหน่วยย่อยแบบ Integer

### Part 3: Database

สร้าง Products Table

ต้องมี:

- Primary Key
- Product Name
- Description
- Price
- Quantity
- Created At

ใช้ SQLx Migrations สำหรับจัดการ Schema

### Part 4: REST API

สร้าง Endpoints อย่างน้อย:

| Method | Endpoint                | หน้าที่           |
| ------ | ----------------------- | ----------------- |
| GET    | /products               | แสดงสินค้าทั้งหมด |
| GET    | /products/{id}          | ดูสินค้า          |
| POST   | /products               | เพิ่มสินค้า       |
| PUT    | /products/{id}          | แก้ไขสินค้า       |
| DELETE | /products/{id}          | ลบสินค้า          |
| PATCH  | /products/{id}/quantity | ปรับจำนวนสินค้า   |

### Part 5: Validation

ตรวจสอบ:

- Product Name ต้องไม่ว่าง
- Price ต้องไม่ติดลบ
- Quantity ต้องไม่ติดลบ
- ID ต้องมีรูปแบบถูกต้อง
- Product ต้องมีอยู่ก่อน Update หรือ Delete

### Part 6: Business Logic

เพิ่มความสามารถ:

- ค้นหาสินค้าจากชื่อ
- กรองสินค้าตามจำนวน
- ตรวจสอบสินค้าที่ใกล้หมด
- ปรับจำนวนสินค้า
- ป้องกัน Quantity ติดลบ

### Part 7: Error Handling

กำหนด Error Cases:

- Product Not Found
- Invalid Input
- Duplicate Product (หากกำหนดชื่อให้ไม่ซ้ำ)
- Database Error
- Internal Error

ส่ง HTTP Status Codes ให้เหมาะสม

### Part 8: Testing

ทดสอบ:

- Create Product
- Read Product
- Update Product
- Delete Product
- Search Product
- Invalid Input
- Product Not Found
- Quantity Validation

ใช้ Test Database แยกจากข้อมูลจริง

## Constraints

- ต้องใช้ Rust
- ต้องใช้ Axum
- ต้องใช้ Tokio
- ต้องใช้ Serde
- ต้องใช้ SQLx
- ต้องมี Database Migrations
- ต้องมี REST API อย่างน้อย 5 Endpoints
- ต้องมี Input Validation
- ต้องมี Error Handling
- ต้องใช้ Parameterized Queries
- ต้องมี Unit Tests
- ต้องมี Integration Tests
- ต้องไม่ใช้ Production Database สำหรับ Tests
- ต้องไม่เก็บ Secrets ใน Source Code

## Expected Output

ตัวอย่าง Create Product:

Request:

```http
POST /products
Content-Type: application/json
```

```json
{
  "name": "Keyboard",
  "description": "Mechanical Keyboard",
  "price": 2500,
  "quantity": 20
}
```

ตัวอย่าง Response:

```json
{
  "id": 1,
  "name": "Keyboard",
  "description": "Mechanical Keyboard",
  "price": 2500,
  "quantity": 20
}
```

ตัวอย่าง List:

```json
{
  "items": [
    {
      "id": 1,
      "name": "Keyboard",
      "price": 2500,
      "quantity": 20
    }
  ]
}
```

รูปแบบ Response สามารถออกแบบเองได้ แต่ต้องสม่ำเสมอและมีข้อมูลครบตาม Requirements

## Hints

แนวทางแบ่ง Modules:

```text
src/
├── main.rs
├── lib.rs
├── models/
├── routes/
├── services/
├── repositories/
├── errors.rs
└── config.rs
```

แนะนำให้แยก:

- Request Models
- Response Models
- Database Models

ตามความจำเป็นของระบบ

## Completion Checklist

- [ ] สร้าง Backend Project ได้
- [ ] สร้าง Product Model ได้
- [ ] สร้าง Database Schema ได้
- [ ] ใช้ SQLx Migrations ได้
- [ ] เชื่อมต่อ Database ได้
- [ ] สร้าง REST API ได้
- [ ] เพิ่มสินค้าได้
- [ ] แสดงสินค้าได้
- [ ] แก้ไขสินค้าได้
- [ ] ลบสินค้าได้
- [ ] ค้นหาสินค้าได้
- [ ] ตรวจสอบ Quantity ได้
- [ ] จัดการ Errors ได้
- [ ] เขียน Unit Tests ได้
- [ ] เขียน Integration Tests ได้

## Deliverables

- Source Code
- Database Migrations
- API Documentation
- README.md
- Unit Tests
- Integration Tests
- Sample API Requests

## Run

```powershell
cargo check
cargo test
cargo run
```
