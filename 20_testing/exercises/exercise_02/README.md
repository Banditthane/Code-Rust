# Exercise 02: Unit Testing

## Project

`unit_testing`

## Objective

ฝึกสร้าง Unit Tests ภายใน Modules และทดสอบทั้ง Public Functions และ Private Functions

## Project Scenario

สร้างระบบจัดการคะแนนนักเรียน โดยมี Functions สำหรับคำนวณคะแนนและตรวจสอบเกรด

## Requirements

### Task 1: Create Grade Module

สร้าง Module ชื่อ `grade` และกำหนด Functions ดังนี้:

| Function            | Description                         |
| ------------------- | ----------------------------------- |
| `calculate_average` | คำนวณคะแนนเฉลี่ย                    |
| `is_passed`         | ตรวจสอบว่าผ่านเกณฑ์หรือไม่          |
| `get_grade`         | แปลงคะแนนเป็นเกรด                   |
| `validate_score`    | ตรวจสอบว่าคะแนนอยู่ในช่วงที่ถูกต้อง |

### Task 2: Define Score Rules

กำหนดเงื่อนไข:

- คะแนนต้องอยู่ระหว่าง 0–100
- คะแนนตั้งแต่ 50 ขึ้นไปถือว่าผ่าน
- คะแนน 80–100 ได้เกรด A
- คะแนน 70–79 ได้เกรด B
- คะแนน 60–69 ได้เกรด C
- คะแนน 50–59 ได้เกรด D
- คะแนน 0–49 ได้เกรด F

คะแนนนอกช่วงต้องได้รับการจัดการอย่างเหมาะสม

### Task 3: Create Unit Test Module

สร้าง Test Module ภายใน `grade` โดยใช้:

- `#[cfg(test)]`
- `mod tests`
- `#[test]`

### Task 4: Test Normal Cases

ทดสอบ:

- คะแนน 90 ได้ A
- คะแนน 75 ได้ B
- คะแนน 65 ได้ C
- คะแนน 55 ได้ D
- คะแนน 30 ได้ F
- คะแนน 50 ถือว่าผ่าน
- คะแนน 49 ถือว่าไม่ผ่าน

### Task 5: Test Edge Cases

ทดสอบ:

- คะแนน 0
- คะแนน 100
- คะแนนติดลบ
- คะแนนมากกว่า 100
- รายการคะแนนว่างสำหรับการคำนวณ Average

กำหนดวิธีจัดการข้อมูลที่ไม่ถูกต้องให้ชัดเจน เช่น ใช้ `Option` หรือ `Result`

### Task 6: Test Private Function

สร้าง Private Function อย่างน้อยหนึ่งตัว และเขียน Unit Test ภายใน Module เพื่อทดสอบ Function ดังกล่าว

## Constraints

- ต้องมี Module `grade`
- ต้องมี Functions อย่างน้อย 4 ตัว
- ต้องมี Private Function อย่างน้อย 1 ตัว
- ต้องมี Test Module
- ต้องใช้ `#[cfg(test)]`
- ต้องใช้ Assertions อย่างเหมาะสม
- ต้องมี Unit Tests อย่างน้อย 10 ตัว
- ต้องทดสอบ Normal Cases และ Edge Cases
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- ต้องจัดการข้อมูลผิดพลาดโดยไม่ทำให้เกิด Panic โดยไม่ตั้งใจ
- Tests ทั้งหมดต้องผ่าน

## Expected Output

ตัวอย่างผลลัพธ์:

```text
running 10 tests

test grade::tests::test_grade_a ... ok
test grade::tests::test_grade_b ... ok
test grade::tests::test_grade_c ... ok
test grade::tests::test_grade_d ... ok
test grade::tests::test_grade_f ... ok
test grade::tests::test_passed ... ok
test grade::tests::test_failed ... ok
test grade::tests::test_min_score ... ok
test grade::tests::test_max_score ... ok
test grade::tests::test_invalid_score ... ok

test result: ok. 10 passed; 0 failed
```

จำนวนและชื่อ Tests สามารถแตกต่างได้ แต่ต้องทดสอบเงื่อนไขที่กำหนดครบถ้วน

## Checklist

- [ ] สร้าง Module `grade` ได้
- [ ] สร้าง `calculate_average()` ได้
- [ ] สร้าง `is_passed()` ได้
- [ ] สร้าง `get_grade()` ได้
- [ ] สร้าง `validate_score()` ได้
- [ ] สร้าง Private Function ได้
- [ ] สร้าง Test Module ได้
- [ ] ใช้ `#[cfg(test)]` ได้
- [ ] ทดสอบ Private Function ได้
- [ ] ทดสอบ Normal Cases ได้
- [ ] ทดสอบ Edge Cases ได้
- [ ] Tests ทั้งหมดผ่าน

## Run Project

```powershell
cd 20_testing/exercises/exercise_02/unit_testing
cargo test
```

แสดง Output:

```powershell
cargo test -- --show-output
```
