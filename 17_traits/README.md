# Chapter 17: Traits

## Overview

Traits เป็นกลไกของ Rust ที่ใช้กำหนดพฤติกรรมหรือความสามารถที่ชนิดข้อมูลสามารถทำได้

ชนิดข้อมูล เช่น Struct และ Enum สามารถ Implement Trait เพื่อให้มี Methods ตามที่ Trait กำหนด

Traits ช่วยให้สามารถใช้พฤติกรรมร่วมกันกับชนิดข้อมูลหลายชนิด และทำงานร่วมกับ Generics ได้

## Learning Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- อธิบายความหมายและประโยชน์ของ Trait ได้
- ประกาศ Trait ด้วย `trait` ได้
- กำหนด Method Signatures ภายใน Trait ได้
- สร้าง Default Method Implementations ได้
- Implement Trait ให้ Struct และ Enum ได้
- Implement หลาย Traits ให้ชนิดข้อมูลเดียวกันได้
- เข้าใจ Derived Traits และ Standard Library Traits
- Implement `Display` ด้วยตนเองได้
- ใช้ Trait Bounds กับ Function ได้
- ใช้ `impl Trait` ใน Parameters และ Return Types ได้
- ใช้ Multiple Trait Bounds และ `where` Clause ได้

## Lessons

| Lesson | Topic                       | Description                             |
| ------ | --------------------------- | --------------------------------------- |
| 01     | Trait Basics                | ความหมาย การประกาศ และโครงสร้าง Trait   |
| 02     | Implementing Traits         | การ Implement Trait ให้ Struct และ Enum |
| 03     | Derived and Standard Traits | Derive และ Standard Library Traits      |
| 04     | Trait Bounds and Impl Trait | การกำหนด Trait Bounds และใช้ impl Trait |

## Exercises

| Exercise | Project         | Description                                       |
| -------- | --------------- | ------------------------------------------------- |
| 01       | basic_traits    | สร้าง Trait และ Implement ให้ Struct              |
| 02       | shape_traits    | สร้างระบบคำนวณพื้นที่ด้วย Traits                  |
| 03       | student_traits  | จัดการข้อมูลนักเรียนด้วย Standard Traits          |
| 04       | trait_challenge | สร้างระบบจัดการข้อมูลด้วย Traits และ Trait Bounds |

## Prerequisites

ควรมีความรู้พื้นฐานจากบทต่อไปนี้:

- Chapter 07: Functions
- Chapter 09: Ownership
- Chapter 10: Borrowing
- Chapter 12: Structs
- Chapter 13: Enums
- Chapter 16: Collections

## Recommended Learning Order

1. เรียนรู้ว่า Trait คืออะไรและใช้ทำอะไร
2. ทดลองประกาศ Trait และ Default Methods
3. ฝึก Implement Trait ให้ Struct และ Enum
4. ศึกษา Derived Traits และ Standard Library Traits
5. ทดลองใช้ Trait Bounds และ `impl Trait`
6. ทำ Exercises ตามลำดับ

## Important Notes

- Trait ใช้กำหนดพฤติกรรม ไม่ใช่ใช้สร้าง Instance โดยตรง
- Struct และ Enum สามารถ Implement Trait ได้
- Trait อาจมี Method Signatures ที่ผู้ Implement ต้องกำหนด
- Trait สามารถมี Default Method Implementations ได้
- `derive` ใช้สร้าง Implementation ให้ Traits บางชนิดโดยอัตโนมัติ
- `Display` ต้อง Implement ด้วยตนเอง ไม่สามารถ Derive ได้โดยตรง
- Trait Bounds ใช้กำหนดข้อกำหนดเกี่ยวกับความสามารถของ Generic Types
- `impl Trait` สามารถใช้เป็น Parameter Type หรือ Return Type ได้
- การ Implement Trait ให้ชนิดข้อมูลและ Trait ที่อยู่นอก Crate ของเราเองมีข้อจำกัดตาม Orphan Rule

## Completion Checklist

- [ ] เข้าใจความหมายของ Trait
- [ ] ประกาศ Trait ได้
- [ ] สร้าง Default Methods ได้
- [ ] Implement Trait ให้ Struct ได้
- [ ] Implement Trait ให้ Enum ได้
- [ ] Implement หลาย Traits ให้ชนิดข้อมูลเดียวกันได้
- [ ] ใช้ Derived Traits ได้
- [ ] Implement `Display` ได้
- [ ] ใช้ `Default`, `From` และ `Into` ได้
- [ ] ใช้ Trait Bounds ได้
- [ ] ใช้ `impl Trait` ได้
- [ ] ใช้ Multiple Trait Bounds ได้
- [ ] ใช้ `where` Clause ได้
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Next Chapter

Chapter 18: Generics
