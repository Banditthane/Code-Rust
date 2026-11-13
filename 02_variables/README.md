# Chapter 02: Variables

## Overview

บทนี้เรียนรู้เกี่ยวกับ Variables (ตัวแปร) ในภาษา Rust ซึ่งเป็นพื้นฐานสำคัญสำหรับการจัดเก็บและจัดการข้อมูลภายในโปรแกรม

เนื้อหาครอบคลุมการประกาศตัวแปร การเปลี่ยนแปลงค่าของตัวแปร และการประกาศค่าคงที่

## Learning Objectives

เมื่อเรียนจบบทนี้แล้ว สามารถ:

- ประกาศตัวแปรด้วย `let` ได้
- เข้าใจ Immutable Variables
- เข้าใจ Mutable Variables
- ใช้ `mut` เพื่อเปลี่ยนแปลงค่าตัวแปรได้
- ประกาศ Constants ด้วย `const` ได้
- เข้าใจความแตกต่างระหว่าง `let`, `mut` และ `const`
- ใช้ตัวแปรร่วมกับ `println!()` ได้

## Lessons

| No. | Lesson               | Topics                               | Status |
| --- | -------------------- | ------------------------------------ | ------ |
| 01  | Variable Declaration | let, variable naming, type inference | ⬜     |
| 02  | Mutable Variables    | mut, changing values                 | ⬜     |
| 03  | Constants            | const, type annotation, naming       | ⬜     |

## Exercises

| No. | Exercise            | Description                 | Status |
| --- | ------------------- | --------------------------- | ------ |
| 01  | Variable Basics     | ประกาศและแสดงค่าตัวแปร      | ⬜     |
| 02  | Mutable Challenge   | ทดลองเปลี่ยนแปลงค่าตัวแปร   | ⬜     |
| 03  | Constants Challenge | ใช้ตัวแปรและค่าคงที่ร่วมกัน | ⬜     |

## Key Concepts

### 1. let

ใช้ประกาศตัวแปร โดยค่าเริ่มต้นตัวแปรจะไม่สามารถเปลี่ยนแปลงค่าได้

```rust
let name = "Alex";
```

### 2. mut

ใช้ร่วมกับ `let` เพื่ออนุญาตให้เปลี่ยนแปลงค่าตัวแปรได้

```rust
let mut age = 20;
age = 21;
```

### 3. const

ใช้ประกาศค่าคงที่ ซึ่งต้องระบุชนิดข้อมูลอย่างชัดเจน

```rust
const COUNTRY: &str = "Thailand";
```

## Practice Commands

คำสั่งที่ใช้ตรวจสอบและรันโปรแกรม

```powershell
cargo check
cargo run
cargo fmt
cargo clippy
```

## Completion Criteria

- [ ] เรียน Lesson 01–03 ครบ
- [ ] เข้าใจการประกาศตัวแปรด้วย `let`
- [ ] เข้าใจความแตกต่างระหว่าง Immutable และ Mutable
- [ ] ใช้ `mut` ได้ถูกต้อง
- [ ] ประกาศ Constants ด้วย `const` ได้
- [ ] ทำ Exercise 01–03 ครบ
- [ ] โปรแกรมทุก Exercise ทำงานได้
- [ ] ผ่าน `cargo check`
- [ ] ผ่าน `cargo clippy`

## References

- [W3Schools Rust Tutorial](https://www.w3schools.com/rust/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

## Next Chapter

เมื่อเรียนบทนี้จบแล้ว ให้ไปต่อที่:

**Chapter 03: Data Types**
