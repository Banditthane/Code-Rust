# 23 - smart_pointers

# Chapter 23: Smart Pointers

## วัตถุประสงค์

เรียนรู้ Smart Pointers ในภาษา Rust เพื่อจัดการหน่วยความจำและความเป็นเจ้าของข้อมูลในกรณีที่ซับซ้อนกว่าการใช้ตัวแปรและ References ทั่วไป

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- อธิบายความแตกต่างระหว่าง References และ Smart Pointers ได้
- ใช้ `Box<T>` สำหรับจัดเก็บข้อมูลบน Heap ได้
- เข้าใจ Shared Ownership ผ่าน `Rc<T>` และ `Arc<T>`
- เข้าใจ Interior Mutability ผ่าน `Cell<T>` และ `RefCell<T>`
- อธิบายการทำงานของ `Deref` และ `Drop` ได้
- เลือก Smart Pointer ให้เหมาะกับลักษณะการใช้งานได้

## เนื้อหาภายในบท

### Lesson 01: Smart Pointer Basics

- Stack และ Heap
- แนวคิดของ Smart Pointer
- `Box<T>`
- การใช้ Box กับข้อมูลที่มีขนาดไม่ทราบแน่นอนในเวลาคอมไพล์
- Recursive Data Types เบื้องต้น

### Lesson 02: Reference Counting

- Shared Ownership
- `Rc<T>`
- `Rc::clone()`
- Reference Count
- `Arc<T>`
- ความแตกต่างระหว่าง `Rc<T>` และ `Arc<T>`

### Lesson 03: Interior Mutability

- แนวคิด Interior Mutability
- `Cell<T>`
- `RefCell<T>`
- Runtime Borrow Checking
- `borrow()` และ `borrow_mut()`
- การใช้ `Rc<RefCell<T>>` เบื้องต้น
- ข้อควรระวังเกี่ยวกับ Borrowing

### Lesson 04: Smart Pointer Traits

- `Deref` Trait
- Deref Coercion
- `Drop` Trait
- การกำหนดพฤติกรรมเมื่อข้อมูลถูกทำลาย
- การเลือกใช้ Smart Pointer ให้เหมาะสม

## แบบฝึกหัด

| Exercise | ชื่อโปรเจกต์            | ระดับ       |
| -------- | ----------------------- | ----------- |
| 01       | box_practice            | Easy        |
| 02       | rc_arc_practice         | Easy-Medium |
| 03       | refcell_practice        | Medium      |
| 04       | smart_pointer_challenge | Challenge   |

## โครงสร้างโฟลเดอร์

```text
23_smart_pointers/
├── README.md
├── 01_smart_pointer_basics/
│   └── README.md
├── 02_reference_counting/
│   └── README.md
├── 03_interior_mutability/
│   └── README.md
├── 04_smart_pointer_traits/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── box_practice/
    ├── exercise_02/
    │   ├── README.md
    │   └── rc_arc_practice/
    ├── exercise_03/
    │   ├── README.md
    │   └── refcell_practice/
    └── exercise_04/
        ├── README.md
        └── smart_pointer_challenge/
```

## คำสั่งที่ใช้บ่อย

```powershell
# เข้าโปรเจกต์
cd 23_smart_pointers/exercises/exercise_01/box_practice

# ตรวจสอบโค้ด
cargo check

# รันโปรแกรม
cargo run

# จัดรูปแบบโค้ด
cargo fmt

# ตรวจสอบคำเตือนและแนวทางปรับปรุง
cargo clippy
```

## แนวทางการเรียน

1. อ่านบทเรียนตามลำดับ 01–04
2. ทดลองสร้างโปรแกรมสั้น ๆ เพื่อสังเกตพฤติกรรมของ Smart Pointer
3. ทำแบบฝึกหัดตามลำดับ 01–04
4. ตรวจสอบด้วย `cargo check`, `cargo run` และ `cargo clippy`
5. ทบทวนว่าแต่ละ Smart Pointer แก้ปัญหาอะไร และมีข้อจำกัดอะไร

## ข้อควรระวัง

- อย่าเลือกใช้ Smart Pointer โดยไม่เข้าใจ Ownership และ Borrowing
- `Rc<T>` ไม่ได้ออกแบบมาสำหรับการแชร์ข้าม Threads
- `RefCell<T>` ตรวจสอบกฎ Borrowing ขณะ Runtime และอาจเกิด Panic ได้
- `Arc<T>` ให้ Thread-safe Reference Counting แต่ไม่ได้ทำให้ข้อมูลภายใน Mutate ได้อย่างปลอดภัยโดยอัตโนมัติ
- ควรเริ่มจาก References และ Ownership ปกติก่อน หากยังไม่จำเป็นต้องใช้ Smart Pointer

## Completion Checklist

- [ ] เข้าใจ Stack และ Heap
- [ ] เข้าใจการทำงานของ `Box<T>`
- [ ] ใช้ `Rc<T>` และอธิบาย Reference Counting ได้
- [ ] เข้าใจความแตกต่างระหว่าง `Rc<T>` และ `Arc<T>`
- [ ] ใช้ `Cell<T>` และ `RefCell<T>` ได้ในตัวอย่างพื้นฐาน
- [ ] เข้าใจ Runtime Borrow Checking
- [ ] อธิบาย `Deref` และ `Drop` ได้
- [ ] ทำแบบฝึกหัดครบทั้ง 4 ข้อ

## เกณฑ์ผ่านบทเรียน

ถือว่าผ่านบทนี้เมื่อสามารถเลือก Smart Pointer ให้เหมาะกับโจทย์ อธิบายเหตุผลในการเลือก และเขียนโปรแกรมที่จัดการ Ownership ได้ถูกต้องโดยไม่พึ่งพา `clone()` หรือ Smart Pointer เกินความจำเป็น
