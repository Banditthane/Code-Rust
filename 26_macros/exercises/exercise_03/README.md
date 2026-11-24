# Exercise 03: Derive Macro Practice

## ระดับ

Medium

## Objective

ฝึกสร้าง Procedural Derive Macro ของตัวเอง โดยใช้ `proc_macro`, `syn` และ `quote`

## Problem Statement

สร้าง Derive Macro ชื่อ `Describe` เพื่อสร้าง Method สำหรับแสดงรายละเอียดของ Struct โดยอัตโนมัติ

ตัวอย่างการใช้งานที่ต้องการ:

```rust
#[derive(Describe)]
struct User {
    name: String,
    age: u32,
}
```

เมื่อเรียกใช้งาน Method ที่ Macro สร้างขึ้น ต้องสามารถแสดงรายละเอียดของ User ได้

## Project Structure

Exercise นี้ต้องมี 2 Cargo Packages:

```text
derive_macro_practice/
├── Cargo.toml
├── src/
│   └── main.rs
└── describe_derive/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

- `derive_macro_practice` เป็น Application ที่นำ Macro ไปใช้งาน
- `describe_derive` เป็น Procedural Macro Library

## Requirements

### Part 1: Procedural Macro Crate

1. สร้าง Library Crate ชื่อ `describe_derive`
2. กำหนด `proc-macro = true`
3. เพิ่ม Dependency `syn`
4. เพิ่ม Dependency `quote`
5. ประกาศ Derive Macro ชื่อ `Describe`

### Part 2: Parse Struct

1. รับ Input เป็น `TokenStream`
2. Parse Input ด้วย `syn`
3. ตรวจสอบชื่อ Struct
4. ตรวจสอบ Fields ของ Struct
5. รองรับ Named Fields เช่น `name` และ `age`

### Part 3: Generate Code

1. ใช้ `quote` สร้างโค้ด
2. สร้าง Trait Implementation หรือ Method ที่เหมาะสม
3. Method ต้องคืนค่า String ที่มีรายละเอียดของ Struct
4. รองรับ Struct ที่มี Named Fields อย่างน้อย 2 Fields
5. จัดการชื่อและข้อมูลของ Field ให้ถูกต้อง

### Part 4: Consumer Application

1. เพิ่ม `describe_derive` เป็น Path Dependency
2. Import Derive Macro
3. ใช้ `#[derive(Describe)]`
4. สร้าง Struct `User`
5. สร้าง Struct `Product`
6. เรียก Method ที่ Macro สร้างขึ้น
7. ตรวจสอบผลลัพธ์

## Constraints

- ต้องสร้าง Procedural Macro Crate แยกจาก Application
- ต้องกำหนด `proc-macro = true`
- ต้องใช้ `proc_macro::TokenStream`
- ต้องใช้ `syn` สำหรับ Parse
- ต้องใช้ `quote` สำหรับ Generate Code
- ต้องใช้ `#[proc_macro_derive(Describe)]`
- ต้องรองรับ Named Fields
- ต้องจัดการ Input ที่ไม่รองรับด้วย Compile Error ที่เข้าใจได้
- ห้ามใช้ `unsafe`

## Expected Output

```text
User Description:
Name: Alice
Age: 25

Product Description:
Name: Laptop
Price: 25000
```

รูปแบบรายละเอียดขึ้นอยู่กับการออกแบบ Method ที่สร้างขึ้น

## Hints

- ใช้ `syn::DeriveInput` เพื่อ Parse Input
- ตรวจสอบ `input.data` เพื่อดูว่าเป็น Struct หรือไม่
- ใช้ `Data::Struct` และ `Fields::Named` สำหรับ Named Fields
- ใช้ `quote!` สร้าง Implementation
- ใช้ `proc_macro2::TokenStream` ในกระบวนการสร้างโค้ดเมื่อจำเป็น
- ใช้ `syn::Error` เพื่อสร้าง Compile Error สำหรับ Input ที่ไม่รองรับ

## การเตรียม Cargo Project

หากยังไม่ได้สร้างโฟลเดอร์ ให้ใช้ PowerShell จาก Root ของ Repository:

```powershell
cd 26_macros/exercises/exercise_03/derive_macro_practice

cargo new --lib describe_derive --vcs none
```

จากนั้นกำหนด `Cargo.toml` ของ `describe_derive` ให้มี:

```toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
```

เพิ่ม Path Dependency ใน `Cargo.toml` ของ Application:

```toml
[dependencies]
describe_derive = { path = "describe_derive" }
```

หมายเหตุ: Application ต้องมี Package ของตัวเอง ส่วน `describe_derive` เป็น Library Crate แยกต่างหาก อย่ากำหนด `proc-macro = true` ให้ Application

## Completion Checklist

- [ ] สร้าง Procedural Macro Crate
- [ ] กำหนด `proc-macro = true`
- [ ] เพิ่ม `syn`
- [ ] เพิ่ม `quote`
- [ ] ประกาศ `Describe` Derive Macro
- [ ] Parse Struct ด้วย `syn`
- [ ] ตรวจสอบ Named Fields
- [ ] Generate Code ด้วย `quote`
- [ ] เพิ่ม Path Dependency
- [ ] ใช้ `#[derive(Describe)]` ได้
- [ ] ทดสอบกับ User
- [ ] ทดสอบกับ Product
- [ ] จัดการ Input ที่ไม่รองรับ

## Run

```powershell
cd 26_macros/exercises/exercise_03/derive_macro_practice

cargo check
cargo run
cargo test
```

## Additional Challenge

1. รองรับ Tuple Struct
2. รองรับ Struct ที่มี Generic Parameters
3. เพิ่มการจัดการ Field ที่มีชนิดข้อมูลไม่รองรับ
4. เพิ่ม Compile Error ที่ระบุตำแหน่งของ Field
5. เพิ่ม Unit Tests สำหรับ Procedural Macro
