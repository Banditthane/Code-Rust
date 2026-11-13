# Exercise 02: Mutable Challenge

## Overview

ฝึกใช้ Mutable Variables โดยประกาศตัวแปรด้วย `let mut` และทดลองเปลี่ยนแปลงค่าหลายครั้ง

## Objectives

- ใช้ `let mut` ได้
- เปลี่ยนแปลงค่าตัวแปรได้
- เข้าใจ Mutable Variables
- แสดงค่าก่อนและหลังการเปลี่ยนแปลง

## Problem

สร้างโปรแกรมจำลองการเปลี่ยนแปลงข้อมูลของผู้ใช้

กำหนดข้อมูลเริ่มต้น:

- Age = 20
- Score = 50
- Level = 1

จากนั้นเปลี่ยนแปลงข้อมูล:

- Age เป็น 21
- Score เป็น 100
- Level เป็น 2

## Requirements

- ประกาศตัวแปรอย่างน้อย 3 ตัว
- ใช้ `let mut` กับตัวแปรที่ต้องเปลี่ยนค่า
- แสดงค่าก่อนเปลี่ยนแปลง
- เปลี่ยนแปลงค่าตัวแปร
- แสดงค่าหลังเปลี่ยนแปลง

## Example

```rust
fn main() {
    let mut age = 20;
    let mut score = 50;
    let mut level = 1;

    println!("Before Update");
    println!("Age: {age}");
    println!("Score: {score}");
    println!("Level: {level}");

    age = 21;
    score = 100;
    level = 2;

    println!("After Update");
    println!("Age: {age}");
    println!("Score: {score}");
    println!("Level: {level}");
}
```

## Expected Output

```text
Before Update
Age: 20
Score: 50
Level: 1
After Update
Age: 21
Score: 100
Level: 2
```

## Restrictions

- ต้องใช้ `let mut`
- ห้ามประกาศตัวแปรซ้ำเพื่อใช้แทนการเปลี่ยนค่า
- ต้องแสดงค่าก่อนและหลัง Update
- ต้องใช้ตัวแปรเดิมในการเปลี่ยนแปลงข้อมูล

## Acceptance Criteria

- [ ] ประกาศ Mutable Variables ครบ
- [ ] ใช้ `mut` ถูกต้อง
- [ ] เปลี่ยนแปลงค่าตัวแปรได้
- [ ] แสดงค่าก่อนเปลี่ยนแปลง
- [ ] แสดงค่าหลังเปลี่ยนแปลง
- [ ] โปรแกรม Compile ผ่าน
- [ ] โปรแกรม Run ได้

## Run

```powershell
cargo check
cargo run
```

## Reflection

1. `mut` มีหน้าที่อะไร?
2. `let age = 20` กับ `let mut age = 20` ต่างกันอย่างไร?
3. หากไม่ใช้ `mut` แล้วพยายามเปลี่ยนค่า จะเกิดอะไรขึ้น?

## Learning Outcome

เข้าใจการเปลี่ยนแปลงค่าตัวแปรและการใช้งาน Mutable Variables ใน Rust
