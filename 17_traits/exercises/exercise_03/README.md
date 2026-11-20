# Exercise 03: Student Traits

## Objective

สร้างระบบจัดการข้อมูลนักเรียนโดยใช้ Derived Traits และ Standard Library Traits เพื่อฝึกแสดงผล คัดลอก เปรียบเทียบ และจัดรูปแบบข้อมูล

## Project

Project Name: `student_traits`

## Requirements

### Part 1: Create Student Struct

สร้าง Struct ชื่อ `Student` ประกอบด้วย Fields:

- `id: u32`
- `name: String`
- `score: f64`

### Part 2: Derived Traits

ใช้ `derive` กับ Traits ต่อไปนี้:

- `Debug`
- `Clone`
- `PartialEq`

### Part 3: Implement Display

Implement Trait `Display` ให้ Student โดยกำหนดรูปแบบ:

```text
Student ID: 1, Name: Alice, Score: 85.00
```

### Part 4: Create Instances

สร้างข้อมูลนักเรียน:

|  ID | Name    | Score |
| --: | ------- | ----: |
|   1 | Alice   |    85 |
|   2 | Bob     |    72 |
|   3 | Charlie |    90 |

### Part 5: Test Traits

โปรแกรมต้องสามารถ:

1. แสดงข้อมูล Alice ด้วย Debug Format
2. แสดงข้อมูล Alice ด้วย Display Format
3. Clone ข้อมูล Alice ไปยังตัวแปรใหม่
4. ตรวจสอบว่า Alice และข้อมูลที่ Clone มีค่าเท่ากันหรือไม่ด้วย `==`
5. แก้ไขข้อมูลของตัวแปรที่ Clone โดยเปลี่ยนคะแนนเป็น 95
6. ตรวจสอบอีกครั้งว่าข้อมูลทั้งสองเท่ากันหรือไม่
7. แสดงข้อมูลนักเรียนทั้งสามคนด้วย Display

## Constraints

- ต้องสร้าง Struct `Student`
- ต้องใช้ `#[derive(Debug, Clone, PartialEq)]`
- ต้อง Implement `std::fmt::Display` ด้วยตนเอง
- ต้องใช้ `fmt::Formatter` และ `fmt::Result`
- ต้องใช้ `clone()` เพื่อสร้างสำเนา
- ต้องใช้ `==` เปรียบเทียบข้อมูล
- ห้าม Implement `Display` ด้วย Debug Formatting
- ห้ามกำหนดข้อความแสดงผลแบบตายตัวโดยไม่อ่านค่าจาก Fields

## Expected Output

```text
Debug:
Student { id: 1, name: "Alice", score: 85.0 }

Display:
Student ID: 1, Name: Alice, Score: 85.00

Clone Comparison:
Original == Cloned: true

After Updating Cloned Score:
Original == Cloned: false

Students:
Student ID: 1, Name: Alice, Score: 85.00
Student ID: 2, Name: Bob, Score: 72.00
Student ID: 3, Name: Charlie, Score: 90.00
```

## Hints

- ใช้ `#[derive(Debug, Clone, PartialEq)]`
- Import `std::fmt`
- Implement `fmt()` ภายใน `impl Display for Student`
- ใช้ `write!()` เพื่อเขียนข้อความลง Formatter
- ใช้ `clone()` เพื่อสร้างสำเนาของ Student
- `PartialEq` ทำให้สามารถใช้ `==` และ `!=` ได้
- เมื่อเปลี่ยน Field ของสำเนา ข้อมูลต้นฉบับจะไม่เปลี่ยน

## Completion Checklist

- [ ] สร้าง Struct `Student`
- [ ] Derive `Debug`
- [ ] Derive `Clone`
- [ ] Derive `PartialEq`
- [ ] Implement `Display`
- [ ] ใช้ Debug Formatting ได้
- [ ] ใช้ Display Formatting ได้
- [ ] Clone Student ได้
- [ ] เปรียบเทียบด้วย `==` ได้
- [ ] แก้ไขข้อมูลของสำเนาได้
- [ ] แสดงข้อมูลนักเรียนครบ 3 คน
- [ ] แสดงผลตรงตาม Expected Output

## Run Project

```powershell
cd 17_traits/exercises/exercise_03/student_traits
cargo run
```

## Challenge

เพิ่มเติมความสามารถ:

- Implement `Default` ให้ Student
- สร้าง Student โดยใช้ `Student::default()`
- เพิ่ม Function สำหรับเปรียบเทียบคะแนน
- ทดลองใช้ `PartialOrd` เพื่อเปรียบเทียบคะแนน
- ทดลองเรียงลำดับนักเรียนตามคะแนน

หมายเหตุ: หากต้องการ Derive `PartialOrd` ให้เพิ่ม Attribute ดังกล่าว และพิจารณาว่าการเปรียบเทียบทุก Field เหมาะกับโจทย์หรือไม่
