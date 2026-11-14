# Lesson 05: Bitwise Operators

## Overview

Bitwise Operators ใช้ดำเนินการกับข้อมูลในระดับบิต (Bit) โดยทำงานกับรูปแบบเลขฐานสอง

บทนี้เป็นการเรียนรู้พื้นฐาน เพื่อให้เข้าใจการทำงานของตัวดำเนินการระดับบิต

## Operators

| Operator | Name        | Description            |
| -------- | ----------- | ---------------------- | --------------------- |
| `&`      | Bitwise AND | เปรียบเทียบบิตด้วย AND |
| `        | `           | Bitwise OR             | เปรียบเทียบบิตด้วย OR |
| `^`      | Bitwise XOR | เปรียบเทียบบิตด้วย XOR |
| `!`      | Bitwise NOT | กลับค่าบิตทั้งหมด      |
| `<<`     | Left Shift  | เลื่อนบิตไปทางซ้าย     |
| `>>`     | Right Shift | เลื่อนบิตไปทางขวา      |

## Binary Basics

ตัวอย่างเลขฐานสอง:

```text
Decimal 5 = 0101
Decimal 3 = 0011
```

## Bitwise AND

```rust
fn main() {
    let a: u8 = 5;
    let b: u8 = 3;

    println!("{}", a & b);
}
```

ผลลัพธ์คือ `1`

## Bitwise OR

```rust
fn main() {
    let a: u8 = 5;
    let b: u8 = 3;

    println!("{}", a | b);
}
```

ผลลัพธ์คือ `7`

## Bitwise XOR

```rust
fn main() {
    let a: u8 = 5;
    let b: u8 = 3;

    println!("{}", a ^ b);
}
```

ผลลัพธ์คือ `6`

## Shift Operators

```rust
fn main() {
    let number: u8 = 5;

    println!("Left Shift: {}", number << 1);
    println!("Right Shift: {}", number >> 1);
}
```

## Important Notes

- Bitwise Operators ทำงานกับบิตของจำนวนเต็ม
- `&`, `|`, `^` เป็นการดำเนินการระดับบิต ไม่ใช่ Logical Operators
- `&&` และ `||` ใช้กับ Boolean Expressions
- `!` ใน Rust ใช้ได้ทั้ง Boolean NOT และ Bitwise NOT ตามชนิดข้อมูลที่ใช้

## Practice

1. กำหนดตัวแปรจำนวนเต็มสองตัว
2. ทดลองใช้ `&`, `|` และ `^`
3. ทดลองใช้ `<<` และ `>>`
4. เปรียบเทียบผลลัพธ์กับเลขฐานสอง

## Knowledge Check

1. Bitwise Operators ทำงานกับข้อมูลระดับใด?
2. `&` แตกต่างจาก `&&` อย่างไร?
3. `<<` ใช้ทำอะไร?
4. `^` คือการดำเนินการแบบใด?
