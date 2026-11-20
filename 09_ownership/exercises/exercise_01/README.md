# Exercise 01: Ownership Basics

## Objective

ฝึกทำความเข้าใจ Owner และ Scope โดยใช้ String และตัวแปรพื้นฐาน

## Project

`ownership_basics`

## Tasks

เขียนโปรแกรมที่ทำงานดังนี้:

1. สร้างตัวแปร `name` เป็น String
2. สร้างตัวแปร `age` เป็น Integer
3. แสดงค่าของตัวแปรทั้งสอง
4. สร้าง Scope ด้านใน
5. ประกาศตัวแปร `country` ภายใน Scope
6. แสดงค่า `country` ภายใน Scope
7. ทดลองใช้งานตัวแปรหลังออกจาก Scope

## Requirements

- ใช้ `String::from()` สร้าง String
- ใช้ `let` ประกาศตัวแปร
- ใช้ `{}` สร้าง Scope ด้านใน
- ใช้ `println!()` แสดงผล
- ต้องเข้าใจว่าตัวแปรที่ประกาศใน Scope ด้านในใช้งานจากภายนอกไม่ได้

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Name: Rust Learner
Age: 25
Country: Thailand
```

## Constraints

- ต้องมีตัวแปรอย่างน้อย 3 ตัว
- ต้องมี Scope ด้านในอย่างน้อย 1 จุด
- ห้ามใช้ตัวแปร `country` นอก Scope ที่ประกาศไว้
- ยังไม่ต้องใช้ Function

## Checklist

- [ ] สร้าง String ได้
- [ ] ประกาศตัวแปรได้
- [ ] แสดงค่าตัวแปรได้
- [ ] สร้าง Scope ได้
- [ ] เข้าใจขอบเขตของตัวแปร

## Run Project

```powershell
cargo check
cargo run
```
