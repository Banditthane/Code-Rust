# Lesson 03: Hash Maps

## Overview

HashMap คือ Collection ที่จัดเก็บข้อมูลในรูปแบบ Key-Value โดยใช้ Key เพื่อค้นหา Value ที่สัมพันธ์กัน

ชนิดข้อมูลคือ `HashMap<K, V>`

- `K` คือชนิดข้อมูลของ Key
- `V` คือชนิดข้อมูลของ Value

Key ต้องรองรับ Trait `Eq` และ `Hash` เพื่อใช้เป็น Key ใน HashMap

## Learning Objectives

- สร้าง HashMap
- เพิ่มและแก้ไขข้อมูล
- ค้นหาข้อมูลด้วย Key
- ลบข้อมูล
- ตรวจสอบ Key
- ใช้ `entry()` และ `or_insert()`
- วนซ้ำข้อมูลใน HashMap
- นับและจัดกลุ่มข้อมูล

## 1. Import HashMap

HashMap อยู่ใน Standard Library ภายใต้ `std::collections`

```rust
use std::collections::HashMap;
```

## 2. Creating a HashMap

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
```

หรือสร้างพร้อมข้อมูลเริ่มต้น:

```rust
let mut scores = HashMap::from([
    ("Alice", 90),
    ("Bob", 80),
]);
```

## 3. Inserting Data

ใช้ `insert()` เพื่อเพิ่มข้อมูลหรือแทนที่ Value ของ Key ที่มีอยู่แล้ว

```rust
scores.insert("Alice", 95);
scores.insert("Bob", 80);
```

หาก Key ซ้ำ ค่าเดิมจะถูกแทนที่ และ `insert()` คืนค่า `Option<V>` ซึ่งเป็นค่าเดิมหากมีอยู่

## 4. Accessing Data

ใช้ `get()` เพื่อค้นหา Value จาก Key

```rust
let score = scores.get("Alice");

println!("{:?}", score);
```

`get()` คืนค่า `Option<&V>`

สามารถใช้ `unwrap_or()` เพื่อกำหนดค่าเริ่มต้นเมื่อไม่พบ Key

```rust
let score = scores.get("Charlie").unwrap_or(&0);
```

## 5. Updating Data

ใช้ `insert()` เพื่อแทนที่ค่าเดิม

หากต้องการเพิ่มค่าจากข้อมูลเดิม สามารถใช้ `entry()` ร่วมกับ `or_insert()`

```rust
let mut counts = HashMap::new();

let count = counts.entry("Rust").or_insert(0);

*count += 1;
```

`or_insert()` จะคืน Mutable Reference ไปยัง Value ของ Key นั้น

## 6. Removing Data

ใช้ `remove()` เพื่อลบข้อมูลตาม Key

```rust
scores.remove("Bob");
```

เมธอดนี้คืนค่า `Option<V>` ของ Value ที่ถูกลบ

## 7. Checking Keys

ใช้ `contains_key()` เพื่อตรวจสอบว่า Key มีอยู่หรือไม่

```rust
println!("{}", scores.contains_key("Alice"));
```

## 8. Iterating Over HashMap

สามารถวนซ้ำ Key และ Value ได้ด้วย `iter()`

```rust
for (name, score) in &scores {
    println!("{name}: {score}");
}
```

HashMap ไม่รับประกันลำดับการวนซ้ำ ดังนั้นไม่ควรคาดหวังว่าข้อมูลจะแสดงตามลำดับที่เพิ่มเข้าไป

## 9. Counting Frequency

HashMap สามารถใช้เก็บจำนวนครั้งที่ข้อมูลแต่ละรายการปรากฏได้

แนวคิด:

1. อ่านข้อมูลทีละรายการ
2. ใช้ข้อมูลนั้นเป็น Key
3. เพิ่มจำนวนใน Value
4. หากยังไม่มี Key ให้เริ่มต้นจาก 0

สามารถใช้ `entry()` และ `or_insert()` เพื่อทำงานนี้ได้

## Useful Methods

| Method           | Description                    |
| ---------------- | ------------------------------ |
| `new()`          | สร้าง HashMap                  |
| `insert()`       | เพิ่มหรือแทนที่ข้อมูล          |
| `get()`          | ค้นหา Value                    |
| `remove()`       | ลบข้อมูล                       |
| `contains_key()` | ตรวจสอบ Key                    |
| `entry()`        | เข้าถึง Entry ของ Key          |
| `or_insert()`    | กำหนดค่าเริ่มต้นหากยังไม่มี    |
| `len()`          | จำนวนคู่ Key-Value             |
| `is_empty()`     | ตรวจสอบว่า HashMap ว่างหรือไม่ |
| `clear()`        | ลบข้อมูลทั้งหมด                |
| `keys()`         | วนซ้ำ Key                      |
| `values()`       | วนซ้ำ Value                    |
| `iter()`         | วนซ้ำ Key และ Value            |

## Practice Checklist

- [ ] Import HashMap ได้
- [ ] สร้าง HashMap ได้
- [ ] เพิ่มข้อมูลด้วย `insert()`
- [ ] ค้นหาข้อมูลด้วย `get()`
- [ ] แก้ไขข้อมูลด้วย `insert()`
- [ ] ใช้ `entry()` และ `or_insert()` ได้
- [ ] ลบข้อมูลด้วย `remove()`
- [ ] ตรวจสอบ Key ด้วย `contains_key()`
- [ ] วนซ้ำ Key และ Value ได้
- [ ] นับความถี่ของข้อมูลได้
- [ ] เข้าใจว่า HashMap ไม่รับประกันลำดับข้อมูล

## Next Lesson

Lesson 04: Collection Patterns
