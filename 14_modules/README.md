# Chapter 14: Modules

## Overview

**Modules** เป็นระบบจัดระเบียบโค้ดในภาษา Rust ช่วยแบ่งโปรแกรมออกเป็นส่วนย่อย ๆ เพื่อจัดกลุ่ม Functions, Structs, Enums, Constants และ Items อื่น ๆ ที่เกี่ยวข้องกัน

Modules ช่วยให้:

- จัดระเบียบโค้ดให้อ่านง่าย
- แยกความรับผิดชอบของแต่ละส่วน
- นำโค้ดกลับมาใช้ซ้ำได้
- ควบคุมการเข้าถึง Items ด้วย `pub`
- แยกโค้ดออกเป็นหลายไฟล์

## Learning Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- อธิบายความหมายของ Module ได้
- ประกาศ Module ด้วย `mod` ได้
- เข้าใจ Module Tree และความสัมพันธ์ของ Parent/Child Modules
- แยก Module ออกเป็นไฟล์ต่าง ๆ ได้
- ควบคุมการเข้าถึง Items ด้วย `pub` ได้
- เข้าใจ Privacy Rules ของ Rust
- ใช้ Paths เพื่ออ้างอิง Items ได้
- ใช้ `use` เพื่อนำ Paths เข้ามาใช้งานได้
- เข้าใจ `crate`, `self` และ `super`
- เข้าใจการ Re-export ด้วย `pub use`

## Lessons

| Lesson | Topic                  | Description                     |
| ------ | ---------------------- | ------------------------------- |
| 01     | Module Basics          | ความหมายและการประกาศ Module     |
| 02     | Module Tree            | โครงสร้าง Module และการแยกไฟล์  |
| 03     | Visibility and Privacy | การควบคุมการเข้าถึงด้วย `pub`   |
| 04     | Use and Paths          | การอ้างอิง Path และการใช้ `use` |

## Exercises

| Exercise | Project               | Description                              |
| -------- | --------------------- | ---------------------------------------- |
| 01       | `module_basics`       | สร้าง Module และเรียกใช้ Function        |
| 02       | `file_modules`        | แยก Module ออกเป็นหลายไฟล์               |
| 03       | `visibility_practice` | ฝึกควบคุมการเข้าถึงด้วย `pub`            |
| 04       | `module_challenge`    | สร้างโปรแกรมที่แบ่ง Module อย่างเป็นระบบ |

## Recommended Learning Order

1. อ่าน Lesson 01 เพื่อเข้าใจ Module และ `mod`
2. อ่าน Lesson 02 เพื่อเข้าใจ Module Tree และการแยกไฟล์
3. อ่าน Lesson 03 เพื่อเข้าใจ Visibility และ Privacy
4. อ่าน Lesson 04 เพื่อเรียนรู้ Paths และ `use`
5. ทำ Exercises ตามลำดับ

## Prerequisites

ควรมีความเข้าใจพื้นฐานเรื่อง:

- Functions
- Structs
- Enums
- Ownership และ Borrowing
- Rust Project และ Cargo

## How to Run Exercises

เข้าไปยังโฟลเดอร์ Cargo Project ของ Exercise ที่ต้องการ แล้วรัน:

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจความหมายของ Module
- [ ] ประกาศ Module ด้วย `mod` ได้
- [ ] เข้าใจ Module Tree
- [ ] แยก Module เป็นไฟล์ได้
- [ ] ใช้ `pub` ควบคุมการเข้าถึงได้
- [ ] เข้าใจ Privacy Rules
- [ ] ใช้ Absolute Path และ Relative Path ได้
- [ ] ใช้ `crate`, `self` และ `super` ได้
- [ ] ใช้ `use` ได้
- [ ] เข้าใจ `pub use`
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Learning Resources

- Rust Book — Managing Growing Projects: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Rust Book — Defining Modules: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
- Rust Reference — Modules: https://doc.rust-lang.org/reference/items/modules.html
