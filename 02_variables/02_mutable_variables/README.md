# Lesson 02: Mutable Variables

## Overview

เรียนรู้เกี่ยวกับ Mutable Variables ในภาษา Rust

โดยปกติ Rust จะไม่อนุญาตให้เปลี่ยนแปลงค่าของตัวแปรที่ประกาศด้วย `let`

หากต้องการเปลี่ยนแปลงค่า ให้ใช้ keyword `mut` ร่วมกับ `let`

## Learning Objectives

เมื่อเรียนจบบทนี้แล้ว สามารถ:

- เข้าใจ Mutable Variables
- ใช้ `mut` ได้
- เปลี่ยนแปลงค่าของตัวแปรได้
- แยกความแตกต่างระหว่าง Mutable และ Immutable Variables ได้
- เข้าใจ Compile Error ที่เกิดจากการเปลี่ยนค่าตัวแปรผิดประเภท

## 1. What is mut?

`mut` ย่อมาจาก Mutable หมายถึงตัวแปรที่สามารถเปลี่ยนแปลงค่าได้

Syntax:

```rust
let mut variable_name = value;
```

Example:

```rust
fn main() {
    let mut age = 20;

    println!("Before: {age}");

    age = 21;

    println!("After: {age}");
}
```

Output:

```text
Before: 20
After: 21
```

## 2. Immutable vs Mutable

Immutable Variable:

```rust
let age = 20;
```

Mutable Variable:

```rust
let mut age = 20;
```

เปรียบเทียบ:

| Feature      | Immutable | Mutable   |
| ------------ | --------- | --------- |
| Declaration  | `let`     | `let mut` |
| Change Value | ไม่ได้    | ได้       |
| Default      | ใช่       | ไม่ใช่    |

## 3. Changing Variable Values

สามารถเปลี่ยนแปลงค่าตัวแปร Mutable ได้หลายครั้ง

Example:

```rust
fn main() {
    let mut score = 0;

    println!("Score: {score}");

    score = 10;
    println!("Score: {score}");

    score = 20;
    println!("Score: {score}");

    score = 30;
    println!("Score: {score}");
}
```

Output:

```text
Score: 0
Score: 10
Score: 20
Score: 30
```

## 4. Common Mistake

ตัวอย่างที่ไม่ถูกต้อง:

```rust
fn main() {
    let score = 100;

    score = 200;
}
```

สาเหตุ:

ตัวแปร `score` ไม่ได้ประกาศด้วย `mut` จึงไม่สามารถเปลี่ยนแปลงค่าได้

วิธีแก้ไข:

```rust
fn main() {
    let mut score = 100;

    score = 200;

    println!("Score: {score}");
}
```

## Practice

ทดลองเขียนโปรแกรมจัดการข้อมูลผู้ใช้

```rust
fn main() {
    let mut age = 20;
    let mut score = 50;

    println!("Initial Age: {age}");
    println!("Initial Score: {score}");

    age = 21;
    score = 100;

    println!("Updated Age: {age}");
    println!("Updated Score: {score}");
}
```

## Checklist

- [ ] เข้าใจ Mutable Variables
- [ ] ใช้ `mut` ได้
- [ ] เปลี่ยนแปลงค่าตัวแปรได้
- [ ] เข้าใจ Immutable Variables
- [ ] แยกความแตกต่างระหว่าง `let` และ `let mut` ได้
- [ ] ทดลองเขียนโปรแกรมด้วยตัวเอง

## References

- [W3Schools Rust Tutorial](https://www.w3schools.com/rust/)
- [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

## Next Lesson

Lesson 03: Constants
