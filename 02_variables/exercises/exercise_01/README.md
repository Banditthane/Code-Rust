# Exercise 01: Variable Basics

## Overview

ฝึกประกาศตัวแปรด้วย `let` และนำค่าตัวแปรมาแสดงผลด้วย `println!()`

## Objectives

- ประกาศตัวแปรด้วย `let`
- ใช้ตัวแปรหลายตัวในโปรแกรมเดียวกัน
- เข้าใจ Immutable Variables
- แสดงค่าตัวแปรด้วย `println!()`

## Problem

สร้างโปรแกรมแสดงข้อมูลส่วนตัว โดยใช้ตัวแปรเก็บข้อมูล

ข้อมูลที่ต้องมี:

1. Name
2. Age
3. Country
4. Language
5. Goal

## Requirements

- ต้องประกาศตัวแปรอย่างน้อย 5 ตัว
- ใช้ `let` ในการประกาศตัวแปร
- ใช้ชื่อตัวแปรแบบ snake_case
- แสดงข้อมูลทุกตัวด้วย `println!()`
- ห้ามใช้ `mut` ใน Exercise นี้

## Example

```rust
fn main() {
    let name = "Alex";
    let age = 20;
    let country = "Thailand";
    let language = "Rust";
    let goal = "Become a Rust Developer";

    println!("Name: {name}");
    println!("Age: {age}");
    println!("Country: {country}");
    println!("Language: {language}");
    println!("Goal: {goal}");
}
```

## Expected Output

```text
Name: Alex
Age: 20
Country: Thailand
Language: Rust
Goal: Become a Rust Developer
```

## Restrictions

- ห้ามใช้ Global Variables
- ห้ามใช้ `mut`
- ห้าม Hardcode ข้อมูลลงใน `println!()` ทั้งหมด
- ต้องจัดเก็บข้อมูลด้วยตัวแปรก่อนแสดงผล

## Acceptance Criteria

- [ ] ประกาศตัวแปรครบ 5 ตัว
- [ ] ใช้ `let` ถูกต้อง
- [ ] ตั้งชื่อตัวแปรเหมาะสม
- [ ] แสดงข้อมูลครบ
- [ ] โปรแกรม Compile ผ่าน
- [ ] โปรแกรม Run ได้

## Run

```powershell
cargo check
cargo run
```

## Reflection

หลังทำ Exercise เสร็จ ให้ตอบคำถาม:

1. `let` ใช้ทำอะไร?
2. เพราะเหตุใดตัวแปรที่ประกาศด้วย `let` จึงเปลี่ยนค่าไม่ได้?
3. Type Inference คืออะไร?

## Learning Outcome

เข้าใจการประกาศตัวแปรและการนำตัวแปรมาใช้งานในโปรแกรม Rust
