# Lesson 01: Generic Basics

## Overview

Generics คือความสามารถที่ช่วยให้เราสามารถเขียนโค้ดที่รองรับข้อมูลหลายชนิด โดยไม่ต้องเขียนโค้ดแยกตาม Data Type

ตัวอย่างเช่น หากต้องการจัดเก็บตัวเลขและข้อความ เราสามารถใช้ Generic Types เพื่อสร้างโครงสร้างข้อมูลที่ยืดหยุ่นได้

## Learning Objectives

- เข้าใจความหมายของ Generics
- เข้าใจประโยชน์ของ Generics
- เข้าใจ Generic Type Parameters
- รู้จักชื่อ Type Parameters เช่น `T`, `U` และ `V`
- เข้าใจความแตกต่างระหว่าง Generic Types และ Concrete Types

## 1. What are Generics?

Generics คือการกำหนด Type Parameters เพื่อให้โค้ดสามารถทำงานกับชนิดข้อมูลที่หลากหลายได้

ตัวอย่างชื่อ Type Parameter:

```rust
T
U
V
```

โดยทั่วไปนิยมใช้ `T` เป็นชื่อแรกของ Type Parameter

ชื่อเหล่านี้เป็นเพียงชื่อที่กำหนดขึ้น ไม่ใช่ชนิดข้อมูลพิเศษของ Rust

## 2. Why Use Generics?

สมมติว่าต้องการสร้าง Function สำหรับรับข้อมูลตัวเลขและข้อความ

หากไม่มี Generics เราอาจต้องสร้าง Function แยกตามชนิดข้อมูล

Generics ช่วยลดการเขียนโค้ดซ้ำ และทำให้สามารถนำโค้ดกลับมาใช้ได้ง่ายขึ้น

## 3. Generic Type Parameters

Type Parameter คือชื่อที่ใช้แทนชนิดข้อมูล ซึ่งจะกำหนดชนิดจริงเมื่อมีการใช้งาน

ตัวอย่างแนวคิด:

```rust
fn display_value<T>(value: T) {
    println!("{:?}", value);
}
```

ตัวอย่างนี้ยังไม่สามารถ Compile ได้ เพราะ `T` ยังไม่ได้กำหนดข้อกำหนดให้รองรับ `Debug`

หากต้องการแสดงค่าด้วย `{:?}` ต้องกำหนด Trait Bound เช่น `T: std::fmt::Debug`

## 4. Concrete Types and Generic Types

**Concrete Type** คือชนิดข้อมูลที่ระบุชัดเจน เช่น:

- `i32`
- `f64`
- `String`
- `bool`

**Generic Type** คือชนิดข้อมูลที่ใช้ Type Parameter เป็นตัวแทน เช่น `T`

## 5. Important Concepts

- Generics ไม่ใช่ชนิดข้อมูลที่ทำให้ทุกชนิดใช้แทนกันได้โดยอัตโนมัติ
- Type Parameter ต้องเป็นไปตามข้อกำหนดของการใช้งาน
- Rust ตรวจสอบชนิดข้อมูลในระหว่าง Compile Time
- Rust ใช้ Monomorphization เพื่อสร้างรูปแบบโค้ดเฉพาะสำหรับชนิดข้อมูลที่ถูกนำไปใช้ในหลายกรณี

## Summary

Generics ช่วยให้โค้ดมีความยืดหยุ่นและลดการเขียนซ้ำ โดยใช้ Type Parameters เป็นตัวแทนชนิดข้อมูล

## Checklist

- [ ] อธิบาย Generics ได้
- [ ] อธิบายประโยชน์ของ Generics ได้
- [ ] เข้าใจ Type Parameters
- [ ] รู้จัก `T`, `U` และ `V`
- [ ] แยก Concrete Types และ Generic Types ได้
