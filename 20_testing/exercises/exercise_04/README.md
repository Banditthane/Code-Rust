# Exercise 04: Testing Challenge

## Project

`testing_challenge`

## Objective

ประยุกต์ใช้ Unit Tests, Integration Tests, Test Attributes และ Documentation Tests เพื่อสร้างระบบจัดการนักเรียนที่สามารถทดสอบได้อย่างเป็นระบบ

## Project Scenario

สร้าง Student Management System ที่สามารถเพิ่มนักเรียน ค้นหานักเรียน คำนวณคะแนน และตรวจสอบเกรด

## Requirements

### Task 1: Create Library Structure

สร้าง Library Crate และจัดโครงสร้างดังนี้:

```text
src/
├── lib.rs
├── student.rs
├── grade.rs
└── classroom.rs

tests/
├── student_test.rs
├── grade_test.rs
└── classroom_test.rs
```

### Task 2: Create Student Struct

สร้าง Struct `Student` โดยมีข้อมูล:

- `id: u32`
- `name: String`
- `score: f64`

กำหนด Fields และ Methods ให้เหมาะสม

### Task 3: Create Student Module

สร้าง Functions สำหรับ:

- สร้างนักเรียน
- ตรวจสอบข้อมูลนักเรียน
- ค้นหานักเรียนด้วย ID
- แสดงข้อมูลนักเรียน

กำหนดเงื่อนไข:

- ID ต้องไม่ซ้ำ
- ชื่อต้องไม่เป็นข้อความว่าง
- คะแนนต้องอยู่ระหว่าง 0–100

### Task 4: Create Grade Module

สร้าง Functions สำหรับ:

- คำนวณคะแนนเฉลี่ย
- ตรวจสอบว่าผ่านเกณฑ์หรือไม่
- แปลงคะแนนเป็นเกรด

กำหนดเกณฑ์:

| Score  | Grade |
| ------ | ----- |
| 80–100 | A     |
| 70–79  | B     |
| 60–69  | C     |
| 50–59  | D     |
| 0–49   | F     |

### Task 5: Create Classroom Module

สร้างระบบ Classroom สำหรับ:

- เพิ่มนักเรียน
- ลบนักเรียน
- ค้นหานักเรียน
- แสดงรายชื่อนักเรียน
- คำนวณคะแนนเฉลี่ยของห้องเรียน

กรณีไม่มีนักเรียน ต้องจัดการผลการคำนวณ Average อย่างเหมาะสม

### Task 6: Create Unit Tests

สร้าง Unit Tests ภายใน Modules

ต้องทดสอบ:

- การสร้างนักเรียน
- การตรวจสอบชื่อ
- การตรวจสอบคะแนน
- การคำนวณเกรด
- การคำนวณ Average
- การค้นหานักเรียน
- การเพิ่มและลบนักเรียน

### Task 7: Create Integration Tests

สร้าง Integration Tests อย่างน้อย 3 ไฟล์ เพื่อทดสอบ Public API

ทดสอบกระบวนการ:

1. สร้างนักเรียน
2. เพิ่มนักเรียนเข้า Classroom
3. ค้นหานักเรียน
4. คำนวณเกรด
5. คำนวณคะแนนเฉลี่ย
6. ลบนักเรียน

### Task 8: Test Error Cases

ทดสอบกรณี:

- ชื่อว่าง
- คะแนนติดลบ
- คะแนนมากกว่า 100
- ID ซ้ำ
- ค้นหา ID ที่ไม่มีอยู่
- ลบนักเรียนที่ไม่มีอยู่
- คำนวณ Average เมื่อไม่มีนักเรียน

ใช้ `Result` หรือ `Option` เพื่อจัดการผลลัพธ์ตามความเหมาะสม

### Task 9: Use Test Attributes

เพิ่ม Tests ที่ใช้:

- `#[should_panic]` สำหรับ Function ที่กำหนดให้เกิด Panic ตามเงื่อนไข
- `#[ignore]` สำหรับ Test ที่ใช้เวลานานหรือยังไม่ต้องการรันเป็นประจำ

ต้องอธิบายเหตุผลที่เลือกใช้ Attribute แต่ละตัว

### Task 10: Documentation Tests

เขียน Documentation Comments ให้ Public Functions อย่างน้อย 2 Functions

ต้องมี Code Examples ที่สามารถ Compile และ Run เป็น Documentation Tests ได้

## Constraints

