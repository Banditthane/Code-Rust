# Lesson 01: Lifetime Basics

## Overview

Lifetime คือระยะเวลาที่ Reference สามารถใช้งานได้อย่างถูกต้อง โดย Reference ต้องไม่ถูกนำไปใช้งานหลังจากข้อมูลต้นทางหมดอายุการใช้งาน

Rust ใช้ Borrow Checker ตรวจสอบความสัมพันธ์นี้ในระหว่าง Compile Time

## Learning Objectives

- เข้าใจความหมายของ Lifetime
- เข้าใจความสัมพันธ์ระหว่าง References และ Lifetimes
- เข้าใจ Scope และ Lifetime
- เข้าใจ Dangling References
- เข้าใจการทำงานของ Borrow Checker
- แยก Lifetime ของข้อมูลต้นทางและ Reference ได้

## 1. What is a Lifetime?

Lifetime คือช่วงเวลาที่ Reference สามารถใช้งานได้อย่างถูกต้อง

ตัวอย่างแนวคิด:

```rust
fn main() {
    let name = String::from("Rust");

    let reference = &name;

    println!("{}", reference);
}
```

ในตัวอย่างนี้:

- `name` เป็นเจ้าของข้อมูล String
- `reference` เป็น Reference ที่ยืมข้อมูลจาก `name`
- Reference สามารถใช้งานได้ในขณะที่ข้อมูลต้นทางยังมีชีวิตอยู่

## 2. Scope and Lifetime

Scope คือขอบเขตที่ตัวแปรสามารถเข้าถึงได้ ส่วน Lifetime คือช่วงเวลาที่ข้อมูลหรือ Reference สามารถใช้งานได้อย่างถูกต้อง

ตัวอย่าง:

```rust
fn main() {
    {
        let value = 100;
        println!("{}", value);
    }

    // value is no longer accessible here
}
```

ตัวแปร `value` มี Scope อยู่ภายใน Block ที่ประกาศไว้ และไม่สามารถเข้าถึงจากภายนอก Block ได้

## 3. References and Lifetimes

Reference จะต้องไม่ถูกใช้งานหลังจากข้อมูลต้นทางหมดอายุ

Rust ตรวจสอบว่า:

- ข้อมูลต้นทางยังมีชีวิตอยู่หรือไม่
- Reference ยังอยู่ในช่วงเวลาที่ใช้งานได้หรือไม่
- มีการใช้งาน Reference ที่ไม่ถูกต้องหรือไม่

## 4. Dangling References

Dangling Reference คือ Reference ที่ชี้ไปยังข้อมูลที่ไม่มีชีวิตอยู่แล้ว

Rust ป้องกันปัญหานี้ด้วยการตรวจสอบใน Compile Time

ตัวอย่างแนวคิดที่ไม่ถูกต้อง:

```rust
fn main() {
    let reference;

    {
        let value = 100;
        reference = &value;
    }

    // reference cannot be used here
}
```

Reference ไม่สามารถถูกนำไปใช้ภายนอกช่วงเวลาที่ข้อมูลต้นทางยังมีชีวิตอยู่ได้

โค้ดลักษณะนี้จะไม่ผ่านการ Compile หากพยายามใช้งาน Reference หลังจาก `value` หมดอายุ

## 5. Borrow Checker

Borrow Checker เป็นกลไกของ Rust ที่ตรวจสอบความถูกต้องของ References และ Borrowing

หน้าที่สำคัญ:

- ตรวจสอบการยืมข้อมูล
- ตรวจสอบการใช้งาน References
- ป้องกัน Dangling References
- ตรวจสอบความสัมพันธ์ของ Lifetimes
- ป้องกันการใช้งานข้อมูลที่ไม่ถูกต้องตามกฎ Ownership และ Borrowing

## 6. Important Concepts

- Lifetime ไม่ใช่คำสั่งที่ทำให้ข้อมูลมีอายุยาวขึ้น
- Reference ต้องไม่ถูกใช้งานเกิน Lifetime ของข้อมูลต้นทาง
- Scope และ Lifetime มีความเกี่ยวข้องกัน แต่ไม่ใช่แนวคิดเดียวกัน
- Rust ตรวจสอบ Lifetime ใน Compile Time
- ไม่จำเป็นต้องเขียน Lifetime Annotation ในทุก Reference เพราะ Rust สามารถอนุมานได้ในหลายกรณี

## Summary

Lifetime เป็นแนวคิดสำคัญที่ช่วยให้ Rust ป้องกันการใช้งาน References ที่ไม่ถูกต้อง โดยทำงานร่วมกับ Ownership, Borrowing และ Borrow Checker

## Checklist

- [ ] อธิบาย Lifetime ได้
- [ ] เข้าใจ Scope ได้
- [ ] อธิบายความสัมพันธ์ระหว่าง Reference และข้อมูลต้นทางได้
- [ ] อธิบาย Dangling Reference ได้
- [ ] เข้าใจหน้าที่ของ Borrow Checker
- [ ] เข้าใจว่า Lifetime ไม่ได้ทำให้ข้อมูลมีอายุยาวขึ้น
