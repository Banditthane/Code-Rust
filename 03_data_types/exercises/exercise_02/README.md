# Exercise 02: User Information

## Overview

ฝึกใช้ Boolean, Character และ String Types เพื่อจัดเก็บข้อมูลผู้ใช้

## Objectives

- ใช้ bool
- ใช้ char
- ใช้ &str
- ใช้ String
- เลือกชนิดข้อมูลให้เหมาะสม

## Problem

สร้างโปรแกรมแสดงข้อมูลผู้ใช้ โดยมีข้อมูลดังนี้:

| Variable   | Example Value | Type   |
| ---------- | ------------- | ------ |
| name       | Alex          | &str   |
| country    | Thailand      | &str   |
| age        | 20            | u8     |
| grade      | A             | char   |
| is_student | true          | bool   |
| message    | Learning Rust | String |

## Requirements

- ประกาศตัวแปรอย่างน้อย 6 ตัว
- ใช้ bool อย่างน้อย 1 ตัว
- ใช้ char อย่างน้อย 1 ตัว
- ใช้ &str อย่างน้อย 2 ตัว
- ใช้ String อย่างน้อย 1 ตัว
- แสดงข้อมูลทั้งหมดด้วย println!

## Expected Output

```text
Name: Alex
Country: Thailand
Age: 20
Grade: A
Student: true
Message: Learning Rust
```

## Restrictions

- char ต้องใช้ Single Quotes
- String Slice ต้องใช้ Double Quotes
- ต้องใช้ String::from() เพื่อสร้าง String
- ห้ามเก็บข้อมูลทุกอย่างเป็น String

## Acceptance Criteria

- [ ] ประกาศตัวแปรครบ 6 ตัว
- [ ] ใช้ bool ถูกต้อง
- [ ] ใช้ char ถูกต้อง
- [ ] ใช้ &str ถูกต้อง
- [ ] ใช้ String ถูกต้อง
- [ ] แสดงข้อมูลครบ
- [ ] ผ่าน cargo check
- [ ] ผ่าน cargo run

## Reflection

1. bool เก็บค่าอะไรได้บ้าง?
2. char ใช้เครื่องหมายอะไร?
3. String กับ &str ต่างกันอย่างไร?
4. String::from() ใช้ทำอะไร?

## Run

```powershell
cargo check
cargo run
```
