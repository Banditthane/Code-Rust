# Exercise 02: Macro Rules Practice

## ระดับ

Easy-Medium

## Objective

ฝึกใช้ Pattern Matching, Fragment Specifiers, Multiple Patterns และ Repetition ใน Declarative Macro

## Problem Statement

สร้าง Macro สำหรับจัดการข้อมูลตัวเลขและข้อความ

ต้องมี Macro อย่างน้อย 4 ตัว:

1. `calculate!`
2. `show_items!`
3. `create_vec!`
4. `create_variable!`

## Requirements

### Part 1: calculate!

สร้าง Macro ชื่อ `calculate!` โดยรองรับรูปแบบ:

- รับตัวเลข 2 ค่าเพื่อบวก
- รับตัวเลข 2 ค่าเพื่อคูณ
- รับตัวเลข 1 ค่าเพื่อคำนวณกำลังสอง

ตัวอย่างการเรียกที่ต้องรองรับ:

```rust
calculate!(add, 10, 20);
calculate!(multiply, 5, 4);
calculate!(square, 6);
```

### Part 2: show_items!

1. รับ Expression ได้หลายรายการ
2. แสดงข้อมูลแต่ละรายการ
3. รองรับการรับข้อมูลอย่างน้อย 3 รายการ
4. ใช้ Repetition

### Part 3: create_vec!

1. รับ Expression หลายรายการ
2. สร้าง Vector จากข้อมูลที่รับเข้ามา
3. สามารถกำหนดผลลัพธ์ให้ตัวแปรได้
4. รองรับจำนวนรายการที่แตกต่างกัน

### Part 4: create_variable!

1. รับ Identifier หนึ่งรายการ
2. รับ Expression หนึ่งรายการ
3. สร้างตัวแปรตามชื่อที่รับเข้ามา
4. สามารถนำตัวแปรที่สร้างไปใช้งานต่อได้

## Constraints

- ต้องใช้ `macro_rules!`
- ต้องมี Macro อย่างน้อย 4 ตัว
- ต้องใช้ Fragment Specifier อย่างน้อย 2 ประเภท
- ต้องใช้ Repetition อย่างน้อย 1 Macro
- ต้องมี Multiple Patterns อย่างน้อย 1 Macro
- ต้องมี Macro ที่สร้าง Expression
- ห้ามใช้ Procedural Macro
- ห้ามใช้ `unsafe`

## Expected Output

```text
Addition: 30
Multiplication: 20
Square: 36

Items:
10
20
30

Vector: [1, 2, 3, 4, 5]

Created variable: 100
```

สามารถออกแบบข้อความและรูปแบบการแสดงผลเพิ่มเติมได้

## Hints

- ใช้ `$operation:ident` สำหรับรับ Identifier
- ใช้ `$value:expr` สำหรับรับ Expression
- ใช้ `$($item:expr),*` สำหรับรับหลายรายการ
- ใช้ `vec![$($item),*]` เป็นแนวทางสร้าง Vector
- ใช้ Multiple Patterns เพื่อรองรับ Operation หลายรูปแบบ

## Completion Checklist

- [ ] สร้าง `calculate!`
- [ ] รองรับ Addition
- [ ] รองรับ Multiplication
- [ ] รองรับ Square
- [ ] สร้าง `show_items!`
- [ ] ใช้ Repetition
- [ ] สร้าง `create_vec!`
- [ ] สร้าง `create_variable!`
- [ ] ใช้ Fragment Specifiers อย่างน้อย 2 ประเภท
- [ ] ทดสอบ Input หลายรูปแบบ
- [ ] ตรวจสอบกรณี Input ไม่ตรง Pattern

## Run

```powershell
cd 26_macros/exercises/exercise_02/macro_rules_practice
cargo run
```
