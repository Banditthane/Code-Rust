# Exercise 04: Unsafe Challenge

## ระดับ

Challenge

## Objective

ประยุกต์ใช้ Raw Pointers, Unsafe Functions, Safety Contracts, FFI และ Safe Abstraction เพื่อสร้างระบบจัดการ Memory Buffer ที่มีขอบเขตชัดเจน

## Problem Statement

สร้างระบบ `Memory Buffer` สำหรับจัดเก็บข้อมูลตัวเลข

ระบบต้องสามารถ:

1. สร้าง Buffer
2. เก็บข้อมูลตัวเลข
3. อ่านข้อมูลผ่าน Index
4. แก้ไขข้อมูลผ่าน Index
5. ตรวจสอบขอบเขต Memory
6. จัดการ Invalid Input
7. เปิด Safe API ให้ผู้ใช้งาน
8. แยก Unsafe Implementation ออกจากส่วนที่ใช้งานทั่วไป

สามารถเลือกใช้ Vec เป็นตัวจัดการ Memory หลัก และใช้ Raw Pointer เฉพาะในส่วนที่จำเป็น

## Requirements

### Part 1: Buffer Design

สร้าง Struct ชื่อ `MemoryBuffer`

กำหนดข้อมูลที่เหมาะสม เช่น:

- Pointer หรือ Storage
- จำนวนข้อมูล
- ความจุของ Buffer

เลือกวิธีจัดการ Memory ที่เหมาะสม และอธิบายเหตุผล

### Part 2: Pointer Operations

1. สร้าง Raw Pointer สำหรับอ่านข้อมูล
2. สร้าง Raw Mutable Pointer สำหรับแก้ไขข้อมูล
3. อ่านข้อมูลตาม Index
4. แก้ไขข้อมูลตาม Index
5. ตรวจสอบขอบเขต
6. รักษา Pointer Validity
7. รักษา Memory Alignment
8. รักษา Aliasing Rules

### Part 3: Safe Abstraction

สร้าง Safe API:

1. `new()` สำหรับสร้าง Buffer
2. `len()` สำหรับคืนจำนวนข้อมูล
3. `is_empty()` สำหรับตรวจสอบข้อมูลว่าง
4. `get()` สำหรับอ่านข้อมูล
5. `set()` สำหรับแก้ไขข้อมูล
6. `push()` สำหรับเพิ่มข้อมูล
7. `clear()` สำหรับล้างข้อมูล

Function ที่ต้องรับข้อมูลผิดพลาดได้ควรคืนค่า `Result` หรือ `Option` ตามความเหมาะสม

### Part 4: Unsafe Implementation

1. แยก Unsafe Code ออกจาก Public API
2. จำกัด Unsafe Block ให้เล็กที่สุด
3. ระบุ Safety Contract
4. ตรวจสอบ Pointer ก่อนใช้งาน
5. หลีกเลี่ยงการเก็บ Pointer ที่หมดอายุ
6. ตรวจสอบกรณี Buffer ถูกเปลี่ยนขนาด
7. ตรวจสอบการเข้าถึง Memory ระหว่าง Mutable และ Immutable Operations

### Part 5: Testing

ทดสอบกรณี:

1. สร้าง Buffer ว่าง
2. เพิ่มข้อมูลหลายรายการ
3. อ่านข้อมูลในตำแหน่งที่ถูกต้อง
4. อ่านข้อมูลนอกขอบเขต
5. แก้ไขข้อมูล
6. เพิ่มข้อมูลจน Buffer ต้องขยาย Capacity
7. ล้างข้อมูล
8. อ่านข้อมูลหลังล้าง Buffer

### Part 6: Safety Review

เขียนเอกสารอธิบาย:

- Memory ถูกจัดสรรและจัดการอย่างไร
- Pointer มีอายุการใช้งานอย่างไร
- เมื่อ Buffer ขยายขนาด Pointer เดิมยังใช้งานได้หรือไม่
- มีโอกาสเกิด Dangling Pointer หรือไม่
- Safe API ป้องกัน Invalid Input อย่างไร
- Unsafe Code มีขอบเขตอยู่ตรงไหน

## Constraints