- ต้องสร้าง Library Crate
- ต้องมี Modules อย่างน้อย 3 Modules
- ต้องมี Struct `Student`
- ต้องมีระบบ Classroom
- ต้องมีระบบ Grade
- ต้องสร้าง Unit Tests
- ต้องสร้าง Integration Tests อย่างน้อย 3 ไฟล์
- ต้องใช้ Assertions อย่างเหมาะสม
- ต้องมี Tests สำหรับ Normal Cases และ Error Cases
- ต้องใช้ `#[should_panic]` อย่างน้อย 1 Test
- ต้องใช้ `#[ignore]` อย่างน้อย 1 Test
- ต้องมี Documentation Tests อย่างน้อย 2 Examples
- ต้องใช้ `Result` หรือ `Option` สำหรับกรณีที่อาจเกิดข้อผิดพลาด
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- ต้องไม่มี Dangling References
- Tests ที่ไม่ได้ Ignore ต้องผ่านทั้งหมด

## Test Data

### Students

| ID  | Name    | Score |
| --- | ------- | ----: |
| 1   | Alice   |    85 |
| 2   | Bob     |    72 |
| 3   | Charlie |    45 |
| 4   | David   |    90 |

### Invalid Data

| ID  | Name  | Score | Expected      |
| --- | ----- | ----: | ------------- |
| 5   | Empty |       | Invalid name  |
| 6   | Eve   |   -10 | Invalid score |
| 7   | Frank |   120 | Invalid score |

## Expected Output

ตัวอย่างผลลัพธ์:

```text
running unit tests

test student::tests::test_create_student ... ok
test student::tests::test_invalid_name ... ok
test student::tests::test_invalid_score ... ok
test grade::tests::test_grade_a ... ok
test grade::tests::test_grade_b ... ok
test grade::tests::test_grade_f ... ok
test classroom::tests::test_add_student ... ok
test classroom::tests::test_remove_student ... ok

running integration tests

test student_test::test_student_workflow ... ok
test grade_test::test_grade_calculation ... ok
test classroom_test::test_classroom_workflow ... ok

running documentation tests

test result: ok

All tests completed successfully.
```

ผลลัพธ์จริงอาจแตกต่างตามชื่อ Test และจำนวน Tests ที่สร้างขึ้น

## Checklist

- [ ] สร้าง Library Crate ได้
- [ ] สร้าง Struct `Student` ได้
- [ ] สร้าง Module `student` ได้
- [ ] สร้าง Module `grade` ได้
- [ ] สร้าง Module `classroom` ได้
- [ ] เพิ่มและลบนักเรียนได้
- [ ] ค้นหานักเรียนได้
- [ ] คำนวณเกรดได้
- [ ] คำนวณคะแนนเฉลี่ยได้
- [ ] จัดการ Invalid Data ได้
- [ ] สร้าง Unit Tests ได้
- [ ] สร้าง Integration Tests ได้
- [ ] ใช้ `#[should_panic]` ได้
- [ ] ใช้ `#[ignore]` ได้
- [ ] เขียน Documentation Comments ได้
- [ ] สร้าง Documentation Tests ได้
- [ ] Tests ที่ไม่ได้ Ignore ผ่านทั้งหมด

## Run Project

```powershell
cd 20_testing/exercises/exercise_04/testing_challenge
```

รัน Tests ทั้งหมด:

```powershell
cargo test
```

แสดง Output จาก Tests:

```powershell
cargo test -- --show-output
```

รัน Ignored Tests:

```powershell
cargo test -- --ignored
```

รัน Documentation Tests:

```powershell
cargo test --doc
```

## Completion Criteria

ถือว่าผ่าน Exercise นี้เมื่อ:

1. ระบบ Student Management ทำงานได้
2. Unit Tests ตรวจสอบการทำงานของ Modules ได้
3. Integration Tests ตรวจสอบ Public API ได้
4. Error Cases ได้รับการจัดการอย่างเหมาะสม
5. Test Attributes ทำงานตามที่กำหนด
6. Documentation Tests ผ่าน
7. Tests ที่ไม่ได้ Ignore ผ่านทั้งหมด

## Final Challenge

เพิ่มระบบค้นหานักเรียนตามชื่อ โดยรองรับการค้นหาที่ไม่แยกตัวพิมพ์เล็กและตัวพิมพ์ใหญ่

จากนั้น:

1. เพิ่ม Function สำหรับค้นหานักเรียน
2. เขียน Unit Tests สำหรับ Function ใหม่
3. เขียน Integration Test สำหรับการค้นหา
4. เพิ่ม Documentation Example
5. รัน `cargo test` และตรวจสอบว่า Tests ทั้งหมดผ่าน

เป้าหมายคือสามารถเพิ่มความสามารถใหม่ให้โปรแกรมได้ โดยยังรักษาความถูกต้องของระบบเดิม
