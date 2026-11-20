# Exercise 03: Inventory Manager

## Objective

สร้างโปรแกรมจัดการสินค้าโดยใช้ HashMap เพื่อฝึกเพิ่ม แก้ไข ลบ ค้นหา และแสดงข้อมูลสินค้า

## Project

Project Name: `inventory_manager`

## Requirements

สร้าง HashMap สำหรับจัดเก็บชื่อสินค้าและจำนวนสินค้า

กำหนดข้อมูลเริ่มต้น:

| Product | Quantity |
| ------- | -------: |
| Apple   |       10 |
| Banana  |       20 |
| Orange  |       15 |

โปรแกรมต้องสามารถ:

1. สร้าง HashMap ชื่อ `inventory`
2. เพิ่มข้อมูลสินค้าเริ่มต้น
3. เพิ่มจำนวน Apple เป็น `15` ด้วย `insert()`
4. เพิ่มสินค้า Mango จำนวน `12`
5. ลบสินค้า Banana ด้วย `remove()`
6. ตรวจสอบว่ามีสินค้า Orange หรือไม่ด้วย `contains_key()`
7. ค้นหาจำนวนสินค้า Apple ด้วย `get()`
8. ใช้ `entry()` และ `or_insert()` เพื่อเพิ่มจำนวนสินค้า Mango อีก `3`
9. แสดงชื่อสินค้าและจำนวนสินค้าทั้งหมด
10. แสดงจำนวนประเภทสินค้าด้วย `len()`

## Constraints

- ต้องใช้ `HashMap<String, u32>`
- ต้องใช้ `insert()`
- ต้องใช้ `remove()`
- ต้องใช้ `contains_key()`
- ต้องใช้ `get()`
- ต้องใช้ `entry()` และ `or_insert()`
- ต้องใช้ `len()`
- ต้องใช้ Loop เพื่อแสดงข้อมูล
- ห้ามสมมติว่าลำดับการวนซ้ำของ HashMap คงที่
- หากค้นหาสินค้าไม่พบ ต้องจัดการกรณี `None`

## Expected Output

```text
Apple Quantity: 15
Contains Orange: true
Mango Quantity: 15
Total Product Types: 3

Inventory:
Apple: 15
Mango: 15
Orange: 15
```

หมายเหตุ: ลำดับรายการสินค้าในส่วน Inventory อาจแตกต่างกันได้ เพราะ HashMap ไม่รับประกันลำดับการวนซ้ำ

## Hints

- ใช้ `String::from()` เพื่อสร้าง Key
- `insert()` ใช้เพิ่มหรือแทนที่ข้อมูล
- `get()` คืนค่า `Option<&V>`
- `entry()` ใช้จัดการ Value ของ Key
- `or_insert()` คืน Mutable Reference ของ Value
- หากต้องการแสดงผลตามลำดับชื่อสินค้า สามารถนำ Key ไปเก็บใน Vector แล้วเรียงลำดับก่อนแสดงผลได้

## Completion Checklist

- [ ] สร้าง HashMap ได้
- [ ] เพิ่มข้อมูลด้วย `insert()`
- [ ] แก้ไขจำนวนสินค้าด้วย `insert()`
- [ ] ลบสินค้าด้วย `remove()`
- [ ] ตรวจสอบ Key ด้วย `contains_key()`
- [ ] ค้นหาข้อมูลด้วย `get()`
- [ ] ใช้ `entry()` และ `or_insert()` ได้
- [ ] นับประเภทสินค้าด้วย `len()`
- [ ] วนซ้ำเพื่อแสดง Key และ Value
- [ ] จัดการกรณีค้นหาไม่พบได้
- [ ] แสดงข้อมูลครบตาม Expected Output

## Run Project

```powershell
cd 16_collections/exercises/exercise_03/inventory_manager
cargo run
```

## Challenge

เพิ่มเติมความสามารถ:

- เพิ่มระบบรับสินค้าจากผู้ใช้
- คำนวณจำนวนสินค้าทั้งหมด
- ค้นหาสินค้าที่มีจำนวนต่ำกว่า 10
- แสดงสินค้าที่มีจำนวนมากที่สุด
- แยกฟังก์ชันสำหรับเพิ่ม ลบ และค้นหาสินค้า
