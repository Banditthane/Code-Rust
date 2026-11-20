# Exercise 02: Move Challenge

## Objective

ฝึกทำความเข้าใจ Move Semantics และการย้าย Ownership ระหว่างตัวแปร

## Project

`move_challenge`

## Tasks

เขียนโปรแกรมทดลอง 3 ส่วน:

### Part 1: Move Between Variables

1. สร้าง String ชื่อ `language1`
2. กำหนดค่า `language1` ให้กับ `language2`
3. แสดงค่าของ `language2`
4. วิเคราะห์ว่า `language1` ยังใช้งานได้หรือไม่

### Part 2: Move into Function

1. สร้าง Function ชื่อ `show_language`
2. กำหนดให้ Function รับ String เป็น Parameter
3. แสดงข้อความภายใน Function
4. สร้าง String ใน `main()`
5. ส่ง String เข้า Function

### Part 3: Return Ownership

1. สร้าง Function ชื่อ `create_language`
2. ให้ Function คืนค่าเป็น String
3. รับค่าที่คืนมาด้วยตัวแปร `language`
4. แสดงค่าของ `language`

## Requirements

- ต้องใช้ `String::from()`
- ต้องมี Function อย่างน้อย 2 Function
- ต้องมีการส่ง String เข้า Function
- ต้องมี Function ที่คืนค่า String
- ต้องแสดงผลด้วย `println!()`

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Language: Rust
Function received: Python
Returned language: Rust
```

## Constraints

- Part 1 ต้องทดลอง Move โดยเฉพาะ
- ห้ามนำตัวแปรที่ถูก Move ไปแล้วมาใช้งานต่อ
- แยกการทดลองกรณีที่ทำให้เกิด Compile Error ออกจากโปรแกรมที่ต้องรันผ่าน
- ไม่ใช้ `.clone()` ในแบบฝึกหัดนี้

## Checklist

- [ ] เข้าใจการ Move ระหว่างตัวแปร
- [ ] เข้าใจการส่ง Ownership เข้า Function
- [ ] เข้าใจการคืน Ownership จาก Function
- [ ] รู้จักข้อจำกัดของตัวแปรหลัง Move
- [ ] โปรแกรมส่วนที่ต้องรันผ่านไม่มี Compile Error

## Run Project

```powershell
cargo check
cargo run
```
