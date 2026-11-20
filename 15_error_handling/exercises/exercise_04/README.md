# Exercise 04: Error Handling Challenge

## Objective

ประยุกต์ใช้ความรู้เรื่อง `Result`, `match`, `?`, Custom Error Enum, `Display` และ `std::error::Error` เพื่อสร้างระบบตรวจสอบและจัดการข้อมูลอย่างเป็นระบบ

## Project

`error_handling_challenge`

## Scenario

สร้างโปรแกรมจัดการข้อมูลนักเรียน โดยรับชื่อและคะแนน ตรวจสอบความถูกต้องของข้อมูล และคำนวณผลการเรียน

โปรแกรมต้องสามารถตรวจสอบและแจ้งข้อผิดพลาดได้หลายประเภท โดยไม่ทำให้โปรแกรม Panic เมื่อได้รับข้อมูลไม่ถูกต้อง

## Requirements

### Part 1: Create Custom Error Enum

สร้าง Enum ชื่อ `StudentError` โดยมี Variants:

| Variant           | Description                     |
| ----------------- | ------------------------------- |
| `EmptyName`       | ไม่ได้ระบุชื่อนักเรียน          |
| `InvalidScore`    | คะแนนไม่สามารถแปลงเป็นตัวเลขได้ |
| `ScoreOutOfRange` | คะแนนอยู่นอกช่วง 0–100          |

### Part 2: Implement Display

Implement Trait `Display` ให้กับ `StudentError`

กำหนดข้อความ Error ดังนี้:

| Error           | Message                         |
| --------------- | ------------------------------- |
| EmptyName       | Student name cannot be empty    |
| InvalidScore    | Score must be a valid number    |
| ScoreOutOfRange | Score must be between 0 and 100 |

สามารถใช้ `match` เพื่อเลือกข้อความตาม Variant ได้

### Part 3: Implement Error Trait

Implement:

```text
std::error::Error
```

ให้กับ `StudentError`

### Part 4: Create Student Struct

สร้าง Struct ชื่อ `Student` โดยมี Fields:

| Field | Data Type | Visibility |
| ----- | --------- | ---------- |
| name  | String    | Private    |
| score | u32       | Private    |

### Part 5: Create Methods

สร้าง Methods ดังนี้:

| Method    | Requirement                          |
| --------- | ------------------------------------ |
| `new()`   | สร้าง Student และตรวจสอบชื่อกับคะแนน |
| `name()`  | คืนชื่อผ่าน Immutable Reference      |
| `score()` | คืนคะแนน                             |
| `grade()` | คืนเกรดตามคะแนน                      |

ข้อกำหนดของ `new()`:

- รับชื่อเป็น String
- รับคะแนนเป็น String
- ตรวจสอบว่าชื่อไม่ว่าง
- แปลงคะแนนเป็นตัวเลข
- ตรวจสอบว่าคะแนนอยู่ระหว่าง 0 ถึง 100
- คืน `Result<Student, StudentError>`

ข้อกำหนดของ `grade()`:

| Score  | Grade |
| ------ | ----- |
| 80–100 | A     |
| 70–79  | B     |
| 60–69  | C     |
| 50–59  | D     |
| 0–49   | F     |

### Part 6: Create Validation Function

สร้าง Function สำหรับตรวจสอบและสร้างข้อมูลนักเรียนจาก Input

ข้อกำหนด:

- รับชื่อและคะแนน
- เรียก `Student::new()`
- ใช้ `?` ส่งต่อ Error
- คืนค่าเป็น `Result<Student, StudentError>`

### Part 7: Main Program

ทดลองสร้างนักเรียนอย่างน้อย 5 กรณี:

1. Alice คะแนน 85
2. Bob คะแนน 72
3. Charlie คะแนน 45
4. นักเรียนที่ไม่มีชื่อ คะแนน 90
5. David คะแนน `"abc"`

เพิ่มการทดลองคะแนนที่อยู่นอกช่วง เช่น 120 ด้วย

แสดงข้อมูลของนักเรียนที่สร้างสำเร็จ และข้อความ Error สำหรับข้อมูลที่ไม่ถูกต้อง

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์:

```text
=== Student Error Handling System ===

--- Student Records ---

Name: Alice
Score: 85
Grade: A
Status: Success

Name: Bob
Score: 72
Grade: B
Status: Success

Name: Charlie
Score: 45
Grade: F
Status: Success

--- Error Records ---

Error: Student name cannot be empty

Error: Score must be a valid number

Error: Score must be between 0 and 100

=== Summary ===
Successful Records: 3
Failed Records: 3
```

หมายเหตุ: จำนวนข้อมูลสำเร็จและล
