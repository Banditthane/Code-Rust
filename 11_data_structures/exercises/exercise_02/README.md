# Exercise 02: Vector Practice

## Objective

ฝึกสร้าง Vector เพิ่ม ลบ ค้นหา และแสดงข้อมูลด้วย Loop

## Project Structure

```text
exercise_02/
├── README.md
└── vector_practice/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Requirements

เขียนโปรแกรมจัดการคะแนนนักเรียนด้วย Vector

1. สร้าง Vector ชื่อ `scores` โดยมีคะแนนเริ่มต้น 5 ค่า
2. เพิ่มคะแนน `95` ด้วย `.push()`
3. ลบสมาชิกตัวสุดท้ายด้วย `.pop()`
4. แสดงจำนวนสมาชิกด้วย `.len()`
5. ตรวจสอบว่ามีคะแนน `80` ด้วย `.contains()`
6. ใช้ `.get()` เพื่อเข้าถึงข้อมูล index 2
7. ใช้ Loop แสดงคะแนนทั้งหมด

## Constraints

- ต้องใช้ `Vec<i32>`
- ต้องใช้ `mut` สำหรับ Vector ที่มีการเปลี่ยนแปลง
- ต้องใช้ `.push()`
- ต้องใช้ `.pop()`
- ต้องใช้ `.len()`
- ต้องใช้ `.contains()`
- ต้องใช้ `.get()`
- ต้องใช้ Loop แสดงข้อมูล

## Expected Output

กำหนดคะแนนเริ่มต้น:

```text
[70, 80, 75, 90, 85]
```

หลังเพิ่ม `95` และลบสมาชิกตัวสุดท้าย ผลลัพธ์สุดท้ายควรเป็น:

```text
Scores: [70, 80, 75, 90, 85]
Total Scores: 5
Contains 80: true
Score at index 2: Some(75)

All Scores:
70
80
75
90
85
```

## How to Run

```powershell
cd .\11_data_structures\exercises\exercise_02\vector_practice

cargo check
cargo run
```

## Completion Checklist

- [ ] สร้าง Vector คะแนน 5 ค่า
- [ ] เพิ่มข้อมูลด้วย `.push()`
- [ ] ลบข้อมูลด้วย `.pop()`
- [ ] นับจำนวนสมาชิกด้วย `.len()`
- [ ] ตรวจสอบข้อมูลด้วย `.contains()`
- [ ] อ่านข้อมูลด้วย `.get()`
- [ ] ใช้ Loop แสดงคะแนน
- [ ] ผลลัพธ์ตรงตาม Expected Output

## Learning Review

1. เพราะเหตุใด Vector จึงต้องใช้ `mut` ในโจทย์นี้?
2. `.pop()` คืนค่าอะไร?
3. `.contains()` ตรวจสอบอะไร?
4. `.get()` แตกต่างจาก Index อย่างไร?
