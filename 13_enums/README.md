# Chapter 13: Enums

## Overview

**Enum (Enumeration)** คือชนิดข้อมูลที่ผู้เขียนโปรแกรมกำหนดขึ้นเอง เพื่อระบุชุดของค่าหรือรูปแบบที่เป็นไปได้

ตัวอย่างเช่น:

- สัญญาณไฟจราจร: Red, Yellow, Green
- สถานะคำสั่งซื้อ: Pending, Processing, Shipped, Delivered
- ประเภทการชำระเงิน: Cash, CreditCard, BankTransfer

Enum ช่วยให้โปรแกรมจัดการตัวเลือกที่มีขอบเขตชัดเจน และสามารถใช้ร่วมกับ `match` เพื่อแยกการทำงานตามแต่ละ Variant ได้

## Learning Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- อธิบายความหมายของ Enum ได้
- ประกาศ Enum และสร้าง Variants ได้
- สร้าง Enum Variants ที่เก็บข้อมูลได้
- ใช้ `match` ตรวจสอบ Enum ได้
- เข้าใจการทำงานของ Pattern Matching
- ใช้ `if let` ตรวจสอบ Variant ที่สนใจได้
- เข้าใจ `Option<T>`, `Some` และ `None`
- จัดการกรณีที่อาจไม่มีค่าได้อย่างเหมาะสม

## Lessons

| Lesson | Topic             | Description                    |
| ------ | ----------------- | ------------------------------ |
| 01     | Enum Basics       | ความหมายและการประกาศ Enum      |
| 02     | Enum Variants     | Variants และการเก็บข้อมูลภายใน |
| 03     | Match with Enums  | ใช้ `match` ตรวจสอบ Enum       |
| 04     | If Let and Option | ใช้ `if let` และ `Option<T>`   |

## Exercises

| Exercise | Project            | Description                                  |
| -------- | ------------------ | -------------------------------------------- |
| 01       | `traffic_light`    | สร้าง Enum สำหรับสัญญาณไฟจราจร               |
| 02       | `shape_enum`       | สร้าง Enum สำหรับรูปทรงที่มีข้อมูลแตกต่างกัน |
| 03       | `order_status`     | จัดการสถานะคำสั่งซื้อด้วย Enum และ match     |
| 04       | `option_challenge` | ฝึกจัดการค่าที่อาจไม่มีด้วย Option           |

## Recommended Learning Order

1. อ่าน Lesson 01 เพื่อเข้าใจ Enum และ Variants
2. อ่าน Lesson 02 เพื่อเรียนรู้การเก็บข้อมูลใน Variants
3. อ่าน Lesson 03 เพื่อใช้ `match` กับ Enum
4. อ่าน Lesson 04 เพื่อเข้าใจ `if let` และ `Option<T>`
5. ทำ Exercises ตามลำดับ

## Prerequisites

ควรมีความเข้าใจพื้นฐานเรื่อง:

- Variables และ Data Types
- Conditions
- Functions
- Structs
- Pattern Matching เบื้องต้นจากการใช้ `match`

## How to Run Exercises

เข้าไปยังโฟลเดอร์ Cargo Project ของ Exercise ที่ต้องการ แล้วรัน:

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจความหมายของ Enum
- [ ] ประกาศ Enum และ Variants ได้
- [ ] สร้าง Enum Instance ได้
- [ ] ใช้ Variants ที่เก็บข้อมูลได้
- [ ] ใช้ `match` กับ Enum ได้
- [ ] เข้าใจ Exhaustive Matching
- [ ] ใช้ `if let` ได้
- [ ] เข้าใจ `Option<T>`, `Some` และ `None`
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Learning Resources

- Rust Book — Enums: https://doc.rust-lang.org/book/ch06-00-enums.html
- Rust Book — The Option Enum: https://doc.rust-lang.org/std/option/enum.Option.html
- Rust by Example — Enums: https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
