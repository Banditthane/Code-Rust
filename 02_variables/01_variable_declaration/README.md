# Lesson 01: Variable Declaration

## Overview

เรียนรู้วิธีประกาศตัวแปรในภาษา Rust ด้วย keyword `let`

ตัวแปรใช้สำหรับจัดเก็บข้อมูลที่โปรแกรมต้องการนำไปใช้งาน เช่น ชื่อ อายุ คะแนน และข้อมูลประเภทต่าง ๆ

## Learning Objectives

เมื่อเรียนจบบทนี้แล้ว สามารถ:

- ประกาศตัวแปรด้วย `let` ได้
- กำหนดค่าให้ตัวแปรได้
- เข้าใจ Immutable Variables
- เข้าใจ Type Inference
- ตั้งชื่อตัวแปรตามหลักการตั้งชื่อของ Rust ได้

## 1. Variable Declaration

การประกาศตัวแปรใน Rust ใช้ `let`

Syntax:

```rust
let variable_name = value;
```

Example:

```rust
fn main() {
    let name = "Alex";
    let age = 20;

    println!("Name: {name}");
    println!("Age: {age}");
}
```

Output:

```text
Name: Alex
Age: 20
```

## 2. Immutable Variables

ตัวแปรที่ประกาศด้วย `let` จะเป็น Immutable โดยค่าเริ่มต้น

หมายความว่าไม่สามารถเปลี่ยนแปลงค่าหลังจากกำหนดค่าแล้วได้

Example:

```rust
fn main() {
    let age = 20;

    println!("Age: {age}");
}
```

ตัวอย่างที่ไม่ถูกต้อง:

```rust
fn main() {
    let age = 20;

    age = 21;
}
```

โค้ดนี้จะเกิด Compile Error เพราะพยายามเปลี่ยนค่าของ Immutable Variable

## 3. Type Inference

Rust สามารถตรวจสอบและอนุมานชนิดข้อมูลจากค่าที่กำหนดให้ตัวแปรได้

Example:

```rust
fn main() {
    let age = 20;
    let price = 99.50;
    let name = "Alex";

    println!("{age}");
    println!("{price}");
    println!("{name}");
}
```

สามารถระบุชนิดข้อมูลเองได้เช่นกัน:

```rust
fn main() {
    let age: i32 = 20;
    let price: f64 = 99.50;
    let name: &str = "Alex";

    println!("{age}");
    println!("{price}");
    println!("{name}");
}
```

## 4. Variable Naming

แนวทางการตั้งชื่อตัวแปรใน Rust:

- ใช้ `snake_case`
- ชื่อควรสื่อความหมาย
- ไม่ควรใช้ชื่อที่ทำให้สับสน
- หลีกเลี่ยงชื่อที่ไม่เกี่ยวข้องกับข้อมูล

ตัวอย่าง:

```rust
let first_name = "Alex";
let user_age = 20;
let total_score = 100;
```

## Practice

ทดลองเขียนโปรแกรม:

```rust
fn main() {
    let first_name = "Alex";
    let last_name = "Smith";
    let age = 20;
    let country = "Thailand";

    println!("First Name: {first_name}");
    println!("Last Name: {last_name}");
    println!("Age: {age}");
    println!("Country: {country}");
}
```

## Checklist

- [ ] เข้าใจ `let`
- [ ] ประกาศตัวแปรได้
- [ ] เข้าใจ Immutable Variables
- [ ] เข้าใจ Type Inference
- [ ] ตั้งชื่อตัวแปรแบบ snake_case ได้
- [ ] ทดลองเขียนโปรแกรมด้วยตัวเอง

## References

- [W3Schools Rust Tutorial](https://www.w3schools.com/rust/)
- [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

## Next Lesson

Lesson 02: Mutable Variables
