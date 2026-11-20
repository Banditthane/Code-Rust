# Exercise 03: Mutable Borrowing

## Objective

ฝึกใช้ Mutable Borrowing เพื่อแก้ไขข้อมูลผ่าน Mutable Reference

## Project

`mutable_borrowing`

## Scenario

สร้างโปรแกรมจัดการคะแนนนักเรียน โดยใช้ Mutable Reference เพื่อเพิ่มและแก้ไขคะแนน

## Tasks

### Part 1: Create Data

สร้างตัวแปร:

- `name` เป็น String
- `score` เป็น Integer
- `level` เป็น Integer

ตัวแปรที่ต้องแก้ไขให้ประกาศด้วย `mut`

### Part 2: Mutable Reference

1. สร้าง Mutable Reference ของ `score`
2. เพิ่มคะแนนผ่าน Mutable Reference
3. แสดงคะแนนหลังแก้ไข

### Part 3: Update Function

สร้าง Function ชื่อ `update_score`

Function ต้อง:

1. รับ Mutable Reference ของคะแนน
2. เพิ่มคะแนนตามจำนวนที่กำหนด
3. คืนค่าเป็น `()`

### Part 4: Update String

1. สร้าง Mutable String ชื่อ `message`
2. สร้าง Mutable Reference
3. เพิ่มข้อความด้วย `push_str()`
4. แสดงข้อความหลังแก้ไข

## Requirements

- ใช้ Mutable Variable (`let mut`)
- ใช้ Mutable Reference (`&mut`)
- ใช้ Dereferencing (`*`) เมื่อแก้ไขค่าตัวเลขผ่าน Reference
- ต้องมี Function อย่างน้อย 1 Function
- Function ต้องรับ Mutable Reference
- ต้องใช้ `push_str()` อย่างน้อย 1 ครั้ง

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Student: Alex

Initial Score: 70
Updated Score: 80

Message: Learning Rust
```

## Constraints

- ห้ามใช้ `.clone()` เพื่อสร้างสำเนาข้อมูล
- ต้องแก้ไขข้อมูลผ่าน Mutable Reference
- ต้องไม่มี Mutable References ที่ใช้งานทับซ้อนกัน
- ตัวแปรเจ้าของข้อมูลต้องประกาศเป็น `mut` เมื่อจำเป็นต้องแก้ไข

## Checklist

- [ ] สร้าง Mutable Reference ได้
- [ ] แก้ไข Integer ผ่าน Reference ได้
- [ ] แก้ไข String ผ่าน Reference ได้
- [ ] ส่ง Mutable Reference เข้า Function ได้
- [ ] เข้าใจความแตกต่างระหว่าง `&` และ `&mut`

## Run Project

```powershell
cargo check
cargo run
```
