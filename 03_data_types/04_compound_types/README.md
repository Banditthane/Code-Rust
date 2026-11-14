# Lesson 04: Compound Types

## Overview

เรียนรู้ Compound Types ซึ่งเป็นชนิดข้อมูลที่ใช้รวมข้อมูลหลายค่าไว้ด้วยกัน

ในบทนี้จะเรียน Tuple และ Array

## Learning Objectives

- เข้าใจ Tuple
- ประกาศ Tuple ได้
- เข้าถึงข้อมูลใน Tuple ได้
- เข้าใจ Array
- ประกาศ Array ได้
- เข้าถึงสมาชิกใน Array ด้วย Index ได้

## 1. Tuple

Tuple ใช้รวมข้อมูลหลายชนิดไว้ในกลุ่มเดียว

ตัวอย่าง:

```rust
fn main() {
    let person: (&str, i32, bool) = ("Alex", 20, true);

    println!("Person: {:?}", person);
}
```

Tuple สามารถเก็บข้อมูลต่างชนิดกันได้

## 2. Access Tuple Elements

ใช้ Index เพื่อเข้าถึงข้อมูลแต่ละตำแหน่งด้วย .0, .1, .2

```rust
fn main() {
    let person = ("Alex", 20, true);

    let name = person.0;
    let age = person.1;
    let is_student = person.2;

    println!("Name: {name}");
    println!("Age: {age}");
    println!("Student: {is_student}");
}
```

## 3. Array

Array ใช้จัดเก็บข้อมูลชนิดเดียวกันหลายค่า และมีขนาดคงที่

ตัวอย่าง:

```rust
fn main() {
    let scores: [i32; 3] = [80, 90, 100];

    println!("Scores: {:?}", scores);
}
```

รูปแบบการประกาศ:

```rust
let array_name: [data_type; size] = [values];
```

## 4. Access Array Elements

Array ใช้ Index เริ่มต้นจาก 0

```rust
fn main() {
    let scores = [80, 90, 100];

    println!("First: {}", scores[0]);
    println!("Second: {}", scores[1]);
    println!("Third: {}", scores[2]);
}
```

## 5. Tuple vs Array

| Feature       | Tuple                                | Array                 |
| ------------- | ------------------------------------ | --------------------- |
| ชนิดข้อมูล    | ต่างชนิดกันได้                       | ชนิดเดียวกัน          |
| จำนวนสมาชิก   | กำหนดตายตัว                          | กำหนดตายตัว           |
| เข้าถึงข้อมูล | .0, .1, .2                           | [0], [1], [2]         |
| เหมาะกับ      | ข้อมูลหลายประเภทที่เป็นกลุ่มเดียวกัน | ชุดข้อมูลชนิดเดียวกัน |

## Practice

ทดลองสร้าง:

- Tuple สำหรับข้อมูลนักเรียน
- Array สำหรับคะแนนสอบ 5 วิชา
- แสดงสมาชิกแต่ละตำแหน่ง

## Checklist

- [ ] เข้าใจ Tuple
- [ ] ประกาศ Tuple ได้
- [ ] เข้าถึง Tuple ด้วย Index ได้
- [ ] เข้าใจ Array
- [ ] ประกาศ Array ได้
- [ ] เข้าถึง Array ด้วย Index ได้
- [ ] แยก Tuple และ Array ได้
- [ ] ผ่าน cargo check

## References

- [The Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

## Next Chapter

Chapter 04: Operators
