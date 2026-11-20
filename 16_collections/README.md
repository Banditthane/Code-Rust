# Chapter 16: Collections

## Overview

บทนี้เรียนรู้การจัดเก็บและจัดการข้อมูลหลายรายการด้วย Collections ในภาษา Rust โดยต่อยอดจากความรู้เรื่อง Vector, String และ HashMap

เน้นการเพิ่ม ลบ แก้ไข ค้นหา เรียงลำดับ และประมวลผลข้อมูลด้วย Iterator

## Learning Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- สร้างและจัดการ Vector ด้วย `Vec<T>` ได้
- เพิ่ม ลบ แก้ไข และค้นหาข้อมูลใน Vector ได้
- ใช้ Iterator เพื่อวนซ้ำและประมวลผลข้อมูลได้
- จัดการข้อความด้วย `String` และ `&str` ได้
- เข้าใจความแตกต่างระหว่างจำนวนไบต์และจำนวนตัวอักษรใน UTF-8
- สร้างและจัดการข้อมูลด้วย `HashMap<K, V>` ได้
- ใช้ `entry()` และ `or_insert()` เพื่อจัดการข้อมูลตาม Key ได้
- ใช้ `map()`, `filter()`, `collect()` และ `sum()` ได้
- เลือกใช้ Collection ให้เหมาะสมกับลักษณะของข้อมูล

## Lessons

| Lesson | Topic               | Description                                     |
| ------ | ------------------- | ----------------------------------------------- |
| 01     | Vectors             | การจัดการข้อมูลด้วย Vec และ Iterator            |
| 02     | Strings             | การสร้าง แก้ไข ค้นหา และแยกข้อความ              |
| 03     | Hash Maps           | การจัดเก็บข้อมูลแบบ Key-Value                   |
| 04     | Collection Patterns | การประยุกต์ใช้ Collections และ Iterator ร่วมกัน |

## Exercises

| Exercise | Project              | Description               |
| -------- | -------------------- | ------------------------- |
| 01       | vector_manager       | โปรแกรมจัดการรายการตัวเลข |
| 02       | string_processor     | โปรแกรมประมวลผลข้อความ    |
| 03       | inventory_manager    | โปรแกรมจัดการสินค้า       |
| 04       | collection_challenge | ระบบจัดการข้อมูลนักเรียน  |

## Prerequisites

ควรมีความรู้พื้นฐานจากบทต่อไปนี้:

- Chapter 07: Functions
- Chapter 08: Strings
- Chapter 09: Ownership
- Chapter 10: Borrowing
- Chapter 11: Data Structures
- Chapter 13: Enums
- Chapter 15: Error Handling

## Recommended Learning Order

1. อ่าน Lesson 01 และทดลองสร้าง Vector ด้วยตนเอง
2. อ่าน Lesson 02 และฝึกจัดการข้อความ
3. อ่าน Lesson 03 และฝึกจัดเก็บข้อมูลแบบ Key-Value
4. อ่าน Lesson 04 และทดลองใช้ Iterator
5. ทำ Exercises ตามลำดับ
6. ตรวจสอบผลลัพธ์และทบทวนข้อผิดพลาด

## Important Notes

- การเข้าถึง Vector ด้วย Index ที่อยู่นอกขอบเขตจะทำให้เกิด Panic
- ควรใช้ `get()` เมื่อไม่แน่ใจว่า Index มีอยู่หรือไม่
- `String` เก็บข้อความในรูปแบบ UTF-8
- `String::len()` คืนค่าจำนวนไบต์ ไม่ใช่จำนวนตัวอักษร
- HashMap ไม่รับประกันลำดับการวนซ้ำ
- Iterator ส่วนใหญ่ทำงานแบบ Lazy คือยังไม่ประมวลผลจนกว่าจะมีการเรียกใช้งาน Iterator เช่น `collect()`, `sum()` หรือ `for`
- ควรเข้าใจ Ownership และ Borrowing ก่อนใช้ Collections ร่วมกัน

## Completion Checklist

- [ ] เข้าใจการสร้างและจัดการ Vector
- [ ] ใช้ Iterator วนซ้ำข้อมูลได้
- [ ] ใช้ `map()` และ `filter()` ได้
- [ ] ใช้ `collect()` สร้าง Collection ได้
- [ ] จัดการ String และข้อความ UTF-8 ได้
- [ ] ใช้ HashMap จัดเก็บข้อมูลได้
- [ ] ใช้ `entry()` และ `or_insert()` ได้
- [ ] เลือกใช้ Collection ให้เหมาะสมกับงานได้
- [ ] ทำ Exercise 01 สำเร็จ
- [ ] ทำ Exercise 02 สำเร็จ
- [ ] ทำ Exercise 03 สำเร็จ
- [ ] ทำ Exercise 04 สำเร็จ

## Next Chapter

Chapter 17: Traits
