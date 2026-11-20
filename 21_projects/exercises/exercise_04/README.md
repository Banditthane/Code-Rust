# Exercise 04: Student Management

## Project

**Student Management — โปรแกรมจัดการข้อมูลนักเรียน**

## Objective

สร้างโปรแกรมจัดการข้อมูลนักเรียนที่รวมความรู้จาก Rust หลายบทเข้าด้วยกัน โดยสามารถเพิ่ม แสดง ค้นหา แก้ไข และลบข้อมูลนักเรียน พร้อมบันทึกข้อมูลลงไฟล์

แบบฝึกหัดนี้เป็น Final Challenge ของ Rust Learning Roadmap

## Requirements

โปรแกรมต้องรองรับความสามารถดังต่อไปนี้:

1. Add Student
2. View All Students
3. Search Student
4. Update Student
5. Delete Student
6. Calculate Class Statistics
7. Save Data
8. Load Data
9. Exit

## Tasks

### Task 1: Design Student Struct

สร้าง Struct สำหรับเก็บข้อมูลนักเรียน

ข้อมูลขั้นต่ำ:

- ID
- Name
- Age
- Score

กำหนดชนิดข้อมูลให้เหมาะสมกับแต่ละ Field

### Task 2: Design Student Management

สร้างระบบจัดการข้อมูลนักเรียน โดยใช้ `Vec<Student>` หรือโครงสร้างข้อมูลที่เหมาะสม

### Task 3: Add Student

รับข้อมูลนักเรียนจากผู้ใช้และเพิ่มข้อมูลเข้าไปในระบบ

ตรวจสอบว่า:

- ID ไม่ซ้ำกัน
- ชื่อต้องไม่เป็นข้อความว่าง
- อายุอยู่ในช่วงที่กำหนด
- คะแนนอยู่ระหว่าง 0–100

### Task 4: View All Students

แสดงข้อมูลนักเรียนทั้งหมด

หากไม่มีข้อมูล ให้แสดงข้อความแจ้งผู้ใช้

### Task 5: Search Student

ค้นหานักเรียนด้วย ID

หากไม่พบข้อมูล ต้องแสดงข้อความแจ้งผู้ใช้โดยไม่ทำให้โปรแกรม Panic

### Task 6: Update Student

แก้ไขข้อมูลนักเรียนที่มีอยู่ เช่น:

- Name
- Age
- Score

ต้องตรวจสอบข้อมูลใหม่ก่อนบันทึก

### Task 7: Delete Student

ลบข้อมูลนักเรียนโดยอ้างอิงจาก ID

### Task 8: Calculate Class Statistics

คำนวณข้อมูลภาพรวมของนักเรียน:

- จำนวนทั้งหมด
- คะแนนรวม
- คะแนนเฉลี่ย
- คะแนนสูงสุด
- คะแนนต่ำสุด
- จำนวนนักเรียนที่ผ่านเกณฑ์

กำหนดเกณฑ์ผ่านเป็นคะแนนตั้งแต่ 50 คะแนนขึ้นไป

ต้องจัดการกรณีไม่มีนักเรียน เพื่อไม่ให้เกิดการหารด้วยศูนย์

### Task 9: Save and Load Data

บันทึกข้อมูลนักเรียนลงไฟล์ และสามารถอ่านข้อมูลกลับมาเมื่อเปิดโปรแกรม

กำหนดรูปแบบข้อมูลให้ชัดเจน และจัดการกรณีข้อมูลในไฟล์ไม่ถูกต้อง

### Task 10: Organize Modules

แบ่งโปรแกรมออกเป็น Module ตามหน้าที่ เช่น:

```text
src/
├── main.rs
├── models.rs
├── student_service.rs
├── storage.rs
└── statistics.rs
```

สามารถปรับโครงสร้างให้เหมาะสมกับการออกแบบของตนเองได้

### Task 11: Error Handling

ออกแบบการจัดการข้อผิดพลาดให้เหมาะสม เช่น:

- Duplicate Student ID
- Empty Student Name
- Invalid Age
- Invalid Score
- Student Not Found
- File Read Error
- File Write Error
- Invalid Stored Data

สามารถใช้ Custom Error Type เพื่อรวบรวมข้อผิดพลาดของระบบได้

### Task 12: Testing

เขียน Tests สำหรับ:

- เพิ่มนักเรียน
- ค้นหานักเรียน
- แก้ไขข้อมูลนักเรียน
- ลบข้อมูลนักเรียน
- ตรวจสอบคะแนน
- คำนวณสถิติ
- บันทึกและโหลดข้อมูล
- จัดการข้อผิดพลาด

