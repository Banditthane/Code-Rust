# Exercise 01: References

## Objective

ฝึกสร้างและใช้งาน Reference เพื่อเข้าถึงข้อมูลโดยไม่ย้าย Ownership

## Project

`references`

## Tasks

เขียนโปรแกรมที่ทำงานดังนี้:

1. สร้าง String ชื่อ `name`
2. สร้าง Integer ชื่อ `age`
3. สร้าง Reference ของตัวแปรทั้งสอง
4. แสดงค่าผ่านตัวแปรเจ้าของข้อมูล
5. แสดงค่าผ่าน Reference
6. สร้าง Function ชื่อ `show_profile`
7. ให้ Function รับ Reference ของ String และ Integer
8. เรียก Function เพื่อแสดงข้อมูล

## Requirements

- ใช้ `String::from()` สร้าง String
- ใช้ `&` สร้าง Reference
- ต้องมี Function อย่างน้อย 1 Function
- Function ต้องรับ Reference เป็น Parameter
- ใช้ `println!()` แสดงผล

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Name: Rust Learner
Age: 25

Reference Name: Rust Learner
Reference Age: 25

Profile:
Name: Rust Learner
Age: 25
```

## Constraints

- ห้ามใช้ `.clone()` เพื่อแก้ปัญหา Ownership
- ห้าม Move String เข้า Function
- ต้องส่ง Reference เข้า Function
- ต้องสามารถใช้ตัวแปรเจ้าของข้อมูลต่อหลังเรียก Function ได้

## Checklist

- [ ] สร้าง Reference ได้
- [ ] ใช้ Reference อ่านข้อมูลได้
- [ ] ส่ง Reference เข้า Function ได้
- [ ] เข้าใจว่า Ownership ไม่ได้ถูกย้าย
- [ ] โปรแกรม Compile ผ่าน

## Run Project

```powershell
cargo check
cargo run
```
