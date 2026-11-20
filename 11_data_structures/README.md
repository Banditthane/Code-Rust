# Chapter 11: Data Structures

## Overview

บทนี้เรียนรู้วิธีจัดเก็บและจัดการข้อมูลหลายค่าใน Rust โดยเลือกใช้โครงสร้างข้อมูลให้เหมาะกับลักษณะของข้อมูลและการใช้งาน

## Learning Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- เข้าใจ Slices และการอ้างอิงข้อมูลบางส่วน
- สร้างและจัดการ Vector ด้วย `Vec<T>`
- จัดเก็บข้อมูลแบบ Key-Value ด้วย `HashMap<K, V>`
- อ่านและจัดการข้อมูลที่มีโครงสร้างซ้อนกัน
- เลือกใช้โครงสร้างข้อมูลพื้นฐานให้เหมาะสมกับโจทย์

## Lessons

| Lesson | Topic       | Description                    |
| ------ | ----------- | ------------------------------ |
| 01     | Slices      | การอ้างอิงข้อมูลบางส่วน        |
| 02     | Vectors     | การจัดเก็บข้อมูลแบบปรับขนาดได้ |
| 03     | Hash Maps   | การจัดเก็บข้อมูลแบบ Key-Value  |
| 04     | Nested Data | การจัดเก็บข้อมูลแบบซ้อนกัน     |

## Exercises

| Exercise | Project                  | Description                          |
| -------- | ------------------------ | ------------------------------------ |
| 01       | slices_practice          | ฝึกใช้งาน Slices                     |
| 02       | vector_practice          | ฝึกสร้างและจัดการ Vector             |
| 03       | hashmap_practice         | ฝึกจัดเก็บข้อมูลด้วย HashMap         |
| 04       | data_structure_challenge | ประยุกต์ใช้โครงสร้างข้อมูลหลายรูปแบบ |

## Prerequisites

ควรมีความเข้าใจพื้นฐานเรื่อง:

- Variables และ Data Types
- Arrays และ Tuples
- Strings
- Ownership
- References และ Borrowing
- Functions และ Loops

## Recommended Learning Order

1. อ่าน README ของแต่ละ Lesson ตามลำดับ
2. ทดลองอ่านตัวอย่างและทำความเข้าใจผลลัพธ์
3. ทำ Exercise ที่เกี่ยวข้องกับ Lesson
4. รันโปรแกรมด้วย `cargo run`
5. ตรวจสอบความถูกต้องและทบทวนสิ่งที่เรียนรู้

## Completion Checklist

- [ ] เข้าใจ Slices
- [ ] เข้าใจ Vector และการเพิ่ม/ลบข้อมูล
- [ ] เข้าใจ HashMap และ Key-Value
- [ ] เข้าใจ Nested Data
- [ ] ทำ Exercise ทั้ง 4 ข้อ
- [ ] สามารถเลือกใช้โครงสร้างข้อมูลให้เหมาะกับโจทย์ได้

## References

- [The Rust Programming Language - Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [The Rust Programming Language - Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [The Rust Programming Language - Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

## Next Chapter

บทที่ 12: Structs
