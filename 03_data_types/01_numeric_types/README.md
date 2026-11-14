# Lesson 01: Numeric Types

## Overview

เรียนรู้ชนิดข้อมูลตัวเลขใน Rust ซึ่งแบ่งออกเป็น Integer และ Floating-Point

## Learning Objectives

- เข้าใจ Integer Types
- เข้าใจ Unsigned และ Signed Integers
- เข้าใจ Floating-Point Types
- เลือกชนิดข้อมูลตัวเลขได้เหมาะสม

## 1. Integer Types

Integer คือชนิดข้อมูลจำนวนเต็ม ไม่มีจุดทศนิยม

Rust มี Integer หลายขนาด:

| Type  |               ขนาด | ช่วงค่า            |
| ----- | -----------------: | ------------------ |
| i8    |              8-bit | -128 ถึง 127       |
| i16   |             16-bit | -32,768 ถึง 32,767 |
| i32   |             32-bit | Signed 32-bit      |
| i64   |             64-bit | Signed 64-bit      |
| i128  |            128-bit | Signed 128-bit     |
| u8    |              8-bit | 0 ถึง 255          |
| u16   |             16-bit | Unsigned 16-bit    |
| u32   |             32-bit | Unsigned 32-bit    |
| u64   |             64-bit | Unsigned 64-bit    |
| u128  |            128-bit | Unsigned 128-bit   |
| isize | Platform-dependent | Signed             |
| usize | Platform-dependent | Unsigned           |

ตัวอักษร i หมายถึง Signed Integer ซึ่งรองรับค่าลบ

ตัวอักษร u หมายถึง Unsigned Integer ซึ่งไม่รองรับค่าลบ

## 2. Integer Example

```rust
fn main() {
    let age: i32 = 20;
    let score: u32 = 100;
    let temperature: i32 = -5;

    println!("Age: {age}");
    println!("Score: {score}");
    println!("Temperature: {temperature}");
}
```

## 3. Floating-Point Types

ใช้จัดเก็บตัวเลขทศนิยม

Rust มี Floating-Point 2 ชนิด:

- f32: 32-bit
- f64: 64-bit

โดยทั่วไป Rust จะอนุมานเลขทศนิยมเป็น f64 หากไม่ได้ระบุชนิดข้อมูล

```rust
fn main() {
    let price: f64 = 99.99;
    let height: f32 = 175.5;

    println!("Price: {price}");
    println!("Height: {height}");
}
```

## 4. Numeric Type Inference

Rust สามารถอนุมานชนิดข้อมูลตัวเลขจากค่าได้

```rust
fn main() {
    let age = 20;
    let price = 99.99;

    println!("Age: {age}");
    println!("Price: {price}");
}
```

## 5. Common Mistakes

การกำหนดค่าเกินขอบเขตของชนิดข้อมูลจะทำให้เกิดข้อผิดพลาด

```rust
let age: u8 = 300;
```

เพราะ u8 รองรับค่าตั้งแต่ 0 ถึง 255 เท่านั้น

## Practice

ทดลองประกาศตัวแปร:

- อายุ
- คะแนน
- อุณหภูมิ
- ราคา
- ส่วนสูง

เลือกใช้ Integer และ Floating-Point ให้เหมาะสม

## Checklist

- [ ] เข้าใจ Integer
- [ ] เข้าใจ Signed และ Unsigned
- [ ] รู้จัก i8 ถึง i128
- [ ] รู้จัก u8 ถึง u128
- [ ] เข้าใจ f32 และ f64
- [ ] ทดลองประกาศตัวแปรตัวเลข
- [ ] ผ่าน cargo check

## References

- [W3Schools Rust Data Types](https://www.w3schools.com/rust/)
- [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

## Next Lesson

Lesson 02: Boolean and Char
