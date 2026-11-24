# Exercise 02: Raw Pointer Practice

## ระดับ

Easy-Medium

## Objective

ฝึกใช้งาน Raw Pointer กับ Array และ Memory Address พร้อมตรวจสอบ Pointer Safety

## Problem Statement

สร้างโปรแกรมจัดการข้อมูล Array โดยใช้ Raw Pointer อ่านและแก้ไขข้อมูลใน Array

กำหนดข้อมูลเริ่มต้น:

```text
[10, 20, 30, 40, 50]
```

โปรแกรมต้องสามารถ:

1. อ่านข้อมูลแต่ละตำแหน่งผ่าน Raw Pointer
2. แก้ไขค่าบางตำแหน่งผ่าน Mutable Raw Pointer
3. ตรวจสอบ Null Pointer
4. แสดงผลลัพธ์หลังการแก้ไข
5. อธิบายเงื่อนไขความปลอดภัยของ Pointer

## Requirements

### Part 1: Const Pointer

1. สร้าง Array จำนวน 5 ค่า
2. สร้าง Raw Const Pointer จาก Array
3. อ่านข้อมูลแต่ละตำแหน่ง
4. ใช้ Pointer Arithmetic ด้วย `.add()`
5. แสดงข้อมูลตามลำดับ

### Part 2: Mutable Pointer

1. สร้าง Mutable Array
2. สร้าง Raw Mutable Pointer
3. แก้ไขค่าที่ Index 1
4. แก้ไขค่าที่ Index 3
5. แสดง Array หลังแก้ไข

### Part 3: Null Pointer

1. สร้าง Null Const Pointer
2. ตรวจสอบด้วย `.is_null()`
3. แสดงสถานะ Null Pointer
4. ห้าม Dereference Null Pointer

### Part 4: Safety Analysis

เขียนคำอธิบายประกอบว่า:

- Pointer ชี้ไปยัง Memory ตำแหน่งใด
- Memory ยังมีอายุการใช้งานอยู่หรือไม่
- Pointer มี Alignment ถูกต้องหรือไม่
- การใช้ `.add()` อยู่ในขอบเขตที่ถูกต้องหรือไม่
- มีการอ่านและเขียนที่ละเมิด Aliasing Rules หรือไม่

## Constraints

- ต้องใช้ `*const T`
- ต้องใช้ `*mut T`
- ต้องใช้ `.add()`
- ต้องใช้ `.is_null()`
- ต้องใช้ Unsafe Block
- ต้องไม่ Dereference Pointer ที่อยู่นอกขอบเขต Array
- ต้องไม่ใช้ Dangling Pointer
- ต้องไม่อ่าน Memory ที่ยังไม่ได้ Initialize
- ต้องไม่มี Data Race
- ห้ามใช้ `unsafe` โดยไม่จำเป็น

## Expected Output

```text
Original Array
[10, 20, 30, 40, 50]

Read Through Const Pointer
10
20
30
40
50

Modified Array
[10, 200, 30, 400, 50]

Null Pointer Check
Is Null: true
```

## Hints

- ใช้ `array.as_ptr()` สำหรับ Raw Const Pointer
- ใช้ `array.as_mut_ptr()` สำหรับ Raw Mutable Pointer
- ใช้ `.add(index)` เพื่อเลื่อนไปยังตำแหน่งที่ต้องการ
- ใช้ Dereference ภายใน Unsafe Block
- ใช้ `.is_null()` ก่อนพิจารณาการใช้งาน Pointer

ข้อควรจำ: การตรวจสอบว่า Pointer ไม่เป็น Null ไม่ได้แปลว่า Pointer นั้นปลอดภัยเสมอไป

## Completion Checklist

- [ ] สร้าง Array ได้
- [ ] สร้าง Const Pointer ได้
- [ ] อ่านข้อมูลด้วย Pointer ได้
- [ ] ใช้ Pointer Arithmetic ได้
- [ ] สร้าง Mutable Pointer ได้
- [ ] แก้ไขข้อมูลผ่าน Pointer ได้
- [ ] ตรวจสอบ Null Pointer ได้
- [ ] ตรวจสอบขอบเขต Array ได้
- [ ] อธิบาย Pointer Safety ได้
- [ ] ไม่มี Undefined Behavior

## Run

```powershell
cd 27_unsafe_rust/exercises/exercise_02/raw_pointer_practice
cargo run
```
