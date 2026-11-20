# Exercise 02: Generic Functions

## Project

`generic_functions`

## Objective

ฝึกสร้าง Generic Functions ที่รองรับข้อมูลหลายชนิด โดยใช้ Type Parameters และ Trait Bounds

## Requirements

สร้างโปรแกรมสำหรับจัดการและแสดงข้อมูล โดยมี Function ต่อไปนี้

### Task 1: Display Function

สร้าง Function ชื่อ `display_data` ที่:

- รับข้อมูลหนึ่งตัว
- ใช้ Generic Type Parameter
- แสดงค่าด้วย `Display`

### Task 2: Compare Function

สร้าง Function ชื่อ `compare_values` ที่:

- รับข้อมูลสองตัว
- ใช้ Generic Type Parameter
- ตรวจสอบว่าข้อมูลทั้งสองมีค่าเท่ากันหรือไม่
- คืนค่าเป็น `bool`

กำหนด Trait Bound ที่จำเป็นเพื่อให้สามารถเปรียบเทียบข้อมูลได้

### Task 3: Find Maximum

สร้าง Function ชื่อ `find_max` ที่:

- รับข้อมูลสองตัว
- คืนค่าข้อมูลที่มีค่ามากกว่า
- รองรับชนิดข้อมูลที่สามารถเปรียบเทียบลำดับได้
- กำหนด Trait Bound ให้เหมาะสม

### Task 4: Test Different Data Types

ทดลองใช้ Function กับ:

- Integer
- Floating Point
- String

เลือกชนิดข้อมูลและค่าทดสอบให้สอดคล้องกับข้อกำหนดของแต่ละ Function

## Constraints

- ต้องใช้ Generic Type Parameters
- ห้ามสร้าง Function แยกตามชนิดข้อมูล
- ต้องกำหนด Trait Bounds ให้เหมาะสม
- Function `compare_values` ต้องใช้ความสามารถในการเปรียบเทียบความเท่ากัน
- Function `find_max` ต้องใช้ความสามารถในการเปรียบเทียบลำดับ
- ต้องทดสอบข้อมูลมากกว่าหนึ่งชนิด

## Expected Output

ตัวอย่างผลลัพธ์:

```text
Integer: 100
String: Rust

Are values equal? true

Maximum Integer: 90
Maximum Float: 25.5
Maximum String: Rust
```

ผลลัพธ์ของการเปรียบเทียบต้องสัมพันธ์กับข้อมูลที่กำหนด

## Checklist

- [ ] สร้าง `display_data` ได้
- [ ] สร้าง `compare_values` ได้
- [ ] สร้าง `find_max` ได้
- [ ] ใช้ Generic Type Parameters ได้
- [ ] กำหนด Trait Bounds ได้ถูกต้อง
- [ ] ทดสอบ Integer ได้
- [ ] ทดสอบ Floating Point ได้
- [ ] ทดสอบ String ได้
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 18_generics/exercises/exercise_02/generic_functions
cargo run
```
