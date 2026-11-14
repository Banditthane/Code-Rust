# Chapter 03: Data Types

## Overview

บทนี้เรียนรู้เกี่ยวกับ Data Types หรือชนิดข้อมูลในภาษา Rust

Data Types ใช้กำหนดประเภทของข้อมูลที่ตัวแปรสามารถจัดเก็บได้ เช่น ตัวเลข จำนวนทศนิยม ข้อความ และค่าทางตรรกะ

## Learning Objectives

เมื่อเรียนจบบทนี้แล้ว สามารถ:

- เข้าใจ Data Types ใน Rust
- ใช้ Integer Types ได้
- ใช้ Floating-Point Types ได้
- ใช้ Boolean ได้
- ใช้ Character ได้
- เข้าใจ String และ String Slice
- ใช้ Tuple และ Array ได้
- เลือกชนิดข้อมูลให้เหมาะสมกับการใช้งาน

## Lessons

| No. | Lesson           | Topics         | Status |
| --- | ---------------- | -------------- | ------ |
| 01  | Numeric Types    | Integer, Float | ⬜     |
| 02  | Boolean and Char | bool, char     | ⬜     |
| 03  | String Types     | String, &str   | ⬜     |
| 04  | Compound Types   | Tuple, Array   | ⬜     |

## Exercises

| No. | Exercise         | Description                 | Status |
| --- | ---------------- | --------------------------- | ------ |
| 01  | Numeric Types    | ฝึกใช้ชนิดข้อมูลตัวเลข      | ⬜     |
| 02  | User Information | จัดเก็บข้อมูลผู้ใช้หลายชนิด | ⬜     |
| 03  | Data Collection  | ฝึกใช้ Tuple และ Array      | ⬜     |

## Key Concepts

### 1. Integer

ใช้เก็บจำนวนเต็ม เช่น 10, 100 และ -20

```rust
let age: i32 = 20;
let score: u32 = 100;
```

### 2. Floating-Point

ใช้เก็บตัวเลขทศนิยม

```rust
let price: f64 = 99.99;
let height: f32 = 175.5;
```

### 3. Boolean

ใช้เก็บค่า true หรือ false

```rust
let is_active: bool = true;
```

### 4. Character

ใช้เก็บอักขระ Unicode หนึ่งตัว

```rust
let grade: char = 'A';
```

### 5. String

ใช้จัดเก็บข้อความ

```rust
let name: &str = "Alex";
let message: String = String::from("Hello Rust");
```

### 6. Tuple

ใช้รวมข้อมูลหลายชนิดไว้ในกลุ่มเดียว

```rust
let person: (&str, i32, bool) = ("Alex", 20, true);
```

### 7. Array

ใช้จัดเก็บข้อมูลชนิดเดียวกันหลายค่า โดยมีขนาดคงที่

```rust
let scores: [i32; 3] = [80, 90, 100];
```

## Practice Commands

```powershell
cargo check
cargo run
cargo fmt
cargo clippy
```

## Completion Criteria

- [ ] เรียน Lesson 01–04 ครบ
- [ ] เข้าใจ Integer และ Float
- [ ] เข้าใจ Boolean และ Char
- [ ] เข้าใจ String และ &str
- [ ] ใช้ Tuple ได้
- [ ] ใช้ Array ได้
- [ ] ทำ Exercise 01–03 ครบ
- [ ] โปรแกรมทุก Exercise ทำงานได้
- [ ] ผ่าน cargo check
- [ ] ผ่าน cargo clippy

## References

- [W3Schools Rust Tutorial](https://www.w3schools.com/rust/)
- [Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

## Next Chapter

Chapter 04: Operators
