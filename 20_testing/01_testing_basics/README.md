# Lesson 01: Testing Basics

## Overview

Testing คือกระบวนการตรวจสอบว่าโปรแกรมทำงานตามที่คาดหวังหรือไม่

Rust มีระบบ Testing ที่ทำงานร่วมกับ Cargo โดยสามารถสร้าง Test Function และใช้คำสั่ง `cargo test` เพื่อรันทดสอบได้

## Learning Objectives

- เข้าใจความหมายของ Software Testing
- เข้าใจ Test Functions
- สร้าง Test Function ด้วย `#[test]` ได้
- ใช้ `cargo test` ได้
- เข้าใจผลลัพธ์ Test Pass และ Test Fail
- อ่านข้อความ Error จากการทดสอบได้

## 1. What is Testing?

Testing คือการตรวจสอบพฤติกรรมของโปรแกรมด้วยการกำหนดข้อมูลทดสอบและตรวจสอบผลลัพธ์ที่ได้

ประโยชน์ของ Testing:

- ตรวจสอบความถูกต้องของโปรแกรม
- ค้นหาข้อผิดพลาด
- ลดโอกาสเกิด Regression
- เพิ่มความมั่นใจในการแก้ไขโค้ด
- ช่วยให้การพัฒนาโปรแกรมเป็นระบบมากขึ้น

## 2. Test Functions

Test Function คือ Function ที่เขียนขึ้นเพื่อทดสอบพฤติกรรมของโปรแกรม

Rust ใช้ Attribute `#[test]` เพื่อระบุว่า Function นั้นเป็น Test

ตัวอย่างรูปแบบ:

```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 3, 5);
}
```

คำอธิบาย:

- `#[test]` ระบุว่า Function เป็น Test
- `test_addition()` คือชื่อ Test Function
- `assert_eq!` ใช้ตรวจสอบว่าค่าทั้งสองเท่ากัน

## 3. Running Tests

ใช้คำสั่ง:

```powershell
cargo test
```

Cargo จะ Compile และรัน Tests ที่ค้นพบในโปรเจกต์

ตัวอย่างผลลัพธ์:

```text
running 1 test
test test_addition ... ok

test result: ok. 1 passed; 0 failed
```

## 4. Test Pass and Test Fail

**Test Pass**

หมายถึง Test ผ่านเงื่อนไขที่กำหนด

**Test Fail**

หมายถึง Test ไม่ผ่านเงื่อนไขที่กำหนด

ตัวอย่าง:

```rust
#[test]
fn test_wrong_result() {
    assert_eq!(2 + 3, 10);
}
```

Test นี้จะไม่ผ่าน เพราะผลลัพธ์จริงไม่เท่ากับค่าที่คาดหวัง

## 5. Test Output

คำสั่งสำหรับแสดง Output ที่ Test พิมพ์ออกมาด้วย `println!`:

```powershell
cargo test -- --show-output
```

โดยปกติ Output จาก Test จะไม่แสดงเมื่อ Test ผ่าน เว้นแต่จะกำหนดให้แสดงด้วย Option ดังกล่าว

## 6. Organizing Test Functions

สามารถสร้าง Test Functions หลายตัวภายในโปรเจกต์เดียวกันได้

ควรตั้งชื่อให้สื่อถึงสิ่งที่ต้องการทดสอบ เช่น:

- `test_addition`
- `test_subtraction`
- `test_multiplication`
- `test_division`

ชื่อ Test ที่ชัดเจนช่วยให้ตรวจสอบผลลัพธ์และค้นหาปัญหาได้ง่ายขึ้น

## 7. Important Concepts

- Test Function ใช้ `#[test]`
- Cargo ใช้ `cargo test` เพื่อรัน Tests
- Test Pass หมายถึงผ่าน Assertions
- Test Fail หมายถึงมี Assertion หรือเงื่อนไขที่ไม่ผ่าน
- Test Function ต้องไม่มี Parameter และต้องคืนค่าเป็น `()`
- Test สามารถทำงานแบบขนานได้ตามค่าเริ่มต้น

## Summary

Testing เป็นเครื่องมือสำคัญในการตรวจสอบความถูกต้องของโปรแกรม Rust โดยเริ่มจาก Test Functions และใช้ Cargo เป็นเครื่องมือสำหรับ Compile และ Run Tests

## Checklist

- [ ] เข้าใจความหมายของ Testing
- [ ] เข้าใจประโยชน์ของ Testing
- [ ] สร้าง Test Function ได้
- [ ] ใช้ `#[test]` ได้
- [ ] ใช้ `cargo test` ได้
- [ ] เข้าใจ Test Pass
- [ ] เข้าใจ Test Fail
- [ ] อ่าน Test Output ได้
