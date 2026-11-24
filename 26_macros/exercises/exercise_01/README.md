# Exercise 01: Macro Basics

## ระดับ

Easy

## Objective

ฝึกสร้าง Declarative Macro เบื้องต้นด้วย `macro_rules!` และรับ Arguments จากผู้เรียก

## Problem Statement

สร้าง Macro สำหรับแสดงข้อมูลและคำนวณค่าพื้นฐาน

ต้องสร้าง Macro อย่างน้อย 3 ตัว:

1. `greet!` สำหรับแสดงข้อความต้อนรับ
2. `show_value!` สำหรับแสดงค่าที่รับเข้ามา
3. `square!` สำหรับคำนวณกำลังสอง

## Requirements

### Part 1: greet!

1. สร้าง Macro ชื่อ `greet!`
2. ไม่ต้องรับ Arguments
3. แสดงข้อความต้อนรับ

### Part 2: show_value!

1. สร้าง Macro ชื่อ `show_value!`
2. รับ Expression หนึ่งรายการ
3. แสดงค่าที่ได้รับ

### Part 3: square!

1. สร้าง Macro ชื่อ `square!`
2. รับ Expression หนึ่งรายการ
3. คำนวณกำลังสอง
4. สามารถนำผลลัพธ์ไปกำหนดให้ตัวแปรได้

## Constraints

- ต้องใช้ `macro_rules!`
- ต้องสร้าง Macro อย่างน้อย 3 ตัว
- ต้องใช้เครื่องหมาย `!` ในการเรียก Macro
- ต้องมี Macro ที่รับ Arguments
- ต้องมี Macro ที่สร้างผลลัพธ์เป็น Expression
- ห้ามใช้ Procedural Macro
- ห้ามใช้ `unsafe`

## Expected Output

```text
Welcome to Rust Macros!

Value: 100
Value: 30

Square: 25
Square: 36
```

## Hints

- ใช้ `()` เป็น Pattern สำหรับ Macro ที่ไม่รับ Arguments
- ใช้ `$value:expr` สำหรับรับ Expression
- ใช้ `=>` แยก Pattern กับ Generated Code
- ใช้ Parentheses ครอบ Expression ที่นำไปคำนวณ

## Completion Checklist

- [ ] สร้าง `greet!`
- [ ] สร้าง `show_value!`
- [ ] สร้าง `square!`
- [ ] ใช้ `macro_rules!`
- [ ] รับ Expression ได้
- [ ] แสดงค่าที่ส่งเข้า Macro ได้
- [ ] นำผลลัพธ์จาก Macro ไปกำหนดตัวแปรได้
- [ ] ทดสอบด้วย Input หลายรูปแบบ

## Run

```powershell
cd 26_macros/exercises/exercise_01/macro_basics
cargo run
```
