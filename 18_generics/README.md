# Chapter 18: Generics

## Overview

Generics คือความสามารถของ Rust ที่ช่วยให้เราสามารถเขียนโค้ดให้รองรับข้อมูลได้หลายชนิด โดยไม่จำเป็นต้องเขียน Function, Struct หรือ Enum แยกสำหรับแต่ละ Data Type

Generics ช่วยลดการเขียนโค้ดซ้ำ และทำให้โปรแกรมมีความยืดหยุ่นมากขึ้น

## Learning Objectives

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบายความหมายและประโยชน์ของ Generics ได้
- เข้าใจ Generic Type Parameters
- สร้าง Generic Functions ได้
- ใช้ Generics กับ Structs ได้
- ใช้ Generics กับ Enums ได้
- สร้าง Generic Methods ได้
- ใช้ Trait Bounds กับ Generics ได้
- ใช้ Multiple Trait Bounds ได้
- ใช้ `where` Clause ได้
- ประยุกต์ใช้ Generics ร่วมกับ Traits ได้

## Lessons

| Lesson | Topic                            | Description                               |
| ------ | -------------------------------- | ----------------------------------------- |
| 01     | Generic Basics                   | ทำความเข้าใจ Generics และ Type Parameters |
| 02     | Generic Functions                | สร้าง Function ที่รองรับหลายชนิดข้อมูล    |
| 03     | Generic Structs and Enums        | ใช้ Generics กับ Struct และ Enum          |
| 04     | Generic Methods and Trait Bounds | ใช้ Generics กับ Methods และ Traits       |

## Exercises

| Exercise | Project            | Description                         |
| -------- | ------------------ | ----------------------------------- |
| 01       | generic_basics     | ฝึกใช้ Generic Type Parameters      |
| 02       | generic_functions  | สร้าง Generic Functions             |
| 03       | generic_structs    | สร้าง Generic Structs และ Enums     |
| 04       | generics_challenge | ประยุกต์ใช้ Generics ร่วมกับ Traits |

## Folder Structure

```text
18_generics/
├── README.md
├── 01_generic_basics/
│   └── README.md
├── 02_generic_functions/
│   └── README.md
├── 03_generic_structs_and_enums/
│   └── README.md
├── 04_generic_methods_and_trait_bounds/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── generic_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── generic_functions/
    ├── exercise_03/
    │   ├── README.md
    │   └── generic_structs/
    └── exercise_04/
        ├── README.md
        └── generics_challenge/
```

## Recommended Learning Sequence

1. ศึกษาความหมายของ Generics
2. ทดลองสร้าง Generic Functions
3. ทดลองสร้าง Generic Structs และ Enums
4. เรียนรู้ Generic Methods
5. ทำความเข้าใจ Trait Bounds
6. ฝึกใช้ Multiple Trait Bounds และ `where` Clause
7. ทำ Exercises ตามลำดับ

## Completion Checklist

- [ ] เข้าใจความหมายของ Generics
- [ ] เข้าใจ Type Parameters
- [ ] สร้าง Generic Functions ได้
- [ ] สร้าง Generic Structs ได้
- [ ] สร้าง Generic Enums ได้
- [ ] สร้าง Generic Methods ได้
- [ ] ใช้ Trait Bounds ได้
- [ ] ใช้ Multiple Trait Bounds ได้
- [ ] ใช้ `where` Clause ได้
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Next Chapter

**Chapter 19: Lifetimes**

เรียนรู้การกำหนดและทำความเข้าใจ Lifetime เพื่อจัดการความสัมพันธ์ระหว่าง References ใน Rust
