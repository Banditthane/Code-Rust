# Exercise 04: Lifetimes Challenge

## Project

`lifetimes_challenge`

## Objective

ประยุกต์ใช้ Lifetimes ร่วมกับ Generics, Structs, Methods และ Traits เพื่อสร้างระบบจัดเก็บและแสดงข้อมูลที่ใช้ References

## Project Scenario

สร้างระบบจัดเก็บข้อมูลบทความ (Article System) ที่สามารถเก็บข้อความต้นฉบับและสร้างข้อมูลสรุปจาก References ได้

## Requirements

### Task 1: Create Article Struct

สร้าง Struct ชื่อ `Article` ที่:

- ใช้ Lifetime Parameter ชื่อ `'a`
- มี Field `title` ชนิด `&'a str`
- มี Field `content` ชนิด `&'a str`
- สามารถจัดเก็บ References ไปยังข้อความต้นทางได้

### Task 2: Implement Article Methods

สร้าง Methods ดังนี้:

**Method: `new`**

- รับ Title และ Content
- สร้าง Article ใหม่
- คืนค่าเป็น `Self`

**Method: `get_title`**

- คืนค่า Reference ของ Title

**Method: `get_content`**

- คืนค่า Reference ของ Content

### Task 3: Create Summary Trait

สร้าง Trait ชื่อ `Summary` ที่มี Method:

```rust
fn summary(&self) -> String;
```

จากนั้น Implement Trait ให้กับ `Article`

รูปแบบ Summary ต้องแสดง Title และข้อความสรุปของ Article

### Task 4: Create Generic Reference Struct

สร้าง Struct ชื่อ `ReferenceData` ที่:

- ใช้ Lifetime Parameter
- ใช้ Generic Type Parameter
- มี Field สำหรับเก็บ Reference ไปยังข้อมูลชนิด T

ทดลองสร้างข้อมูลที่อ้างอิงไปยัง Integer และ Floating Point

### Task 5: Create Generic Function with Trait Bounds

สร้าง Function ชื่อ `print_summary` ที่:

- รับ Reference ไปยังข้อมูลชนิด Generic T
- กำหนด Trait Bound ให้ T รองรับ `Summary`
- เรียกใช้ Method `summary()`
- แสดงผลลัพธ์

### Task 6: Use Where Clause

สร้าง Generic Function อีกหนึ่งตัวที่:

- รับ Reference ไปยังข้อมูล
- ใช้ `where` Clause
- กำหนด Trait Bounds ให้เหมาะสม
- แสดง Summary ของข้อมูล

หากมีข้อกำหนด Lifetime เพิ่มเติม ให้ระบุ Lifetime Bounds ให้ถูกต้อง

### Task 7: Test Lifetime Relationships

ทดลองสร้างข้อมูลต้นทางและ Article โดย:

- ประกาศข้อมูลต้นทางก่อนสร้าง Article
- สร้าง Article ที่อ้างอิงข้อมูลต้นทาง
- เรียกใช้ Methods และ Trait
- ตรวจสอบว่า Reference ถูกใช้งานภายในช่วงเวลาที่ข้อมูลต้นทางยังมีชีวิตอยู่

## Constraints

- ต้องสร้าง Struct `Article` ที่เก็บ References
- ต้องใช้ Lifetime Parameters
- ต้องสร้าง Methods อย่างน้อยสามตัว
- ต้องสร้าง Trait `Summary`
- ต้อง Implement `Summary` ให้ Article
- ต้องสร้าง Generic Struct `ReferenceData`
- ต้องใช้ Lifetimes ร่วมกับ Generics
- ต้องสร้าง Generic Function ที่ใช้ Trait Bounds
- ต้องใช้ `where` Clause
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- ห้ามคืน Reference ไปยัง Local Data ที่หมดอายุแล้ว
- ต้องไม่มี Dangling References
- โปรแกรมต้อง Compile และ Run ผ่าน

## Test Data

**Article 1**

- Title: Rust Ownership
- Content: Understanding ownership and borrowing in Rust.

**Article 2**

- Title: Rust Lifetimes
- Content: Lifetimes help ensure references remain valid.

**Reference Data**

- Integer: 100
- Floating Point: 25.5

## Expected Output

ตัวอย่างผลลัพธ์:

```text
=== Article 1 ===
Title: Rust Ownership
Content: Understanding ownership and borrowing in Rust.

=== Article 2 ===
Title: Rust Lifetimes
Content: Lifetimes help ensure references remain valid.

=== Summary ===
Rust Ownership: Understanding ownership and borrowing in Rust.
Rust Lifetimes: Lifetimes help ensure references remain valid.

=== Reference Data ===
Integer: 100
Float: 25.5

=== Lifetime Challenge ===
All references are used while their source data is valid.
```

รูปแบบข้อความสามารถแตกต่างได้ แต่ต้องแสดงข้อมูลครบตามโจทย์

## Checklist

- [ ] สร้าง `Article` ได้
- [ ] ใช้ Lifetime Parameters ใน Struct ได้
- [ ] สร้าง `new()` ได้
- [ ] สร้าง `get_title()` ได้
- [ ] สร้าง `get_content()` ได้
- [ ] สร้าง Trait `Summary` ได้
- [ ] Implement `Summary` ให้ Article ได้
- [ ] สร้าง Generic Struct `ReferenceData` ได้
- [ ] ใช้ Lifetimes ร่วมกับ Generics ได้
- [ ] สร้าง Generic Function ที่ใช้ Trait Bounds ได้
- [ ] ใช้ `where` Clause ได้
- [ ] เข้าใจความสัมพันธ์ระหว่าง References และข้อมูลต้นทาง
- [ ] ไม่มี Dangling References
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 19_lifetimes/exercises/exercise_04/lifetimes_challenge
cargo run
```

## Completion Criteria

ถือว่าผ่าน Exercise นี้เมื่อ:

1. สามารถสร้าง Struct ที่เก็บ References ได้
2. สามารถกำหนด Lifetime Parameters ให้ Struct ได้
3. สามารถสร้าง Methods ที่ทำงานกับ References ได้
4. สามารถใช้ Traits ร่วมกับ Generic Types ได้
5. สามารถใช้ Trait Bounds และ `where` Clause ได้
6. เข้าใจว่า Reference ต้องไม่ถูกใช้งานเกิน Lifetime ของข้อมูลต้นทาง
7. โปรแกรม Compile และ Run ผ่าน

## Final Challenge

เพิ่ม Struct ใหม่ชื่อ `Book` ที่มี Title และ Content โดยใช้ Lifetime Parameters และ Implement Trait `Summary`

จากนั้นนำ Book ไปใช้งานกับ Generic Function `print_summary` ที่สร้างไว้แล้ว โดยไม่ต้องแก้ไข Generic Function

จุดประสงค์คือทดสอบว่าโครงสร้างที่ออกแบบสามารถรองรับข้อมูลชนิดใหม่ได้ โดยไม่ต้องเขียน Function แยกสำหรับแต่ละ Struct
