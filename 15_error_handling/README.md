# Chapter 15: Error Handling

## Overview

Error Handling คือการจัดการข้อผิดพลาดที่อาจเกิดขึ้นระหว่างการทำงานของโปรแกรม เพื่อให้โปรแกรมสามารถรับมือกับสถานการณ์ที่ไม่เป็นไปตามปกติได้อย่างเหมาะสม

Rust แบ่งการจัดการข้อผิดพลาดออกเป็น 2 กลุ่มหลัก:

1. **Unrecoverable Errors** — ข้อผิดพลาดที่ไม่สามารถดำเนินการต่อได้ตามปกติ ใช้ `panic!`
2. **Recoverable Errors** — ข้อผิดพลาดที่โปรแกรมสามารถตรวจสอบและจัดการได้ ใช้ `Result<T, E>`

## Learning Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- อธิบายความแตกต่างระหว่าง Recoverable และ Unrecoverable Errors ได้
- ใช้ `panic!` ได้
- เข้าใจการทำงานของ `unwrap()` และ `expect()`
- เข้าใจ Enum `Result<T, E>`
- ใช้ `Ok` และ `Err` ได้
- จัดการ Error ด้วย `match` ได้
- ใช้ `unwrap_or()` เพื่อกำหนดค่าเริ่มต้นได้
- ส่งต่อ Error ด้วย `?` Operator ได้
- สร้าง Custom Error Types ได้
- ใช้ `Display` เพื่อแสดงข้อความ Error ได้

## Lessons

| Lesson | Topic              | Description                    |
| ------ | ------------------ | ------------------------------ |
| 01     | Panic              | การจัดการ Unrecoverable Errors |
| 02     | Result Enum        | การจัดการ Recoverable Errors   |
| 03     | Propagating Errors | การส่งต่อข้อผิดพลาดด้วย `?`    |
| 04     | Custom Errors      | การสร้าง Error Types ของตัวเอง |

## Exercises

| Exercise | Project                    | Description                             |
| -------- | -------------------------- | --------------------------------------- |
| 01       | `panic_practice`           | ทดลองใช้ Panic และตรวจสอบข้อผิดพลาด     |
| 02       | `result_practice`          | ใช้ Result จัดการการหารและการแปลงข้อมูล |
| 03       | `error_propagation`        | ส่งต่อ Error จากการอ่านและแปลงข้อมูล    |
| 04       | `error_handling_challenge` | สร้างระบบจัดการ Error ของตัวเอง         |

## Recommended Learning Order

1. อ่าน Lesson 01 เพื่อเข้าใจ `panic!`
2. อ่าน Lesson 02 เพื่อเข้าใจ `Result<T, E>`
3. อ่าน Lesson 03 เพื่อเรียนรู้การส่งต่อ Error
4. อ่าน Lesson 04 เพื่อสร้าง Custom Errors
5. ทำ Exercises ตามลำดับ

## Prerequisites

ควรมีความเข้าใจพื้นฐานเรื่อง:

- Variables และ Data Types
- Conditions และ Match
- Functions และ Return Values
- Enums
- Modules
- Ownership และ Borrowing

## How to Run Exercises

เข้าไปยังโฟลเดอร์ Cargo Project ของ Exercise ที่ต้องการ แล้วรัน:

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Recoverable Errors
- [ ] เข้าใจ Unrecoverable Errors
- [ ] ใช้ `panic!` ได้
- [ ] เข้าใจ `unwrap()` และ `expect()`
- [ ] เข้าใจ `Result<T, E>`
- [ ] ใช้ `Ok` และ `Err` ได้
- [ ] ใช้ `match` จัดการ Result ได้
- [ ] ใช้ `unwrap_or()` ได้
- [ ] ใช้ `?` Operator ได้
- [ ] สร้าง Custom Error Enum ได้
- [ ] ใช้ `Display` แสดง Error ได้
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Learning Resources

- Rust Book — Error Handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- Rust Book — Unrecoverable Errors with panic!: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
- Rust Book — Recoverable Errors with Result: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
- Rust Book — To panic! or Not to panic!: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
- Rust Standard Library — Result: https://doc.rust-lang.org/std/result/enum.Result.html
