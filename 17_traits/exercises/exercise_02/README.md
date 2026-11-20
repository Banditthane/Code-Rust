# Exercise 02: Shape Traits

## Objective

สร้างระบบคำนวณพื้นที่และเส้นรอบรูปโดยใช้ Trait ชื่อ `Shape` และ Implement ให้ Circle, Rectangle และ Square

## Project

Project Name: `shape_traits`

## Requirements

### Part 1: Define Trait

สร้าง Trait ชื่อ `Shape` โดยมี Methods:

- `area(&self) -> f64`
- `perimeter(&self) -> f64`
- `name(&self) -> &str`

### Part 2: Create Structs

สร้าง Struct ต่อไปนี้:

**Circle**

- `radius: f64`

**Rectangle**

- `width: f64`
- `height: f64`

**Square**

- `side: f64`

### Part 3: Implement Shape

Implement Trait `Shape` ให้ Struct ทั้งสามชนิด

สูตรที่ใช้:

Circle:

- Area = π × radius × radius
- Perimeter = 2 × π × radius

Rectangle:

- Area = width × height
- Perimeter = 2 × (width + height)

Square:

- Area = side × side
- Perimeter = 4 × side

กำหนดค่า PI ด้วย `std::f64::consts::PI`

### Part 4: Create Instances

สร้างข้อมูล:

| Shape     | Dimensions            |
| --------- | --------------------- |
| Circle    | radius = 5            |
| Rectangle | width = 4, height = 6 |
| Square    | side = 4              |

แสดงชื่อ พื้นที่ และเส้นรอบรูปของแต่ละ Shape

## Constraints

- ต้องประกาศ Trait `Shape`
- ต้องมี Methods ครบทั้ง 3 Methods
- ต้อง Implement Trait ให้ Struct ทั้งสามชนิด
- ต้องใช้ `std::f64::consts::PI`
- ต้องคำนวณพื้นที่และเส้นรอบรูปจาก Fields
- ห้ามกำหนดผลลัพธ์พื้นที่และเส้นรอบรูปเป็นค่าคงที่
- ใช้ `println!()` แสดงทศนิยม 2 ตำแหน่ง
- ไม่จำเป็นต้องใช้ External Crates

## Expected Output

```text
Shape: Circle
Area: 78.54
Perimeter: 31.42

Shape: Rectangle
Area: 24.00
Perimeter: 20.00

Shape: Square
Area: 16.00
Perimeter: 16.00
```

หมายเหตุ: ผลลัพธ์ของ Circle ใช้ค่า PI จาก Standard Library จึงอาจแสดงค่าทศนิยมตามการปัดเศษสองตำแหน่ง

## Hints

- ใช้ `std::f64::consts::PI`
- ใช้ `write` หรือ `format!()` ไม่จำเป็นสำหรับการคำนวณ
- `println!("{:.2}", value)` ใช้แสดงทศนิยม 2 ตำแหน่ง
- Methods ของ Trait สามารถอ่าน Fields ผ่าน `&self`
- ควรแยกการคำนวณของแต่ละ Shape ไว้ใน Trait Implementation ของตัวเอง

## Completion Checklist

- [ ] ประกาศ Trait `Shape`
- [ ] สร้าง Struct `Circle`
- [ ] สร้าง Struct `Rectangle`
- [ ] สร้าง Struct `Square`
- [ ] Implement `area()` ให้ครบ
- [ ] Implement `perimeter()` ให้ครบ
- [ ] Implement `name()` ให้ครบ
- [ ] ใช้ PI จาก Standard Library
- [ ] คำนวณพื้นที่ได้ถูกต้อง
- [ ] คำนวณเส้นรอบรูปได้ถูกต้อง
- [ ] แสดงทศนิยม 2 ตำแหน่ง
- [ ] แสดงผลตรงตาม Expected Output

## Run Project

```powershell
cd 17_traits/exercises/exercise_02/shape_traits
cargo run
```

## Challenge

เพิ่มเติมความสามารถ:

- เพิ่ม Struct `Triangle`
- เพิ่ม Trait Method สำหรับตรวจสอบว่ารูปทรงมีพื้นที่มากกว่าค่าที่กำหนดหรือไม่
- สร้าง Function ที่รับ Shape หลายชนิดผ่าน Trait Bounds
- ทดลองเปรียบเทียบพื้นที่ของรูปทรงต่าง ๆ
