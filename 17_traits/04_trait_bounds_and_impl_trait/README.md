# Lesson 04: Trait Bounds and Impl Trait

## Overview

Trait Bounds ใช้กำหนดข้อกำหนดให้ Generic Types ว่าต้อง Implement Trait ใดบ้าง

ส่วน `impl Trait` เป็นรูปแบบที่ช่วยเขียน Function Parameters และ Return Types โดยระบุความสามารถของชนิดข้อมูลผ่าน Trait

บทนี้เป็นพื้นฐานสำหรับการเรียน Generics ใน Chapter 18

## Learning Objectives

- เข้าใจ Trait Bounds
- กำหนด Trait Bounds ให้ Function
- ใช้ `impl Trait` ใน Parameters
- ใช้ `impl Trait` ใน Return Types
- กำหนด Multiple Trait Bounds
- ใช้ `where` Clause
- เข้าใจความสัมพันธ์ระหว่าง Traits และ Generics

## 1. What Is a Trait Bound?

Trait Bound คือข้อกำหนดว่าชนิดข้อมูลที่นำมาใช้ต้อง Implement Trait ที่ระบุ

ตัวอย่างแนวคิด:

หาก Function ต้องการแสดงข้อมูลด้วย `Debug` ชนิดข้อมูลที่ส่งเข้ามาจะต้อง Implement `Debug`

## 2. Trait Bounds in Functions

สามารถกำหนด Trait Bound ด้วย Syntax `T: Trait`

```rust
fn display_value<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}
```

ในตัวอย่างนี้:

- `T` เป็น Generic Type
- `T: Debug` กำหนดให้ T ต้อง Implement Debug
- Function สามารถรับข้อมูลได้หลายชนิด ตราบใดที่ชนิดข้อมูลนั้นรองรับ Debug

## 3. impl Trait in Parameters

สามารถใช้ `impl Trait` เพื่อระบุความสามารถของ Parameter ได้

```rust
fn display_value(value: impl std::fmt::Debug) {
    println!("{:?}", value);
}
```

รูปแบบนี้เหมาะกับ Function ที่ต้องการรับข้อมูลชนิดใดก็ได้ที่ Implement Trait ที่กำหนด

## 4. Multiple Trait Bounds

บางครั้งต้องการให้ชนิดข้อมูล Implement หลาย Traits พร้อมกัน

สามารถใช้เครื่องหมาย `+`

```rust
fn display_and_compare<T: std::fmt::Debug + PartialEq>(value: T) {
    println!("{:?}", value);
}
```

ในตัวอย่างนี้ T ต้อง Implement ทั้ง Debug และ PartialEq

สามารถใช้รูปแบบ `impl Trait + Trait` กับ Parameters ได้เช่นกัน

## 5. Where Clause

เมื่อ Trait Bounds มีความยาวมาก สามารถใช้ `where` Clause เพื่อให้อ่านง่ายขึ้น

```rust
fn compare_values<T>(first: T, second: T)
where
    T: std::fmt::Debug + PartialEq,
{
    println!("{:?}", first);
    println!("{:?}", second);

    println!("{}", first == second);
}
```

`where` ใช้แยกข้อกำหนดของ Generic Types ออกจากส่วนประกาศ Function

## 6. impl Trait in Return Types

สามารถใช้ `impl Trait` เพื่อระบุว่า Function คืนค่าชนิดข้อมูลที่ Implement Trait ที่กำหนด

```rust
fn create_message() -> impl std::fmt::Display {
    String::from("Hello Rust")
}
```

ผู้เรียก Function สามารถใช้ค่าที่คืนมาในฐานะชนิดข้อมูลที่รองรับ Display ได้

ข้อควรจำ: Return Position `impl Trait` แบบปกติจะต้องคืนค่าชนิดข้อมูล Concrete Type เดียวกันจากทุกเส้นทางที่คืนค่า

## 7. Trait Bounds with References

Trait Bounds สามารถใช้กับ Reference ได้เช่นกัน

ตัวอย่างแนวคิด:

- Function รับ `&T`
- กำหนดให้ T Implement Trait ที่ต้องการ
- Function เรียก Trait Method ผ่าน Reference ได้

การใช้ Reference ช่วยให้ Function อ่านข้อมูลโดยไม่จำเป็นต้องรับ Ownership ของข้อมูล

## 8. Comparing Common Forms

| Syntax             | Description                                    |
| ------------------ | ---------------------------------------------- |
| `T: Debug`         | กำหนด Trait Bound ให้ Generic Type             |
| `impl Debug`       | ระบุ Parameter ที่ Implement Debug             |
| `T: Debug + Clone` | กำหนดหลาย Traits ให้ T                         |
| `where T: Debug`   | แยก Trait Bounds ไปไว้ใน where Clause          |
| `-> impl Display`  | คืนค่าชนิด Concrete Type ที่ Implement Display |

## 9. Important Notes

- Trait Bounds ไม่ได้สร้างชนิดข้อมูลใหม่
- Generic Type ที่กำหนด Trait Bound สามารถเรียก Methods จาก Trait นั้นได้
- `impl Trait` ใน Parameters เป็นรูปแบบย่อของ Generic Parameter ที่มี Trait Bound
- `impl Trait` ใน Return Position ใช้ซ่อนชื่อ Concrete Type ที่ Function คืนค่า
- `where` Clause ช่วยให้ Generic Signatures อ่านง่ายขึ้น
- Return Position `impl Trait` แบบปกติไม่สามารถคืน Concrete Types หลายชนิดที่แตกต่างกันตามเงื่อนไขได้
- Trait Bounds เป็นพื้นฐานสำคัญสำหรับ Chapter 18: Generics

## Practice Checklist

- [ ] เข้าใจ Trait Bounds
- [ ] ใช้ `T: Trait` ได้
- [ ] ใช้ Trait Bounds กับ Function ได้
- [ ] ใช้ `impl Trait` ใน Parameters ได้
- [ ] ใช้ `impl Trait` ใน Return Types ได้
- [ ] ใช้ Multiple Trait Bounds ได้
- [ ] ใช้เครื่องหมาย `+` ได้
- [ ] ใช้ `where` Clause ได้
- [ ] เข้าใจการใช้ Trait Bounds กับ References
- [ ] เข้าใจความสัมพันธ์ระหว่าง Traits และ Generics

## Next Chapter

Chapter 18: Generics
