# Exercise 01: Slices Practice

## Objective

ฝึกสร้างและใช้งาน Slice เพื่ออ้างอิงข้อมูลบางส่วนจาก Array และส่ง Slice เข้า Function

## Project Structure

```text
exercise_01/
├── README.md
└── slices_practice/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Requirements

เขียนโปรแกรมตามเงื่อนไขต่อไปนี้:

1. สร้าง Array ชื่อ `numbers` โดยมีข้อมูล 8 ค่า
2. ใช้ Slice เพื่อเลือกข้อมูลตั้งแต่ index 2 ถึง index 5
3. ใช้ Slice เพื่อเลือกข้อมูลตั้งแต่ index 0 ถึง index 3
4. สร้าง Function ชื่อ `display_slice`
5. Function ต้องรับ Parameter เป็น `&[i32]`
6. เรียก Function เพื่อแสดงข้อมูล Slice ทั้งสองชุด

## Constraints

- ต้องใช้ Array
- ต้องใช้ Slice (`&[i32]`)
- ต้องสร้าง Function แยกจาก `main()`
- ห้ามสร้าง Array ใหม่เพื่อเก็บข้อมูลที่เลือก
- ต้องแสดงผลข้อมูลทั้งสองชุด

## Expected Output

กำหนดให้ Array มีข้อมูลดังนี้:

```text
[10, 20, 30, 40, 50, 60, 70, 80]
```

ผลลัพธ์ที่ต้องการ:

```text
First Slice: [30, 40, 50, 60]
Second Slice: [10, 20, 30, 40]
```

## How to Run

เข้าไปในโฟลเดอร์โปรเจกต์:

```powershell
cd .\11_data_structures\exercises\exercise_01\slices_practice
```

ตรวจสอบโค้ด:

```powershell
cargo check
```

รันโปรแกรม:

```powershell
cargo run
```

## Completion Checklist

- [ ] สร้าง Array 8 ค่า
- [ ] สร้าง Slice ชุดที่ 1
- [ ] สร้าง Slice ชุดที่ 2
- [ ] สร้าง Function `display_slice`
- [ ] ส่ง Slice เข้า Function
- [ ] แสดงผลตรงตาม Expected Output
- [ ] โปรแกรมทำงานได้โดยไม่มี Compile Error

## Learning Review

หลังทำเสร็จ ควรอธิบายได้ว่า:

1. Slice แตกต่างจาก Array อย่างไร?
2. `&[i32]` หมายถึงอะไร?
3. ทำไม Function จึงรับ Slice ได้โดยไม่ต้องรับ Array ทั้งชุด?
