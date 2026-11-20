# Exercise 04: Text Processor

## วัตถุประสงค์

ฝึกประยุกต์ใช้ String, String Methods, Function และ String Formatting เพื่อสร้างโปรแกรมจัดการข้อความ

## โจทย์

สร้างโปรแกรมจัดการข้อความ (Text Processor) โดยกำหนดข้อความเริ่มต้น แล้วนำข้อความไปผ่านขั้นตอนต่าง ๆ ตามเงื่อนไข

## ข้อมูลเริ่มต้น

กำหนดข้อความ:

```rust
let message = String::from("  Welcome to Rust Programming  ");
```

## เงื่อนไข

### ส่วนที่ 1: แสดงข้อความต้นฉบับ

1. สร้างตัวแปร `message`
2. แสดงข้อความต้นฉบับ
3. ใช้ `len()` แสดงจำนวนไบต์ของข้อความ

### ส่วนที่ 2: ทำความสะอาดข้อความ

1. ใช้ `trim()` ตัดช่องว่างบริเวณต้นและท้าย
2. เก็บผลลัพธ์ไว้ในตัวแปรใหม่
3. แสดงข้อความหลังตัดช่องว่าง

### ส่วนที่ 3: ค้นหาคำในข้อความ

1. ใช้ `contains()` ตรวจสอบคำว่า `Rust`
2. ใช้ `contains()` ตรวจสอบคำว่า `Python`
3. แสดงผลการตรวจสอบทั้งสองคำ

### ส่วนที่ 4: แปลงข้อความ

1. ใช้ `to_uppercase()` แปลงข้อความเป็นตัวพิมพ์ใหญ่
2. ใช้ `to_lowercase()` แปลงข้อความเป็นตัวพิมพ์เล็ก
3. แสดงผลลัพธ์ทั้งสองแบบ

### ส่วนที่ 5: แทนที่ข้อความ

1. ใช้ `replace()` เปลี่ยนคำว่า `Rust` เป็น `Python`
2. แสดงข้อความหลังแทนที่
3. ตรวจสอบว่าข้อความเดิมยังคงอยู่หรือไม่

### ส่วนที่ 6: สร้าง Function

สร้าง Function ต่อไปนี้:

| Function         | หน้าที่                                              |
| ---------------- | ---------------------------------------------------- |
| `show_text`      | รับข้อความและแสดงผล                                  |
| `count_bytes`    | รับข้อความและส่งจำนวนไบต์กลับ                        |
| `contains_word`  | รับข้อความและคำค้นหา แล้วส่งค่า `bool` กลับ          |
| `format_message` | รับชื่อและข้อความ แล้วส่งข้อความที่จัดรูปแบบแล้วกลับ |

ข้อกำหนด:

- กำหนด Parameters ให้เหมาะสม
- ระบุชนิดข้อมูลของ Parameters
- กำหนด Return Type ให้ถูกต้อง
- นำ Function มาเรียกใช้งานภายใน `main()`

## ตัวอย่างผลลัพธ์

```text
Original:   Welcome to Rust Programming
Length: 33

Trimmed: Welcome to Rust Programming

Contains Rust: true
Contains Python: false

Uppercase:   WELCOME TO RUST PROGRAMMING
Lowercase:   welcome to rust programming

Replaced:   Welcome to Python Programming

Message: Learning Rust - Welcome to Rust Programming
```

หมายเหตุ: จำนวนไบต์ขึ้นอยู่กับข้อความต้นฉบับที่กำหนด

## โจทย์เพิ่มเติม

- ทดลองเปลี่ยนข้อความต้นฉบับ
- ทดลองค้นหาคำที่ไม่มีอยู่ในข้อความ
- ทดลองใช้ข้อความภาษาไทย
- สร้าง Function เพิ่มเติมสำหรับนับจำนวน Unicode Scalar Values
- ทดลองใช้ `format!()` เพื่อสร้างข้อความหลายรูปแบบ

## เกณฑ์การตรวจสอบ

- [ ] สร้างและจัดเก็บ String ได้
- [ ] ใช้ `trim()` ได้
- [ ] ใช้ `len()` ได้
- [ ] ใช้ `contains()` ได้
- [ ] ใช้ `replace()` ได้
- [ ] ใช้ `to_uppercase()` และ `to_lowercase()` ได้
- [ ] สร้าง Function ที่รับ Parameters ได้
- [ ] สร้าง Function ที่ส่ง Return Value ได้
- [ ] ใช้ `format!()` ได้
- [ ] โปรแกรมทำงานได้โดยไม่มี Compile Error

## วิธีรันโปรแกรม

```powershell
cargo check
cargo run
```
