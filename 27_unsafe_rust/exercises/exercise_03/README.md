# Exercise 03: Unsafe Functions

## ระดับ

Medium

## Objective

ฝึกสร้าง Unsafe Function และเรียนรู้การกำหนด Safety Contract รวมถึงการสร้าง Safe Function ที่ครอบ Unsafe Implementation

## Problem Statement

สร้างระบบจัดการข้อมูลตัวเลขด้วย Pointer โดยแยกการทำงานออกเป็น Function

ระบบต้องมี:

1. Unsafe Function สำหรับอ่านค่า
2. Unsafe Function สำหรับแก้ไขค่า
3. Safe Function สำหรับตรวจสอบ Input
4. Safe API สำหรับเรียกใช้งาน Unsafe Function
5. Documentation ที่อธิบาย Safety Contract

กำหนดข้อมูลเริ่มต้น:

```text
10, 20, 30, 40, 50
```

## Requirements

### Part 1: Unsafe Read Function

สร้าง Function:

```text
read_value(pointer)
```

ความสามารถ:

1. รับ Raw Const Pointer
2. อ่านค่าที่ Pointer ชี้อยู่
3. คืนค่าที่อ่านได้
4. ระบุ Safety Contract
5. ใช้ Unsafe Block ภายใน Function

### Part 2: Unsafe Write Function

สร้าง Function:

```text
write_value(pointer, value)
```

ความสามารถ:

1. รับ Raw Mutable Pointer
2. รับค่าที่ต้องการเขียน
3. เขียนค่าลง Memory
4. ระบุ Safety Contract
5. ตรวจสอบให้แน่ใจว่า Memory เขียนได้อย่างปลอดภัย

### Part 3: Safe Wrapper

สร้าง Safe Function สำหรับอ่านและแก้ไขข้อมูลผ่าน Index

ความสามารถ:

1. รับ Slice หรือ Array
2. รับ Index
3. ตรวจสอบ Index
4. ตรวจสอบขอบเขตข้อมูล
5. เรียก Unsafe Function เฉพาะเมื่อเงื่อนไขถูกต้อง
6. คืนค่า `Option` หรือ `Result` ตามความเหมาะสม

### Part 4: Documentation

ทุก Unsafe Function ต้องมี Documentation ส่วน `# Safety`

อธิบาย:

- Pointer ต้องไม่เป็น Null
- Pointer ต้องมี Alignment ถูกต้อง
- Memory ต้องยังมีอายุการใช้งาน
- Memory ต้องมีชนิดข้อมูลถูกต้อง
- การอ่านและเขียนต้องไม่ละเมิด Aliasing Rules

## Constraints

- ต้องมี Unsafe Function อย่างน้อย 2 Function
- ต้องมี Safe Wrapper อย่างน้อย 2 Function
- ต้องใช้ Raw Pointer
- ต้องใช้ Unsafe Block
- ต้องมี `# Safety` Documentation
- ต้องตรวจสอบ Index ก่อนเรียก Unsafe Operation
- ต้องจัดการ Invalid Input
- ห้ามใช้ `static mut`
- ห้ามใช้ Dangling Pointer
- ห้ามสร้าง Undefined Behavior

## Expected Output

```text
Original Data
[10, 20, 30, 40, 50]

Read Index 2
Value: 30

Update Index 1
New Value: 200

Updated Data
[10, 200, 30, 40, 50]

Invalid Index
Error: Index out of bounds
```

## Hints

- ใช้ Slice เพื่อรับข้อมูลที่มีขนาดยืดหยุ่น
- ใช้ `.get()` ตรวจสอบ Index
- ใช้ `.get_mut()` สำหรับ Mutable Access
- ใช้ `Result<T, E>` สำหรับ Error ที่ต้องการรายละเอียด
- Unsafe Function ควรมีหน้าที่แคบและชัดเจน
- Safe Wrapper ต้องรักษา Invariants ให้ครบก่อนเรียก Unsafe Code

## Completion Checklist

- [ ] สร้าง Unsafe Read Function
- [ ] สร้าง Unsafe Write Function
- [ ] กำหนด Safety Contract
- [ ] สร้าง Safe Wrapper สำหรับอ่าน
- [ ] สร้าง Safe Wrapper สำหรับเขียน
- [ ] ตรวจสอบ Index
- [ ] จัดการ Invalid Input
- [ ] ใช้ Result หรือ Option
- [ ] ทดสอบกรณีข้อมูลถูกต้อง
- [ ] ทดสอบกรณี Index ผิด
- [ ] ตรวจสอบว่า Unsafe Scope มีขนาดเล็ก
- [ ] ไม่มี Undefined Behavior

## Run

```powershell
cd 27_unsafe_rust/exercises/exercise_03/unsafe_functions
cargo run
cargo test
```

## Additional Challenge

1. เพิ่ม Function สำหรับอ่านข้อมูลหลายตำแหน่ง
2. เพิ่ม Function สำหรับค้นหาค่าสูงสุด
3. เพิ่ม Safe Wrapper ที่รองรับ Mutable Slice
4. ทดลองตรวจสอบด้วย Miri
