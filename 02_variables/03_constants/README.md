# Lesson 03: Constants

## Overview

เรียนรู้เกี่ยวกับ Constants หรือค่าคงที่ในภาษา Rust

Constants ใช้สำหรับกำหนดค่าที่ต้องการให้คงเดิมตลอดการทำงาน โดยประกาศด้วย keyword `const`

## Learning Objectives

เมื่อเรียนจบบทนี้แล้ว สามารถ:

- เข้าใจ Constants
- ประกาศค่าคงที่ด้วย `const` ได้
- ระบุชนิดข้อมูลให้ Constants ได้
- ตั้งชื่อ Constants ตาม Convention ของ Rust ได้
- แยกความแตกต่างระหว่าง Variables และ Constants ได้

## 1. What is const?

`const` ใช้ประกาศค่าคงที่ ซึ่งไม่สามารถเปลี่ยนแปลงค่าได้

Syntax:

```rust
const CONSTANT_NAME: data_type = value;
```

Example:

```rust
fn main() {
    const COUNTRY: &str = "Thailand";

    println!("Country: {COUNTRY}");
}
```

Output:

```text
Country: Thailand
```

## 2. Constant Type Annotation

การประกาศ Constants จำเป็นต้องระบุชนิดข้อมูลอย่างชัดเจน

ตัวอย่าง:

```rust
const MAX_SCORE: i32 = 100;
const PI: f64 = 3.14159;
const COUNTRY: &str = "Thailand";
```

## 3. Constant Naming Convention

Rust นิยมตั้งชื่อ Constants ด้วยตัวพิมพ์ใหญ่ทั้งหมด และใช้ `_` คั่นคำ

ตัวอย่าง:

```rust
const MAX_USERS: u32 = 100;
const MIN_AGE: u8 = 18;
const TAX_RATE: f64 = 0.07;
```

## 4. Variables vs Constants

| Feature         | Variable             | Constant                                 |
| --------------- | -------------------- | ---------------------------------------- |
| Declaration     | `let`                | `const`                                  |
| Mutable         | ใช้ `mut` ได้        | ไม่ได้                                   |
| Type Annotation | ไม่จำเป็นเสมอไป      | จำเป็น                                   |
| Naming          | snake_case           | SCREAMING_SNAKE_CASE                     |
| Value           | ขึ้นอยู่กับการประกาศ | ต้องเป็นค่าคงที่ที่ประเมินได้ขณะ Compile |

ตัวอย่าง:

```rust
fn main() {
    let name = "Alex";
    let mut age = 20;

    const COUNTRY: &str = "Thailand";
    const MAX_SCORE: u32 = 100;

    age = 21;

    println!("Name: {name}");
    println!("Age: {age}");
    println!("Country: {COUNTRY}");
    println!("Max Score: {MAX_SCORE}");
}
```

## 5. Common Mistakes

ตัวอย่างที่ไม่ถูกต้อง:

```rust
const COUNTRY = "Thailand";
```

สาเหตุ:

ไม่ได้ระบุชนิดข้อมูลให้ Constant

วิธีแก้ไข:

```rust
const COUNTRY: &str = "Thailand";
```

อีกตัวอย่างหนึ่ง:

```rust
const MAX_SCORE: i32 = 100;

MAX_SCORE = 200;
```

สาเหตุ:

Constants ไม่สามารถเปลี่ยนแปลงค่าได้

## Practice

ทดลองประกาศข้อมูลต่อไปนี้:

```rust
fn main() {
    const APP_NAME: &str = "Rust Learning";
    const MAX_LEVEL: u32 = 10;
    const COUNTRY: &str = "Thailand";

    println!("Application: {APP_NAME}");
    println!("Maximum Level: {MAX_LEVEL}");
    println!("Country: {COUNTRY}");
}
```

## Checklist

- [ ] เข้าใจ Constants
- [ ] ประกาศด้วย `const` ได้
- [ ] ระบุชนิดข้อมูลได้
- [ ] ตั้งชื่อ Constants ได้ถูกต้อง
- [ ] เข้าใจความแตกต่างระหว่าง `let` และ `const`
- [ ] ทดลองเขียนโปรแกรมด้วยตัวเอง

## References

- [W3Schools Rust Tutorial](https://www.w3schools.com/rust/)
- [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust Reference: Constant Items](https://doc.rust-lang.org/reference/items/constant-items.html)

## Next Chapter

Chapter 03: Data Types
