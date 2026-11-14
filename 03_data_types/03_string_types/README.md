# Lesson 03: String Types

## Overview

เรียนรู้การจัดเก็บข้อความในภาษา Rust โดยเน้น String Slice (&str) และ String

## Learning Objectives

- เข้าใจ String Slice
- เข้าใจ String
- แยกความแตกต่างระหว่าง &str และ String ได้
- ประกาศข้อความได้
- สร้าง String ด้วย String::from() ได้

## 1. String Slice (&str)

&str ใช้แทนการอ้างอิงถึงข้อความ

ตัวอย่าง:

```rust
fn main() {
    let name: &str = "Alex";
    let country: &str = "Thailand";

    println!("Name: {name}");
    println!("Country: {country}");
}
```

## 2. String

String เป็นชนิดข้อมูลข้อความที่สามารถจัดการและเปลี่ยนแปลงเนื้อหาได้

สร้าง String ด้วย String::from()

```rust
fn main() {
    let message = String::from("Hello Rust");

    println!("{message}");
}
```

## 3. String vs &str

| Feature           | String                | &str                    |
| ----------------- | --------------------- | ----------------------- |
| ประเภท            | Owned String          | String Slice            |
| การจัดเก็บ        | เป็นเจ้าของข้อมูล     | อ้างอิงข้อความ          |
| เปลี่ยนเนื้อหาได้ | ได้ หากประกาศ mut     | ไม่สามารถแก้ไขผ่าน &str |
| ตัวอย่าง          | String::from("Hello") | "Hello"                 |

## 4. Mutable String

สามารถเพิ่มข้อความให้ String ที่ประกาศเป็น mut ได้

```rust
fn main() {
    let mut message = String::from("Hello");

    message.push_str(" Rust");

    println!("{message}");
}
```

Output:

```text
Hello Rust
```

## 5. Common Methods

ตัวอย่าง Method ที่ใช้กับ String:

```rust
fn main() {
    let mut message = String::from("Hello");

    message.push('!');
    message.push_str(" Rust");

    println!("{message}");
    println!("Length: {}", message.len());
}
```

หมายเหตุ: len() คืนค่าความยาวเป็นจำนวน Bytes ไม่ใช่จำนวนตัวอักษรที่ผู้ใช้มองเห็นเสมอไป

## Practice

สร้างโปรแกรมแนะนำตัว:

- name เป็น &str
- country เป็น &str
- message เป็น String

เพิ่มข้อความให้ message ด้วย push_str()

## Checklist

- [ ] เข้าใจ &str
- [ ] เข้าใจ String
- [ ] แยกความแตกต่างได้
- [ ] ใช้ String::from() ได้
- [ ] ใช้ push_str() ได้
- [ ] เข้าใจการใช้ mut กับ String
- [ ] ผ่าน cargo check

## References

- [The Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust Standard Library: String](https://doc.rust-lang.org/std/string/struct.String.html)

## Next Lesson

Lesson 04: Compound Types
