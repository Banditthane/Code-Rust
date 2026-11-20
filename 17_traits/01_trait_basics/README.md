# Lesson 01: Trait Basics

## Overview

Trait คือกลไกที่ใช้กำหนดพฤติกรรมร่วมกันให้ชนิดข้อมูลต่าง ๆ

ตัวอย่างเช่น Struct หลายชนิดอาจมีความสามารถในการแสดงรายละเอียดของตัวเอง แม้จะมีข้อมูลภายในแตกต่างกัน

เราสามารถกำหนดพฤติกรรมดังกล่าวไว้ใน Trait เดียว แล้วให้ Struct แต่ละชนิด Implement Trait นั้นได้

## Learning Objectives

- เข้าใจความหมายของ Trait
- ประกาศ Trait ด้วย `trait`
- กำหนด Method Signatures
- เรียนรู้ Default Method Implementations
- เข้าใจการเรียกใช้ Trait Methods

## 1. What Is a Trait?

Trait ใช้กำหนดชุดของ Methods ที่ชนิดข้อมูลสามารถนำไป Implement ได้

ตัวอย่างแนวคิด:

- Dog สามารถส่งเสียงได้
- Cat สามารถส่งเสียงได้
- Bird สามารถส่งเสียงได้

แม้สัตว์แต่ละชนิดจะทำงานแตกต่างกัน แต่สามารถใช้ Trait เดียวกันกำหนดพฤติกรรมการส่งเสียงได้

## 2. Defining a Trait

ใช้ Keyword `trait` เพื่อประกาศ Trait

```rust
trait Animal {
    fn make_sound(&self);
}
```

ส่วนประกอบ:

- `trait` ใช้ประกาศ Trait
- `Animal` คือชื่อ Trait
- `fn make_sound(&self)` คือ Method Signature
- `&self` ใช้รับ Reference ไปยัง Instance

## 3. Trait Method Signatures

Method Signature คือการกำหนดชื่อ Method และรูปแบบ Parameters กับ Return Type โดยยังไม่ได้กำหนดการทำงานจริง

ตัวอย่าง:

```rust
trait Describe {
    fn description(&self) -> String;
}
```

ชนิดข้อมูลที่ Implement Trait นี้จะต้องกำหนด Method `description()` ให้ตรงตาม Signature

## 4. Default Method Implementations

Trait สามารถกำหนด Method ที่มีการทำงานเริ่มต้นไว้ได้

```rust
trait Animal {
    fn make_sound(&self);

    fn sleep(&self) {
        println!("The animal is sleeping.");
    }
}
```

ในตัวอย่างนี้:

- `make_sound()` ไม่มี Default Implementation
- `sleep()` มี Default Implementation

ชนิดข้อมูลที่ Implement Trait สามารถใช้ Default Method ได้ทันที หรือเขียน Implementation ใหม่เพื่อ Override ก็ได้

## 5. Calling Trait Methods

เมื่อชนิดข้อมูล Implement Trait แล้ว สามารถเรียก Method ผ่าน Instance ได้

แนวคิด:

1. ประกาศ Trait
2. Implement Trait ให้ชนิดข้อมูล
3. สร้าง Instance
4. เรียก Trait Method ผ่าน Instance

## 6. Trait and Struct

Struct ใช้เก็บข้อมูล ส่วน Trait ใช้กำหนดพฤติกรรม

| Concept               | Purpose                     |
| --------------------- | --------------------------- |
| Struct                | กำหนดข้อมูลและ Fields       |
| Trait                 | กำหนดพฤติกรรมและ Methods    |
| impl Struct           | กำหนด Methods ของ Struct    |
| impl Trait for Struct | Implement พฤติกรรมจาก Trait |

## 7. Important Rules

- Trait ไม่ได้สร้าง Instance โดยตรง
- Trait Method ที่ไม่มี Default Implementation ต้องถูก Implement
- Method ที่มี Default Implementation สามารถใช้ได้โดยไม่ต้องเขียนใหม่
- Method ที่รับ `&self` ไม่ได้ย้าย Ownership ของ Instance
- การเรียก Method อาจต้องนำ Trait เข้ามาใน Scope หาก Trait มาจาก Module หรือ Crate อื่น

## Practice Checklist

- [ ] อธิบายความหมายของ Trait ได้
- [ ] ประกาศ Trait ด้วย `trait` ได้
- [ ] กำหนด Method Signature ได้
- [ ] เข้าใจ `&self` ใน Trait Method
- [ ] สร้าง Default Method ได้
- [ ] อธิบายความแตกต่างระหว่าง Trait และ Struct ได้
- [ ] อธิบายขั้นตอนการเรียก Trait Method ได้

## Next Lesson

Lesson 02: Implementing Traits
