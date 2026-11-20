# Exercise 04: Struct Challenge

## Objective

ประยุกต์ใช้ความรู้เรื่อง Structs, Instances, Fields, Methods และ Associated Functions เพื่อสร้างระบบจัดการข้อมูลสินค้าอย่างง่าย

## Project

`struct_challenge`

## Scenario

คุณได้รับมอบหมายให้สร้างโปรแกรมจัดการสินค้า โดยสินค้าแต่ละรายการมีข้อมูลชื่อ ราคา จำนวน และสถานะว่ามีสินค้าในคลังหรือไม่

โปรแกรมต้องสามารถแสดงข้อมูล คำนวณมูลค่าสินค้าทั้งหมด และปรับจำนวนสินค้าได้

## Requirements

สร้าง Struct ชื่อ `Product` โดยมี Fields ดังนี้:

| Field    | Data Type | Description       |
| -------- | --------- | ----------------- |
| name     | String    | ชื่อสินค้า        |
| price    | f64       | ราคาต่อชิ้น       |
| quantity | u32       | จำนวนสินค้า       |
| in_stock | bool      | สถานะสินค้าในคลัง |

## Tasks

### Part 1: Struct and Instances

1. ประกาศ Struct ชื่อ `Product`
2. สร้าง Associated Function `new()` สำหรับสร้างสินค้า
3. สร้าง Product อย่างน้อย 3 Instances
4. กำหนดข้อมูลสินค้าแต่ละรายการให้แตกต่างกัน

### Part 2: Create Methods

สร้าง Methods ดังต่อไปนี้:

| Method              | Requirement                         |
| ------------------- | ----------------------------------- |
| `display_info()`    | แสดงข้อมูลสินค้าทั้งหมด             |
| `total_value()`     | คำนวณราคา × จำนวนสินค้า             |
| `is_available()`    | ตรวจสอบว่าสินค้าพร้อมจำหน่ายหรือไม่ |
| `update_quantity()` | ปรับจำนวนสินค้า                     |
| `apply_discount()`  | คำนวณราคาหลังหักส่วนลด              |

ข้อกำหนดเพิ่มเติม:

- `display_info()` ใช้ `&self`
- `total_value()` ใช้ `&self` และคืนค่าเป็น `f64`
- `is_available()` ใช้ `&self` และคืนค่าเป็น `bool`
- `update_quantity()` ใช้ `&mut self`
- `apply_discount()` ใช้ `&self` และคืนราคาหลังหักส่วนลด โดยไม่เปลี่ยนราคาเดิมใน Struct

### Part 3: Test the Methods

ทดลองเรียก Methods กับสินค้าแต่ละรายการ:

1. แสดงข้อมูลสินค้าก่อนแก้ไข
2. คำนวณมูลค่าสินค้าทั้งหมด
3. ตรวจสอบสถานะสินค้า
4. เปลี่ยนจำนวนสินค้าด้วย `update_quantity()`
5. คำนวณราคาหลังหักส่วนลด
6. แสดงข้อมูลสินค้าอีกครั้งหลังแก้ไข

### Part 4: Additional Challenge

เพิ่ม Method ชื่อ `restock()` เพื่อเพิ่มจำนวนสินค้าในคลัง

ข้อกำหนด:

- รับจำนวนสินค้าที่ต้องการเพิ่มผ่าน Parameter
- ใช้ `&mut self`
- เพิ่มจำนวนเข้าไปจากจำนวนเดิม
- แสดงจำนวนสินค้าหลังเติมสินค้า

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์:

```text
=== Product Information ===

Name: Keyboard
Price: 1200
Quantity: 10
In Stock: true
Total Value: 12000

=== Availability
```
