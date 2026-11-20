# Lesson 02: Lifetime Annotations

## Overview

Lifetime Annotations คือการระบุชื่อ Lifetime เพื่อแสดงความสัมพันธ์ระหว่าง References

Rust สามารถอนุมาน Lifetimes ได้ในหลายกรณี แต่บางกรณีจำเป็นต้องระบุ Lifetime Annotation เพื่อให้ Compiler เข้าใจความสัมพันธ์ของ References ได้อย่างชัดเจน

## Learning Objectives

- เข้าใจ Lifetime Annotations
- รู้จักรูปแบบ `'a` และ `'b`
- ประกาศ Lifetime Parameters ได้
- ใช้ Lifetime Annotations กับ Function Parameters ได้
- กำหนด Lifetime ให้ Return References ได้
- เข้าใจความสัมพันธ์ระหว่าง Input และ Output Lifetimes
- เข้าใจ Lifetime Elision Rules

## 1. What are Lifetime Annotations?

Lifetime Annotation คือชื่อที่ใช้ระบุ Lifetime Parameters

รูปแบบ:

```rust
'a
'b
'c
```

เครื่องหมาย `'` ใช้นำหน้าชื่อ Lifetime

ตัวอย่าง:

```rust
&i32
&'a i32
```

- `&i32` คือ Reference ที่ไม่ได้เขียน Lifetime Annotation อย่างชัดเจน
- `&'a i32` คือ Reference ที่ระบุ Lifetime Parameter ชื่อ `'a`

ชื่อ Lifetime เป็นชื่อที่ผู้เขียนกำหนด ไม่ใช่ค่าระยะเวลาเป็นวินาทีหรือจำนวนบรรทัด

## 2. Lifetime Parameters

Lifetime Parameters ใช้ระบุความสัมพันธ์ของ References

ตัวอย่างรูปแบบ Function:

```rust
fn function_name<'a>(value: &'a str) {
    // Function body
}
```

คำอธิบาย:

- `<'a>` ประกาศ Lifetime Parameter
- `&'a str` ระบุว่า Reference มี Lifetime ที่สัมพันธ์กับ `'a`

## 3. Lifetime Annotations in Function Parameters

เมื่อ Function รับ References หลายตัว สามารถกำหนด Lifetime Annotation ให้แต่ละ Reference ได้

ตัวอย่างแนวคิด:

```rust
fn compare<'a, 'b>(first: &'a str, second: &'b str) {
    // Function body
}
```

ในตัวอย่างนี้:

- `first` ใช้ Lifetime `'a`
- `second` ใช้ Lifetime `'b`

ทั้งสอง References ไม่จำเป็นต้องมี Lifetime Parameter ชื่อเดียวกัน

## 4. Lifetime Annotations in Return References

หาก Function คืนค่า Reference ที่สัมพันธ์กับ Input References สามารถใช้ Lifetime Parameters ระบุความสัมพันธ์ได้

ตัวอย่าง:

```rust
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}
```

ความหมายของตัวอย่าง:

- Input References ทั้งสองใช้ Lifetime Parameter `'a`
- Return Reference ใช้ Lifetime Parameter `'a`
- Reference ที่คืนค่าต้องมีช่วงการใช้งานที่ปลอดภัยตามความสัมพันธ์ที่กำหนด

ข้อสำคัญ: `'a` ไม่ได้บังคับให้ References ทั้งสองมีอายุเท่ากัน แต่กำหนดให้ Return Reference ใช้งานได้ภายในช่วงที่ทั้งสอง Input References ใช้งานได้ร่วมกัน

## 5. Multiple Lifetime Parameters

Function สามารถมี Lifetime Parameters มากกว่าหนึ่งตัวได้

ตัวอย่าง:

```rust
fn select<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    first
}
```

ตัวอย่างนี้คืน Reference ที่สัมพันธ์กับ `'a` เท่านั้น และไม่ได้คืน Reference จาก `second`

## 6. Lifetime Elision Rules

Lifetime Elision Rules คือกฎที่ Rust ใช้ช่วยอนุมาน Lifetimes โดยไม่ต้องเขียน Annotation ทุกตำแหน่ง

กฎหลักที่ควรรู้:

1. แต่ละ Input Reference Parameter จะได้รับ Lifetime Parameter ของตัวเอง หากยังไม่ได้ระบุ
2. หากมี Input Lifetime เพียงตัวเดียว Lifetime นั้นจะถูกกำหนดให้ Output Reference
3. หากเป็น Method ที่มี `&self` หรือ `&mut self` Lifetime ของ `self` จะถูกกำหนดให้ Output References ที่ไม่ได้ระบุ Lifetime อย่างชัดเจน

ตัวอย่าง:

```rust
fn first_word(text: &str) -> &str {
    // Function body
    text
}
```

กรณีนี้ Rust สามารถอนุมาน Lifetime ได้จาก Input Reference เพียงตัวเดียว

## 7. Important Concepts

- Lifetime Annotation ใช้ระบุความสัมพันธ์ระหว่าง References
- Lifetime Parameter เริ่มต้นด้วยเครื่องหมาย `'`
- ชื่อ Lifetime สามารถกำหนดเองได้
- Lifetime Annotation ไม่ได้เปลี่ยนระยะเวลาจริงของข้อมูล
- ไม่จำเป็นต้องเขียน Annotation ทุกครั้ง
- Lifetime Elision Rules ช่วยลดการเขียน Annotation ที่ไม่จำเป็น

## Summary

Lifetime Annotations ช่วยระบุความสัมพันธ์ระหว่าง References โดยเฉพาะกรณีที่ Function รับและคืนค่า References หลายตัว

## Checklist

- [ ] เข้าใจ Lifetime Annotation
- [ ] รู้จัก `'a` และ `'b`
- [ ] ประกาศ Lifetime Parameters ได้
- [ ] ใช้ Lifetimes กับ Function Parameters ได้
- [ ] ใช้ Lifetimes กับ Return References ได้
- [ ] เข้าใจความสัมพันธ์ของ Input และ Output Lifetimes
- [ ] เข้าใจ Lifetime Elision Rules
