# Exercise 01: Student Struct

## Objective

ฝึกประกาศ Struct และสร้าง Struct Instance เพื่อจัดเก็บข้อมูลนักเรียน

## Project

`student_struct`

## Requirements

สร้าง Struct ชื่อ `Student` โดยมี Fields ดังนี้:

| Field     | Data Type | Description       |
| --------- | --------- | ----------------- |
| name      | String    | ชื่อนักเรียน      |
| age       | u8        | อายุ              |
| grade     | char      | ระดับชั้นหรือเกรด |
| score     | f64       | คะแนน             |
| is_active | bool      | สถานะการเรียน     |

จากนั้นสร้าง Instance อย่างน้อย 2 คน โดยกำหนดข้อมูลให้แตกต่างกัน

## Tasks

1. ประกาศ Struct ชื่อ `Student`
2. กำหนด Fields ทั้ง 5 รายการตามตาราง
3. สร้าง Instance ชื่อ `student1`
4. สร้าง Instance ชื่อ `student2`
5. แสดงข้อมูลของนักเรียนแต่ละคนด้วย `println!()`
6. เข้าถึง Fields ผ่านเครื่องหมายจุด `.`

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์ที่ต้องการ:

```text
Student 1
Name: Alice
Age: 20
Grade: A
Score: 85.5
Active: true

Student 2
Name: Bob
Age: 21
Grade: B
Score: 78
Active: true
```

หมายเหตุ: สามารถกำหนดชื่อและข้อมูลเป็นค่าอื่นได้ แต่ต้องแสดงข้อมูลครบทุก Field

## Constraints

- ต้องใช้ Struct ชื่อ `Student`
- ต้องมี Fields ครบทั้ง 5 รายการ
- ต้องสร้าง Instances อย่างน้อย 2 ตัว
- ต้องใช้ Data Type ตามที่กำหนด
- ต้องเข้าถึงข้อมูลผ่าน Fields ของ Instance
- ห้ามใช้ตัวแปรแยกแทน Fields ทั้งหมดโดยไม่สร้าง Struct

## Checklist

- [ ] ประกาศ Struct `Student`
- [ ] กำหนด Fields ครบถ้วน
- [ ] สร้าง Instance อย่างน้อย 2 ตัว
- [ ] กำหนดค่าให้ทุก Field
- [ ] แสดงข้อมูลของนักเรียนทั้งสองคน
- [ ] ใช้ `cargo check` ผ่าน
- [ ] ใช้ `cargo run` แล้วได้ผลลัพธ์ตามต้องการ

## Challenge (Optional)

ทดลองสร้าง Instance ตัวที่ 3 โดยใช้ Field Init Shorthand อย่างน้อย 2 Fields
