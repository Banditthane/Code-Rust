# Exercise 01: Testing Basics

## Project

`testing_basics`

## Objective

ฝึกสร้าง Test Functions และตรวจสอบผลลัพธ์ของ Functions ด้วย Assertions

## Requirements

### Task 1: Create Calculator Functions

สร้าง Functions ดังนี้:

| Function   | Description       |
| ---------- | ----------------- |
| `add`      | บวกตัวเลขสองจำนวน |
| `subtract` | ลบตัวเลขสองจำนวน  |
| `multiply` | คูณตัวเลขสองจำนวน |
| `divide`   | หารตัวเลขสองจำนวน |

กำหนดชนิดข้อมูลให้เหมาะสม

สำหรับ `divide` ให้จัดการกรณีหารด้วยศูนย์ด้วย `Option` หรือ `Result`

### Task 2: Create Test Functions

สร้าง Test Functions สำหรับทดสอบ Functions ทั้ง 4 ตัว

อย่างน้อยต้องมี Tests สำหรับ:

- การบวกตัวเลข
- การลบตัวเลข
- การคูณตัวเลข
- การหารตัวเลข

### Task 3: Test Normal Cases

ทดสอบข้อมูลปกติ เช่น:

- `10 + 5`
- `10 - 5`
- `10 * 5`
- `10 / 5`

### Task 4: Test Edge Cases

ทดสอบกรณีเพิ่มเติม:

- การบวกกับ 0
- การลบแล้วได้ค่าติดลบ
- การคูณด้วย 0
- การหารที่มีผลลัพธ์เป็น 0
- การหารด้วย 0

### Task 5: Use Assertions

ต้องใช้ Assertions อย่างน้อย 3 ประเภท:

- `assert!`
- `assert_eq!`
- `assert_ne!`

## Constraints

- ต้องสร้าง Calculator Functions อย่างน้อย 4 ตัว
- ต้องสร้าง Test Functions อย่างน้อย 8 ตัว
- ต้องใช้ `#[test]`
- ต้องใช้ Assertions ครบทั้ง 3 ประเภท
- ต้องทดสอบ Normal Cases
- ต้องทดสอบ Edge Cases
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- ต้องไม่ปล่อยให้การหารด้วยศูนย์ทำให้เกิด Panic โดยไม่ตั้งใจ
- โปรแกรมต้อง Compile และ Tests ต้องผ่าน

## Expected Output

ตัวอย่างผลลัพธ์:

```text
running 8 tests

test tests::test_add ... ok
test tests::test_subtract ... ok
test tests::test_multiply ... ok
test tests::test_divide ... ok
test tests::test_add_zero ... ok
test tests::test_negative_result ... ok
test tests::test_multiply_zero ... ok
test tests::test_divide_by_zero ... ok

test result: ok. 8 passed; 0 failed
```

ชื่อ Test และจำนวน Tests สามารถแตกต่างได้ตามการออกแบบ แต่ต้องครอบคลุมเงื่อนไขทั้งหมด

## Checklist

- [ ] สร้าง `add()` ได้
- [ ] สร้าง `subtract()` ได้
- [ ] สร้าง `multiply()` ได้
- [ ] สร้าง `divide()` ได้
- [ ] สร้าง Test Functions ได้
- [ ] ใช้ `#[test]` ได้
- [ ] ใช้ `assert!` ได้
- [ ] ใช้ `assert_eq!` ได้
- [ ] ใช้ `assert_ne!` ได้
- [ ] ทดสอบ Normal Cases ได้
- [ ] ทดสอบ Edge Cases ได้
- [ ] จัดการหารด้วยศูนย์ได้
- [ ] Tests ทั้งหมดผ่าน

## Run Project

```powershell
cd 20_testing/exercises/exercise_01/testing_basics
cargo test
```

แสดง Output จาก Tests:

```powershell
cargo test -- --show-output
```
