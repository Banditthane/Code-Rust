# Exercise 03: Struct Methods

## Objective

ฝึกสร้าง Methods ด้วย `impl` และเรียนรู้การใช้ `&self`, `&mut self` และ Associated Functions

## Project

`struct_methods`

## Requirements

สร้าง Struct ชื่อ `Rectangle` โดยมี Fields ดังนี้:

| Field  | Data Type | Description |
| ------ | --------- | ----------- |
| width  | u32       | ความกว้าง   |
| height | u32       | ความสูง     |

จากนั้นสร้าง Methods สำหรับคำนวณและจัดการข้อมูลของ Rectangle

## Tasks

### Part 1: Create Struct

1. ประกาศ Struct ชื่อ `Rectangle`
2. กำหนด Fields `width` และ `height`
3. สร้าง Instance อย่างน้อย 1 ตัว

### Part 2: Create Methods

สร้าง Methods ต่อไปนี้ภายใน `impl Rectangle`:

| Method        | Requirement                                |
| ------------- | ------------------------------------------ |
| `area()`      | คำนวณพื้นที่จาก width × height             |
| `perimeter()` | คำนวณเส้นรอบรูปจาก 2 × (width + height)    |
| `is_square()` | ตรวจสอบว่า width และ height เท่ากันหรือไม่ |

ข้อกำหนด:

- Methods ทั้งสามต้องใช้ `&self`
- `area()` ต้องคืนค่าเป็น `u32`
- `perimeter()` ต้องคืนค่าเป็น `u32`
- `is_square()` ต้องคืนค่าเป็น `bool`

### Part 3: Mutable Method

สร้าง Method ชื่อ `resize()` เพื่อเปลี่ยนความกว้างและความสูงของ Rectangle

ข้อกำหนด:

- รับค่า width และ height ใหม่ผ่าน Parameters
- ใช้ `&mut self`
- แก้ไข Fields ของ Instance
- เรียก Method ผ่าน Mutable Instance

### Part 4: Associated Function

สร้าง Associated Function ชื่อ `new()` เพื่อสร้าง Rectangle จากค่า width และ height ที่รับเข้ามา

ข้อกำหนด:

- ไม่ต้องรับ `self`
- คืนค่าเป็น `Self`
- เรียกใช้งานผ่าน `Rectangle::new(...)`

## Expected Output

กำหนดตัวอย่าง width = 10 และ height = 5

```text
Width: 10
Height: 5
Area: 50
Perimeter: 30
Is Square: false

After Resize
Width: 8
Height: 8
Area: 64
Perimeter: 32
Is Square: true
```

## Constraints

- ต้องใช้ Struct ชื่อ `Rectangle`
- ต้องมี Fields `width` และ `height`
- ต้องสร้าง Methods ตามที่กำหนด
- Methods สำหรับคำนวณต้องใช้ `&self`
- Method `resize()` ต้องใช้ `&mut self`
- ต้องมี Associated Function `new()`
- ต้องแสดงผลก่อนและหลัง Resize

## Checklist

- [ ] ประกาศ Struct `Rectangle`
- [ ] สร้าง `impl Rectangle`
- [ ] สร้าง Method `area()`
- [ ] สร้าง Method `perimeter()`
- [ ] สร้าง Method `is_square()`
- [ ] สร้าง Method `resize()`
- [ ] ใช้ `&self` ได้ถูกต้อง
- [ ] ใช้ `&mut self` ได้ถูกต้อง
- [ ] สร้าง Associated Function `new()`
- [ ] เรียกใช้ `Rectangle::new()`
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` แสดงผลได้ถูกต้อง

## Challenge (Optional)

เพิ่ม Method `can_fit_inside()` เพื่อเปรียบเทียบว่า Rectangle หนึ่งสามารถใส่ในอีก Rectangle หนึ่งได้หรือไม่ โดยกำหนดเงื่อนไขการหมุนหรือไม่หมุนให้ชัดเจน