## Constraints

- ใช้ Struct สำหรับ Student
- ใช้ Enum หรือ Custom Error สำหรับสถานะและข้อผิดพลาดตามความเหมาะสม
- ใช้ Collections สำหรับจัดเก็บข้อมูล
- ใช้ Function แยกหน้าที่การทำงาน
- ใช้ Module แบ่งโครงสร้างโปรแกรม
- ใช้ Ownership และ Borrowing อย่างถูกต้อง
- ใช้ Result และ Option จัดการข้อผิดพลาด
- สามารถใช้ Trait เพื่อแยกพฤติกรรมที่เหมาะสมได้
- ต้องมีการจัดเก็บข้อมูลลงไฟล์
- ต้องมี Tests สำหรับ Function สำคัญ
- หลีกเลี่ยงการใช้ `unwrap()` กับข้อมูลจากผู้ใช้และการอ่านไฟล์โดยไม่มีการตรวจสอบ

## Expected Output

ตัวอย่างเมนูหลัก:

```text
===== STUDENT MANAGEMENT =====
1. Add Student
2. View All Students
3. Search Student
4. Update Student
5. Delete Student
6. Class Statistics
7. Save Data
8. Load Data
0. Exit

Select:
```

ตัวอย่างเพิ่มนักเรียน:

```text
Select: 1

Student ID: 1001
Name: Alice
Age: 20
Score: 85

Student added successfully.
```

ตัวอย่างแสดงข้อมูล:

```text
===== STUDENT LIST =====

ID: 1001
Name: Alice
Age: 20
Score: 85
Status: Pass
```

ตัวอย่างสถิติ:

```text
===== CLASS STATISTICS =====

Total Students: 1
Total Score: 85
Average Score: 85.00
Highest Score: 85
Lowest Score: 85
Passed Students: 1
Failed Students: 0
```

ผลลัพธ์อาจมีรูปแบบต่างจากตัวอย่างได้ แต่ต้องคำนวณและแสดงข้อมูลได้ถูกต้อง

## Testing Requirements

ควรมี Tests ครอบคลุม:

- เพิ่มนักเรียนด้วยข้อมูลถูกต้อง
- เพิ่มนักเรียนด้วย ID ซ้ำ
- เพิ่มนักเรียนด้วยชื่อว่าง
- เพิ่มนักเรียนด้วยคะแนนต่ำกว่า 0
- เพิ่มนักเรียนด้วยคะแนนมากกว่า 100
- ค้นหานักเรียนที่มีอยู่
- ค้นหานักเรียนที่ไม่มีอยู่
- แก้ไขข้อมูลนักเรียน
- ลบนักเรียน
- คำนวณสถิติเมื่อมีข้อมูล
- คำนวณสถิติเมื่อไม่มีข้อมูล
- บันทึกและโหลดข้อมูล
- จัดการข้อมูลในไฟล์ที่ไม่ถูกต้อง

## Final Completion Checklist

- [ ] สามารถเพิ่มนักเรียนได้
- [ ] สามารถแสดงข้อมูลนักเรียนได้
- [ ] สามารถค้นหานักเรียนได้
- [ ] สามารถแก้ไขข้อมูลนักเรียนได้
- [ ] สามารถลบข้อมูลนักเรียนได้
- [ ] สามารถคำนวณสถิติได้
- [ ] สามารถบันทึกข้อมูลลงไฟล์ได้
- [ ] สามารถโหลดข้อมูลจากไฟล์ได้
- [ ] มีการแบ่ง Module
- [ ] มี Error Handling
- [ ] มี Custom Error ตามความเหมาะสม
- [ ] มี Unit Tests
- [ ] มี Integration Tests
- [ ] `cargo check` ผ่าน
- [ ] `cargo fmt --check` ผ่าน
- [ ] `cargo clippy` ผ่าน
- [ ] `cargo test` ผ่าน
- [ ] `cargo run` ทำงานได้ตามปกติ

## Final Challenge

เป้าหมายของแบบฝึกหัดนี้คือการพัฒนาโปรแกรมที่มีโครงสร้างชัดเจน จัดการข้อมูลได้อย่างถูกต้อง รองรับข้อผิดพลาด และสามารถทดสอบได้

ไม่จำเป็นต้องเขียนโค้ดให้สั้นที่สุด แต่ควรเขียนให้เข้าใจง่าย แบ่งหน้าที่ชัดเจน และสามารถดูแลรักษาได้ในอนาคต
