# Exercise 04: Option Challenge

## Objective

ฝึกใช้งาน `Option<T>`, `Some`, `None`, `match` และ `if let` เพื่อจัดการข้อมูลที่อาจมีหรือไม่มี

## Project

`option_challenge`

## Scenario

สร้างโปรแกรมค้นหาข้อมูลนักเรียนจากรายการคะแนน โดยกำหนดให้บางกรณีอาจไม่พบข้อมูลที่ต้องการ

โปรแกรมต้องสามารถตรวจสอบผลการค้นหาและจัดการกรณีที่ไม่พบข้อมูลได้โดยไม่เกิด Panic

## Requirements

### Part 1: Create Student Scores

สร้าง Vector สำหรับเก็บคะแนนนักเรียนอย่างน้อย 5 ค่า

ตัวอย่างข้อมูล:

```text
[75, 80, 65, 90, 85]
```

### Part 2: Search Score with get()

ใช้ `.get()` เพื่อค้นหาคะแนนจาก Index ที่กำหนด

ข้อกำหนด:

- ทดลองค้นหา Index ที่มีข้อมูล
- ทดลองค้นหา Index ที่ไม่มีข้อมูล
- เก็บผลลัพธ์ในตัวแปรชนิด `Option`

### Part 3: Handle with match

ใช้ `match` ตรวจสอบผลลัพธ์:

- `Some(value)` แสดงคะแนนที่ค้นพบ
- `None` แสดงข้อความว่าไม่พบข้อมูล

### Part 4: Handle with if let

ทดลองใช้ `if let` ตรวจสอบผลลัพธ์อีกครั้ง

ข้อกำหนด:

- หากเป็น `Some(value)` ให้แสดงคะแนน
- หากไม่มีค่า ให้ใช้ `else` แสดงข้อความแจ้งเตือน

### Part 5: Create a Function

สร้าง Function ชื่อ `find_score()` เพื่อรับ:

- Reference ของ Vector คะแนน
- Index ที่ต้องการค้นหา

กำหนด Return Type เป็น:

```rust
Option<&u32>
```

Function ต้องคืนผลลัพธ์จากการค้นหา โดยไม่ทำให้โปรแกรม Panic เมื่อไม่พบข้อมูล

## Expected Output

ตัวอย่างกรณีค้นพบข้อมูล:

```text
=== Student Score Search ===

Searching Index: 2
Score Found: 65
```

ตัวอย่างกรณีไม่พบข้อมูล:

```text
=== Student Score Search ===

Searching Index: 10
Score Not Found
```

ตัวอย่างการใช้ `match` และ `if let` ต้องแสดงผลสอดคล้องกับข้อมูลที่ค้นหา

## Constraints

- ต้องใช้ Vector สำหรับเก็บคะแนน
- ต้องใช้ `.get()` ในการค้นหา
- ต้องจัดการผลลัพธ์ด้วย `Option`
- ต้องใช้ `match` อย่างน้อย 1 ครั้ง
- ต้องใช้ `if let` อย่างน้อย 1 ครั้ง
- ต้องมี Function `find_score()`
- Function ต้องคืนค่าเป็น `Option<&u32>`
- ห้ามใช้ Indexing โดยตรงในกรณีที่อาจไม่มีข้อมูล เช่น `scores[index]`
- ห้ามใช้ `unwrap()` เพื่อบังคับเอาค่าออกมาโดยไม่ตรวจสอบ
- โปรแกรมต้องไม่ Panic เมื่อค้นหา Index ที่ไม่มีข้อมูล

## Checklist

- [ ] สร้าง Vector คะแนน
- [ ] ใช้ `.get()` ค้นหาข้อมูล
- [ ] ทดลองค้นหา Index ที่มีข้อมูล
- [ ] ทดลองค้นหา Index ที่ไม่มีข้อมูล
- [ ] เข้าใจ `Some(value)`
- [ ] เข้าใจ `None`
- [ ] ใช้ `match` จัดการผลลัพธ์
- [ ] ใช้ `if let` จัดการผลลัพธ์
- [ ] สร้าง Function `find_score()`
- [ ] กำหนด Return Type เป็น `Option<&u32>`
- [ ] จัดการกรณีไม่พบข้อมูลได้
- [ ] โปรแกรมไม่เกิด Panic เมื่อค้นหา Index ที่ไม่มีข้อมูล
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` ทำงานได้ตามต้องการ

## Challenge (Optional)

เพิ่ม Function `find_highest_score()` เพื่อค้นหาคะแนนสูงสุดจาก Vector

ข้อกำหนด:

- ต้องคืนค่าเป็น `Option<&u32>`
- หาก Vector ว่างให้คืน `None`
- หากมีข้อมูลให้คืน Reference ของคะแนนสูงสุด
- ทดลองใช้ `if let` แสดงผลคะแนนสูงสุด
