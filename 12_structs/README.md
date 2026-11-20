# Chapter 12: Structs

## Overview

บทนี้เรียนรู้เรื่อง **Structs** ในภาษา Rust ซึ่งเป็นวิธีสร้างชนิดข้อมูลของเราเอง (Custom Data Types) เพื่อรวบรวมข้อมูลหลายฟิลด์ที่เกี่ยวข้องกันไว้ภายใต้โครงสร้างเดียว

ตัวอย่างเช่น ข้อมูลนักเรียนหนึ่งคนอาจประกอบด้วยชื่อ อายุ คะแนน และสถานะการเรียน แทนที่จะสร้างตัวแปรแยกกัน เราสามารถใช้ `struct` รวมข้อมูลเหล่านี้ไว้ด้วยกันได้

## Learning Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- อธิบายความหมายและประโยชน์ของ Structs ได้
- ประกาศ Struct และกำหนด Fields ได้
- สร้าง Struct Instances และเข้าถึงข้อมูลแต่ละ Field ได้
- แก้ไขค่าของ Fields ภายใน Mutable Instance ได้
- ใช้ Struct Update Syntax ได้
- สร้าง Methods ด้วย `impl` ได้
- เข้าใจความแตกต่างระหว่าง `&self`, `&mut self` และ `self`
- ใช้ Tuple Structs และ Unit-Like Structs ได้

## Lessons

| Lesson | Topic                          | Description                       |
| ------ | ------------------------------ | --------------------------------- |
| 01     | Struct Basics                  | การประกาศ Struct และ Fields       |
| 02     | Struct Instances               | การสร้าง Instance และจัดการข้อมูล |
| 03     | Struct Methods                 | การสร้าง Methods ด้วย `impl`      |
| 04     | Tuple Structs and Unit Structs | Struct รูปแบบ Tuple และ Unit-Like |

## Exercises

| Exercise | Project            | Description                              |
| -------- | ------------------ | ---------------------------------------- |
| 01       | `student_struct`   | สร้าง Struct สำหรับเก็บข้อมูลนักเรียน    |
| 02       | `product_struct`   | สร้าง Struct สำหรับจัดเก็บข้อมูลสินค้า   |
| 03       | `struct_methods`   | สร้าง Methods เพื่อจัดการข้อมูลใน Struct |
| 04       | `struct_challenge` | ประยุกต์ใช้ Struct และ Methods ร่วมกัน   |

## Recommended Learning Order

1. อ่าน Lesson 01 เพื่อทำความเข้าใจ Struct และ Fields
2. อ่าน Lesson 02 และทดลองสร้าง Instances
3. อ่าน Lesson 03 เพื่อเรียนรู้การสร้าง Methods
4. อ่าน Lesson 04 เพื่อรู้จัก Struct รูปแบบอื่น
5. ทำ Exercises ตามลำดับจาก Exercise 01 ถึง Exercise 04

## Prerequisites

ควรมีความเข้าใจพื้นฐานเรื่องต่อไปนี้:

- Variables และ Data Types
- String และ Numeric Types
- Functions และ Parameters
- Ownership และ Borrowing เบื้องต้น

## How to Run Exercises

เข้าไปยังโฟลเดอร์โปรเจกต์ของ Exercise ที่ต้องการ แล้วรัน:

```powershell
cargo run
```

ตรวจสอบว่าโค้ดสามารถ Compile ได้โดยไม่เกิด Error:

```powershell
cargo check
```

## Completion Checklist

- [ ] เข้าใจความหมายของ Struct
- [ ] ประกาศ Struct พร้อม Fields ได้
- [ ] สร้างและใช้งาน Struct Instances ได้
- [ ] อ่านและแก้ไขค่า Fields ได้
- [ ] เข้าใจ Struct Update Syntax
- [ ] สร้าง Methods ด้วย `impl` ได้
- [ ] เข้าใจ `&self`, `&mut self` และ `self`
- [ ] ใช้งาน Tuple Structs และ Unit-Like Structs ได้
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Learning Resources

- Rust Book: https://doc.rust-lang.org/book/ch05-00-structs.html
- Rust by Example: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
