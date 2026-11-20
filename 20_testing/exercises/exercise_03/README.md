# Exercise 03: Integration Testing

## Project

`integration_testing`

## Objective

ฝึกสร้าง Library ที่แบ่งการทำงานออกเป็นหลาย Modules และทดสอบการทำงานร่วมกันผ่าน Public API ด้วย Integration Tests

## Project Scenario

สร้างระบบจัดการสินค้า (Product Management System) โดยแบ่งการทำงานเป็น Modules

## Requirements

### Task 1: Create Library

สร้าง Library Crate และกำหนด Modules ดังนี้:

```text
src/
├── lib.rs
├── product.rs
├── inventory.rs
└── order.rs
```

### Task 2: Product Module

สร้าง Struct `Product` โดยมีข้อมูล:

- `id`
- `name`
- `price`

กำหนด Fields และ Methods ให้เหมาะสม

### Task 3: Inventory Module

สร้างระบบ Inventory สำหรับ:

- เพิ่มสินค้า
- ค้นหาสินค้าด้วย ID
- ตรวจสอบจำนวนสินค้า
- ลบสินค้า

### Task 4: Order Module

สร้างระบบ Order สำหรับ:

- สร้างรายการสั่งซื้อ
- คำนวณราคารวม
- ตรวจสอบว่าสินค้ามีอยู่หรือไม่
- ตรวจสอบจำนวนสินค้าให้เพียงพอก่อนสั่งซื้อ

### Task 5: Public API

กำหนด Public Functions และ Types ที่จำเป็นสำหรับการใช้งานจากภายนอก Library

ไม่จำเป็นต้องเปิดเผย Internal Implementation ทั้งหมด

### Task 6: Create Integration Tests

สร้างโฟลเดอร์:

```text
tests/
├── product_test.rs
├── inventory_test.rs
└── order_test.rs
```

ทดสอบ Public API จากแต่ละ Module

### Task 7: Test Multiple Modules

ทดสอบการทำงานร่วมกัน:

1. สร้างสินค้า
2. เพิ่มสินค้าเข้า Inventory
3. ค้นหาสินค้า
4. สร้าง Order
5. คำนวณราคารวม
6. ตรวจสอบกรณีสินค้าไม่เพียงพอ

## Constraints

- ต้องสร้าง Library Crate
- ต้องมี Modules อย่างน้อย 3 Modules
- ต้องมี Struct `Product`
- ต้องมีระบบ Inventory
- ต้องมีระบบ Order
- ต้องมี Public API ที่เหมาะสม
- ต้องสร้าง Integration Test Files อย่างน้อย 3 ไฟล์
- Integration Tests ต้องทดสอบผ่าน Public API
- ต้องทดสอบ Normal Cases
- ต้องทดสอบ Error Cases
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- ต้องจัดการข้อมูลผิดพลาดอย่างเหมาะสม
- Tests ทั้งหมดต้องผ่าน

## Expected Output

ตัวอย่างผลลัพธ์:

```text
running integration tests

test product_test::test_create_product ... ok
test inventory_test::test_add_product ... ok
test inventory_test::test_find_product ... ok
test inventory_test::test_remove_product ... ok
test order_test::test_create_order ... ok
test order_test::test_calculate_total ... ok
test order_test::test_insufficient_inventory ... ok

test result: ok. 7 passed; 0 failed
```

จำนวนและชื่อ Tests สามารถแตกต่างได้ตามการออกแบบ

## Checklist

- [ ] สร้าง Library Crate ได้
- [ ] สร้าง Module `product` ได้
- [ ] สร้าง Module `inventory` ได้
- [ ] สร้าง Module `order` ได้
- [ ] สร้าง Struct `Product` ได้
- [ ] สร้าง Public API ได้
- [ ] สร้างโฟลเดอร์ `tests/` ได้
- [ ] สร้าง Integration Test Files ได้
- [ ] ทดสอบ Public API ได้
- [ ] ทดสอบการทำงานร่วมกันของ Modules ได้
- [ ] ทดสอบ Error Cases ได้
- [ ] Integration Tests ทั้งหมดผ่าน

## Run Project

```powershell
cd 20_testing/exercises/exercise_03/integration_testing
cargo test
```

รัน Integration Test เฉพาะไฟล์:

```powershell
cargo test --test product_test
cargo test --test inventory_test
cargo test --test order_test
```
