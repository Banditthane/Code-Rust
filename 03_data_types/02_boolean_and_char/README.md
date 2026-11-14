# Lesson 02: Boolean and Char

## Overview

เรียนรู้ชนิดข้อมูล Boolean และ Character ในภาษา Rust

## Learning Objectives

- เข้าใจ Boolean Type
- ใช้ true และ false ได้
- เข้าใจ Character Type
- ประกาศตัวแปร bool และ char ได้

## 1. Boolean Type

Boolean ใช้เก็บค่าทางตรรกะ มีเพียง 2 ค่า:

- true
- false

ตัวอย่าง:

```rust
fn main() {
    let is_active: bool = true;
    let is_logged_in: bool = false;

    println!("Active: {is_active}");
    println!("Logged in: {is_logged_in}");
}
```

## 2. Boolean Usage

Boolean เหมาะกับข้อมูลที่มีสถานะจริงหรือเท็จ เช่น:

- เปิดหรือปิด
- ผ่านหรือไม่ผ่าน
- เข้าสู่ระบบหรือยัง
- ใช้งานได้หรือไม่

```rust
fn main() {
    let is_passed: bool = true;

    println!("Passed: {is_passed}");
}
```

## 3. Character Type

Rust ใช้ char สำหรับเก็บอักขระ Unicode หนึ่งตัว

```rust
fn main() {
    let grade: char = 'A';
    let symbol: char = '#';
    let thai_char: char = 'ก';

    println!("Grade: {grade}");
    println!("Symbol: {symbol}");
    println!("Thai: {thai_char}");
}
```

## 4. Single Quotes and Double Quotes

ข้อแตกต่างที่ต้องจำ:

```rust
let letter: char = 'A';
let name: &str = "Alex";
```

- char ใช้ Single Quotes
- String Slice ใช้ Double Quotes

ตัวอย่างที่ไม่ถูกต้อง:

```rust
let letter: char = "A";
```

เพราะ "A" เป็น String Slice ไม่ใช่ char

## Practice

สร้างตัวแปร:

- is_student
- is_active
- grade
- favorite_symbol

จากนั้นแสดงค่าทั้งหมดด้วย println!

## Checklist

- [ ] เข้าใจ bool
- [ ] ใช้ true และ false ได้
- [ ] เข้าใจ char
- [ ] ใช้ Single Quotes ได้ถูกต้อง
- [ ] แยก char กับ &str ได้
- [ ] ทดลองเขียนโปรแกรม
- [ ] ผ่าน cargo check

## References

- [W3Schools Rust Tutorial](https://www.w3schools.com/rust/)
- [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

## Next Lesson

Lesson 03: String Types
