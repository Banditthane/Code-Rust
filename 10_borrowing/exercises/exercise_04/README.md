# Exercise 04: Borrowing Challenge

## Objective

ประยุกต์ใช้ References, Immutable Borrowing, Mutable Borrowing และ Borrowing Rules ในโปรแกรมเดียวกัน

## Project

`borrowing_challenge`

## Scenario

สร้างโปรแกรมจัดการข้อมูลนักเรียน โดยสามารถแสดงข้อมูล ตรวจสอบคะแนน และแก้ไขข้อมูลผ่าน Reference

## Tasks

### Part 1: Create Student Data

สร้างตัวแปร:

- `name` เป็น String
- `country` เป็น String
- `score` เป็น Integer
- `active` เป็น Boolean

กำหนดค่าเริ่มต้นให้ครบทุกตัวแปร

### Part 2: Display Student Information

สร้าง Function ชื่อ `show_student`

Function ต้อง:

1. รับ Reference ของชื่อ
2. รับ Reference ของประเทศ
3. รับ Reference ของคะแนน
4. รับ Reference ของสถานะ
5. แสดงข้อมูลทั้งหมด

### Part 3: Calculate Score

สร้าง Function ชื่อ `calculate_score`

Function ต้อง:

1. รับ Immutable Reference ของคะแนน
2. คืนค่าคะแนนที่ได้รับ
3. ไม่แก้ไขข้อมูลต้นฉบับ

### Part 4: Update Score

สร้าง Function ชื่อ `update_score`

Function ต้อง:

1. รับ Mutable Reference ของคะแนน
2. เพิ่มคะแนนตามจำนวนที่กำหนด
3. แก้ไขคะแนนผ่าน Reference

### Part 5: Update Student Name

1. สร้าง Function ชื่อ `update_name`
2. รับ Mutable Reference ของ String
3. เพิ่มข้อความต่อท้ายชื่อด้วย `push_str()`
4. แสดงชื่อหลังแก้ไข

### Part 6: Borrowing Rules

ทดลองเขียนโปรแกรมแยกสำหรับกรณีต่อไปนี้:

1. สร้าง Immutable References หลายตัวจากข้อมูลเดียวกัน
2. สร้าง Mutable Reference เพื่อแก้ไขข้อมูล
3. ทดลองสร้าง Immutable และ Mutable References ที่มีช่วงเวลาทับซ้อนกัน
4. วิเคราะห์ว่าเหตุใดบางกรณีจึง Compile Error

## Requirements

- ต้องใช้ String
- ต้องใช้ Integer
- ต้องใช้ Boolean
- ต้องใช้ Immutable Reference
- ต้องใช้ Mutable Reference
- ต้องมี Function อย่างน้อย 4 Functions
- ต้องมี Function ที่รับ `&T`
- ต้องมี Function ที่รับ `&mut T`
- ต้องแก้ไข String ผ่าน Mutable Reference
- ต้องแก้ไข Integer ผ่าน Mutable Reference

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Student Information
Name: Alex
Country: Thailand
Score: 70
Active: true

Calculated Score: 70

Updated Score: 80

Updated Name: Alex - Rust Learner

Borrowing Challenge Completed
```

## Constraints

- ห้ามใช้ `.clone()` เพื่อหลีกเลี่ยง Borrowing
- ห้าม Move ข้อมูลโดยไม่จำเป็น
- ต้องใช้ Reference ในการส่งข้อมูลเข้า Function
- ต้องไม่สร้าง Mutable References ที่ใช้งานทับซ้อนกัน
- ต้องไม่ใช้ Reference หลังจากข้อมูลที่อ้างถึงหมดอายุ
- กรณีที่ตั้งใจให้เกิด Compile Error ต้องแยกออกจากโปรแกรมหลัก

## Final Questions

ตอบคำถามต่อไปนี้ใน README หรือบันทึกการเรียน:

1. Reference คืออะไร?
2. Borrowing ต่างจาก Ownership อย่างไร?
3. Immutable Borrowing คืออะไร?
4. Mutable Borrowing คืออะไร?
5. ทำไมจึงไม่สามารถมี Mutable References ที่ใช้งานทับซ้อนกันหลายตัวได้?
6. ทำไม Immutable และ Mutable References จึงไม่สามารถใช้งานทับซ้อนกันได้?
7. Reference สามารถมีอายุยาวกว่าข้อมูลที่อ้างถึงได้หรือไม่?
8. Borrowing ช่วยลดการใช้ `.clone()` ได้อย่างไร?

## Checklist

- [ ] สร้าง Reference ได้
- [ ] ใช้ Immutable Borrowing ได้
- [ ] ใช้ Mutable Borrowing ได้
- [ ] ส่ง Reference เข้า Function ได้
- [ ] แก้ไขข้อมูลผ่าน Mutable Reference ได้
- [ ] เข้าใจ Borrowing Rules
- [ ] เข้าใจข้อจำกัดของ Reference
- [ ] ตอบ Final Questions ได้

## Run Project

```powershell
cargo check
cargo run
```
