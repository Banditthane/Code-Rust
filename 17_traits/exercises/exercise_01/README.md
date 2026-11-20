# Exercise 01: Basic Traits

## Objective

สร้าง Trait ชื่อ `Describe` เพื่อกำหนดพฤติกรรมการแสดงรายละเอียดของข้อมูล และ Implement ให้ Struct หลายชนิด

## Project

Project Name: `basic_traits`

## Requirements

### Part 1: Define Trait

สร้าง Trait ชื่อ `Describe` โดยมี Methods ดังนี้:

1. `describe(&self) -> String`
2. `category(&self) -> String` โดยกำหนด Default Implementation ให้คืนค่า `"General"`

### Part 2: Create Structs

สร้าง Struct จำนวน 2 ชนิด:

**Person**

Fields:

- `name: String`
- `age: u32`

**Product**

Fields:

- `name: String`
- `price: f64`

### Part 3: Implement Trait

Implement Trait `Describe` ให้กับ Struct ทั้งสองชนิด

ข้อกำหนด:

- `Person` ต้องคืนรายละเอียดชื่อและอายุ
- `Product` ต้องคืนรายละเอียดชื่อและราคา
- Override Default Method `category()` ให้ Person คืน `"Human"` และ Product คืน `"Goods"`

### Part 4: Create Instances

สร้างข้อมูล:

- Person: Alice อายุ 25
- Product: Laptop ราคา 25000.0

เรียก Methods ของ Trait และแสดงผลข้อมูล

## Constraints

- ต้องประกาศ Trait ด้วย `trait`
- ต้องมี Method Signature อย่างน้อย 1 Method
- ต้องมี Default Method อย่างน้อย 1 Method
- ต้อง Implement Trait ให้ Struct ทั้งสองชนิด
- ต้อง Override Default Method
- ต้องใช้ `&self` สำหรับ Trait Methods ที่อ่านข้อมูล
- ห้ามสร้าง Function แยกที่ทำงานแทน Trait Methods

## Expected Output

```text
Person: Alice, Age: 25
Category: Human

Product: Laptop, Price: 25000
Category: Goods
```

## Hints

- ใช้ `format!()` เพื่อสร้าง String
- Trait Method ที่คืนค่า String สามารถนำไปแสดงด้วย `{}` ได้
- Default Method สามารถถูก Override ภายใน `impl Describe for Type`
- ตรวจสอบชื่อ Method และ Return Type ให้ตรงกับ Trait Definition

## Completion Checklist

- [ ] ประกาศ Trait `Describe`
- [ ] สร้าง Method `describe()`
- [ ] สร้าง Default Method `category()`
- [ ] สร้าง Struct `Person`
- [ ] สร้าง Struct `Product`
- [ ] Implement Trait ให้ Person
- [ ] Implement Trait ให้ Product
- [ ] Override Default Method ทั้งสองชนิด
- [ ] สร้าง Instance และเรียก Methods ได้
- [ ] แสดงผลตรงตาม Expected Output

## Run Project

```powershell
cd 17_traits/exercises/exercise_01/basic_traits
cargo run
```

## Challenge

เพิ่มเติมความสามารถ:

- เพิ่ม Struct `Book`
- Implement Trait `Describe` ให้ Book
- เพิ่ม Default Method สำหรับแสดงข้อความต้อนรับ
- ทดลองเปลี่ยน Default Method ของ Trait แล้วสังเกตผลลัพธ์
