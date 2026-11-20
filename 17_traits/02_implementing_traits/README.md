# Lesson 02: Implementing Traits

## Overview

หลังจากประกาศ Trait แล้ว ขั้นตอนต่อไปคือการนำ Trait ไป Implement ให้กับชนิดข้อมูล เช่น Struct และ Enum

การ Implement ทำให้ชนิดข้อมูลมี Methods ตามที่ Trait กำหนด

## Learning Objectives

- Implement Trait ด้วย `impl Trait for Type`
- Implement Trait ให้ Struct
- Implement Trait ให้ Enum
- Implement หลาย Traits ให้ชนิดข้อมูลเดียวกัน
- Override Default Methods
- เข้าใจการใช้ `self`, `&self` และ `&mut self`

## 1. Implementing a Trait

รูปแบบการ Implement Trait:

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("Person: {}", self.name)
    }
}
```

ส่วนประกอบ:

- `trait Describe` ประกาศ Trait
- `struct Person` ประกาศชนิดข้อมูล
- `impl Describe for Person` Implement Trait ให้ Person
- `describe()` กำหนดการทำงานของ Method

## 2. Implementing Traits for Structs

Struct สามารถ Implement Trait ได้ตามความเหมาะสม

ตัวอย่างแนวคิด:

- Person Implement Trait สำหรับแสดงรายละเอียด
- Product Implement Trait สำหรับคำนวณราคา
- Circle Implement Trait สำหรับคำนวณพื้นที่

แต่ละ Struct สามารถมี Implementation ที่แตกต่างกันได้ แม้จะใช้ Trait เดียวกัน

## 3. Implementing Traits for Enums

Enum ก็สามารถ Implement Trait ได้เช่นกัน

ตัวอย่าง:

```rust
enum Status {
    Active,
    Inactive,
}
```

สามารถสร้าง Trait สำหรับแสดงรายละเอียดสถานะ และ Implement ให้ Enum นี้ได้

## 4. Implementing Multiple Traits

ชนิดข้อมูลหนึ่งสามารถ Implement หลาย Traits ได้

ตัวอย่างแนวคิด:

- Trait `Describe` สำหรับแสดงรายละเอียด
- Trait `Calculate` สำหรับคำนวณ
- Trait `Validate` สำหรับตรวจสอบข้อมูล

แต่ละ Trait จะกำหนดพฤติกรรมคนละด้าน

## 5. Overriding Default Methods

หาก Trait มี Default Method ชนิดข้อมูลสามารถเขียน Implementation ของ Method นั้นใหม่ได้

ตัวอย่างแนวคิด:

- Trait กำหนด Default Method สำหรับแสดงข้อความ
- Struct หนึ่งใช้ Default Method
- Struct อีกชนิดหนึ่ง Override Method เพื่อแสดงข้อความเฉพาะของตัวเอง

## 6. self, &self and &mut self

Trait Methods สามารถรับ Self ได้หลายรูปแบบ

| Form        | Description                |
| ----------- | -------------------------- |
| `&self`     | ยืม Instance แบบ Immutable |
| `&mut self` | ยืม Instance แบบ Mutable   |
| `self`      | รับ Ownership ของ Instance |

ตัวอย่างแนวคิด:

- ใช้ `&self` สำหรับอ่านข้อมูล
- ใช้ `&mut self` สำหรับแก้ไขข้อมูล
- ใช้ `self` เมื่อ Method ต้องรับ Ownership

## 7. Trait and Inherent Methods

Rust แยกการ Implement Methods ออกเป็นสองลักษณะ

### Inherent Implementation

```rust
impl Person {
    fn new(name: String) -> Self {
        Self { name }
    }
}
```

เป็น Method ที่กำหนดโดยตรงให้ Struct

### Trait Implementation

```rust
impl Describe for Person {
    // Implement trait methods
}
```

เป็น Method ที่มาจาก Trait

## 8. Important Rules

- ชื่อ Method และ Parameters ต้องตรงตาม Trait Definition
- Return Type ต้องตรงตามที่กำหนด
- Method ที่ไม่มี Default Implementation ต้อง Implement
- Method ที่มี Default Implementation สามารถ Override ได้
- การ Implement Trait ให้ Type และ Trait จากภายนอก Crate มีข้อจำกัดตาม Orphan Rule
- สามารถ Implement หลาย Traits ให้ Type เดียวกันได้

## Practice Checklist

- [ ] Implement Trait ให้ Struct ได้
- [ ] Implement Trait ให้ Enum ได้
- [ ] Implement หลาย Traits ให้ Type เดียวกันได้
- [ ] Override Default Method ได้
- [ ] เข้าใจ `self`
- [ ] เข้าใจ `&self`
- [ ] เข้าใจ `&mut self`
- [ ] แยก Inherent Methods และ Trait Methods ได้

## Next Lesson

Lesson 03: Derived and Standard Traits
