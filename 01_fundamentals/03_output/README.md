# Lesson 03: Rust Output

**Chapter:** 01 - Rust Fundamentals  
**Level:** L1 - Beginner  
**Status:** Learning

## 1. Overview

Rust Output คือการแสดงข้อมูลหรือข้อความออกทางหน้าจอ

Rust มี Macro ที่ใช้แสดงผล เช่น `println!()` และ `print!()`

## 2. Learning Objectives

- ใช้ `println!()` ได้
- ใช้ `print!()` ได้
- เข้าใจความแตกต่างของคำสั่งทั้งสอง
- ใช้ `\n` เพื่อขึ้นบรรทัดใหม่ได้

## 3. println!()

ใช้แสดงข้อความและขึ้นบรรทัดใหม่โดยอัตโนมัติ

ตัวอย่าง:

```rust
fn main() {
    println!("Hello World!");
    println!("Welcome to Rust!");
}
```

ผลลัพธ์:

```text
Hello World!
Welcome to Rust!
```

## 4. print!()

ใช้แสดงข้อความโดยไม่ขึ้นบรรทัดใหม่โดยอัตโนมัติ

ตัวอย่าง:

```rust
fn main() {
    print!("Hello ");
    print!("Rust!");
}
```

ผลลัพธ์:

```text
Hello Rust!
```

## 5. New Line

ใช้ `\n` เพื่อขึ้นบรรทัดใหม่ภายในข้อความ

ตัวอย่าง:

```rust
fn main() {
    print!("Hello\nRust");
}
```

ผลลัพธ์:

```text
Hello
Rust
```

## 6. Comparison

| Macro        | Description                     |
| ------------ | ------------------------------- |
| `println!()` | แสดงข้อความและขึ้นบรรทัดใหม่    |
| `print!()`   | แสดงข้อความโดยไม่ขึ้นบรรทัดใหม่ |

## 7. Practice

ทดลองเขียนโปรแกรม:

```rust
fn main() {
    println!("================");
    println!("RUST PROGRAMMING");
    println!("================");

    print!("Learn ");
    print!("Practice ");
    println!("Build");

    println!("Welcome to Rust!");
}
```

## 8. Completion Checklist

- [ ] ใช้ `println!()` ได้
- [ ] ใช้ `print!()` ได้
- [ ] อธิบายความแตกต่างได้
- [ ] ใช้ `\n` ได้
- [ ] ทดลองรันโปรแกรมได้

## 9. References

- https://www.w3schools.com/rust/rust_output.php

## 10. Next Lesson

[Lesson 04: Rust Comments](../04_comments/README.md)
