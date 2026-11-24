# Exercise 01: Unsafe Basics

## ระดับ

Easy

## Objective

ฝึกทำความเข้าใจ Unsafe Block และการใช้งาน Raw Pointer เบื้องต้น

## Problem Statement

สร้างโปรแกรมจัดการข้อมูลตัวเลข โดยทดลองใช้งานทั้ง Safe Rust และ Unsafe Rust

กำหนดข้อมูล:

```text
Value 1: 100
Value 2: 200
Value 3: 300
```

โปรแกรมต้องแสดงข้อมูลผ่าน Reference และ Raw Pointer พร้อมเปรียบเทียบการใช้งาน

## Requirements

### Part 1: Safe Rust

1. สร้างตัวแปรตัวเลขอย่างน้อย 3 ตัว
2. ใช้ Reference อ่านค่าตัวแปร
3. แสดงค่าผ่าน Reference
4. สร้าง Function สำหรับแสดงข้อมูล

### Part 2: Raw Pointer

1. สร้าง Raw Const Pointer ด้วย `&raw const`
2. สร้าง Raw Mutable Pointer ด้วย `&raw mut`
3. ทดลอง Dereference Raw Pointer
4. ใช้ Unsafe Block ในจุดที่จำเป็น
5. แสดงค่าที่อ่านจาก Pointer

### Part 3: Comparison

1. เปรียบเทียบ Reference กับ Raw Pointer
2. อธิบายว่าทำไม Raw Pointer ต้องใช้ Unsafe เมื่อ Dereference
3. อธิบายความแตกต่างระหว่าง `*const T` และ `*mut T`
4. ระบุความเสี่ยงของ Raw Pointer

## Constraints

- ต้องใช้ Safe Rust ร่วมกับ Unsafe Rust
- ต้องสร้าง Raw Pointer ทั้ง 2 ประเภท
- ต้องใช้ `unsafe` Block
- ต้องไม่ Dereference Null Pointer
- ต้องไม่ใช้ Dangling Pointer
- ต้องไม่มี Data Race
- ห้ามใช้ `static mut`
- ห้ามใช้ `unsafe` เกินความจำเป็น

## Expected Output

```text
Safe Rust
Value 1: 100
Value 2: 200
Value 3: 300

Raw Pointer
Pointer 1 Value: 100
Pointer 2 Value: 200
Pointer 3 Value: 300

Comparison
Reference: Checked by Rust
Raw Pointer: Requires Safety Guarantees
```

สามารถออกแบบข้อความเพิ่มเติมได้

## Hints

- ใช้ `&value` เพื่อสร้าง Reference
- ใช้ `&raw const value` เพื่อสร้าง Raw Const Pointer
- ใช้ `&raw mut value` เพื่อสร้าง Raw Mutable Pointer
- ใช้ `unsafe {}` สำหรับ Dereference
- ใช้ `{:p}` แสดง Address ของ Pointer

## Completion Checklist

- [ ] สร้างตัวแปร 3 ตัว
- [ ] ใช้ Reference อ่านข้อมูลได้
- [ ] สร้าง Raw Const Pointer ได้
- [ ] สร้าง Raw Mutable Pointer ได้
- [ ] Dereference Pointer ได้
- [ ] ใช้ Unsafe Block ได้ถูกต้อง
- [ ] เปรียบเทียบ Reference กับ Raw Pointer ได้
- [ ] อธิบายความเสี่ยงของ Raw Pointer ได้
- [ ] ไม่มี Undefined Behavior

## Run

```powershell
cd 27_unsafe_rust/exercises/exercise_01/unsafe_basics
cargo run
```
