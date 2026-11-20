# Lesson 02: Strings

## Overview

String เป็นชนิดข้อมูลสำหรับจัดเก็บข้อความใน Rust โดยข้อความใช้การเข้ารหัส UTF-8

Rust มีชนิดข้อมูลข้อความที่ควรรู้จักสองชนิด ได้แก่:

- `String` เป็นข้อความที่มี Ownership และสามารถเปลี่ยนแปลงได้เมื่อประกาศเป็น `mut`
- `&str` เป็น String Slice หรือการอ้างอิงไปยังข้อความ

## Learning Objectives

- เข้าใจความแตกต่างระหว่าง `String` และ `&str`
- สร้าง String ได้หลายวิธี
- ต่อและแก้ไขข้อความ
- ค้นหาและแทนที่ข้อความ
- แยกข้อความออกเป็นส่วน ๆ
- เข้าใจการจัดการข้อความ UTF-8

## 1. String vs &str

| Type     | Description                               |
| -------- | ----------------------------------------- |
| `String` | ข้อความที่เป็นเจ้าของข้อมูลและขยายขนาดได้ |
| `&str`   | การอ้างอิงไปยังข้อความ                    |

ตัวอย่าง:

```rust
let name: &str = "Rust";

let language = String::from("Rust");
```

## 2. Creating Strings

วิธีสร้าง String:

```rust
let first = String::new();

let second = String::from("Hello");

let third = "Rust".to_string();
```

## 3. Appending Text

### push_str()

เพิ่มข้อความจาก `&str`

```rust
let mut message = String::from("Hello");

message.push_str(" Rust");
```

### push()

เพิ่มตัวอักษรชนิด `char`

```rust
let mut message = String::from("Rust");

message.push('!');
```

## 4. Concatenation

สามารถต่อ String ด้วยเครื่องหมาย `+` ได้

```rust
let first = String::from("Hello");

let second = String::from(" Rust");

let result = first + &second;
```

ข้อควรจำ: การใช้ `+` กับ String จะรับ Ownership ของ String ตัวซ้าย และยืมข้อมูลของ String ตัวขวา

หากต้องการต่อข้อความโดยไม่ย้าย Ownership ของตัวแปรต้นทาง สามารถใช้ `format!()`

## 5. Formatting Strings

ใช้ `format!()` เพื่อสร้าง String ใหม่จากข้อมูลหลายส่วน

```rust
let name = "Alice";

let age = 20;

let message = format!("{name} is {age} years old.");
```

## 6. Searching Strings

เมธอดที่ใช้ค้นหา:

- `contains()` ตรวจสอบว่ามีข้อความหรือไม่
- `starts_with()` ตรวจสอบข้อความเริ่มต้น
- `ends_with()` ตรวจสอบข้อความลงท้าย
- `find()` ค้นหาตำแหน่งเริ่มต้นของข้อความและคืนค่า `Option<usize>`

ตัวอย่าง:

```rust
let message = String::from("Hello Rust");

println!("{}", message.contains("Rust"));
```

## 7. Replacing Text

ใช้ `replace()` เพื่อสร้าง String ใหม่ที่แทนที่ข้อความตามที่กำหนด

```rust
let message = "Hello Rust";

let result = message.replace("Rust", "World");
```

`replace()` ไม่ได้แก้ไข String ต้นฉบับ แต่คืนค่า String ใหม่

## 8. Trimming Text

ใช้ `trim()` เพื่อตัดช่องว่างและอักขระช่องว่างบริเวณหัวและท้ายข้อความ

```rust
let message = "  Hello Rust  ";

let result = message.trim();
```

## 9. Splitting Text

ใช้ `split()` เพื่อแบ่งข้อความตามตัวคั่น

```rust
let data = "Apple,Banana,Orange";

for item in data.split(',') {
    println!("{item}");
}
```

ใช้ `lines()` เพื่อวนอ่านข้อความทีละบรรทัด

## 10. UTF-8 and String Length

ข้อความใน Rust ใช้ UTF-8 จึงต้องเข้าใจความแตกต่างระหว่าง:

- จำนวนไบต์: `len()`
- จำนวน Unicode Scalar Values: `chars().count()`
- จำนวน Grapheme Clusters: ต้องใช้การจัดการเพิ่มเติม เช่น Crate ภายนอก

ตัวอย่าง:

```rust
let text = "Hello";

println!("{}", text.len());

println!("{}", text.chars().count());
```

สำหรับข้อความภาษาไทยหรือ Emoji จำนวนไบต์และจำนวนตัวอักษรอาจไม่เท่ากัน

ไม่สามารถเข้าถึง String ด้วย Index เช่น `text[0]` ได้ เพราะ UTF-8 ใช้จำนวนไบต์ต่ออักขระไม่เท่ากัน

## 11. Iterating Over Characters

ใช้ `chars()` เพื่อวนซ้ำตัวอักษรชนิด `char`

```rust
let text = "Rust";

for character in text.chars() {
    println!("{character}");
}
```

## Useful Methods

| Method           | Description                     |
| ---------------- | ------------------------------- |
| `len()`          | จำนวนไบต์                       |
| `is_empty()`     | ตรวจสอบข้อความว่าง              |
| `push()`         | เพิ่ม char                      |
| `push_str()`     | เพิ่มข้อความ                    |
| `contains()`     | ค้นหาข้อความ                    |
| `replace()`      | แทนที่ข้อความ                   |
| `trim()`         | ตัดช่องว่างหัวและท้าย           |
| `split()`        | แบ่งข้อความ                     |
| `lines()`        | แยกตามบรรทัด                    |
| `chars()`        | วนซ้ำ Unicode Scalar Values     |
| `to_uppercase()` | แปลงเป็นตัวพิมพ์ใหญ่ตาม Unicode |
| `to_lowercase()` | แปลงเป็นตัวพิมพ์เล็กตาม Unicode |

## Practice Checklist

- [ ] เข้าใจ `String` และ `&str`
- [ ] สร้าง String ได้
- [ ] เพิ่มข้อความด้วย `push_str()`
- [ ] เพิ่มตัวอักษรด้วย `push()`
- [ ] ต่อข้อความด้วย `+`
- [ ] ใช้ `format!()` ได้
- [ ] ค้นหาด้วย `contains()` และ `find()`
- [ ] แทนที่ข้อความด้วย `replace()`
- [ ] ตัดช่องว่างด้วย `trim()`
- [ ] แบ่งข้อความด้วย `split()`
- [ ] วนซ้ำตัวอักษรด้วย `chars()`
- [ ] เข้าใจความแตกต่างระหว่างจำนวนไบต์กับจำนวนตัวอักษร

## Next Lesson

Lesson 03: Hash Maps
