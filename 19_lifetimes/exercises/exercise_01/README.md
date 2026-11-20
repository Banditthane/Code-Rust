# Exercise 01: Lifetime Basics

## Project

`lifetime_basics`

## Objective

ฝึกทำความเข้าใจ Scope, Lifetime และความสัมพันธ์ระหว่าง References กับข้อมูลต้นทาง

## Requirements

### Task 1: Create References

สร้างตัวแปร String และสร้าง Immutable Reference ไปยังตัวแปรดังกล่าว

แสดงผล:

- ข้อมูลต้นทาง
- Reference
- ความยาวของข้อความผ่าน Reference

### Task 2: Multiple References

สร้าง Immutable References อย่างน้อยสองตัวที่อ้างอิงไปยังข้อมูลต้นทางเดียวกัน

ทดลองอ่านข้อมูลผ่าน References ทั้งสอง

### Task 3: Scope and Lifetime

สร้าง Block ซ้อนภายใน `main()`

- ประกาศตัวแปรภายใน Block
- สร้าง Reference ไปยังตัวแปรดังกล่าว
- ทดลองใช้งาน Reference ภายใน Block
- สังเกตว่าเมื่อออกจาก Block แล้ว ตัวแปรภายใน Block ไม่สามารถเข้าถึงได้อีก

### Task 4: Borrowing Practice

ทดลองใช้ Immutable Borrowing และ Mutable Borrowing แยกเป็นกรณี

สังเกตว่า Rust อนุญาตให้ใช้งาน References ตามกฎ Borrowing อย่างไร

### Task 5: Understand Dangling References

ศึกษาและอธิบายว่าเหตุใด Reference จึงไม่สามารถใช้งานหลังจากข้อมูลต้นทางหมดอายุได้

ไม่จำเป็นต้องนำโค้ดที่ไม่ผ่าน Compile มารวมไว้ในโปรแกรมหลัก

## Constraints

- ต้องใช้ References จริง
- ต้องมี Block ซ้อนอย่างน้อยหนึ่งระดับ
- ต้องทดลอง Immutable Borrowing
- ต้องทดลอง Mutable Borrowing
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- โปรแกรมหลักต้อง Compile และ Run ผ่าน
- กรณีที่ไม่ผ่าน Compile ให้บันทึกคำอธิบายใน README แทนการรวมไว้ในโปรแกรมหลัก

## Expected Output

ตัวอย่างผลลัพธ์:

```text
=== References ===
Original: Rust Programming
Reference: Rust Programming
Length: 17

=== Multiple References ===
Reference 1: Rust Programming
Reference 2: Rust Programming

=== Scope ===
Inside block: 100
Reference inside block: 100

=== Borrowing ===
Before update: Rust
After update: Rust Programming

=== Lifetime Understanding ===
A reference must not outlive its source data.
```

รูปแบบข้อความสามารถแตกต่างได้ แต่ต้องแสดงข้อมูลครบตามโจทย์

## Checklist

- [ ] สร้าง Immutable Reference ได้
- [ ] สร้าง References หลายตัวได้
- [ ] ทดลอง Scope ได้
- [ ] เข้าใจ Lifetime ของข้อมูลต้นทาง
- [ ] ทดลอง Immutable Borrowing ได้
- [ ] ทดลอง Mutable Borrowing ได้
- [ ] อธิบาย Dangling Reference ได้
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 19_lifetimes/exercises/exercise_01/lifetime_basics
cargo run
```
