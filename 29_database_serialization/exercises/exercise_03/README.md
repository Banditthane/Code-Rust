# Exercise 03: Serde Practice

## ระดับ

Medium

## Objective

ฝึกใช้ Serde เพื่อ Serialize และ Deserialize ข้อมูล Rust Struct กับ JSON และ TOML

## Problem Statement

สร้างโปรแกรมจัดการข้อมูลนักเรียน (Student Data Manager)

โปรแกรมต้องสามารถ:

1. สร้าง Student Struct
2. Serialize Student เป็น JSON
3. Deserialize JSON เป็น Student
4. Serialize Student เป็น TOML
5. Deserialize TOML เป็น Student
6. อ่านและเขียนไฟล์ข้อมูล
7. จัดการ Serialization Errors

## Data Model

กำหนด Student:

| Field    | Rust Type      |
| -------- | -------------- |
| id       | u32            |
| name     | String         |
| age      | u8             |
| email    | String         |
| active   | bool           |
| nickname | Option<String> |

## Requirements

### Part 1: Student Struct

สร้าง Struct:

```text
Student
```

กำหนด Fields ตามตาราง

ต้องสามารถ Serialize และ Deserialize ได้

### Part 2: JSON Serialization

สร้าง Student ตัวอย่าง:

```text
ID: 1
Name: Alice
Age: 20
Email: alice@example.com
Active: true
Nickname: Ali
```

Serialize เป็น JSON

ต้องรองรับ Pretty JSON เพื่อให้อ่านง่าย

### Part 3: JSON Deserialization

กำหนด JSON Input:

```json
{
  "id": 1,
  "name": "Alice",
  "age": 20,
  "email": "alice@example.com",
  "active": true,
  "nickname": "Ali"
}
```

Deserialize เป็น Student Struct

แสดงข้อมูล Student หลัง Deserialize

### Part 4: Serde Attributes

ทดลองใช้ Attributes อย่างน้อย 3 รายการ:

- `rename`
- `rename_all`
- `default`
- `skip_serializing`
- `skip_serializing_if`

เลือก Attributes ให้เหมาะกับข้อมูลที่ต้องการแปลง

อธิบายผลของแต่ละ Attribute ใน README หรือ Comment

### Part 5: TOML Serialization

สร้างข้อมูล Configuration หรือ Student แล้ว Serialize เป็น TOML

ตัวอย่างรูปแบบ:

```toml
id = 1
name = "Alice"
age = 20
email = "alice@example.com"
active = true
```

### Part 6: File Handling

สร้างไฟล์:

```text
student.json
student.toml
```

ความสามารถ:

1. เขียน JSON ลงไฟล์
2. อ่าน JSON จากไฟล์
3. เขียน TOML ลงไฟล์
4. อ่าน TOML จากไฟล์
5. แปลงข้อมูลกลับเป็น Student Struct

### Part 7: Error Handling

จัดการกรณี:

- JSON Syntax ผิด
- TOML Syntax ผิด
- Field ที่จำเป็นหายไป
- ชนิดข้อมูลไม่ตรงกัน
- อ่านไฟล์ไม่สำเร็จ
- เขียนไฟล์ไม่สำเร็จ

## Constraints

- ต้องใช้ Serde
- ต้องใช้ `serde_json`
- ต้องใช้ `toml`
- ต้องใช้ `Serialize`
- ต้องใช้ `Deserialize`
- ต้องใช้ Rust Struct
- ต้องใช้ JSON
- ต้องใช้ TOML
- ต้องอ่านและเขียนไฟล์ได้
- ต้องจัดการ Error ด้วย `Result`
- ห้ามสมมติว่า Input ถูกต้องเสมอ
- ห้ามใช้ `unwrap()` กับ File และ Serialization Operations โดยไม่มีการจัดการ

## Expected Output

```text
Serde Practice

Creating Student...
Student created!

JSON Serialization
--------------------------------
{
  "id": 1,
  "name": "Alice",
  "age": 20,
  "email": "alice@example.com",
  "active": true,
  "nickname": "Ali"
}
--------------------------------

JSON Deserialization
Student:
ID: 1
Name: Alice
Age: 20
Email: alice@example.com
Active: true
Nickname: Ali

JSON file saved: student.json

TOML Serialization
--------------------------------
id = 1
name = "Alice"
age = 20
email = "alice@example.com"
active = true
--------------------------------

TOML file saved: student.toml

Serialization completed.
```

ผลลัพธ์จริงอาจมีรูปแบบแตกต่างตาม Serde Attributes ที่เลือก

## Hints

ติดตั้ง Dependencies:

```powershell
cargo add serde --features derive
cargo add serde_json
cargo add toml
```

แนวทาง:

- ใช้ `#[derive(Serialize, Deserialize)]`
- ใช้ `serde_json::to_string_pretty()`
- ใช้ `serde_json::from_str()`
- ใช้ `toml::to_string()`
- ใช้ `toml::from_str()`
- ใช้ `std::fs::write()` สำหรับเขียนไฟล์
- ใช้ `std::fs::read_to_string()` สำหรับอ่านไฟล์
- ใช้ `Option<T>` สำหรับข้อมูลที่อาจไม่มีค่า

## Completion Checklist

- [ ] สร้าง Student Struct ได้
- [ ] Derive Serialize ได้
- [ ] Derive Deserialize ได้
- [ ] Serialize JSON ได้
- [ ] Deserialize JSON ได้
- [ ] ใช้ Pretty JSON ได้
- [ ] ใช้ Serde Attributes อย่างน้อย 3 รายการ
- [ ] Serialize TOML ได้
- [ ] Deserialize TOML ได้
- [ ] เขียน JSON File ได้
- [ ] อ่าน JSON File ได้
- [ ] เขียน TOML File ได้
- [ ] อ่าน TOML File ได้
- [ ] จัดการ Invalid JSON ได้
- [ ] จัดการ Invalid TOML ได้
- [ ] จัดการ File Errors ได้

## Run

```powershell
cd 29_database_serialization/exercises/exercise_03/serde_practice

cargo check
cargo run
```
