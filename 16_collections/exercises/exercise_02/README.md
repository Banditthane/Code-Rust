# Exercise 02: String Processor

## Objective

สร้างโปรแกรมประมวลผลข้อความด้วย `String` และ `&str` เพื่อฝึกการต่อข้อความ ค้นหา แทนที่ แยกข้อความ และนับจำนวนข้อมูล

## Project

Project Name: `string_processor`

## Requirements

กำหนดข้อความเริ่มต้น:

```text
"  Rust is fun and Rust is powerful  "
```

โปรแกรมต้องสามารถ:

1. สร้างข้อความด้วย `String`
2. ตัดช่องว่างหัวและท้ายด้วย `trim()`
3. แสดงข้อความหลังตัดช่องว่าง
4. ตรวจสอบว่าข้อความมีคำว่า `Rust` หรือไม่ด้วย `contains()`
5. นับจำนวนครั้งที่คำว่า `Rust` ปรากฏ โดยใช้ `matches()` และ `count()`
6. แทนที่คำว่า `Rust` ด้วย `Programming` ด้วย `replace()`
7. แยกข้อความตามช่องว่างด้วย `split_whitespace()`
8. แสดงคำแต่ละคำ
9. นับจำนวนคำ
10. นับจำนวน Unicode Scalar Values ด้วย `chars().count()`

## Constraints

- ต้องใช้ `String`
- ต้องใช้ `trim()`
- ต้องใช้ `contains()`
- ต้องใช้ `matches()` และ `count()`
- ต้องใช้ `replace()`
- ต้องใช้ `split_whitespace()`
- ต้องใช้ `chars().count()`
- ห้ามใช้ Index เช่น `text[0]` เพื่อเข้าถึง String
- ต้องเก็บข้อความที่ผ่านการประมวลผลไว้ในตัวแปรเพื่อแสดงผล

## Expected Output

```text
Original:   Rust is fun and Rust is powerful
Trimmed: Rust is fun and Rust is powerful
Contains Rust: true
Rust Count: 2
Replaced: Programming is fun and Programming is powerful
Words:
Rust
is
fun
and
Rust
is
powerful
Word Count: 7
Character Count: 34
```

หมายเหตุ: `Character Count` ในโจทย์นี้นับ Unicode Scalar Values ด้วย `chars().count()` ไม่ใช่จำนวน Grapheme Clusters

## Hints

- `trim()` คืนค่า String Slice ที่อ้างอิงข้อความเดิม
- `replace()` คืนค่า String ใหม่
- `split_whitespace()` แยกข้อความโดยใช้ช่องว่างต่อเนื่องเป็นตัวคั่น
- `matches("Rust").count()` ใช้นับจำนวนครั้งที่พบ Pattern
- `chars().count()` นับ Unicode Scalar Values

## Completion Checklist

- [ ] สร้าง String ได้
- [ ] ตัดช่องว่างด้วย `trim()`
- [ ] ค้นหาด้วย `contains()`
- [ ] นับคำที่พบด้วย `matches()` และ `count()`
- [ ] แทนที่ข้อความด้วย `replace()`
- [ ] แยกข้อความด้วย `split_whitespace()`
- [ ] นับจำนวนคำได้
- [ ] นับจำนวน Unicode Scalar Values ได้
- [ ] แสดงผลลัพธ์ตรงตาม Expected Output

## Run Project

```powershell
cd 16_collections/exercises/exercise_02/string_processor
cargo run
```

## Challenge

เพิ่มเติมความสามารถ:

- รับข้อความจากผู้ใช้
- แปลงข้อความด้วย `to_uppercase()` และ `to_lowercase()`
- นับจำนวนตัวเลขและตัวอักษร
- ทดลองข้อความภาษาไทยและ Emoji เพื่อเปรียบเทียบ `len()` กับ `chars().count()`
