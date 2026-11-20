# Exercise 02: Immutable Borrowing

## Objective

ฝึกใช้ Immutable Borrowing เพื่ออ่านข้อมูลผ่าน Reference โดยไม่แก้ไขข้อมูลต้นฉบับ

## Project

`immutable_borrowing`

## Scenario

สร้างโปรแกรมแสดงข้อมูลนักเรียน โดยให้ Function รับข้อมูลผ่าน Immutable Reference

## Tasks

### Part 1: Create Data

สร้างตัวแปร:

- `name` เป็น String
- `country` เป็น String
- `score` เป็น Integer

### Part 2: Create References

1. สร้าง Immutable Reference ของ `name`
2. สร้าง Immutable Reference ของ `country`
3. สร้าง Immutable Reference ของ `score`
4. แสดงข้อมูลผ่าน Reference

### Part 3: Create Function

สร้าง Function ชื่อ `show_student`

Function ต้อง:

1. รับ Reference ของชื่อ
2. รับ Reference ของประเทศ
3. รับ Reference ของคะแนน
4. แสดงข้อมูลทั้งหมด

### Part 4: Multiple References

1. สร้าง Immutable References อย่างน้อย 2 ตัวที่อ้างถึง String เดียวกัน
2. ใช้ Reference ทั้งสองตัวแสดงข้อมูล
3. ทดลองเรียก Function โดยส่ง Immutable Reference

## Requirements

- ใช้ Immutable Reference (`&`)
- ต้องมี Function อย่างน้อย 1 Function
- ต้องมี Immutable References อย่างน้อย 2 ตัว
- ต้องมี String อย่างน้อย 2 ตัว
- ต้องมี Integer อย่างน้อย 1 ตัว

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Student Information
Name: Alex
Country: Thailand
Score: 85

Reference Name: Alex
Reference Name 2: Alex

Student Profile
Name: Alex
Country: Thailand
Score: 85
```

## Constraints

- ห้ามใช้ Mutable Reference
- ห้ามแก้ไขข้อมูลผ่าน Reference
- ห้าม Move ข้อมูลเข้า Function
- ต้องใช้ Reference ในการส่งข้อมูลเข้า Function

## Checklist

- [ ] สร้าง Immutable Reference ได้
- [ ] สร้าง Immutable References หลายตัวได้
- [ ] ใช้ Reference กับ Function ได้
- [ ] อ่านข้อมูลผ่าน Reference ได้
- [ ] เข้าใจว่า Immutable Borrowing ไม่อนุญาตให้แก้ไขข้อมูล

## Run Project

```powershell
cargo check
cargo run
```
