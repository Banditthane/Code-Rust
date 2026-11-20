# Lesson 01: Vectors

## Overview

Vector คือ Collection ที่ใช้จัดเก็บข้อมูลหลายรายการชนิดเดียวกัน โดยสามารถเพิ่มหรือลบข้อมูลระหว่างการทำงานได้

ชนิดข้อมูลของ Vector คือ `Vec<T>` โดย `T` หมายถึงชนิดข้อมูลของสมาชิกภายใน Vector

## Learning Objectives

- เข้าใจการสร้าง Vector
- เพิ่มและลบข้อมูลใน Vector
- อ่านและแก้ไขสมาชิก
- ค้นหาและกรองข้อมูล
- เรียงลำดับข้อมูล
- ใช้ Iterator เพื่อประมวลผลข้อมูล

## 1. Creating a Vector

สามารถสร้าง Vector ได้หลายวิธี

- `Vec::new()` สร้าง Vector ว่าง
- `vec![]` สร้าง Vector พร้อมข้อมูลเริ่มต้น

ตัวอย่าง:

```rust
let numbers: Vec<i32> = Vec::new();

let scores = vec![70, 80, 90];
```

## 2. Adding Elements

ใช้ `push()` เพื่อเพิ่มข้อมูลต่อท้าย Vector

```rust
let mut numbers = vec![10, 20];

numbers.push(30);
```

ต้องประกาศ Vector เป็น `mut` ก่อนจึงจะเปลี่ยนแปลงข้อมูลได้

## 3. Removing Elements

เมธอดที่ใช้ลบข้อมูล:

- `pop()` ลบและคืนค่าสมาชิกตัวสุดท้ายเป็น `Option<T>`
- `remove(index)` ลบสมาชิกตาม Index และคืนค่าสมาชิกที่ถูกลบ
- `clear()` ลบสมาชิกทั้งหมด

ข้อควรระวัง: `remove(index)` จะ Panic หาก Index อยู่นอกขอบเขต

## 4. Accessing Elements

มีวิธีเข้าถึงข้อมูลหลัก ๆ สองแบบ

### Index

```rust
let numbers = vec![10, 20, 30];

println!("{}", numbers[0]);
```

หาก Index ไม่มีอยู่ โปรแกรมจะ Panic

### get()

```rust
let numbers = vec![10, 20, 30];

println!("{:?}", numbers.get(1));
```

`get()` คืนค่า `Option<&T>` จึงสามารถจัดการกรณีที่ไม่พบ Index ได้อย่างปลอดภัยกว่า

## 5. Updating Elements

ใช้ Index เพื่อแก้ไขสมาชิก โดย Vector ต้องเป็น Mutable

```rust
let mut numbers = vec![10, 20, 30];

numbers[1] = 99;
```

## 6. Searching Elements

เมธอดที่ใช้ค้นหา:

- `contains(&value)` ตรวจสอบว่ามีข้อมูลหรือไม่
- `iter().find(...)` ค้นหาสมาชิกที่ตรงกับเงื่อนไข
- `position(...)` ค้นหา Index ของสมาชิกที่ตรงกับเงื่อนไข

ตัวอย่าง:

```rust
let numbers = vec![10, 20, 30];

println!("{}", numbers.contains(&20));
```

## 7. Sorting Elements

ใช้ `sort()` เพื่อเรียงลำดับ Vector

```rust
let mut numbers = vec![30, 10, 20];

numbers.sort();
```

สำหรับข้อมูลตัวเลข สามารถใช้ `sort_unstable()` ได้เช่นกัน โดยไม่รับประกันการรักษาลำดับเดิมของสมาชิกที่มีค่าเท่ากัน

## 8. Iterating Over a Vector

### Borrow Elements

```rust
let numbers = vec![10, 20, 30];

for number in &numbers {
    println!("{number}");
}
```

การวนซ้ำผ่าน `&numbers` เป็นการยืมข้อมูล ไม่ได้ย้าย Ownership ของ Vector

### Mutable Borrow

ใช้ `iter_mut()` เมื่อต้องการแก้ไขสมาชิกผ่าน Iterator

### Ownership

ใช้ `into_iter()` เมื่อต้องการนำสมาชิกออกมาเป็นค่า โดยอาจย้าย Ownership ของสมาชิกออกจาก Vector

## 9. Retain Elements

ใช้ `retain()` เพื่อเก็บเฉพาะสมาชิกที่ผ่านเงื่อนไข

```rust
let mut numbers = vec![10, 15, 20, 25, 30];

numbers.retain(|number| number % 2 == 0);
```

## 10. Useful Methods

| Method       | Description                   |
| ------------ | ----------------------------- |
| `len()`      | จำนวนสมาชิก                   |
| `is_empty()` | ตรวจสอบว่า Vector ว่างหรือไม่ |
| `push()`     | เพิ่มสมาชิก                   |
| `pop()`      | ลบสมาชิกตัวสุดท้าย            |
| `remove()`   | ลบสมาชิกตาม Index             |
| `clear()`    | ลบสมาชิกทั้งหมด               |
| `contains()` | ตรวจสอบสมาชิก                 |
| `sort()`     | เรียงลำดับ                    |
| `retain()`   | เก็บสมาชิกตามเงื่อนไข         |
| `get()`      | เข้าถึงสมาชิกแบบ Option       |
| `first()`    | สมาชิกตัวแรกแบบ Option        |
| `last()`     | สมาชิกตัวสุดท้ายแบบ Option    |

## Practice Checklist

- [ ] สร้าง Vector ว่างได้
- [ ] สร้าง Vector พร้อมข้อมูลได้
- [ ] เพิ่มข้อมูลด้วย `push()`
- [ ] ลบข้อมูลด้วย `pop()` และ `remove()`
- [ ] เข้าถึงข้อมูลด้วย Index และ `get()`
- [ ] แก้ไขสมาชิกใน Vector ได้
- [ ] ค้นหาข้อมูลด้วย `contains()`
- [ ] เรียงลำดับด้วย `sort()`
- [ ] วนซ้ำด้วย `for` และ `iter()`
- [ ] แก้ไขข้อมูลด้วย `iter_mut()`
- [ ] กรองข้อมูลด้วย `retain()`

## Next Lesson

Lesson 02: Strings
