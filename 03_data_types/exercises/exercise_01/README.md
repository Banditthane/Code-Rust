# Exercise 01: Numeric Types

## Overview

ฝึกใช้งาน Integer และ Floating-Point Types เพื่อจัดเก็บข้อมูลตัวเลข

## Objectives

- ประกาศ Integer Variables
- ใช้ Signed และ Unsigned Integer
- ประกาศ Floating-Point Variables
- เลือกชนิดข้อมูลให้เหมาะสม

## Problem

สร้างโปรแกรมแสดงข้อมูลทางตัวเลขของนักเรียน

ข้อมูลที่ต้องมี:

| Variable    | Value | Suggested Type |
| ----------- | ----: | -------------- |
| age         |    20 | u8             |
| score       |    95 | u32            |
| temperature |  36.5 | f64            |
| height      | 175.5 | f32            |
| balance     |  -500 | i32            |

## Requirements

- ประกาศตัวแปรอย่างน้อย 5 ตัว
- ใช้ Integer อย่างน้อย 2 ชนิด
- ใช้ Floating-Point อย่างน้อย 2 ชนิด
- ระบุชนิดข้อมูลให้ชัดเจน
- แสดงผลข้อมูลทุกตัว

## Expected Output

```text
Age: 20
Score: 95
Temperature: 36.5
Height: 175.5
Balance: -500
```

## Restrictions

- ห้ามใช้ข้อมูลตัวเลขแบบ Hardcode ใน println! ทั้งหมด
- ต้องเก็บข้อมูลในตัวแปรก่อน
- เลือกชนิดข้อมูลให้รองรับค่าที่กำหนด

## Acceptance Criteria

- [ ] ประกาศตัวแปรครบ 5 ตัว
- [ ] ใช้ Integer ถูกต้อง
- [ ] ใช้ Floating-Point ถูกต้อง
- [ ] ระบุชนิดข้อมูลได้ถูกต้อง
- [ ] แสดงผลครบทุกค่า
- [ ] ผ่าน cargo check
- [ ] ผ่าน cargo run

## Reflection

1. Integer ต่างจาก Floating-Point อย่างไร?
2. i32 กับ u32 ต่างกันอย่างไร?
3. f32 กับ f64 ต่างกันอย่างไร?
4. ทำไม age จึงสามารถใช้ u8 ได้?

## Run

```powershell
cargo check
cargo run
```
