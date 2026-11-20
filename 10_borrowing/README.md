# Chapter 10: Borrowing

## Overview

Borrowing คือแนวคิดใน Rust ที่อนุญาตให้ตัวแปรหนึ่งยืมข้อมูลจากตัวแปรอื่นได้ โดยไม่ต้องย้าย Ownership

การ Borrowing ใช้ Reference เพื่อเข้าถึงข้อมูล และแบ่งออกเป็น 2 รูปแบบหลัก:

- Immutable Borrowing (`&T`) — ยืมข้อมูลเพื่ออ่าน
- Mutable Borrowing (`&mut T`) — ยืมข้อมูลเพื่อแก้ไข

## Learning Objectives

เมื่อเรียนจบบทนี้ คุณจะสามารถ:

- เข้าใจ Reference
- สร้าง Immutable Reference ได้
- สร้าง Mutable Reference ได้
- ใช้ Reference เป็น Function Parameter ได้
- เข้าใจกฎและข้อจำกัดของ Borrowing
- อธิบายความแตกต่างระหว่าง Ownership, Move และ Borrowing ได้

## Lessons

| Lesson | Topic               | Description                    |
| ------ | ------------------- | ------------------------------ |
| 01     | References          | ความหมายและการใช้งาน Reference |
| 02     | Immutable Borrowing | การยืมข้อมูลเพื่ออ่าน          |
| 03     | Mutable Borrowing   | การยืมข้อมูลเพื่อแก้ไข         |
| 04     | Borrowing Rules     | กฎและข้อจำกัดของ Borrowing     |

## Exercises

| Exercise | Project             | Description               |
| -------- | ------------------- | ------------------------- |
| 01       | references          | สร้างและใช้งาน Reference  |
| 02       | immutable_borrowing | ทดลอง Immutable Borrowing |
| 03       | mutable_borrowing   | ทดลอง Mutable Borrowing   |
| 04       | borrowing_challenge | ประยุกต์ใช้ Borrowing     |

## Key Concepts

- Reference (`&`)
- Immutable Reference (`&T`)
- Mutable Reference (`&mut T`)
- Borrowing
- Dereferencing (`*`)
- Borrowing Rules
- Dangling References

## Important Rules

กฎพื้นฐานของ Borrowing:

1. สามารถมี Immutable References หลายตัวที่อ้างถึงข้อมูลเดียวกันได้
2. สามารถมี Mutable Reference ได้เพียงหนึ่งตัวในช่วงเวลาที่มีการยืมแบบ Mutable นั้น
3. ไม่สามารถมี Immutable Reference และ Mutable Reference ที่ใช้งานทับซ้อนกันกับข้อมูลเดียวกันได้
4. Reference ต้องไม่ถูกใช้งานเกินอายุของข้อมูลที่มันอ้างถึง

หมายเหตุ: Rust ตรวจสอบช่วงเวลาการยืมจากการใช้งานจริงของ Reference ด้วย ไม่ได้พิจารณาเฉพาะตำแหน่งที่ประกาศตัวแปรเท่านั้น

## Learning Path

แนะนำให้เรียนตามลำดับ:

1. อ่านเนื้อหาแต่ละ Lesson
2. ทดลองตัวอย่าง Reference
3. ฝึกใช้ Immutable Borrowing
4. ฝึกใช้ Mutable Borrowing
5. ทดลองกรณีที่ผิดกฎ Borrowing
6. ทำ Exercises ทั้ง 4 ข้อ

## References

- [The Rust Programming Language - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust By Example - Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)

## Completion Checklist

- [ ] เข้าใจ Reference
- [ ] เข้าใจ Immutable Borrowing
- [ ] เข้าใจ Mutable Borrowing
- [ ] ใช้ Reference กับ Function ได้
- [ ] เข้าใจกฎ Borrowing
- [ ] ทำ Exercises ทั้ง 4 ข้อเสร็จแล้ว
