# Lesson 03: Generic Structs and Enums

## Overview

Rust รองรับการใช้ Generics กับ Structs และ Enums เพื่อให้โครงสร้างข้อมูลสามารถจัดเก็บข้อมูลได้หลายชนิด

เราสามารถกำหนด Type Parameters ให้ Fields หรือ Variants ได้

## Learning Objectives

- เข้าใจ Generic Structs
- สร้าง Struct ที่มี Generic Type Parameters ได้
- ใช้ Generic Parameters หลายตัวใน Struct ได้
- เข้าใจ Generic Enums
- เข้าใจการใช้ `Option<T>` และ `Result<T, E>`
- สร้าง Enum ที่รองรับข้อมูลหลายชนิดได้

## 1. Generic Structs

Generic Struct คือ Struct ที่ใช้ Type Parameters แทนชนิดข้อมูลของ Fields

รูปแบบ:

```rust
struct Data<T> {
    value: T,
}
```

คำอธิบาย:

- `Data` คือชื่อ Struct
- `<T>` คือ Generic Type Parameter
- `value: T` คือ Field ที่ใช้ชนิดข้อมูล T

เมื่อสร้าง Instance เราจะกำหนดชนิดข้อมูลจริงให้ T

## 2. Multiple Generic Parameters in Structs

Struct สามารถใช้ Type Parameters หลายตัวได้

ตัวอย่างแนวคิด:

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}
```

ในตัวอย่างนี้:

- `first` ใช้ชนิดข้อมูล T
- `second` ใช้ชนิดข้อมูล U

ทั้งสอง Fields สามารถใช้ชนิดข้อมูลต่างกันได้

## 3. Generic Enums

Enum สามารถใช้ Generic Type Parameters ได้เช่นเดียวกับ Struct

ตัวอย่าง:

```rust
enum Value<T> {
    Number(T),
    Empty,
}
```

Variant `Number` สามารถเก็บข้อมูลชนิด T ได้

## 4. The Option Enum

`Option<T>` เป็น Enum มาตรฐานของ Rust ที่ใช้แทนค่าที่อาจมีหรือไม่มี

แนวคิดของ Variants:

```rust
Some(T)
None
```

- `Some(value)` หมายถึงมีค่า
- `None` หมายถึงไม่มีค่า

ตัวอย่างการใช้งานที่ควรรู้:

```rust
let value: Option<i32> = Some(10);
```

## 5. The Result Enum

`Result<T, E>` เป็น Enum ที่ใช้แทนผลลัพธ์ที่อาจสำเร็จหรือเกิดข้อผิดพลาด

Variants:

```rust
Ok(T)
Err(E)
```

- `Ok(value)` หมายถึงการทำงานสำเร็จ
- `Err(error)` หมายถึงเกิดข้อผิดพลาด

ตัวอย่าง:

```rust
let result: Result<i32, String> = Ok(100);
```

## 6. Important Concepts

- Struct สามารถใช้ Generic Type Parameters ได้
- Enum สามารถใช้ Generic Type Parameters ได้
- Generic Struct สามารถมี Type Parameters หลายตัว
- `Option<T>` ใช้แทนค่าที่อาจไม่มี
- `Result<T, E>` ใช้แทนผลลัพธ์ที่อาจผิดพลาด
- Type Parameters ของ Struct และ Enum จะต้องสอดคล้องกับข้อมูลที่นำมาใช้งาน

## Summary

Generics สามารถใช้กับ Structs และ Enums เพื่อสร้างโครงสร้างข้อมูลที่ยืดหยุ่น และนำไปใช้กับข้อมูลหลายชนิดได้

## Checklist

- [ ] สร้าง Generic Struct ได้
- [ ] ใช้ Generic Fields ได้
- [ ] ใช้ Type Parameters หลายตัวได้
- [ ] สร้าง Generic Enum ได้
- [ ] เข้าใจ `Option<T>`
- [ ] เข้าใจ `Result<T, E>`
