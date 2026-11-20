# Lesson 04: Generic Methods and Trait Bounds

## Overview

Generic Methods คือ Methods ที่ทำงานร่วมกับ Generic Types ส่วน Trait Bounds คือข้อกำหนดที่ใช้ระบุว่า Generic Type ต้องรองรับ Trait ใดบ้าง

เนื้อหานี้ต่อยอดจาก Chapter 17: Traits โดยนำ Traits มาใช้ร่วมกับ Generics

## Learning Objectives

- สร้าง Generic Methods ได้
- เข้าใจ Generic `impl` Blocks
- ใช้ Type Parameters ใน Methods ได้
- กำหนด Trait Bounds ได้
- ใช้ Multiple Trait Bounds ได้
- ใช้ `where` Clause ได้
- เข้าใจการใช้ Generics ร่วมกับ Traits

## 1. Generic impl Blocks

เราสามารถสร้าง Methods ให้กับ Generic Struct ได้

รูปแบบ:

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}
```

คำอธิบาย:

- `impl<T>` ประกาศการทำงานกับ Generic Type T
- `Container<T>` คือ Struct ที่ใช้ Generic Type T
- `&T` คือ Reference ที่ชี้ไปยังข้อมูลชนิด T

## 2. Methods with Their Own Generic Parameters

Method สามารถมี Generic Type Parameters ของตัวเองได้เช่นกัน

ตัวอย่างแนวคิด:

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn compare_with<U>(&self, other: &U) {
        // Method body
    }
}
```

ในตัวอย่างนี้:

- `T` เป็น Generic Parameter ของ Struct
- `U` เป็น Generic Parameter ของ Method

การประกาศ `U` ทำให้ Method สามารถรับข้อมูลอีกชนิดหนึ่งได้ แต่การใช้งานจริงต้องกำหนดว่าจะทำอะไรกับข้อมูลนั้น

## 3. Trait Bounds

Trait Bounds คือข้อกำหนดว่า Generic Type ต้องรองรับ Trait ที่กำหนด

รูปแบบ:

```rust
fn display_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

ตัวอย่างนี้กำหนดว่า T ต้องรองรับ `Display`

หากไม่กำหนด Trait Bound จะไม่สามารถเรียกใช้ความสามารถเฉพาะของ Trait นั้นได้

## 4. Multiple Trait Bounds

Generic Type สามารถกำหนดให้รองรับหลาย Traits ได้

รูปแบบ:

```rust
fn print_value<T: std::fmt::Display + std::fmt::Debug>(value: T) {
    println!("{}", value);
    println!("{:?}", value);
}
```

ในตัวอย่างนี้ T ต้องรองรับทั้ง:

- `Display`
- `Debug`

เครื่องหมาย `+` ใช้เชื่อม Trait Bounds หลายตัว

## 5. Where Clause

เมื่อ Trait Bounds มีหลายข้อ การเขียนไว้หลัง Type Parameter อาจทำให้อ่านยาก

สามารถใช้ `where` Clause เพื่อจัดรูปแบบให้ชัดเจนขึ้น

ตัวอย่าง:

```rust
fn print_value<T>(value: T)
where
    T: std::fmt::Display + std::fmt::Debug,
{
    println!("{}", value);
    println!("{:?}", value);
}
```

ทั้งสองรูปแบบมีจุดประสงค์เดียวกัน คือกำหนดข้อกำหนดให้ Generic Type

## 6. Generics with Traits

Generics สามารถใช้ร่วมกับ Traits เพื่อสร้าง Function ที่ทำงานกับ Types หลายชนิด โดยกำหนดความสามารถที่ต้องการผ่าน Trait Bounds

ตัวอย่างแนวคิด:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

fn print_summary<T: Summary>(value: &T) {
    println!("{}", value.summarize());
}
```

Function นี้รับข้อมูลชนิด T ที่ต้องรองรับ Trait `Summary`

## 7. Important Concepts

- Generic `impl` ใช้กำหนด Methods ให้ Generic Struct
- Struct และ Method สามารถมี Type Parameters ของตัวเองได้
- Trait Bounds กำหนดความสามารถที่ Generic Type ต้องรองรับ
- Multiple Trait Bounds ใช้ `+`
- `where` Clause ช่วยให้อ่าน Trait Bounds ได้ง่ายขึ้น
- Generic Types ไม่สามารถเรียกใช้ Methods ของ Trait ที่ไม่ได้กำหนดไว้

## Summary

Generics และ Traits ทำงานร่วมกันเพื่อสร้างโค้ดที่ยืดหยุ่น โดยใช้ Type Parameters แทนชนิดข้อมูล และใช้ Trait Bounds กำหนดความสามารถที่ต้องการ

## Checklist

- [ ] สร้าง Generic `impl` Block ได้
- [ ] สร้าง Generic Methods ได้
- [ ] แยก Type Parameters ของ Struct และ Method ได้
- [ ] กำหนด Trait Bounds ได้
- [ ] ใช้ Multiple Trait Bounds ได้
- [ ] ใช้ `where` Clause ได้
- [ ] ใช้ Generics ร่วมกับ Traits ได้
