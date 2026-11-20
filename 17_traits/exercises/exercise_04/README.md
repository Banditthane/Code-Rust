# Exercise 04: Trait Challenge

## Objective

สร้างระบบจัดการข้อมูลหลายชนิดโดยใช้ Traits, Trait Bounds และ `impl Trait` เพื่อฝึกนำความรู้ทั้งหมดในบทนี้มาประยุกต์ใช้

## Project

Project Name: `trait_challenge`

## Scenario

ต้องการสร้างระบบที่สามารถจัดเก็บและแสดงข้อมูลของสินค้าและนักเรียน โดยทั้งสองชนิดข้อมูลมีรายละเอียดแตกต่างกัน แต่สามารถใช้พฤติกรรมร่วมกันได้

## Requirements

### Part 1: Define Traits

สร้าง Trait ชื่อ `Summary` โดยมี Methods:

- `summary(&self) -> String`
- `category(&self) -> &str` พร้อม Default Implementation คืนค่า `"General"`

### Part 2: Create Structs

สร้าง Struct จำนวน 2 ชนิด

**Product**

Fields:

- `name: String`
- `price: f64`

**Student**

Fields:

- `name: String`
- `score: u32`

### Part 3: Implement Summary

Implement Trait `Summary` ให้ Product และ Student

กำหนดรายละเอียด:

- Product ต้องคืนชื่อสินค้าและราคา
- Student ต้องคืนชื่อนักเรียนและคะแนน
- Product Override `category()` ให้คืน `"Product"`
- Student Override `category()` ให้คืน `"Student"`

### Part 4: Implement Display

Implement Trait `Display` ให้ Product และ Student

รูปแบบการแสดงผล:

Product:

```text
Product: Laptop, Price: 25000.00
```

Student:

```text
Student: Alice, Score: 85
```

### Part 5: Create Generic Function with Trait Bounds

สร้าง Function ชื่อ `print_info()` โดยรับ Generic Type ที่ต้อง Implement ทั้ง:

- `Summary`
- `Display`

Function ต้องแสดง:

1. ข้อมูลด้วย Display
2. Category
3. Summary

### Part 6: Use impl Trait

สร้าง Function ชื่อ `print_summary()` โดยใช้ `impl Summary` เป็น Parameter

Function ต้องแสดง Summary ของข้อมูลที่รับเข้ามา

### Part 7: Use Return Position impl Trait

สร้าง Function ชื่อ `create_default_message()` ซึ่งคืนค่าเป็น `impl Display`

กำหนดให้ Function คืน String ที่มีข้อความ:

```text
Welcome to Trait Challenge!
```

## Test Data

สร้างข้อมูล:

| Type    | Name   |    Value |
| ------- | ------ | -------: |
| Product | Laptop | 25000.00 |
| Product | Mouse  |   500.00 |
| Student | Alice  |       85 |
| Student | Bob    |       72 |

เรียก `print_info()` และ `print_summary()` กับข้อมูลที่สร้างขึ้น

## Constraints

- ต้องประกาศ Trait `Summary`
- ต้องมี Default Method `category()`
- ต้องสร้าง Struct `Product`
- ต้องสร้าง Struct `Student`
- ต้อง Implement `Summary` ให้ทั้งสอง Struct
- ต้อง Override `category()` ทั้งสอง Struct
- ต้อง Implement `Display` ให้ทั้งสอง Struct
- ต้องใช้ Generic Trait Bounds เช่น `T: Summary + Display`
- ต้องใช้ `impl Summary` ใน Parameter อย่างน้อยหนึ่ง Function
- ต้องใช้ `-> impl Display` ใน Return Type อย่างน้อยหนึ่ง Function
- ต้องใช้ `where` Clause กับ Generic Function อย่างน้อยหนึ่งจุด
- ห้ามใช้ `dyn Trait` ในโจทย์หลัก
- ห้ามกำหนด Summary เป็นข้อความตายตัวโดยไม่อ่านข้อมูลจาก Fields

## Expected Output

```text
Product: Laptop, Price: 25000.00
Category: Product
Summary: Product Laptop costs 25000.00

Product: Mouse, Price: 500.00
Category: Product
Summary: Product Mouse costs 500.00

Student: Alice, Score: 85
Category: Student
Summary: Student Alice has score 85

Student: Bob, Score: 72
Category: Student
Summary: Student Bob has score 72

Summary Only:
Product Laptop costs 25000.00
Student Alice has score 85

Welcome to Trait Challenge!
```

หมายเหตุ: สามารถจัดรูปแบบข้อความ Summary ให้ตรงกับตัวอย่างได้ โดยต้องอ่านข้อมูลจาก Fields จริงของแต่ละ Struct

## Hints

- `impl Summary for Product` ใช้ Implement Trait ให้ Product
- `impl Summary for Student` ใช้ Implement Trait ให้ Student
- `fmt::Display` ใช้กำหนดรูปแบบแสดงผล
- `T: Summary + Display` กำหนดให้ T Implement ทั้งสอง Traits
- `where` Clause ช่วยแยก Trait Bounds ออกจาก Function Signature
- `impl Summary` ใช้รับค่าที่ Implement Summary
- `-> impl Display` ใช้คืน Concrete Type ที่ Implement Display
- Function ที่ใช้ Return Position `impl Trait` แบบปกติ ต้องคืน Concrete Type เดียวกันในทุกเส้นทางที่คืนค่า

## Completion Checklist

- [ ] ประกาศ Trait `Summary`
- [ ] สร้าง Default Method `category()`
- [ ] สร้าง Struct `Product`
- [ ] สร้าง Struct `Student`
- [ ] Implement `Summary` ให้ Product
- [ ] Implement `Summary` ให้ Student
- [ ] Override `category()` ทั้งสอง Struct
- [ ] Implement `Display` ให้ Product
- [ ] Implement `Display` ให้ Student
- [ ] สร้าง Generic Function ด้วย Trait Bounds
- [ ] ใช้ `Summary + Display` ร่วมกันได้
- [ ] ใช้ `where` Clause ได้
- [ ] ใช้ `impl Trait` ใน Parameter ได้
- [ ] ใช้ `impl Trait` ใน Return Type ได้
- [ ] แสดงข้อมูลครบทั้ง 4 Instances
- [ ] แสดงผลตรงตาม Expected Output

## Run Project

```powershell
cd 17_traits/exercises/exercise_04/trait_challenge
cargo run
```

## Challenge

เพิ่มเติมความสามารถ:

- เพิ่ม Struct `Book`
- Implement `Summary` และ `Display` ให้ Book
- สร้าง Function ที่รับ Reference ของข้อมูลหลายชนิด
- ทดลองใช้ Multiple Trait Bounds เพิ่มเติม
- สร้าง Function สำหรับตรวจสอบข้อมูลก่อนแสดงผล
- ศึกษา Trait Objects ด้วย `dyn Trait` เป็นหัวข้อต่อยอด

## Final Goal

สามารถออกแบบ Trait เพื่อกำหนดพฤติกรรมร่วมกัน และใช้ Trait Bounds กับ Generic Functions เพื่อรองรับข้อมูลหลายชนิดได้อย่างเหมาะสม
