# Lesson 04: Tuple Structs and Unit-Like Structs

## 1. Overview

นอกจาก Struct แบบ Named Fields ที่เรียนมาก่อนหน้านี้แล้ว Rust ยังมี Struct รูปแบบอื่น ได้แก่:

1. Tuple Structs
2. Unit-Like Structs

แต่ละรูปแบบเหมาะกับการจัดเก็บข้อมูลที่มีลักษณะแตกต่างกัน

## 2. Tuple Structs

Tuple Struct คือ Struct ที่ใช้รูปแบบคล้าย Tuple โดยไม่มีชื่อ Field แต่ละตำแหน่ง

ตัวอย่าง:

```rust
struct Color(u8, u8, u8);
```

Struct `Color` มีข้อมูล 3 ตำแหน่ง:

- ตำแหน่งที่ 0 เก็บค่าสีแดง
- ตำแหน่งที่ 1 เก็บค่าสีเขียว
- ตำแหน่งที่ 2 เก็บค่าสีน้ำเงิน

## 3. Creating Tuple Struct Instances

ตัวอย่างการสร้าง Instance:

```rust
let black = Color(0, 0, 0);
let white = Color(255, 255, 255);
```

เข้าถึงข้อมูลด้วย Index:

```rust
println!("Red: {}", black.0);
println!("Green: {}", black.1);
println!("Blue: {}", black.2);
```

ข้อควรจำ:

- Index เริ่มต้นที่ 0
- ต้องใช้ `.` ตามด้วย Index
- แต่ละตำแหน่งมี Data Type ตามที่ประกาศไว้

## 4. Unit-Like Structs

Unit-Like Struct คือ Struct ที่ไม่มี Fields เลย

ตัวอย่าง:

```rust
struct Marker;
```

การสร้าง Instance:

```rust
let marker = Marker;
```

Unit-Like Struct เหมาะกับการสร้างชนิดข้อมูลที่ไม่จำเป็นต้องเก็บข้อมูลภายใน แต่ยังต้องการมีชนิดข้อมูลของตัวเอง

ตัวอย่างการใช้งานเพิ่มเติมสามารถพบได้ในงานที่เกี่ยวข้องกับ Traits และการกำหนดพฤติกรรมของชนิดข้อมูล

## 5. Comparing Struct Types

| Type               | Structure                      | Field Access |
| ------------------ | ------------------------------ | ------------ |
| Named-Field Struct | `struct User { name: String }` | `user.name`  |
| Tuple Struct       | `struct Color(u8, u8, u8);`    | `color.0`    |
| Unit-Like Struct   | `struct Marker;`               | ไม่มี Fields |

## 6. Important Notes

- Tuple Struct มี Fields ตามตำแหน่ง
- Tuple Struct ใช้ Index เช่น `.0`, `.1`
- Unit-Like Struct ไม่มี Fields
- Struct ทั้งสามรูปแบบเป็น Custom Data Types
- Tuple Struct แต่ละชนิดยังถือเป็นคนละชนิดข้อมูล แม้จะมี Fields เหมือนกันก็ตาม

## 7. Knowledge Check

1. Tuple Struct แตกต่างจาก Named-Field Struct อย่างไร?
2. Tuple Struct ใช้เครื่องหมายใดเข้าถึงข้อมูล?
3. Index แรกของ Tuple Struct คือหมายเลขใด?
4. Unit-Like Struct มี Fields หรือไม่?
5. Tuple Struct เหมาะกับข้อมูลลักษณะใด?

## 8. Completion Checklist

- [ ] ประกาศ Tuple Struct ได้
- [ ] สร้าง Tuple Struct Instance ได้
- [ ] เข้าถึงข้อมูลด้วย `.0`, `.1`, `.2` ได้
- [ ] ประกาศ Unit-Like Struct ได้
- [ ] สร้าง Unit-Like Struct Instance ได้
- [ ] อธิบายความแตกต่างของ Struct ทั้ง 3 รูปแบบได้
