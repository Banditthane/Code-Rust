# Exercise 03: Data Collection

## Overview

ฝึกใช้งาน Tuple และ Array เพื่อจัดเก็บข้อมูลหลายค่าในโปรแกรมเดียวกัน

## Objectives

- ประกาศ Tuple
- เข้าถึงสมาชิก Tuple
- ประกาศ Array
- เข้าถึงสมาชิก Array ด้วย Index
- แยกความแตกต่างระหว่าง Tuple และ Array

## Problem

สร้างโปรแกรมจัดเก็บข้อมูลนักเรียน

### Part 1: Student Tuple

สร้าง Tuple เพื่อจัดเก็บ:

- Name
- Age
- Is Student

ตัวอย่างข้อมูล:

```rust
("Alex", 20, true)
```

### Part 2: Exam Scores

สร้าง Array สำหรับคะแนนสอบ 5 วิชา:

```rust
[80, 90, 75, 85, 95]
```

### Part 3: Display Information

แสดงข้อมูลนักเรียนและคะแนนสอบทุกตำแหน่ง

## Requirements

- สร้าง Tuple อย่างน้อย 1 ตัว
- Tuple ต้องมีข้อมูลต่างชนิดกัน
- สร้าง Array อย่างน้อย 1 ตัว
- Array ต้องมีข้อมูลชนิดเดียวกัน
- แสดงข้อมูล Tuple ด้วย Index
- แสดงข้อมูล Array ด้วย Index
- แสดงคะแนนครบทั้ง 5 วิชา

## Expected Output

```text
Student Information
Name: Alex
Age: 20
Is Student: true

Exam Scores
Subject 1: 80
Subject 2: 90
Subject 3: 75
Subject 4: 85
Subject 5: 95
```

## Restrictions

- ห้ามใช้ Vec ใน Exercise นี้
- ต้องใช้ Tuple สำหรับข้อมูลนักเรียน
- ต้องใช้ Array สำหรับคะแนนสอบ
- ห้ามใช้ Index ที่เกินขอบเขต Array

## Acceptance Criteria

- [ ] ประกาศ Tuple ได้
- [ ] Tuple มีข้อมูลหลายชนิด
- [ ] เข้าถึง Tuple ด้วย .0, .1, .2 ได้
- [ ] ประกาศ Array ได้
- [ ] Array มีคะแนนครบ 5 ค่า
- [ ] เข้าถึง Array ด้วย Index ได้
- [ ] แสดงผลครบตามโจทย์
- [ ] ผ่าน cargo check
- [ ] ผ่าน cargo run

## Reflection

1. Tuple ใช้ทำอะไร?
2. Array เก็บข้อมูลต่างชนิดกันได้หรือไม่?
3. Index ของ Array เริ่มต้นที่เท่าไร?
4. Tuple และ Array ต่างกันอย่างไร?
5. ถ้าเข้าถึง Array ด้วย Index ที่ไม่มีอยู่ จะเกิดอะไรขึ้น?

## Run

```powershell
cargo check
cargo run
```