- ต้องมี Struct `MemoryBuffer`
- ต้องมี Safe API อย่างน้อย 6 Methods
- ต้องมี Raw Pointer ในส่วน Implementation
- ต้องมี Unsafe Function หรือ Unsafe Block
- ต้องมี Safety Documentation
- ต้องจัดการ Buffer Capacity
- ต้องตรวจสอบ Index
- ต้องจัดการ Memory Lifetime
- ต้องไม่อ่านหรือเขียน Memory นอกขอบเขต
- ต้องไม่มี Data Race
- ห้ามใช้ `static mut`
- ห้ามใช้ `unsafe` เพื่อหลีกเลี่ยง Ownership หรือ Borrow Checker โดยไม่มีเหตุผล
- ต้องไม่มี Undefined Behavior

## Expected Output

```text
Memory Buffer

Created
Length: 0
Capacity: 0

Adding Values
10
20
30
40
50

Current Buffer
[10, 20, 30, 40, 50]

Read Index 2
Value: 30

Update Index 1
New Value: 200

Updated Buffer
[10, 200, 30, 40, 50]

Invalid Index
Error: Index out of bounds

Clear Buffer
Length: 0

Memory Buffer Test Completed
```

รายละเอียด Capacity อาจแตกต่างกันตามการจัดสรร Memory

## Hints

- พิจารณาใช้ `Vec<T>` เป็น Storage เพื่อให้ Rust จัดการ Allocation และ Deallocation
- ใช้ `.as_ptr()` และ `.as_mut_ptr()` เพื่อรับ Raw Pointer
- ใช้ `.len()` และ `.capacity()` ตรวจสอบ Buffer
- ระวัง Pointer Invalidated หลังจาก Vec เปลี่ยน Allocation
- ใช้ `.get()` และ `.get_mut()` เป็น Safe Alternative หากไม่จำเป็นต้องใช้ Raw Pointer
- แยก Unsafe Operation ออกจาก Validation
- อย่าเก็บ Raw Pointer ข้ามการเปลี่ยนขนาดของ Vec โดยไม่ตรวจสอบ Validity ใหม่

## Completion Checklist

### Buffer Design

- [ ] สร้าง MemoryBuffer
- [ ] จัดการ Storage
- [ ] จัดการ Length
- [ ] จัดการ Capacity

### Pointer Operations

- [ ] สร้าง Raw Const Pointer
- [ ] สร้าง Raw Mutable Pointer
- [ ] อ่านข้อมูลผ่าน Pointer
- [ ] แก้ไขข้อมูลผ่าน Pointer
- [ ] ตรวจสอบ Index
- [ ] ตรวจสอบ Memory Validity

### Safe Abstraction

- [ ] สร้าง `new()`
- [ ] สร้าง `len()`
- [ ] สร้าง `is_empty()`
- [ ] สร้าง `get()`
- [ ] สร้าง `set()`
- [ ] สร้าง `push()`
- [ ] สร้าง `clear()`

### Safety

- [ ] จำกัด Unsafe Scope
- [ ] เขียน Safety Documentation
- [ ] จัดการ Pointer Lifetime
- [ ] ตรวจสอบ Capacity Changes
- [ ] ไม่มี Dangling Pointer
- [ ] ไม่มี Out-of-bounds Access
- [ ] ไม่มี Undefined Behavior

### Testing

- [ ] ทดสอบ Buffer ว่าง
- [ ] ทดสอบเพิ่มข้อมูล
- [ ] ทดสอบอ่านข้อมูล
- [ ] ทดสอบแก้ไขข้อมูล
- [ ] ทดสอบ Invalid Index
- [ ] ทดสอบ Buffer Expansion
- [ ] ทดสอบ Clear

## Run

```powershell
cd 27_unsafe_rust/exercises/exercise_04/unsafe_challenge

cargo check
cargo test
cargo run
```

## Final Challenge

เมื่อทำโจทย์สำเร็จแล้ว ให้ทดลองเพิ่มความสามารถ:

1. เพิ่ม Generic Type ให้ MemoryBuffer
2. เพิ่ม Iterator สำหรับอ่านข้อมูล
3. เพิ่ม Function สำหรับค้นหาข้อมูล
4. เพิ่มการตรวจสอบด้วย Miri
5. เปรียบเทียบ Safe Implementation กับ Unsafe Implementation
6. อธิบายว่า Unsafe Code ที่เพิ่มเข้ามามีความจำเป็นจริงหรือไม่

**หมายเหตุ:** จุดประสงค์ของ Challenge ไม่ใช่การเขียน Unsafe ให้มากที่สุด แต่เป็นการออกแบบระบบที่รักษา Memory Safety และใช้ Unsafe เฉพาะในจุดที่จำเป็น
