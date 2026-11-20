# Lesson 02: Generic Functions

## Overview

Generic Functions คือ Function ที่ใช้ Generic Type Parameters เพื่อรองรับข้อมูลหลายชนิด

แทนที่จะสร้าง Function หลายตัวสำหรับแต่ละ Data Type เราสามารถสร้าง Function ที่ใช้ Type Parameter ได้

## Learning Objectives

- เข้าใจ Generic Functions
- ประกาศ Generic Type Parameters ใน Function ได้
- ใช้ Generic Parameters กับ Function Parameters ได้
- ใช้ Generic Parameters กับ Return Types ได้
- ใช้ Type Parameters มากกว่าหนึ่งตัวได้
- เข้าใจการกำหนด Trait Bounds ให้ Function

## 1. Generic Function Syntax

รูปแบบการประกาศ:

```rust
fn function_name<T>(parameter: T) {
    // Function body
}
```

คำอธิบาย:

- `fn` ใช้ประกาศ Function
- `function_name` คือชื่อ Function
- `<T>` คือ Generic Type Parameter
- `parameter: T` คือ Parameter ที่ใช้ชนิดข้อมูล T

## 2. Generic Function Parameters

Generic Function สามารถรับข้อมูลผ่าน Parameter ที่ใช้ Type Parameter ได้

ตัวอย่างแนวคิด:

```rust
fn display_value<T>(value: T) {
    // Function body
}
```

Function นี้รับข้อมูลชนิด T แต่จะสามารถทำงานกับข้อมูลนั้นได้เพียงเท่าที่ Type T อนุญาต

หากต้องการแสดงค่าด้วย `println!` ต้องกำหนด Trait Bound ที่เหมาะสม เช่น `T: std::fmt::Display`

## 3. Generic Return Types

Generic Function สามารถส่งค่ากลับด้วย Generic Type ได้

รูปแบบ:

```rust
fn get_value<T>(value: T) -> T {
    value
}
```

ชนิดข้อมูลที่ส่งเข้าและส่งกลับเป็นชนิดเดียวกัน

## 4. Multiple Type Parameters

Function สามารถใช้ Type Parameters มากกว่าหนึ่งตัวได้

ตัวอย่างแนวคิด:

```rust
fn create_pair<T, U>(first: T, second: U) {
    // Function body
}
```

ในตัวอย่างนี้:

- `T` ใช้แทนชนิดข้อมูลของ `first`
- `U` ใช้แทนชนิดข้อมูลของ `second`

ทั้งสอง Parameter สามารถมีชนิดข้อมูลแตกต่างกันได้

## 5. Trait Bounds in Functions

Generic Type Parameter ไม่สามารถเรียกใช้ Method ทุกชนิดได้โดยอัตโนมัติ

หากต้องการใช้ความสามารถจาก Trait ต้องกำหนด Trait Bound

ตัวอย่าง:

```rust
fn display_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

ตัวอย่างนี้กำหนดให้ T ต้องรองรับ Trait `Display`

## 6. Important Concepts

- Generic Function ใช้ Type Parameters ภายในเครื่องหมาย `< >`
- Function สามารถมี Type Parameters ได้มากกว่าหนึ่งตัว
- Return Type สามารถใช้ Generic Type ได้
- การเรียก Method ต้องมี Trait Bound ที่รองรับ
- Rust ตรวจสอบความถูกต้องของชนิดข้อมูลใน Compile Time

## Summary

Generic Functions ช่วยให้สามารถสร้าง Function ที่รองรับข้อมูลหลายชนิด โดยใช้ Type Parameters และ Trait Bounds ตามความจำเป็น

## Checklist

- [ ] ประกาศ Generic Function ได้
- [ ] ใช้ Generic Parameter ได้
- [ ] ใช้ Generic Return Type ได้
- [ ] ใช้ Type Parameters หลายตัวได้
- [ ] เข้าใจ Trait Bounds ใน Function
