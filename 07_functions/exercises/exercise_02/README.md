# Exercise 02: Function Parameters

## วัตถุประสงค์

ฝึกสร้าง Function ที่รับ Parameters และส่ง Arguments เข้าไปใช้งาน

## โจทย์

เขียนโปรแกรมแสดงข้อมูลบุคคลและคำนวณผลลัพธ์จากข้อมูลที่ส่งเข้า Function

## เงื่อนไข

### ส่วนที่ 1: แสดงข้อมูลบุคคล

1. สร้าง Function ชื่อ `show_info`
2. รับ Parameter `name` ชนิด `&str`
3. รับ Parameter `age` ชนิด `u32`
4. แสดงชื่อและอายุออกทางหน้าจอ
5. เรียกใช้งาน Function โดยส่งชื่อและอายุเข้าไป

### ส่วนที่ 2: คำนวณผลบวก

1. สร้าง Function ชื่อ `add`
2. รับ Parameters จำนวน 2 ตัว
3. กำหนดชนิดข้อมูลเป็น `i32`
4. แสดงผลบวกของตัวเลขทั้งสองภายใน Function

### ส่วนที่ 3: คำนวณผลคูณ

1. สร้าง Function ชื่อ `multiply`
2. รับ Parameters จำนวน 2 ตัว
3. กำหนดชนิดข้อมูลเป็น `i32`
4. แสดงผลคูณของตัวเลขทั้งสองภายใน Function

## ตัวอย่างผลลัพธ์

```text
Name: Alex
Age: 20
Addition: 30
Multiplication: 200
```

## โจทย์เพิ่มเติม

- ทดลองเปลี่ยนชื่อและอายุ
- ทดลองส่งตัวเลขที่แตกต่างกันเข้า Function
- สร้าง Function ชื่อ `show_score` เพื่อรับคะแนนและแสดงผล

## เกณฑ์การตรวจสอบ

- [ ] สร้าง Function ที่มี Parameters ได้
- [ ] ระบุชนิดข้อมูลของ Parameters ได้
- [ ] ส่ง Arguments เข้า Function ได้
- [ ] ใช้ Parameters ภายใน Function ได้
- [ ] สร้าง Function ที่มี Parameters หลายตัวได้

## วิธีรันโปรแกรม

```powershell
cargo check
cargo run
```
