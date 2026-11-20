# Chapter 19: Lifetimes

## Overview

Lifetimes คือกลไกของ Rust ที่ใช้ตรวจสอบระยะเวลาการใช้งานของ References เพื่อให้มั่นใจว่า Reference จะไม่ถูกนำไปใช้งานหลังจากข้อมูลต้นทางหมดอายุการใช้งานแล้ว

Lifetimes ทำงานร่วมกับ Ownership และ Borrowing โดย Rust ใช้ Borrow Checker ตรวจสอบความถูกต้องในระหว่าง Compile Time

## Learning Objectives

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบายความหมายของ Lifetimes ได้
- เข้าใจความสัมพันธ์ระหว่าง References และ Lifetimes
- เข้าใจ Scope และ Lifetime
- อธิบาย Dangling References ได้
- เข้าใจการทำงานของ Borrow Checker
- เขียน Lifetime Annotations ได้
- ใช้ Lifetime Parameters ใน Functions ได้
- ใช้ Lifetimes กับ Structs ได้
- ใช้ Lifetimes กับ Methods ได้
- เข้าใจ Lifetime Elision Rules
- ใช้ Lifetime Bounds ได้
- เข้าใจ `'static` Lifetime
- ประยุกต์ใช้ Lifetimes ร่วมกับ Generics และ Traits ได้

## Lessons

| Lesson | Topic                              | Description                                      |
| ------ | ---------------------------------- | ------------------------------------------------ |
| 01     | Lifetime Basics                    | ทำความเข้าใจ Lifetimes, Scope และ References     |
| 02     | Lifetime Annotations               | เรียนรู้การประกาศ Lifetime Parameters            |
| 03     | Lifetimes in Functions and Structs | ใช้ Lifetimes กับ Functions, Structs และ Methods |
| 04     | Lifetime Bounds and Static         | เรียนรู้ Lifetime Bounds และ `'static`           |

## Exercises

| Exercise | Project             | Description                                       |
| -------- | ------------------- | ------------------------------------------------- |
| 01       | lifetime_basics     | ทดลอง Scope และ Lifetime ของ References           |
| 02       | lifetime_functions  | สร้าง Functions ที่ใช้ Lifetime Annotations       |
| 03       | lifetime_structs    | สร้าง Struct ที่เก็บ References                   |
| 04       | lifetimes_challenge | ประยุกต์ใช้ Lifetimes ร่วมกับ Generics และ Traits |

## Folder Structure

```text
19_lifetimes/
├── README.md
├── 01_lifetime_basics/
│   └── README.md
├── 02_lifetime_annotations/
│   └── README.md
├── 03_lifetimes_in_functions_and_structs/
│   └── README.md
├── 04_lifetime_bounds_and_static/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── lifetime_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── lifetime_functions/
    ├── exercise_03/
    │   ├── README.md
    │   └── lifetime_structs/
    └── exercise_04/
        ├── README.md
        └── lifetimes_challenge/
```

## Recommended Learning Sequence

1. ทบทวน Ownership และ Borrowing
2. ทำความเข้าใจ Scope และ Lifetime
3. เรียนรู้ Dangling References และ Borrow Checker
4. ศึกษา Lifetime Annotations
5. ทดลองใช้ Lifetimes กับ Functions
6. ทดลองใช้ Lifetimes กับ Structs และ Methods
7. เรียนรู้ Lifetime Bounds และ `'static`
8. ทำ Exercises ตามลำดับ

## Completion Checklist

- [ ] เข้าใจความหมายของ Lifetimes
- [ ] เข้าใจความสัมพันธ์ระหว่าง References และ Lifetimes
- [ ] เข้าใจ Scope และ Lifetime
- [ ] อธิบาย Dangling References ได้
- [ ] เข้าใจ Borrow Checker
- [ ] เขียน Lifetime Annotations ได้
- [ ] ใช้ Lifetimes กับ Functions ได้
- [ ] ใช้ Lifetimes กับ Structs ได้
- [ ] ใช้ Lifetimes กับ Methods ได้
- [ ] เข้าใจ Lifetime Elision Rules
- [ ] เข้าใจ Lifetime Bounds
- [ ] เข้าใจ `'static`
- [ ] ใช้ Lifetimes ร่วมกับ Generics และ Traits ได้
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Important Note

Lifetime Annotations ไม่ได้ทำให้ Reference มีอายุยาวขึ้น แต่เป็นการระบุความสัมพันธ์ระหว่าง Lifetimes เพื่อให้ Rust ตรวจสอบความถูกต้องของ References ได้

## Next Chapter

**Chapter 20: Testing**

เรียนรู้การทดสอบโปรแกรมด้วย Unit Tests, Integration Tests และเครื่องมือทดสอบของ Rust
