# Exercise 03: Error Propagation

## Objective

ฝึกส่งต่อ Error ด้วย `?` Operator และสร้าง Functions ที่คืนค่า `Result<T, E>` เพื่อแยกหน้าที่การทำงานออกจากกัน

## Project

`error_propagation`

## Scenario

สร้างโปรแกรมอ่านข้อมูลจากไฟล์ข้อความ แล้วแปลงข้อมูลแต่ละบรรทัดเป็นตัวเลข พร้อมคำนวณผลรวม

โปรแกรมต้องสามารถส่งต่อ Error เมื่ออ่านไฟล์หรือแปลงข้อมูลไม่สำเร็จ

## Required File Structure

สร้างไฟล์ข้อมูลภายใน Project:

```text
error_propagation/
├── Cargo.toml
├── data.txt
└── src/
    └── main.rs
```

## Input File

สร้างไฟล์ `data.txt` และใส่ข้อมูลตัวอย่าง:

```text
10
20
30
40
50
```

## Requirements

### Part 1: Read File

สร้าง Function:

```text
read_numbers()
```

ข้อกำหนด:

- รับ Path ของไฟล์
- อ่านข้อมูลจากไฟล์ด้วย `std::fs::read_to_string`
- คืนค่าเป็น `Result<String, std::io::Error>`
- ใช้ `?` ส่งต่อ Error

### Part 2: Parse Numbers

สร้าง Function:

```text
parse_numbers()
```

ข้อกำหนด:

- รับข้อความที่อ่านได้จากไฟล์
- แยกข้อมูลด้วย `lines()`
- แปลงแต่ละบรรทัดเป็น `i32`
- คืนค่าเป็น `Result<Vec<i32>, ParseIntError>`
- ใช้ `?` ส่งต่อ Error เมื่อแปลงไม่สำเร็จ

### Part 3: Calculate Sum

สร้าง Function:

```text
calculate_sum()
```

ข้อกำหนด:

- รับข้อมูลตัวเลข
- คำนวณผลรวม
- คืนค่าผลรวมเป็น `i32`

### Part 4: Main Program

ภายใน `main()`:

1. เรียก Function อ่านไฟล์
2. ส่งข้อมูลไปยัง Function แปลงตัวเลข
3. คำนวณผลรวม
4. แสดงผลลัพธ์
5. จัดการ Error ที่ส่งกลับมา

## Expected Output

เมื่อใช้ข้อมูลตัวอย่าง:

```text
=== Error Propagation ===

File Read: Success
Parsed Numbers: [10, 20, 30, 40, 50]
Total: 150
```

เมื่อไฟล์ไม่มีอยู่:

```text
=== Error Propagation ===

Error: Failed to read file
```

เมื่อมีข้อความที่แปลงเป็นตัวเลขไม่ได้:

```text
=== Error Propagation ===

Error: Failed to parse number
```

ข้อความ Error สามารถแตกต่างได้ แต่ต้องสื่อความหมายตรงกับข้อผิดพลาด

## Constraints

- ต้องอ่านไฟล์ด้วย `std::fs::read_to_string`
- ต้องใช้ `Result` เป็น Return Type ตามที่กำหนด
- ต้องใช้ `?` ใน Function อ่านไฟล์
- ต้องใช้ `?` ใน Function แปลงตัวเลข
- ต้องใช้ `lines()` แยกข้อมูล
- ต้องจัดการกรณีอ่านไฟล์ไม่สำเร็จ
- ต้องจัดการกรณีแปลงข้อมูลไม่สำเร็จ
- ห้ามใช้ `unwrap()` เพื่อจัดการ Error
- ต้องแสดงผลรวมเมื่อข้อมูลถูกต้อง
- ไม่จำเป็นต้องใช้ Custom Error ใน Exercise นี้

## Checklist

- [ ] สร้างไฟล์ `data.txt`
- [ ] ใส่ข้อมูลตัวเลขในไฟล์
- [ ] สร้าง Function `read_numbers()`
- [ ] ใช้ `std::fs::read_to_string`
- [ ] กำหนด Return Type เป็น Result
- [ ] ใช้ `?` ในการอ่านไฟล์
- [ ] สร้าง Function `parse_numbers()`
- [ ] ใช้ `lines()` แยกข้อมูล
- [ ] แปลงข้อความเป็น `i32`
- [ ] ใช้ `?` ส่งต่อ Parse Error
- [ ] สร้าง Function `calculate_sum()`
- [ ] แสดงผลรวมได้ถูกต้อง
- [ ] ทดลองกรณีไม่มีไฟล์
- [ ] ทดลองกรณีข้อมูลไม่ใช่ตัวเลข
- [ ] จัดการ Error ที่ `main()`
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` ทำงานได้ตามต้องการ

## Challenge (Optional)

เพิ่มการตรวจสอบว่าไฟล์ต้องมีข้อมูลอย่างน้อย 1 บรรทัด หากไฟล์ว่างให้ส่ง Error กลับไปแทนการคำนวณผลรวมเป็นศูนย์
