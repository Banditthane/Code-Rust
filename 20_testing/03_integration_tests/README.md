# Lesson 03: Integration Tests

## Overview

Integration Testing คือการทดสอบการทำงานร่วมกันของส่วนต่าง ๆ ในโปรแกรม โดยมักทดสอบผ่าน Public API ของ Library

Rust ใช้โฟลเดอร์ `tests/` ที่อยู่ระดับเดียวกับ `src/` สำหรับจัดเก็บ Integration Test Files

## Learning Objectives

- เข้าใจ Integration Testing
- เข้าใจความแตกต่างระหว่าง Unit Tests และ Integration Tests
- สร้างโฟลเดอร์ `tests/` ได้
- สร้าง Integration Test Files ได้
- ทดสอบ Public API ได้
- ทดสอบการทำงานร่วมกันของหลาย Modules ได้
- ใช้ Shared Test Helpers ได้
- รัน Integration Tests แยกไฟล์ได้

## 1. What is Integration Testing?

Integration Testing คือการทดสอบการทำงานร่วมกันของส่วนประกอบต่าง ๆ ในโปรแกรม

ตัวอย่าง:

- ทดสอบ Function ที่เรียกใช้งานหลาย Modules
- ทดสอบการส่งข้อมูลระหว่าง Modules
- ทดสอบ Public API ของ Library
- ทดสอบการทำงานของระบบในภาพรวม

## 2. Unit Tests vs Integration Tests

| Topic                    | Unit Tests                   | Integration Tests          |
| ------------------------ | ---------------------------- | -------------------------- |
| จุดประสงค์               | ทดสอบส่วนย่อย                | ทดสอบการทำงานร่วมกัน       |
| ตำแหน่ง                  | ภายใน Source Modules         | โฟลเดอร์ `tests/`          |
| การเข้าถึง Private Items | เข้าถึงได้ตาม Module Privacy | เข้าถึงได้เฉพาะ Public API |
| ขอบเขต                   | ระดับ Function หรือ Module   | ระดับการใช้งาน Library     |

## 3. Integration Test Folder

โครงสร้างโปรเจกต์ Library:

```text
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

ไฟล์ภายใน `tests/` แต่ละไฟล์หลักจะถูก Cargo มองเป็น Integration Test Crate แยกกัน

## 4. Testing Public API

Integration Tests ใช้ทดสอบ Public API ของ Library

ตัวอย่างการ Import Library:

```rust
use my_project::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

ชื่อ `my_project` ต้องตรงกับชื่อ Crate ที่กำหนดใน `Cargo.toml` โดยชื่อที่ใช้ใน Rust อาจต่างจาก Package Name หากมีการกำหนด `lib.name` แยกไว้

## 5. Testing Multiple Modules

หาก Library มีหลาย Modules สามารถเรียกใช้ Public Items จากแต่ละ Module ใน Integration Tests ได้

ควรทดสอบ:

- การเรียกใช้ Functions
- การส่งข้อมูลระหว่าง Modules
- การทำงานร่วมกันของ Components
- ผลลัพธ์ของระบบในภาพรวม

## 6. Shared Test Helpers

สามารถสร้าง Subdirectory ภายใน `tests/` เพื่อเก็บไฟล์ช่วยเหลือที่ไม่ต้องการให้ Cargo มองเป็น Integration Test Crate แยก

ตัวอย่าง:

```text
tests/
├── common/
│   └── mod.rs
├── calculator_test.rs
└── user_test.rs
```

จากนั้นนำ Helper Module มาใช้ใน Test Files ได้

ตัวอย่าง:

```rust
mod common;
```

## 7. Running Integration Tests

รัน Tests ทั้งหมด:

```powershell
cargo test
```

รัน Integration Test ที่ระบุชื่อไฟล์:

```powershell
cargo test --test integration_test
```

รัน Test ที่ตรงกับชื่อ:

```powershell
cargo test test_add
```

## 8. Important Concepts

- Integration Tests อยู่ในโฟลเดอร์ `tests/`
- Integration Tests ใช้งาน Public API
- Integration Tests ไม่สามารถเข้าถึง Private Items ของ Library โดยตรง
- แต่ละไฟล์ Integration Test หลักเป็น Test Crate แยกกัน
- Shared Test Helpers สามารถจัดเก็บใน Subdirectory เช่น `tests/common/`
- Binary-only Crate ไม่สามารถถูก Import เป็น Library ใน Integration Tests ได้โดยตรง หากต้องการทดสอบผ่าน Public API ควรแยก Logic ที่ต้องการทดสอบไว้ใน Library Crate

## Summary

Integration Testing ช่วยตรวจสอบการทำงานร่วมกันของ Modules และ Components ผ่าน Public API ทำให้มั่นใจได้ว่าโปรแกรมสามารถทำงานร่วมกันได้อย่างถูกต้อง

## Checklist

- [ ] เข้าใจ Integration Testing
- [ ] อธิบายความแตกต่างระหว่าง Unit และ Integration Tests ได้
- [ ] สร้างโฟลเดอร์ `tests/` ได้
- [ ] สร้าง Integration Test Files ได้
- [ ] ทดสอบ Public API ได้
- [ ] ทดสอบหลาย Modules ร่วมกันได้
- [ ] สร้าง Shared Test Helpers ได้
- [ ] รัน Integration Tests แยกไฟล์ได้
