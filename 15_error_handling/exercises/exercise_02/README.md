# Exercise 02: Result Practice

## Objective

ฝึกใช้ `Result<T, E>`, `Ok`, `Err`, `match` และ `unwrap_or()` เพื่อจัดการข้อผิดพลาดโดยไม่ทำให้โปรแกรม Panic โดยไม่จำเป็น

## Project

`result_practice`

## Scenario

สร้างโปรแกรมเครื่องคิดเลขอย่างง่ายที่สามารถหารตัวเลขและแปลงข้อความเป็นตัวเลข โดยตรวจสอบข้อผิดพลาดก่อนนำข้อมูลไปใช้งาน

## Requirements

### Part 1: Division Function

สร้าง Function:

```text
divide(a: f64, b: f64)
```

ข้อกำหนด:

- Return Type เป็น `Result<f64, String>`
- หากตัวหารเป็นศูนย์ ให้คืน `Err`
- หากหารได้ ให้คืน `Ok`
- ต้องไม่ใช้ `panic!` เพื่อจัดการกรณีตัวหารเป็นศูนย์

### Part 2: Parse Number

รับข้อความและทดลองแปลงเป็น `i32` ด้วย `parse()`

ทดสอบข้อมูล:

- `"100"`
- `"250"`
- `"abc"`

ใช้ `match` แสดงผลเมื่อแปลงสำเร็จหรือเกิด Error

### Part 3: Default Value

สร้าง Result ที่มีทั้งกรณี `Ok` และ `Err`

ทดลองใช้ `unwrap_or()` เพื่อกำหนดค่าเริ่มต้นเป็น `0`

### Part 4: Summary

แสดงผลการทดลองทั้งหมด โดยแบ่งเป็น:

- Division
- Number Conversion
- Default Value

## Expected Output

```text
=== Result Practice ===

--- Division ---
10 / 2 = 5
10 / 0 = Error: Cannot divide by zero

--- Number Conversion ---
Input: 100
Converted: 100

Input: 250
Converted: 250

Input: abc
Error: Invalid number

--- Default Value ---
Result: 0
```

ข้อความ Error อาจแตกต่างได้ แต่ต้องสื่อความหมายตรงกับสถานการณ์

## Constraints

- ต้องมี Function `divide()`
- ต้องคืนค่า `Result<f64, String>` จาก `divide()`
- ต้องใช้ `Ok` และ `Err`
- ต้องใช้ `match` จัดการผลลัพธ์
- ต้องทดลอง `parse()` กับข้อมูลที่แปลงได้และแปลงไม่ได้
- ต้องใช้ `unwrap_or()` อย่างน้อย 1 ครั้ง
- ห้ามใช้ `unwrap()` เพื่อจัดการ Input ที่อาจไม่ถูกต้อง
- โปรแกรมต้องไม่ Panic เมื่อได้รับข้อมูลตัวอย่างที่ไม่ถูกต้อง

## Checklist

- [ ] สร้าง Function `divide()`
- [ ] กำหนด Return Type เป็น Result
- [ ] จัดการกรณีหารด้วยศูนย์
- [ ] คืน `Ok` เมื่อหารสำเร็จ
- [ ] คืน `Err` เมื่อหารไม่สำเร็จ
- [ ] ทดลอง `parse()` กับข้อความตัวเลข
- [ ] ทดลอง `parse()` กับข้อความที่ไม่ใช่ตัวเลข
- [ ] ใช้ `match` จัดการ Result
- [ ] ใช้ `unwrap_or()` กำหนดค่าเริ่มต้น
- [ ] แสดงผลครบทุกส่วน
- [ ] โปรแกรมไม่ Panic จากข้อมูลตัวอย่างที่ไม่ถูกต้อง
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` ทำงานได้ตามต้องการ

## Challenge (Optional)

เพิ่ม Function `safe_parse()` สำหรับแปลงข้อความเป็น `i32` และคืนค่า `Result<i32, String>`

หากแปลงไม่สำเร็จ ให้คืน Error Message ที่เข้าใจง่าย แทนการแสดง Error ภายในของ Rust โดยตรง
