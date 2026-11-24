# Exercise 01: Async Basics

## ระดับ

Easy

## Objective

ฝึกสร้าง Async Function และเรียกใช้งานด้วย `.await` ผ่าน Tokio Runtime

## Problem Statement

สร้างโปรแกรมจำลองการทำงานของระบบที่มีขั้นตอน 3 ขั้นตอน:

1. โหลดข้อมูลผู้ใช้
2. ตรวจสอบข้อมูล
3. แสดงผลข้อมูล

แต่ละขั้นตอนต้องเป็น Async Function และคืนค่าที่เหมาะสมให้ขั้นตอนถัดไป

## Requirements

1. สร้าง Async Function ชื่อ `load_user()`
2. สร้าง Async Function ชื่อ `validate_user()`
3. สร้าง Async Function ชื่อ `display_user()`
4. ใช้ `async fn` ประกาศทุก Function
5. ใช้ `.await` เรียก Async Function
6. ใช้ Tokio Runtime ผ่าน `#[tokio::main]`
7. ใช้ `tokio::time::sleep()` จำลองการรอในแต่ละขั้นตอน
8. แสดงข้อความเมื่อแต่ละขั้นตอนทำงานเสร็จ

## Constraints

- ต้องใช้ Tokio
- ต้องมี Async Function อย่างน้อย 3 Function
- ต้องใช้ `.await`
- ต้องไม่ใช้ `std::thread::sleep()` ใน Async Function
- ต้องไม่มีการใช้ `unsafe`
- ไม่ต้องสร้าง Thread ด้วยตนเอง

## Expected Output

```text
Loading user...
User loaded

Validating user...
User validated

Displaying user...
User ID: 1001
User Name: Alice

Process completed
```

## Hints

- ใช้ Struct `User` เก็บข้อมูลผู้ใช้
- ใช้ `tokio::time::sleep(Duration::from_secs(1)).await`
- ให้ `load_user()` คืนค่า `User`
- ให้ `validate_user()` รับข้อมูล User และคืนผลการตรวจสอบ
- ให้ `display_user()` รับข้อมูลที่ผ่านการตรวจสอบแล้ว

## Completion Checklist

- [ ] สร้าง User Struct
- [ ] สร้าง `load_user()`
- [ ] สร้าง `validate_user()`
- [ ] สร้าง `display_user()`
- [ ] ทุก Function เป็น Async Function
- [ ] ใช้ `.await` ถูกต้อง
- [ ] ใช้ Tokio Runtime
- [ ] ใช้ Async Delay
- [ ] แสดงผลตามขั้นตอนครบถ้วน

## Run

```powershell
cd 25_async_programming/exercises/exercise_01/async_basics
cargo add tokio --features full
cargo run
```
