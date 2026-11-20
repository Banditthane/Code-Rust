# Lesson 03: Lifetimes in Functions and Structs

## Overview

Rust สามารถใช้ Lifetime Parameters ร่วมกับ Functions, Structs และ Methods เพื่อระบุความสัมพันธ์ระหว่าง References และข้อมูลที่ถูกจัดเก็บหรือส่งผ่าน

บทนี้จะนำ Lifetime Annotations จาก Lesson 02 มาประยุกต์ใช้กับโครงสร้างโปรแกรม

## Learning Objectives

- ใช้ Lifetimes กับ Functions ได้
- เข้าใจ Struct ที่เก็บ References
- สร้าง Generic Struct ที่มี Lifetime Parameters ได้
- ใช้ Lifetimes กับ Methods ได้
- เข้าใจ Lifetime Parameters ใน `impl` Blocks
- ใช้ Lifetimes ร่วมกับ Generics ได้

## 1. Lifetimes in Functions

Function สามารถรับและคืนค่า References ได้ โดยใช้ Lifetime Parameters ระบุความสัมพันธ์ระหว่าง References

ตัวอย่าง:

```rust
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}
```

สิ่งที่ควรเข้าใจ:

- Function รับ References สองตัว
- References ทั้งสองใช้ Lifetime Parameter `'a`
- Return Reference ใช้ Lifetime Parameter `'a`
- ผู้เรียก Function ต้องส่ง References ที่มีช่วงการใช้งานเหมาะสม

## 2. Structs Holding References

Struct สามารถเก็บ References ได้ แต่ต้องระบุ Lifetime Parameters เพื่อแสดงความสัมพันธ์ของ References ที่เก็บอยู่

ตัวอย่าง:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

คำอธิบาย:

- `ImportantExcerpt` เป็น Struct
- `<'a>` ประกาศ Lifetime Parameter
- `part` เป็น Reference ของ String Slice
- Reference ที่เก็บอยู่ต้องไม่ถูกใช้งานเกิน Lifetime ของข้อมูลต้นทาง

## 3. Creating Instances of Lifetime Structs

เมื่อสร้าง Instance ของ Struct ที่มี Lifetime Parameter ข้อมูลต้นทางต้องมีชีวิตอยู่นานพอสำหรับการใช้งาน Instance และ References ภายใน

ตัวอย่างแนวคิด:

```rust
fn main() {
    let text = String::from("Rust programming");

    let excerpt = ImportantExcerpt {
        part: &text,
    };

    println!("{}", excerpt.part);
}
```

ตัวอย่างนี้ใช้ Reference ไปยังข้อมูลของ `text` และนำมาเก็บไว้ใน Struct

## 4. Lifetimes in Methods

Methods ของ Struct ที่มี Lifetime Parameters สามารถใช้ Lifetimes ใน `impl` Block ได้

ตัวอย่าง:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn get_part(&self) -> &str {
        self.part
    }
}
```

คำอธิบาย:

- `impl<'a>` ประกาศ Lifetime Parameter สำหรับ `impl` Block
- `ImportantExcerpt<'a>` คือ Struct ที่ใช้ Lifetime `'a`
- `&self` เป็น Reference ไปยัง Instance
- Return Reference สามารถอนุมาน Lifetime จาก `&self` ได้ตาม Lifetime Elision Rules

## 5. Methods with Different Lifetime Parameters

Method สามารถมี Lifetime Parameters เพิ่มเติมจาก Lifetime ของ Struct ได้

แนวคิด:

```rust
struct Container<'a> {
    value: &'a str,
}

impl<'a> Container<'a> {
    fn compare<'b>(&self, other: &'b str) {
        // Method body
    }
}
```

ในตัวอย่างนี้:

- `'a` เป็น Lifetime ของ Struct
- `'b` เป็น Lifetime Parameter ของ Method
- ทั้งสอง Lifetimes สามารถใช้แทนความสัมพันธ์ที่แตกต่างกันได้

## 6. Lifetimes with Generics

Struct หรือ Function สามารถใช้ Lifetime Parameters ร่วมกับ Generic Type Parameters ได้

ตัวอย่าง:

```rust
struct Data<'a, T> {
    reference: &'a T,
}
```

ในตัวอย่างนี้:

- `'a` ใช้ระบุ Lifetime ของ Reference
- `T` ใช้ระบุชนิดข้อมูล
- Struct ใช้ทั้ง Lifetime Parameter และ Generic Type Parameter

## 7. Important Concepts

- Struct ที่เก็บ References ต้องระบุ Lifetime Parameters
- Lifetime Parameters ของ Struct ใช้ระบุความสัมพันธ์กับ References ที่จัดเก็บ
- `impl<'a>` ใช้ประกาศ Lifetime Parameter ใน `impl` Block
- Methods สามารถมี Lifetime Parameters ของตัวเองได้
- Lifetimes สามารถใช้ร่วมกับ Generics ได้
- Lifetime Annotation ไม่ได้ทำให้ข้อมูลต้นทางมีอายุยาวขึ้น

## Summary

Lifetimes ใน Functions, Structs และ Methods ช่วยให้ Rust ตรวจสอบความสัมพันธ์ของ References ได้อย่างถูกต้อง และช่วยป้องกันการใช้งาน References ที่ไม่ปลอดภัย

## Checklist

- [ ] ใช้ Lifetimes กับ Functions ได้
- [ ] สร้าง Struct ที่เก็บ References ได้
- [ ] ประกาศ Lifetime Parameters ใน Struct ได้
- [ ] ใช้ Lifetimes กับ Methods ได้
- [ ] ประกาศ Lifetime ใน `impl` Block ได้
- [ ] ใช้ Lifetime Parameters ของ Method ได้
- [ ] ใช้ Lifetimes ร่วมกับ Generics ได้
