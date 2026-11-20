# Exercise 04: Generics Challenge

## Project

`generics_challenge`

## Objective

ประยุกต์ใช้ Generics ร่วมกับ Structs, Methods, Traits และ Trait Bounds เพื่อสร้างระบบจัดเก็บและแสดงข้อมูล

## Project Scenario

สร้างระบบจัดเก็บข้อมูลของสมาชิกและสินค้า โดยใช้ Generic Struct เพื่อให้สามารถจัดเก็บข้อมูลได้หลายชนิด

## Requirements

### Task 1: Create Generic Struct

สร้าง Struct ชื่อ `Record<T>` ที่:

- ใช้ Generic Type Parameter
- มี Field ชื่อ `id` ชนิด `u32`
- มี Field ชื่อ `data` ชนิด T

Struct นี้ต้องสามารถจัดเก็บข้อมูลได้หลายชนิด

### Task 2: Implement Generic Methods

สร้าง Methods สำหรับ `Record<T>` ดังนี้:

**Method: `new`**

- รับค่า `id` และ `data`
- สร้าง Record ใหม่
- คืนค่าเป็น `Self`

**Method: `get_data`**

- คืนค่า Reference ของข้อมูลใน Field `data`

**Method: `get_id`**

- คืนค่า ID ของ Record

### Task 3: Implement Generic Method with Trait Bounds

สร้าง Method หรือ Function สำหรับแสดงข้อมูลที่มีข้อกำหนดว่า T ต้องรองรับ `Display`

แสดงผล:

- ID
- Data

### Task 4: Create Summary Trait

สร้าง Trait ชื่อ `Summary` ที่มี Method:

```rust
fn summary(&self) -> String;
```

จากนั้นสร้าง Struct อย่างน้อยสองชนิด:

**Student**

- `name: String`
- `score: f64`

**Product**

- `name: String`
- `price: f64`

ให้ทั้งสอง Struct Implement Trait `Summary`

### Task 5: Generic Function with Trait Bounds

สร้าง Function ชื่อ `print_summary` ที่:

- รับข้อมูล Generic Type
- กำหนด Trait Bound ให้รองรับ `Summary`
- เรียกใช้ Method `summary()`
- แสดงผลลัพธ์

### Task 6: Use Where Clause

สร้าง Generic Function อีกหนึ่งตัวโดยใช้ `where` Clause

กำหนดให้ Generic Type รองรับทั้ง:

- `Summary`
- `Display`

Function ต้องสามารถแสดงข้อมูลสรุปและข้อมูลที่รองรับ Display ได้

## Constraints

- ต้องใช้ Generic Struct `Record<T>`
- ต้องใช้ Generic Methods
- ต้องมี Method ที่ใช้ Trait Bound
- ต้องสร้าง Trait `Summary`
- ต้อง Implement Trait `Summary` ให้ Student และ Product
- ต้องสร้าง Generic Function ที่ใช้ Trait Bounds
- ต้องมี Function ที่ใช้ `where` Clause
- ห้ามสร้าง Function แยกตามชนิดข้อมูล
- ต้องใช้ข้อมูลหลายชนิดในการทดสอบ
- ต้องไม่มี Runtime Panic จากการใช้งานตามโจทย์

## Test Data

ทดลองสร้างข้อมูลอย่างน้อยดังนี้:

**Records**

- Record ที่จัดเก็บ Integer
- Record ที่จัดเก็บ String
- Record ที่จัดเก็บ Floating Point

**Students**

- Alice คะแนน 85
- Bob คะแนน 72

**Products**

- Laptop ราคา 25000
- Mouse ราคา 500

## Expected Output

ตัวอย่างผลลัพธ์:

```text
=== Records ===
ID: 1
Data: 100

ID: 2
Data: Rust

ID: 3
Data: 25.5

=== Students ===
Student: Alice
Score: 85

Student: Bob
Score: 72

=== Products ===
Product: Laptop
Price: 25000

Product: Mouse
Price: 500

=== Summary ===
Alice scored 85 points
Bob scored 72 points
Laptop costs 25000
Mouse costs 500
```

รูปแบบข้อความสามารถแตกต่างได้ แต่ต้องแสดงข้อมูลครบตามโจทย์

## Checklist

- [ ] สร้าง Generic Struct `Record<T>` ได้
- [ ] สร้าง `new()` ได้
- [ ] สร้าง `get_data()` ได้
- [ ] สร้าง `get_id()` ได้
- [ ] ใช้ Generic Methods ได้
- [ ] ใช้ Trait Bounds กับ Methods ได้
- [ ] สร้าง Trait `Summary` ได้
- [ ] Implement `Summary` ให้ Student ได้
- [ ] Implement `Summary` ให้ Product ได้
- [ ] สร้าง Generic Function `print_summary` ได้
- [ ] ใช้ Multiple Trait Bounds ได้
- [ ] ใช้ `where` Clause ได้
- [ ] ทดสอบข้อมูลหลายชนิดได้
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 18_generics/exercises/exercise_04/generics_challenge
cargo run
```

## Completion Criteria

ถือว่าผ่าน Exercise นี้เมื่อ:

1. โปรแกรมสามารถจัดเก็บข้อมูลหลายชนิดด้วย Generic Struct ได้
2. สามารถเรียกใช้ Generic Methods ได้
3. สามารถใช้ Trait Bounds เพื่อกำหนดความสามารถของข้อมูลได้
4. สามารถใช้ Generics ร่วมกับ Traits ได้
5. สามารถใช้ `where` Clause ได้
6. โปรแกรมทำงานครบตาม Test Data โดยไม่มี Compile Error

## Final Challenge

ลองเพิ่ม Struct ใหม่ชื่อ `Employee` ที่มีชื่อและเงินเดือน แล้ว Implement Trait `Summary` โดยไม่ต้องแก้ไข Generic Function `print_summary`

จุดประสงค์คือทดสอบว่าโค้ดที่เขียนไว้สามารถรองรับชนิดข้อมูลใหม่ได้ โดยไม่ต้องเขียน Function แยกเพิ่มเติม
