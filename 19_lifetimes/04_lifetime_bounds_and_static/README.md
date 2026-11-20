# Lesson 04: Lifetime Bounds and Static

## Overview

Lifetime Bounds ใช้กำหนดข้อกำหนดด้าน Lifetime ให้กับ Generic Types และ References ส่วน `'static` เป็น Lifetime ที่มีช่วงการใช้งานตลอดระยะเวลาของโปรแกรมสำหรับ References ที่มี Lifetime ดังกล่าว

บทนี้จะนำ Lifetimes มาประยุกต์ใช้ร่วมกับ Generics และ Traits

## Learning Objectives

- เข้าใจ Lifetime Bounds
- ใช้ Lifetime Bounds กับ Generic Types ได้
- เข้าใจความสัมพันธ์ระหว่าง Lifetimes และ Trait Bounds
- เข้าใจ Lifetime Subtyping ในระดับพื้นฐาน
- เข้าใจ `'static` Lifetime
- แยก String Literals และ Owned Strings ได้
- เข้าใจข้อควรระวังในการใช้ `'static`

## 1. What are Lifetime Bounds?

Lifetime Bounds คือข้อกำหนดที่ระบุว่า Generic Type ต้องมีความสัมพันธ์กับ Lifetime ที่กำหนด

ตัวอย่าง:

```rust
fn display_value<T: std::fmt::Display + 'static>(value: T) {
    println!("{}", value);
}
```

ในตัวอย่างนี้:

- `T: std::fmt::Display` กำหนดให้ T รองรับ Trait Display
- `T: 'static` กำหนดให้ T ต้องมี Lifetime อย่างน้อย `'static`

ข้อสำคัญ: `'static` ในกรณีนี้เป็นข้อกำหนดของ Type ไม่ได้หมายความว่าค่าของ T จะต้องถูกเก็บไว้จนโปรแกรมปิดเสมอไป

## 2. Lifetime Bounds with References

สามารถกำหนด Lifetime Bounds ให้ Generic Types ได้

ตัวอย่าง:

```rust
struct Data<'a, T: 'a> {
    value: &'a T,
}
```

ความหมาย:

- Struct ใช้ Lifetime Parameter `'a`
- `T: 'a` กำหนดให้ T ต้องมี Lifetime ที่เหมาะสมกับ `'a`
- `value` เป็น Reference ที่ใช้ Lifetime `'a`

สำหรับตัวอย่างนี้ ข้อกำหนดดังกล่าวแสดงความสัมพันธ์ว่า Type T ต้องสามารถใช้งานได้ตลอดช่วง Lifetime ที่ Reference ต้องการ

## 3. Lifetimes with Trait Bounds

สามารถใช้ Lifetime Bounds ร่วมกับ Trait Bounds ได้

ตัวอย่าง:

```rust
fn print_value<T>(value: &T)
where
    T: std::fmt::Display,
{
    println!("{}", value);
}
```

ตัวอย่างนี้ใช้ Trait Bound เพื่อกำหนดความสามารถในการแสดงผล

หากต้องการกำหนดข้อกำหนดด้าน Lifetime เพิ่มเติม สามารถเพิ่ม Lifetime Bounds ใน `where` Clause ได้

## 4. The Static Lifetime

`'static` เป็น Lifetime ที่มีช่วงการใช้งานตลอดระยะเวลาของโปรแกรม

ตัวอย่าง String Literal:

```rust
let message: &'static str = "Hello Rust";
```

String Literal มีข้อมูลที่ถูกเก็บไว้ใน Binary ของโปรแกรม และ Reference สามารถใช้งานได้ตลอดอายุของโปรแกรม

## 5. Static References and Owned Data

ควรแยกความแตกต่างระหว่าง:

**Static Reference**

Reference ที่มี Lifetime เป็น `'static`

ตัวอย่าง:

```rust
let message: &'static str = "Rust";
```

**Owned String**

ข้อมูล String ที่เป็นเจ้าของข้อมูลเอง

ตัวอย่าง:

```rust
let message = String::from("Rust");
```

Owned String ไม่ได้มี Lifetime เป็น `'static` โดยอัตโนมัติ แต่สามารถจัดการ Ownership และย้ายหรือส่งต่อข้อมูลได้ตามปกติ

## 6. Common Mistakes with Static

ข้อผิดพลาดที่ควรระวัง:

- คิดว่า `'static` หมายถึงข้อมูลทุกชนิดต้องอยู่จนโปรแกรมปิด
- เข้าใจว่า `String` ทุกตัวมี Lifetime เป็น `'static`
- ใช้ `'static` เพื่อหลีกเลี่ยงการทำความเข้าใจ Borrowing
- กำหนด Lifetime Bounds โดยไม่เข้าใจความสัมพันธ์ของข้อมูล

ไม่ควรใช้ `'static` เพียงเพื่อแก้ Compile Error โดยไม่เข้าใจสาเหตุ

## 7. Important Concepts

- Lifetime Bounds กำหนดข้อกำหนดด้าน Lifetime
- Trait Bounds กำหนดความสามารถของ Generic Types
- สามารถใช้ Lifetime Bounds และ Trait Bounds ร่วมกันได้
- `'static` เป็น Lifetime ที่ยาวตลอดอายุโปรแกรม
- String Literals มี Reference ที่เป็น `'static`
- Owned String ไม่ได้เป็น `'static` โดยอัตโนมัติ
- การกำหนด `'static` ไม่ได้ทำให้ข้อมูลทุกชนิดมีอายุยาวขึ้น

## Summary

Lifetime Bounds และ `'static` ช่วยกำหนดข้อกำหนดด้าน Lifetime ให้กับ Generic Types และ References โดยต้องเข้าใจความสัมพันธ์ระหว่าง Ownership, Borrowing และ Lifetimes ก่อนนำไปใช้

## Checklist
