# Exercise 01: Generic Basics

## Project

`generic_basics`

## Objective

ฝึกทำความเข้าใจและใช้งาน Generic Type Parameters โดยสร้าง Function ที่สามารถรับข้อมูลหลายชนิด

## Requirements

สร้างโปรแกรมที่มี Function สำหรับจัดการข้อมูล โดยใช้ Generic Type Parameter

### Task 1: Generic Function

สร้าง Function ชื่อ `display_value` ที่:

- รับ Parameter หนึ่งตัว
- ใช้ Generic Type Parameter ชื่อ `T`
- แสดงค่าที่รับเข้ามาด้วย `Debug`

### Task 2: Test Different Data Types

ทดลองเรียกใช้ Function ด้วยข้อมูลต่อไปนี้:

- Integer: `100`
- Floating Point: `25.5`
- Boolean: `true`
- String: `"Rust"`
- Character: `'R'`

### Task 3: Generic Type Understanding

สร้าง Function อีกหนึ่งตัวชื่อ `return_value` ที่:

- รับข้อมูลชนิด T
- ส่งข้อมูลชนิด T กลับ
- ทดลองเรียกใช้กับ Integer และ String

## Constraints

- ต้องใช้ Generic Type Parameter
- ห้ามสร้าง Function แยกสำหรับแต่ละ Data Type
- ต้องกำหนด Trait Bound ที่จำเป็นสำหรับการแสดงผลด้วย `Debug`
- ต้องใช้ Function ที่สร้างขึ้นจริงในการทดลอง
- ห้ามใช้ `unwrap()` โดยไม่มีความจำเป็น

## Expected Output

ตัวอย่างผลลัพธ์ที่ต้องการ:

```text
Value: 100
Value: 25.5
Value: true
Value: "Rust"
Value: 'R'

Returned Integer: 500
Returned String: Generics
```

รูปแบบข้อความสามารถแตกต่างได้ แต่ต้องแสดงข้อมูลครบตามโจทย์

## Checklist

- [ ] สร้าง Generic Function ได้
- [ ] ใช้ Type Parameter `T` ได้
- [ ] ใช้ Trait Bound `Debug` ได้
- [ ] ทดสอบข้อมูลหลายชนิดได้
- [ ] สร้าง Function ที่คืนค่า Generic Type ได้
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 18_generics/exercises/exercise_01/generic_basics
cargo run
```
