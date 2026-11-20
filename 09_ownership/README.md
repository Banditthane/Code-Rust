# Chapter 09: Ownership

## Overview

Ownership คือระบบจัดการหน่วยความจำของ Rust ที่กำหนดว่าใครเป็นเจ้าของข้อมูล และข้อมูลจะถูกใช้งานได้นานแค่ไหน

Rust ใช้ Ownership เพื่อช่วยป้องกันปัญหาการจัดการหน่วยความจำ เช่น การใช้ข้อมูลหลังจากถูกคืนหน่วยความจำ และการจัดการข้อมูลที่ไม่ถูกต้อง

## Learning Objectives

เมื่อเรียนจบบทนี้ คุณจะสามารถ:

- เข้าใจกฎพื้นฐานของ Ownership
- อธิบายความแตกต่างระหว่าง Move และ Copy
- ใช้ Clone เพื่อคัดลอกข้อมูล
- เข้าใจ Scope และการทำงานของ Drop
- วิเคราะห์ได้ว่าตัวแปรใดเป็นเจ้าของข้อมูล

## Lessons

| Lesson | Topic            | Description                      |
| ------ | ---------------- | -------------------------------- |
| 01     | Ownership Basics | ทำความเข้าใจกฎ Ownership         |
| 02     | Move Semantics   | เข้าใจการย้าย Ownership          |
| 03     | Copy and Clone   | เข้าใจการคัดลอกข้อมูล            |
| 04     | Scope and Drop   | เข้าใจขอบเขตและการคืนหน่วยความจำ |

## Exercises

| Exercise | Project             | Description                |
| -------- | ------------------- | -------------------------- |
| 01       | ownership_basics    | ทดลองกฎ Ownership          |
| 02       | move_challenge      | ทดลองการย้าย Ownership     |
| 03       | copy_clone          | เปรียบเทียบ Copy และ Clone |
| 04       | ownership_challenge | ประยุกต์ใช้ Ownership      |

## Key Concepts

- Ownership
- Owner
- Scope
- Move
- Copy
- Clone
- Drop

## Important Rules

Rust มี Ownership Rules หลัก 3 ข้อ:

1. แต่ละค่าจะมีตัวแปรหนึ่งตัวเป็นเจ้าของข้อมูลในแต่ละช่วงเวลา
2. สามารถมีเจ้าของข้อมูลได้เพียงหนึ่งตัวในเวลาเดียวกัน
3. เมื่อเจ้าของข้อมูลออกจาก Scope ค่านั้นจะถูก Drop

## Learning Path

แนะนำให้เรียนตามลำดับ:

1. อ่านเนื้อหาแต่ละ Lesson
2. ทดลองอ่านและทำความเข้าใจตัวอย่าง
3. ทำ Exercise ตามลำดับ
4. ใช้ `cargo check` ตรวจสอบโค้ด
5. ใช้ `cargo run` ทดลองผลลัพธ์

## References

- [The Rust Programming Language - Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

## Completion Checklist

- [ ] เข้าใจกฎ Ownership ทั้ง 3 ข้อ
- [ ] เข้าใจ Move Semantics
- [ ] เข้าใจ Copy และ Clone
- [ ] เข้าใจ Scope และ Drop
- [ ] ทำ Exercises ทั้ง 4 ข้อเสร็จแล้ว
