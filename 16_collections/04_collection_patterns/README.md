# Lesson 04: Collection Patterns

## Overview

บทนี้นำ Vector, String และ HashMap มาใช้งานร่วมกัน พร้อมเรียนรู้ Iterator เพื่อประมวลผลข้อมูลโดยไม่จำเป็นต้องเขียน Loop ทุกกรณี

Iterator ช่วยให้เราสามารถแปลง กรอง และรวบรวมข้อมูลได้อย่างเป็นระบบ

## Learning Objectives

- เลือกใช้ Collection ให้เหมาะสมกับงาน
- ใช้ Iterator กับ Collections
- เข้าใจ Lazy Evaluation
- ใช้ `map()` เพื่อแปลงข้อมูล
- ใช้ `filter()` เพื่อกรองข้อมูล
- ใช้ `collect()` เพื่อสร้าง Collection ใหม่
- ใช้ `sum()` เพื่อรวมข้อมูล
- ใช้ Collections หลายชนิดร่วมกัน

## 1. Choosing a Collection

| Collection      | Suitable Use                           |
| --------------- | -------------------------------------- |
| `Vec<T>`        | เก็บรายการข้อมูลที่เรียงตาม Index      |
| `String`        | เก็บข้อความ UTF-8 ที่เป็นเจ้าของข้อมูล |
| `&str`          | อ้างอิงข้อความ                         |
| `HashMap<K, V>` | เก็บข้อมูลแบบ Key-Value                |

ควรพิจารณาลักษณะข้อมูลและวิธีเข้าถึงข้อมูลก่อนเลือก Collection

## 2. What Is an Iterator?

Iterator คือเครื่องมือสำหรับอ่านข้อมูลทีละรายการจาก Collection หรือแหล่งข้อมูลอื่น ๆ

ตัวอย่าง:

```rust
let numbers = vec![10, 20, 30];

let iterator = numbers.iter();
```

Iterator มีเมธอด `next()` สำหรับอ่านสมาชิกถัดไป โดยจะคืนค่า `Option`

## 3. Lazy Evaluation

Iterator Adapter ส่วนใหญ่ เช่น `map()` และ `filter()` จะยังไม่ประมวลผลทันที

ต้องมีการเรียกใช้ Iterator เพื่อให้เกิดการประมวลผลจริง เช่น:

- `collect()`
- `sum()`
- `count()`
- `for` Loop

## 4. Using map()

`map()` ใช้แปลงข้อมูลแต่ละรายการให้เป็นค่าใหม่

ตัวอย่างแนวคิด:

- รับตัวเลขแต่ละตัว
- คูณด้วย 2
- ส่งค่าที่แปลงแล้วออกไปยัง Iterator

```rust
let numbers = vec![1, 2, 3];

let doubled: Vec<i32> =
    numbers.iter().map(|number| number * 2).collect();
```

## 5. Using filter()

`filter()` ใช้เลือกเฉพาะสมาชิกที่ผ่านเงื่อนไข

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let even_numbers: Vec<&i32> = numbers
    .iter()
    .filter(|number| **number % 2 == 0)
    .collect();
```

ผลลัพธ์จะมีเฉพาะตัวเลขคู่

หมายเหตุ: `iter()` ให้สมาชิกเป็น Reference และ `filter()` รับ Reference ของ Item ที่ Iterator ส่งออกมา จึงอาจต้องใช้ `**number` ในตัวอย่างนี้

## 6. Using collect()

`collect()` ใช้รวบรวมข้อมูลจาก Iterator ให้เป็น Collection ใหม่

ตัวอย่าง:

```rust
let numbers = vec![1, 2, 3];

let doubled: Vec<i32> =
    numbers.iter().map(|number| number * 2).copied().collect();
```

ชนิดของตัวแปรปลายทางหรือ Type Annotation ช่วยให้ Rust ทราบว่าจะสร้าง Collection ชนิดใด

## 7. Using sum()

`sum()` ใช้รวมค่าจาก Iterator

```rust
let numbers = vec![10, 20, 30];

let total: i32 = numbers.iter().sum();
```

## 8. Combining Iterator Methods

สามารถนำ Iterator Adapter หลายตัวมาต่อกันได้ เช่น:

1. อ่านข้อมูลจาก Vector
2. กรองข้อมูลด้วย `filter()`
3. แปลงข้อมูลด้วย `map()`
4. รวบรวมผลลัพธ์ด้วย `collect()`

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let results: Vec<i32> = numbers
    .iter()
    .filter(|number| **number > 3)
    .map(|number| number * 10)
    .collect();
```

## 9. Working with Multiple Collections

ตัวอย่างงานที่ใช้หลาย Collection ร่วมกัน:

- ใช้ `Vec<String>` เก็บรายชื่อ
- ใช้ `HashMap<String, u32>` เก็บคะแนน
- ใช้ Iterator เพื่ออ่านรายชื่อ
- ค้นหาคะแนนจาก HashMap
- คำนวณคะแนนรวมหรือค่าเฉลี่ย

ควรตรวจสอบกรณีที่ค้นหา Key ไม่พบ โดย `HashMap::get()` คืนค่า `Option<&V>`

## 10. Useful Iterator Methods

| Method        | Description                               |
| ------------- | ----------------------------------------- |
| `iter()`      | อ่านข้อมูลผ่าน Immutable Reference        |
| `iter_mut()`  | อ่านข้อมูลผ่าน Mutable Reference          |
| `into_iter()` | สร้าง Iterator ที่อาจย้าย Ownership       |
| `map()`       | แปลงข้อมูล                                |
| `filter()`    | กรองข้อมูล                                |
| `collect()`   | รวบรวมผลลัพธ์                             |
| `sum()`       | รวมค่าตัวเลข                              |
| `count()`     | นับจำนวนสมาชิก                            |
| `find()`      | ค้นหาสมาชิกแรกที่ตรงเงื่อนไข              |
| `any()`       | ตรวจสอบว่ามีสมาชิกที่ผ่านเงื่อนไขหรือไม่  |
| `all()`       | ตรวจสอบว่าสมาชิกทุกตัวผ่านเงื่อนไขหรือไม่ |
| `enumerate()` | เพิ่ม Index ให้สมาชิก                     |
| `fold()`      | สะสมผลลัพธ์จากสมาชิก                      |

## Practice Checklist

- [ ] เข้าใจ Iterator
- [ ] เข้าใจ Lazy Evaluation
- [ ] ใช้ `iter()` ได้
- [ ] เข้าใจความแตกต่างระหว่าง `iter()`, `iter_mut()` และ `into_iter()`
- [ ] ใช้ `map()` แปลงข้อมูลได้
- [ ] ใช้ `filter()` กรองข้อมูลได้
- [ ] ใช้ `collect()` สร้าง Collection ได้
- [ ] ใช้ `sum()` รวมข้อมูลได้
- [ ] ใช้ Iterator หลายเมธอดต่อกันได้
- [ ] ใช้ Vector และ HashMap ร่วมกันได้
- [ ] เข้าใจ Ownership และ Borrowing ขณะใช้ Iterator

## Next Chapter

Chapter 17: Traits
