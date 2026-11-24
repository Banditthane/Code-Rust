# Exercise 04: Macro Challenge

## ระดับ

Challenge

## Objective

ประยุกต์ใช้ Declarative Macro และ Procedural Macro เพื่อสร้างเครื่องมือสำหรับลดโค้ดซ้ำในระบบจัดการข้อมูล

## Problem Statement

สร้างระบบ Product Management ที่ใช้ Macro ช่วยสร้างโค้ดสำหรับแสดงข้อมูล ตรวจสอบข้อมูล และสร้างโครงสร้างข้อมูล

ระบบต้องประกอบด้วย:

1. Declarative Macro สำหรับแสดงข้อมูล
2. Declarative Macro สำหรับสร้าง Collection
3. Declarative Macro สำหรับตรวจสอบข้อมูล
4. Derive Macro สำหรับสร้าง Method อัตโนมัติ

## Requirements

### Part 1: Display Macro

สร้าง Macro ชื่อ `display_info!`

ความสามารถ:

1. รับข้อมูลหลายรายการ
2. แสดงชื่อและค่าของแต่ละรายการ
3. รองรับ Arguments หลายรายการ
4. ใช้ Repetition

### Part 2: Collection Macro

สร้าง Macro ชื่อ `create_collection!`

ความสามารถ:

1. รับข้อมูลหลายรายการ
2. สร้าง Vector
3. รองรับ Expression
4. รองรับจำนวนรายการที่แตกต่างกัน

### Part 3: Validation Macro

สร้าง Macro ชื่อ `validate!`

ความสามารถ:

1. รับ Condition
2. รับข้อความ Error
3. ตรวจสอบ Condition
4. ส่งผลลัพธ์ที่สามารถนำไปจัดการต่อได้
5. รองรับกรณีผ่านและไม่ผ่าน

### Part 4: Derive Macro

สร้าง Derive Macro ชื่อ `ProductInfo`

ความสามารถ:

1. ใช้กับ Product Struct
2. อ่านชื่อ Struct และ Fields
3. สร้าง Method สำหรับแสดงข้อมูล
4. สร้าง Method สำหรับนับจำนวน Fields หรือแสดงข้อมูลตามรูปแบบที่กำหนด
5. รองรับ Named Fields
6. จัดการ Input ที่ไม่รองรับด้วย Compile Error

### Part 5: Application

สร้างระบบ Product Management ที่มีข้อมูลอย่างน้อย:

| Field    | Type   |
| -------- | ------ |
| id       | u32    |
| name     | String |
| price    | f64    |
| quantity | u32    |

ระบบต้องสามารถ:

1. สร้าง Product
2. แสดงข้อมูล Product
3. เก็บ Product หลายรายการใน Vector
4. ตรวจสอบ Price และ Quantity
5. ใช้ Derive Macro กับ Product Struct
6. ใช้ Declarative Macro ในส่วนที่เหมาะสม

## Constraints

- ต้องมี Declarative Macro อย่างน้อย 3 ตัว
- ต้องมี Procedural Derive Macro อย่างน้อย 1 ตัว
- ต้องใช้ `macro_rules!`
- ต้องใช้ `syn`
- ต้องใช้ `quote`
- ต้องมี Procedural Macro Crate แยกต่างหาก
- ต้องมี Application Crate สำหรับใช้งาน Macro
- ต้องจัดการ Invalid Input
- ห้ามใช้ `unsafe`
- ห้ามใช้ Macro แทน Function ทุกกรณี
- ต้องมี Test สำหรับ Macro ที่สร้างขึ้น

## Expected Output

```text
Product Management System

Product 1
ID: 1001
Name: Keyboard
Price: 1200
Quantity: 10

Product 2
ID: 1002
Name: Mouse
Price: 500
Quantity: 20

Validation
Product 1: Valid
Product 2: Valid

Total Products: 2

All checks completed
```

รูปแบบการแสดงผลสามารถออกแบบเพิ่มเติมได้

## Hints

- แยก Declarative Macro และ Procedural Macro ออกจากกัน
- ใช้ Repetition สำหรับรับข้อมูลหลายรายการ
- ใช้ `Result` สำหรับ Validation ที่อาจเกิด Error
- ใช้ `syn::DeriveInput` สำหรับ Parse Struct
- ใช้ `quote!` สำหรับสร้าง Method
- ใช้ Path Dependency เชื่อม Procedural Macro Crate กับ Application
- ทดสอบ Macro แต่ละตัวแยกกันก่อนนำมารวมใน Application

## Completion Checklist

### Declarative Macros

- [ ] สร้าง `display_info!`
- [ ] สร้าง `create_collection!`
- [ ] สร้าง `validate!`
- [ ] ใช้ Pattern Matching
- [ ] ใช้ Repetition

### Procedural Macro

- [ ] สร้าง Procedural Macro Crate
- [ ] สร้าง `ProductInfo`
- [ ] Parse Struct ด้วย `syn`
- [ ] Generate Code ด้วย `quote`
- [ ] รองรับ Named Fields
- [ ] จัดการ Invalid Input

### Application

- [ ] สร้าง Product Struct
- [ ] ใช้ Derive Macro
- [ ] สร้าง Product หลายรายการ
- [ ] ใช้ Declarative Macro
- [ ] ตรวจสอบข้อมูล Product
- [ ] แสดงข้อมูลครบถ้วน

### Testing

- [ ] ทดสอบ Declarative Macros
- [ ] ทดสอบ Derive Macro
- [ ] ทดสอบ Invalid Input
- [ ] ตรวจสอบ Generated Code
- [ ] โปรแกรมทำงานได้โดยไม่มี `unsafe`

## Run

```powershell
cd 26_macros/exercises/exercise_04/macro_challenge

cargo check
cargo test
cargo run
```

## Final Challenge

เมื่อทำโจทย์สำเร็จแล้ว ให้ทดลองเพิ่มความสามารถ:

1. เพิ่ม Attribute Macro สำหรับตรวจสอบ Struct
2. เพิ่ม Function-like Macro สำหรับสร้างข้อมูล Product
3. รองรับ Generic Struct
4. รองรับ Tuple Struct
5. เพิ่ม Error Message ที่ระบุตำแหน่งปัญหา
6. ตรวจสอบ Macro Expansion ด้วย `cargo expand`

**หมายเหตุ:** ควรแยก Macro ที่สร้างขึ้นเป็น Library Crate และแยก Application ที่นำ Macro ไปใช้งาน เพื่อให้โครงสร้างโปรเจกต์ชัดเจนและนำกลับไปใช้ซ้ำได้
