# Exercise 03: Constants Challenge

## Overview

ฝึกใช้ Variables และ Constants ร่วมกันในโปรแกรมเดียวกัน

เรียนรู้การเลือกใช้ `let`, `let mut` และ `const` ให้เหมาะสมกับข้อมูล

## Objectives

- ประกาศ Variables ด้วย `let`
- ประกาศ Mutable Variables ด้วย `let mut`
- ประกาศ Constants ด้วย `const`
- ระบุชนิดข้อมูลของ Constants
- ตั้งชื่อ Constants ตาม Convention ของ Rust
- เลือกใช้ตัวแปรและค่าคงที่ได้เหมาะสม

## Problem

สร้างโปรแกรมแสดงข้อมูลเกี่ยวกับแอปพลิเคชัน Rust Learning

กำหนดข้อมูลดังนี้:

| Variable  | Value         | Type      |
| --------- | ------------- | --------- |
| app_name  | Rust Learning | Immutable |
| age       | 20            | Mutable   |
| country   | Thailand      | Constant  |
| max_level | 10            | Constant  |
| score     | 50            | Mutable   |

จากนั้นเปลี่ยนแปลงข้อมูล:

- Age จาก 20 เป็น 21
- Score จาก 50 เป็น 100

ส่วน Constants ต้องคงค่าเดิม

## Requirements

- ใช้ `let` อย่างน้อย 1 ตัว
- ใช้ `let mut` อย่างน้อย 2 ตัว
- ใช้ `const` อย่างน้อย 2 ตัว
- Constants ต้องระบุชนิดข้อมูล
- ตั้งชื่อ Constants ด้วย SCREAMING_SNAKE_CASE
- แสดงข้อมูลก่อนและหลังเปลี่ยนแปลงค่า

## Example

```rust
fn main() {
    let app_name = "Rust Learning";

    let mut age = 20;
    let mut score = 50;

    const COUNTRY: &str = "Thailand";
    const MAX_LEVEL: u32 = 10;

    println!("Application: {app_name}");
    println!("Country: {COUNTRY}");
    println!("Maximum Level: {MAX_LEVEL}");

    println!("Before Update");
    println!("Age: {age}");
    println!("Score: {score}");

    age = 21;
    score = 100;

    println!("After Update");
    println!("Age: {age}");
    println!("Score: {score}");
}
```

## Expected Output

```text
Application: Rust Learning
Country: Thailand
Maximum Level: 10
Before Update
Age: 20
Score: 50
After Update
Age: 21
Score: 100
```

## Restrictions

- ห้ามใช้ `mut` กับ Constants
- ห้ามเปลี่ยนแปลงค่า Constants
- ต้องระบุชนิดข้อมูลของ Constants
- ต้องใช้ตัวแปรให้ตรงตามโจทย์
- ห้ามใช้ข้อมูลทั้งหมดแบบ Hardcode ใน `println!()`

## Acceptance Criteria

- [ ] ใช้ `let` ถูกต้อง
- [ ] ใช้ `let mut` ถูกต้อง
- [ ] ใช้ `const` ถูกต้อง
- [ ] ระบุชนิดข้อมูล Constants ครบ
- [ ] ตั้งชื่อ Constants ถูกต้อง
- [ ] เปลี่ยนค่า Mutable Variables ได้
- [ ] Constants ยังคงค่าเดิม
- [ ] โปรแกรม Compile ผ่าน
- [ ] โปรแกรม Run ได้

## Run

```powershell
cargo check
cargo run
```

## Reflection

1. `let` ต่างจาก `let mut` อย่างไร?
2. `const` ต่างจาก `let` อย่างไร?
3. เพราะเหตุใด Constants จึงต้องระบุชนิดข้อมูล?
4. ทำไม Constants จึงไม่สามารถเปลี่ยนค่าได้?
5. ควรเลือกใช้ `let`, `let mut` และ `const` ในสถานการณ์ใด?

## Learning Outcome

สามารถเลือกใช้ Immutable Variables, Mutable Variables และ Constants ได้อย่างถูกต้องในโปรแกรม Rust
