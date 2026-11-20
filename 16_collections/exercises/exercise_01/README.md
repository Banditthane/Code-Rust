# Exercise 01: Vector Manager

## Objective

สร้างโปรแกรมจัดการรายการตัวเลขด้วย Vector (`Vec<i32>`) เพื่อฝึกเพิ่ม ลบ ค้นหา เรียงลำดับ และกรองข้อมูล

## Project

Project Name: `vector_manager`

## Requirements

โปรแกรมต้องสามารถ:

1. สร้าง Vector เริ่มต้นด้วยตัวเลข `[50, 20, 40, 10, 30]`
2. แสดงข้อมูลเริ่มต้น
3. เพิ่มตัวเลข `60` ด้วย `push()`
4. ลบข้อมูลที่ Index `1` ด้วย `remove()`
5. ตรวจสอบว่ามีตัวเลข `40` หรือไม่
6. เรียงลำดับตัวเลขจากน้อยไปมาก
7. กรองเฉพาะตัวเลขที่มากกว่าหรือเท่ากับ `30` ด้วย `retain()`
8. แสดงจำนวนสมาชิกด้วย `len()`
9. แสดงผลรวมของตัวเลขด้วย Iterator และ `sum()`
10. แสดงข้อมูลทั้งหมดด้วย Loop

## Constraints

- ต้องใช้ `Vec<i32>`
- ต้องใช้ `push()`
- ต้องใช้ `remove()`
- ต้องใช้ `contains()`
- ต้องใช้ `sort()`
- ต้องใช้ `retain()`
- ต้องใช้ `len()`
- ต้องใช้ Iterator และ `sum()`
- ห้ามกำหนดผลลัพธ์สุดท้ายเป็นค่าคงที่โดยไม่คำนวณจาก Vector
- ห้ามใช้ Index ที่อยู่นอกขอบเขต

## Expected Output

```text
Initial Vector: [50, 20, 40, 10, 30]
After Push: [50, 20, 40, 10, 30, 60]
After Remove: [50, 40, 10, 30, 60]
Contains 40: true
Sorted Vector: [10, 30, 40, 50, 60]
Filtered Vector: [30, 40, 50, 60]
Total Items: 4
Sum: 180
```

## Hints

- `remove(1)` จะลบสมาชิกที่ Index 1
- `sort()` เรียงจากน้อยไปมากสำหรับตัวเลข
- `retain()` เก็บเฉพาะสมาชิกที่ผ่านเงื่อนไข
- ใช้ `iter().sum()` เพื่อคำนวณผลรวมโดยไม่ย้าย Ownership ของ Vector

## Completion Checklist

- [ ] สร้าง Vector ได้
- [ ] เพิ่มข้อมูลด้วย `push()`
- [ ] ลบข้อมูลด้วย `remove()`
- [ ] ค้นหาข้อมูลด้วย `contains()`
- [ ] เรียงลำดับด้วย `sort()`
- [ ] กรองข้อมูลด้วย `retain()`
- [ ] นับจำนวนสมาชิกด้วย `len()`
- [ ] คำนวณผลรวมด้วย Iterator
- [ ] แสดงผลลัพธ์ตรงตาม Expected Output

## Run Project

```powershell
cd 16_collections/exercises/exercise_01/vector_manager
cargo run
```

## Challenge

เพิ่มเติมความสามารถ:

- รับตัวเลขจากผู้ใช้
- หาค่ามากที่สุดและน้อยที่สุด
- แสดงค่าเฉลี่ย
- ตรวจสอบ Vector ว่าว่างหรือไม่ด้วย `is_empty()`
