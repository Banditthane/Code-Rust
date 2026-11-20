# Exercise 02: Shape Enum

## Objective

ฝึกสร้าง Enum ที่มี Variants หลายรูปแบบ และเรียนรู้การเก็บข้อมูลภายใน Tuple Variant

## Project

`shape_enum`

## Scenario

สร้างโปรแกรมจัดเก็บข้อมูลรูปทรงเรขาคณิต โดยรองรับวงกลม สี่เหลี่ยมผืนผ้า และสี่เหลี่ยมจัตุรัส

รูปทรงแต่ละประเภทต้องเก็บข้อมูลที่จำเป็นต่อการคำนวณพื้นที่

## Requirements

ประกาศ Enum ชื่อ `Shape` โดยมี Variants ดังนี้:

| Variant   | Data       | Description         |
| --------- | ---------- | ------------------- |
| Circle    | `f64`      | รัศมี               |
| Rectangle | `f64, f64` | ความกว้างและความสูง |
| Square    | `f64`      | ความยาวด้าน         |

## Tasks

### Part 1: Create Enum

1. ประกาศ Enum ชื่อ `Shape`
2. สร้าง Variant `Circle`
3. สร้าง Variant `Rectangle`
4. สร้าง Variant `Square`
5. กำหนด Data Type ของแต่ละ Variant ตามตาราง

### Part 2: Create Instances

สร้างตัวแปรสำหรับเก็บรูปทรงอย่างน้อย 3 รายการ:

- Circle รัศมี 5.0
- Rectangle กว้าง 10.0 สูง 5.0
- Square ด้านยาว 4.0

### Part 3: Calculate Area

ใช้ `match` ตรวจสอบ Variant และคำนวณพื้นที่ตามสูตร:

| Shape     | Formula             |
| --------- | ------------------- |
| Circle    | π × radius × radius |
| Rectangle | width × height      |
| Square    | side × side         |

กำหนดค่า π เป็น `std::f64::consts::PI`

### Part 4: Display Results

แสดงชื่อรูปทรงและพื้นที่ของแต่ละรายการ โดยแสดงพื้นที่ทศนิยม 2 ตำแหน่ง

## Expected Output

```text
=== Shape Area Calculator ===

Shape: Circle
Radius: 5
Area: 78.54

Shape: Rectangle
Width: 10
Height: 5
Area: 50.00

Shape: Square
Side: 4
Area: 16.00
```

## Constraints

- ต้องใช้ Enum ชื่อ `Shape`
- ต้องมี Variants ครบทั้ง 3 รูปแบบ
- ต้องใช้ `f64` สำหรับข้อมูลตัวเลข
- ต้องใช้ `match` ตรวจสอบ Variant
- ต้องคำนวณพื้นที่ด้วยสูตรที่ถูกต้อง
- ต้องแสดงพื้นที่ทศนิยม 2 ตำแหน่ง

## Checklist

- [ ] ประกาศ Enum `Shape`
- [ ] สร้าง Tuple Variant `Circle`
- [ ] สร้าง Tuple Variant `Rectangle`
- [ ] สร้าง Tuple Variant `Square`
- [ ] สร้าง Instance ครบ 3 รูปทรง
- [ ] ใช้ `match` ดึงข้อมูลจาก Variants
- [ ] คำนวณพื้นที่วงกลมได้
- [ ] คำนวณพื้นที่สี่เหลี่ยมผืนผ้าได้
- [ ] คำนวณพื้นที่สี่เหลี่ยมจัตุรัสได้
- [ ] แสดงผลทศนิยม 2 ตำแหน่ง
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` ทำงานได้ตามต้องการ

## Challenge (Optional)

เพิ่ม Variant `Triangle(f64, f64)` สำหรับเก็บฐานและความสูง แล้วคำนวณพื้นที่ด้วยสูตร:

พื้นที่ = 0.5 × ฐาน × สูง
