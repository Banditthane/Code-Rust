# Lesson 02: Rust Syntax

**Chapter:** 01 - Rust Fundamentals  
**Level:** L1 - Beginner  
**Status:** Learning

## 1. Overview

Rust Syntax คือกฎและรูปแบบการเขียนคำสั่งในภาษา Rust

บทนี้จะเรียนรู้โครงสร้างพื้นฐานของโปรแกรม Rust และส่วนประกอบสำคัญของคำสั่ง

## 2. Learning Objectives

- เข้าใจโครงสร้างโปรแกรม Rust
- เข้าใจการประกาศ Function
- เข้าใจ `fn main()`
- เข้าใจการใช้ `{ }`
- เข้าใจการใช้ Semicolon `;`

## 3. Basic Program Structure

ตัวอย่าง:

```rust
fn main() {
    println!("Hello World!");
}
```

โครงสร้างประกอบด้วย:

| Syntax       | Description             |
| ------------ | ----------------------- |
| `fn`         | ใช้ประกาศ Function      |
| `main`       | ชื่อ Function หลัก      |
| `{ }`        | กำหนดขอบเขตของ Function |
| `println!()` | Macro สำหรับแสดงข้อความ |
| `;`          | ปิด Statement           |

## 4. Main Function

`main()` เป็น Function หลักที่ใช้เป็นจุดเริ่มต้นของโปรแกรม Rust แบบ Binary

ตัวอย่าง:

```rust
fn main() {
    println!("My First Program");
}
```

## 5. Curly Braces

เครื่องหมาย `{ }` ใช้กำหนดขอบเขตของ Code Block

```rust
fn main() {
    println!("Inside main function");
}
```

## 6. Semicolon

เครื่องหมาย `;` ใช้ปิด Statement

```rust
fn main() {
    println!("Hello");
    println!("Rust");
}
```

## 7. Practice

ทดลองเขียนโปรแกรม:

```rust
fn main() {
    println!("My Name");
    println!("Learning Rust");
    println!("Using VS Code");
}
```

## 8. Completion Checklist

- [ ] เข้าใจ `fn`
- [ ] เข้าใจ `main()`
- [ ] เข้าใจ `{ }`
- [ ] เข้าใจ `;`
- [ ] เขียนโปรแกรมพื้นฐานได้

## 9. References

- https://www.w3schools.com/rust/rust_syntax.php

## 10. Next Lesson

[Lesson 03: Rust Output](../03_output/README.md)
