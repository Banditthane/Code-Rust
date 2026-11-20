# Exercise 03: HashMap Practice

## Objective

ฝึกใช้ HashMap เพื่อจัดเก็บข้อมูลสินค้าในรูปแบบ Key-Value พร้อมเพิ่ม ค้นหา อัปเดต และลบข้อมูล

## Project Structure

```text
exercise_03/
├── README.md
└── hashmap_practice/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Requirements

สร้างโปรแกรมจัดการราคาสินค้า

1. Import `HashMap` จาก `std::collections`
2. สร้าง HashMap ชื่อ `products`
3. กำหนด Key เป็นชื่อสินค้า และ Value เป็นราคาชนิด `i32`
4. เพิ่มสินค้าอย่างน้อย 3 รายการ
5. ค้นหาราคาสินค้าด้วย `.get()`
6. อัปเดตราคาสินค้าหนึ่งรายการด้วย `.insert()`
7. ตรวจสอบ Key ด้วย `.contains_key()`
8. ลบสินค้า 1 รายการด้วย `.remove()`
9. ใช้ Loop แสดงข้อมูลสินค้าทั้งหมด

## Constraints

- ต้องใช้ `HashMap`
- ต้องใช้ `.insert()`
- ต้องใช้ `.get()`
- ต้องใช้ `.contains_key()`
- ต้องใช้ `.remove()`
- ต้องใช้ Loop แสดง Key และ Value
- ต้องไม่สมมติว่าลำดับข้อมูลใน HashMap จะคงที่

## Expected Output

ใช้ข้อมูลตัวอย่างดังนี้:

| Product | Price |
| ------- | ----: |
| Apple   |    30 |
| Banana  |    20 |
| Orange  |    40 |

หลังจากอัปเดต Apple เป็น 35 และลบ Banana แล้ว ต้องมีข้อมูลเหลือ:

```text
Apple: 35
Orange: 40
```

ตัวอย่างผลลัพธ์เพิ่มเติม:

```text
Apple Price: Some(30)
Contains Apple: true
```

หมายเหตุ: ลำดับการแสดงรายการสินค้าสามารถแตกต่างกันได้

## How to Run

```powershell
cd .\11_data_structures\exercises\exercise_03\hashmap_practice

cargo check
cargo run
```

## Completion Checklist

- [ ] Import HashMap
- [ ] สร้าง HashMap สำหรับสินค้า
- [ ] เพิ่มสินค้าอย่างน้อย 3 รายการ
- [ ] ค้นหาราคาด้วย `.get()`
- [ ] อัปเดตราคาด้วย `.insert()`
- [ ] ตรวจสอบ Key ด้วย `.contains_key()`
- [ ] ลบสินค้าด้วย `.remove()`
- [ ] ใช้ Loop แสดงข้อมูล
- [ ] ข้อมูลสุดท้ายถูกต้อง

## Learning Review

1. HashMap จัดเก็บข้อมูลอย่างไร?
2. Key และ Value มีหน้าที่อะไร?
3. เมื่อ Insert Key ที่มีอยู่แล้วจะเกิดอะไรขึ้น?
4. ทำไมจึงไม่ควรพึ่งพาลำดับการแสดงผลของ HashMap?
