# Exercise 03: Lifetime Structs

## Project

`lifetime_structs`

## Objective

ฝึกสร้าง Struct ที่จัดเก็บ References และใช้ Lifetime Parameters เพื่อระบุความสัมพันธ์ระหว่าง Struct กับข้อมูลต้นทาง

## Requirements

### Task 1: Create ImportantExcerpt Struct

สร้าง Struct ชื่อ `ImportantExcerpt` ที่:

- ใช้ Lifetime Parameter ชื่อ `'a`
- มี Field ชื่อ `part`
- Field `part` เป็น String Slice Reference
- จัดเก็บ Reference ไปยังข้อความต้นทาง

### Task 2: Create Struct Instance

สร้าง String ต้นทางและสร้าง Instance ของ `ImportantExcerpt`

ทดลองใช้ข้อมูล:

```text
Rust is a systems programming language.
```

นำข้อความบางส่วนมาเก็บไว้ใน Field `part`

### Task 3: Implement Methods

สร้าง Methods สำหรับ Struct ดังนี้:

**Method: `get_part`**

- คืนค่า Reference ของ Field `part`

**Method: `display`**

- แสดงข้อความที่จัดเก็บอยู่ใน `part`

### Task 4: Implement Generic Struct with Lifetime

สร้าง Struct ชื่อ `ReferenceData` ที่:

- ใช้ Lifetime Parameter
- ใช้ Generic Type Parameter
- มี Field ที่เก็บ Reference ไปยังข้อมูลชนิด T

ทดลองใช้กับ:

- Integer
- Floating Point

### Task 5: Scope and Struct Lifetime

สร้าง Block เพื่อทดลองความสัมพันธ์ระหว่าง Lifetime ของข้อมูลต้นทางกับ Instance ของ Struct

อธิบายว่าเหตุใด Instance ที่เก็บ Reference จึงไม่สามารถถูกนำไปใช้งานหลังจากข้อมูลต้นทางหมดอายุได้

## Constraints

- ต้องสร้าง Struct ที่เก็บ References
- ต้องใช้ Lifetime Parameters ใน Struct
- ต้องสร้าง Methods อย่างน้อยสองตัว
- ต้องใช้ Lifetime ใน `impl` Block
- ต้องสร้าง Generic Struct ที่ใช้ Lifetime ร่วมกับ Type Parameter
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- ต้องไม่มี Dangling References
- โปรแกรมต้อง Compile และ Run ผ่าน

## Expected Output

ตัวอย่างผลลัพธ์:

```text
=== ImportantExcerpt ===
Original: Rust is a systems programming language.
Part: Rust is a systems programming language.
Get Part: Rust is a systems programming language.

=== ReferenceData ===
Integer: 100
Float: 25.5

=== Lifetime Understanding ===
The source data must remain valid while the struct reference is used.
```

รูปแบบข้อความสามารถแตกต่างได้ แต่ต้องแสดงข้อมูลครบตามโจทย์

## Checklist

- [ ] สร้าง `ImportantExcerpt` ได้
- [ ] ใช้ Lifetime Parameter ใน Struct ได้
- [ ] สร้าง Struct Instance ที่เก็บ Reference ได้
- [ ] สร้าง `get_part()` ได้
- [ ] สร้าง `display()` ได้
- [ ] ใช้ Lifetime ใน `impl` Block ได้
- [ ] สร้าง Generic Struct ที่ใช้ Lifetimes ได้
- [ ] ทดลอง Integer และ Floating Point ได้
- [ ] อธิบาย Scope และ Lifetime ของ Struct ได้
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 19_lifetimes/exercises/exercise_03/lifetime_structs
cargo run
```
