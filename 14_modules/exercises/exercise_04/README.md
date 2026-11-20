# Exercise 04: Module Challenge

## Objective

ประยุกต์ใช้ความรู้เรื่อง Modules, Module Tree, File Modules, Visibility และ Paths เพื่อสร้างโปรแกรมจัดการร้านค้าอย่างเป็นระบบ

## Project

`module_challenge`

## Scenario

สร้างโปรแกรม Store Management System โดยแบ่งส่วนการทำงานออกเป็น Modules เพื่อจัดการสินค้า ลูกค้า และคำสั่งซื้อ

แต่ละ Module ต้องมีหน้าที่ชัดเจน และสามารถเรียกใช้งานจาก `main.rs` ได้

## Required Structure

จัดโครงสร้างภายใน `src/` ดังนี้:

```text
src/
├── main.rs
├── products.rs
├── customers.rs
└── orders.rs
```

สามารถเพิ่มไฟล์หรือ Nested Modules ได้ตามความเหมาะสม

## Requirements

### Part 1: Products Module

สร้าง Module สำหรับจัดการสินค้า

ประกาศ Struct ชื่อ `Product` โดยมี Fields:

| Field    | Data Type | Visibility |
| -------- | --------- | ---------- |
| name     | String    | Public     |
| price    | f64       | Private    |
| quantity | u32       | Private    |

สร้าง Methods:

| Method           | Requirement            |
| ---------------- | ---------------------- |
| `new()`          | สร้างสินค้า            |
| `display_info()` | แสดงข้อมูลสินค้า       |
| `total_value()`  | คำนวณ price × quantity |

### Part 2: Customers Module

สร้าง Module สำหรับจัดการลูกค้า

ประกาศ Struct ชื่อ `Customer` โดยมี Fields:

| Field | Data Type | Visibility |
| ----- | --------- | ---------- |
| name  | String    | Public     |
| email | String    | Private    |

สร้าง Methods:

| Method           | Requirement       |
| ---------------- | ----------------- |
| `new()`          | สร้างข้อมูลลูกค้า |
| `display_info()` | แสดงข้อมูลลูกค้า  |

### Part 3: Orders Module

สร้าง Module สำหรับจัดการคำสั่งซื้อ

ประกาศ Enum ชื่อ `OrderStatus` โดยมี Variants:

- Pending
- Processing
- Completed
- Cancelled

ประกาศ Struct ชื่อ `Order` โดยมี Fields:

| Field         | Data Type   | Visibility |
| ------------- | ----------- | ---------- |
| order_id      | u32         | Public     |
| customer_name | String      | Private    |
| status        | OrderStatus | Private    |

สร้าง Methods:

| Method            | Requirement            |
| ----------------- | ---------------------- |
| `new()`           | สร้างคำสั่งซื้อ        |
| `display_info()`  | แสดงข้อมูลคำสั่งซื้อ   |
| `update_status()` | เปลี่ยนสถานะคำสั่งซื้อ |

การแสดงสถานะต้องใช้ `match` และจัดการ Variants ให้ครบทุกกรณี

### Part 4: Main Program

ภายใน `main.rs`:

1. ประกาศ Modules ทั้งสาม
2. นำ Items ที่จำเป็นเข้ามาใช้งาน
3. สร้าง Product อย่างน้อย 2 รายการ
4. สร้าง Customer อย่างน้อย 1 รายการ
5. สร้าง Order อย่างน้อย 2 รายการ
6. แสดงข้อมูลของสินค้า ลูกค้า และคำสั่งซื้อ
7. แสดงมูลค่าสินค้าแต่ละรายการ
8. ทดลองเปลี่ยนสถานะคำสั่งซื้อ

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์:

```text
=== Store Management System ===

--- Products ---
Product: Keyboard
Price: 1200
Quantity: 10
Total Value: 12000

Product: Mouse
Price: 500
Quantity: 20
Total Value: 10000

--- Customers ---
Name: Alice
Email: alice@example.com

--- Orders ---
Order ID: 1001
Customer: Alice
Status: Pending

Order ID: 1002
Customer: Alice
Status: Processing

--- Update Order ---
Order ID: 1001
Old Status: Pending
New Status: Completed
```

หมายเหตุ: ข้อมูลตัวอย่างสามารถเปลี่ยนได้ แต่ต้องแสดงข้อมูลครบตามข้อกำหนด

## Constraints

- ต้องแบ่งโค้ดออกเป็น Modules ตามหน้าที่
- ต้องมีไฟล์ `products.rs`, `customers.rs` และ `orders.rs`
- ต้องใช้ Struct และ Enum ตามที่กำหนด
- ต้องควบคุม Visibility ของ Fields
- ต้องใช้ Methods ในการจัดการ Private Fields
- ต้องใช้ `match` แสดงสถานะคำสั่งซื้อ
- ต้องเรียกใช้งาน Modules จาก `main.rs`
- ห้ามรวมการทำงานทั้งหมดไว้ใน `main.rs`
- ต้องใช้ Paths หรือ `use` ในการเรียก Items
- โปรแกรมต้อง Compile และ Run ได้

## Checklist

- [ ] สร้างไฟล์ `products.rs`
- [ ] สร้างไฟล์ `customers.rs`
- [ ] สร้างไฟล์ `orders.rs`
- [ ] ประกาศ Modules ใน `main.rs`
- [ ] สร้าง Struct `Product`
- [ ] สร้าง Methods ของ Product
- [ ] สร้าง Struct `Customer`
- [ ] สร้าง Methods ของ Customer
- [ ] สร้าง Enum `OrderStatus`
- [ ] สร้าง Struct `Order`
- [ ] สร้าง Methods ของ Order
- [ ] ใช้ `match` จัดการ OrderStatus
- [ ] กำหนด Public และ Private Items ถูกต้อง
- [ ] ใช้ Module Paths หรือ `use`
- [ ] สร้าง Instances ตามจำนวนที่กำหนด
- [ ] แสดงข้อมูลครบทุก Module
- [ ] ทดลองเปลี่ยนสถานะคำสั่งซื้อ
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` ทำงานได้ตามต้องการ

## Completion Criteria

ถือว่าผ่าน Exercise นี้เมื่อ:

1. แบ่งโปรแกรมออกเป็น Modules ได้ถูกต้อง
2. Module แต่ละตัวมีหน้าที่ชัดเจน
3. สามารถเรียก Items ข้าม Module ได้
4. ควบคุม Visibility ได้ถูกต้อง
5. ใช้ Struct, Enum และ Methods ร่วมกันได้
6. โปรแกรม Compile และ Run ได้โดยไม่มี Error
