# Exercise 02: Product Struct

## Objective

ฝึกสร้าง Struct Instance หลายตัว การเข้าถึง Fields การแก้ไขข้อมูลด้วย Mutable Instance และการใช้ Struct Update Syntax

## Project

`product_struct`

## Requirements

สร้าง Struct ชื่อ `Product` โดยมี Fields ดังนี้:

| Field    | Data Type | Description       |
| -------- | --------- | ----------------- |
| name     | String    | ชื่อสินค้า        |
| price    | f64       | ราคาสินค้า        |
| quantity | u32       | จำนวนสินค้า       |
| in_stock | bool      | สถานะสินค้าในคลัง |

## Tasks

1. ประกาศ Struct ชื่อ `Product`
2. สร้าง Instance ชื่อ `product1` โดยกำหนดข้อมูลสินค้า
3. สร้าง Instance ชื่อ `product2` โดยกำหนดข้อมูลสินค้าอีกชนิดหนึ่ง
4. แสดงข้อมูลของสินค้าทั้งสองรายการ
5. ประกาศ Instance ที่สามารถแก้ไขค่าได้ด้วย `mut`
6. แก้ไข `price` และ `quantity` ของสินค้าที่เลือก
7. แสดงข้อมูลหลังจากแก้ไขแล้ว
8. ทดลองใช้ Struct Update Syntax เพื่อสร้างสินค้าอีกหนึ่งรายการ โดยนำ Fields บางส่วนจาก Instance เดิมมาใช้

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์:

```text
Product 1
Name: Keyboard
Price: 1200
Quantity: 10
In Stock: true

Product 2
Name: Mouse
Price: 500
Quantity: 20
In Stock: true

Updated Product
Name: Keyboard
Price: 1000
Quantity: 15
In Stock: true
```

หมายเหตุ: ตัวเลขและชื่อสินค้าเปลี่ยนได้ตามที่กำหนดเอง

## Constraints

- ต้องใช้ Struct ชื่อ `Product`
- ต้องมี Fields ครบทั้ง 4 รายการ
- ต้องสร้างสินค้าอย่างน้อย 2 Instances
- ต้องใช้ `mut` กับ Instance ที่ต้องการแก้ไข
- ต้องแสดงข้อมูลก่อนและหลังแก้ไข
- ต้องทดลองใช้ Struct Update Syntax อย่างน้อย 1 ครั้ง
- ต้องคำนึงถึง Ownership เมื่อใช้ข้อมูลจาก Instance เดิม

## Checklist

- [ ] ประกาศ Struct `Product`
- [ ] กำหนด Fields ครบทั้ง 4 รายการ
- [ ] สร้าง Product Instances อย่างน้อย 2 ตัว
- [ ] แสดงข้อมูลสินค้าได้
- [ ] แก้ไข Field ผ่าน Mutable Instance ได้
- [ ] แสดงข้อมูลหลังแก้ไขได้
- [ ] ทดลองใช้ Struct Update Syntax
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` ทำงานได้ตามต้องการ

## Challenge (Optional)

เพิ่ม Field ชื่อ `discount` เพื่อเก็บส่วนลดสินค้า แล้วแสดงราคาหลังหักส่วนลด
