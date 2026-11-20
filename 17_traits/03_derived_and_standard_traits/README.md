# Lesson 03: Derived and Standard Traits

## Overview

Rust มี Traits มาตรฐานที่ช่วยให้ชนิดข้อมูลสามารถทำงานต่าง ๆ ได้ เช่น แสดงรายละเอียด เปรียบเทียบข้อมูล คัดลอกข้อมูล และกำหนดค่าเริ่มต้น

นอกจากนี้ Rust ยังมี Attribute `derive` ที่ช่วยสร้าง Implementation ให้ Traits บางชนิดโดยอัตโนมัติ

## Learning Objectives

- เข้าใจ Derived Traits
- ใช้ `#[derive(Debug)]`
- ใช้ `Clone` และ `Copy`
- ใช้ `PartialEq` และ `Eq`
- ใช้ `PartialOrd` และ `Ord`
- Implement `Display`
- ใช้ `Default`
- เข้าใจ `From` และ `Into`

## 1. What Is derive?

`derive` เป็น Attribute ที่สั่งให้ Rust สร้าง Trait Implementation ให้ชนิดข้อมูลโดยอัตโนมัติ สำหรับ Traits ที่รองรับการ Derive

รูปแบบ:

```rust
#[derive(Debug)]
struct Person {
    name: String,
}
```

## 2. Debug Trait

`Debug` ใช้แสดงรายละเอียดของข้อมูลเพื่อช่วยตรวจสอบระหว่างพัฒนาโปรแกรม

ตัวอย่าง:

```rust
#[derive(Debug)]
struct Person {
    name: String,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };

    println!("{:?}", person);
    println!("{:#?}", person);
}
```

- `{:?}` แสดง Debug Format
- `{:#?}` แสดง Debug Format แบบอ่านง่ายหลายบรรทัด

## 3. Clone and Copy Traits

### Clone

`Clone` ใช้สร้างสำเนาของข้อมูลอย่างชัดเจนด้วย `.clone()`

```rust
let first = String::from("Rust");

let second = first.clone();
```

### Copy

`Copy` ใช้กับชนิดข้อมูลที่สามารถคัดลอกค่าโดยอัตโนมัติเมื่อมีการกำหนดค่าให้ตัวแปรใหม่หรือส่งค่าเข้า Function

ตัวอย่างชนิดข้อมูลที่ Copy ได้:

- Integer
- Floating-point
- Boolean
- Character
- Tuple ที่สมาชิกทุกตัวเป็น Copy

ไม่สามารถ Derive `Copy` ให้ Struct ที่มี `String` เป็น Field ได้ เพราะ String ไม่ใช่ Copy

## 4. PartialEq and Eq

ใช้สำหรับเปรียบเทียบความเท่ากัน

- `PartialEq` รองรับการเปรียบเทียบความเท่ากันและไม่เท่ากัน
- `Eq` เป็น Trait ที่รับประกันความสัมพันธ์แบบเท่ากันตามข้อกำหนดของ Trait

ตัวอย่าง:

```rust
#[derive(PartialEq, Eq)]
struct UserId(u32);
```

สามารถใช้ `==` และ `!=` กับชนิดข้อมูลนี้ได้

## 5. PartialOrd and Ord

ใช้สำหรับเปรียบเทียบลำดับ

- `PartialOrd` ใช้เปรียบเทียบลำดับแบบที่อาจไม่มีคำตอบสำหรับบางคู่
- `Ord` ใช้กับชนิดข้อมูลที่สามารถจัดลำดับได้อย่างสมบูรณ์

ตัวอย่าง:

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Score(u32);
```

สามารถนำไปใช้กับการเปรียบเทียบและเรียงลำดับข้อมูลได้

## 6. Display Trait

`Display` ใช้กำหนดรูปแบบการแสดงผลสำหรับผู้ใช้

แตกต่างจาก `Debug` ที่เน้นการแสดงรายละเอียดเพื่อการตรวจสอบ

ตัวอย่าง:

```rust
use std::fmt;

struct Person {
    name: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person: {}", self.name)
    }
}
```

เมื่อ Implement `Display` แล้ว สามารถใช้ `{}` ใน `println!()` ได้

## 7. Default Trait

`Default` ใช้กำหนดค่าเริ่มต้นของชนิดข้อมูล

ตัวอย่าง:

```rust
#[derive(Default)]
struct Settings {
    volume: u32,
    enabled: bool,
}
```

สามารถสร้างค่าด้วย:

```rust
let settings = Settings::default();
```

ค่าเริ่มต้นของแต่ละ Field จะขึ้นอยู่กับ Default Implementation ของชนิดข้อมูลนั้น

## 8. From and Into Traits

ใช้สำหรับแปลงค่าจากชนิดข้อมูลหนึ่งไปเป็นอีกชนิดข้อมูลหนึ่ง

### From

`From<T>` ใช้กำหนดวิธีสร้างชนิดข้อมูลจากอีกชนิดหนึ่ง

```rust
let number = i32::from(10_u8);
```

### Into

`Into<T>` ใช้แปลงค่าปัจจุบันไปเป็นชนิดข้อมูลเป้าหมาย

```rust
let number: i32 = 10_u8.into();
```

โดยทั่วไป เมื่อ Implement `From<T>` แล้ว Rust จะมี `Into` ที่สอดคล้องกันให้ด้วย

## 9. Derived vs Manual Implementation

| Method                | Description                                              |
| --------------------- | -------------------------------------------------------- |
| Derive                | ให้ Compiler สร้าง Implementation สำหรับ Trait ที่รองรับ |
| Manual Implementation | เขียนการทำงานของ Trait ด้วยตนเอง                         |

ตัวอย่าง:

- `Debug` สามารถ Derive ได้
- `Clone` สามารถ Derive ได้เมื่อสมาชิกทุกตัวรองรับ
- `PartialEq` สามารถ Derive ได้เมื่อสมาชิกทุกตัวรองรับ
- `Display` ต้อง Implement เอง
- `Default` สามารถ Derive ได้เมื่อสมาชิกทุกตัวรองรับ

## Important Notes

- `Debug` และ `Display` มีว
