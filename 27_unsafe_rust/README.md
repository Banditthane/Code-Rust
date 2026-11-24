# Chapter 27: Unsafe Rust

## เป้าหมายของบทเรียน

เรียนรู้ Unsafe Rust เข้าใจความแตกต่างระหว่าง Safe Rust และ Unsafe Rust และสามารถใช้งาน Low-level Features อย่างระมัดระวัง

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบาย Safe Rust และ Unsafe Rust ได้
- เข้าใจข้อจำกัดของ Borrow Checker
- ใช้ `unsafe` Block ได้
- เข้าใจ Raw Pointers
- แยกความแตกต่างระหว่าง `*const T` และ `*mut T` ได้
- เข้าใจการ Dereference Pointer
- สร้าง Unsafe Function ได้
- เข้าใจ Unsafe Trait และ Unsafe Implementation
- เข้าใจ Undefined Behavior (UB)
- เข้าใจ Static Mutable Variables
- เข้าใจ Foreign Function Interface (FFI)
- ออกแบบ Safe Abstraction ครอบ Unsafe Code ได้

## บทเรียน

| Lesson | หัวข้อ                      | เนื้อหา                                           |
| ------ | --------------------------- | ------------------------------------------------- |
| 01     | Unsafe Fundamentals         | Safe Rust, Unsafe Rust, Unsafe Blocks             |
| 02     | Raw Pointers                | Raw Pointer, Dereferencing, Pointer Safety        |
| 03     | Unsafe Functions and Traits | Unsafe Functions, Unsafe Traits, Safety Contracts |
| 04     | Unsafe Boundaries           | Static Mutable, UB, FFI, Encapsulation            |

## แบบฝึกหัด

| Exercise | ชื่อ                 | ระดับ       |
| -------- | -------------------- | ----------- |
| 01       | Unsafe Basics        | Easy        |
| 02       | Raw Pointer Practice | Easy-Medium |
| 03       | Unsafe Functions     | Medium      |
| 04       | Unsafe Challenge     | Challenge   |

## โครงสร้างโฟลเดอร์

```text
27_unsafe_rust/
├── README.md
├── 01_unsafe_fundamentals/
│   └── README.md
├── 02_raw_pointers/
│   └── README.md
├── 03_unsafe_functions_traits/
│   └── README.md
├── 04_unsafe_boundaries/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── unsafe_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── raw_pointer_practice/
    ├── exercise_03/
    │   ├── README.md
    │   └── unsafe_functions/
    └── exercise_04/
        ├── README.md
        └── unsafe_challenge/
```

## แนวคิดสำคัญ

### Safe Rust

Rust ตรวจสอบความปลอดภัยด้าน Memory และ Ownership ใน Compile Time ภายใต้กฎของภาษา

### Unsafe Rust

เปิดให้ใช้งาน Operation บางอย่างที่ Safe Rust ไม่อนุญาต โดยผู้เขียนต้องรับผิดชอบต่อเงื่อนไขความปลอดภัยที่ Compiler ตรวจสอบให้ไม่ได้

### Raw Pointers

Pointer ที่ไม่ได้อยู่ภายใต้กฎ Borrowing แบบ Reference ปกติ

มี 2 ประเภทหลัก:

- `*const T`
- `*mut T`

### Undefined Behavior (UB)

พฤติกรรมที่ไม่ถูกกำหนดโดยภาษา Rust ซึ่งอาจทำให้โปรแกรมทำงานผิดพลาดหรือมีผลลัพธ์ที่คาดเดาไม่ได้

## เครื่องมือที่ใช้

- Rust Compiler
- `unsafe`
- Raw Pointers
- `std::ptr`
- `std::ffi`
- `std::os::raw`

## คำสั่งที่ใช้บ่อย

```powershell
cargo new unsafe_basics
cd unsafe_basics

cargo check
cargo run
cargo test
```

ตรวจสอบ Lint:

```powershell
cargo clippy
```

## แนวทางการเรียน

1. เข้าใจว่า Safe Rust ป้องกันปัญหาอะไร
2. เรียนรู้ข้อจำกัดและขอบเขตของ `unsafe`
3. ทดลองใช้ Raw Pointers
4. เรียนรู้ Unsafe Functions และ Traits
5. ศึกษา Undefined Behavior และ FFI
6. ฝึกสร้าง Safe Abstraction ครอบ Unsafe Code

## ข้อควรระวัง

- `unsafe` ไม่ได้ทำให้โค้ดปลอดภัยโดยอัตโนมัติ
- Compiler จะไม่ตรวจสอบ Safety Contract บางส่วนให้
- การ Dereference Raw Pointer ต้องตรวจสอบความถูกต้องของ Pointer
- ต้องรักษาเงื่อนไข Ownership, Alignment และ Validity
- การเข้าถึง Mutable Static ต้องจัดการ Synchronization ให้ถูกต้อง
- FFI ต้องตรวจสอบ ABI และชนิดข้อมูลให้ตรงกับระบบภายนอก
- ควรจำกัด Unsafe Code ให้อยู่ในขอบเขตเล็กที่สุด
- หลีกเลี่ยง `unsafe` หาก Safe Rust สามารถแก้ปัญหาได้

## Completion Checklist

- [ ] เข้าใจ Safe Rust
- [ ] เข้าใจ Unsafe Rust
- [ ] ใช้ Unsafe Block ได้
- [ ] เข้าใจ Raw Pointers
- [ ] ใช้ `*const T` ได้
- [ ] ใช้ `*mut T` ได้
- [ ] เข้าใจ Unsafe Functions
- [ ] เข้าใจ Unsafe Traits
- [ ] เข้าใจ Undefined Behavior
- [ ] เข้าใจ Static Mutable Variables
- [ ] เข้าใจ FFI
- [ ] ออกแบบ Safe Abstraction ได้

## เกณฑ์ผ่านบทเรียน

สามารถอธิบายข้อจำกัดและความเสี่ยงของ Unsafe Rust เข้าใจการใช้ Raw Pointers และสามารถกำหนด Safety Contract สำหรับโค้ดที่ต้องใช้ Unsafe ได้

**หลักสำคัญ:** ใช้ Safe Rust เป็นค่าเริ่มต้น และใช้ Unsafe เฉพาะเมื่อมีเหตุผลที่ชัดเจนและสามารถรักษา Safety Invariants ได้
