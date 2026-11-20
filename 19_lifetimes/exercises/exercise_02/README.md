# Exercise 02: Lifetime Functions

## Project

`lifetime_functions`

## Objective

ฝึกสร้าง Functions ที่รับและคืนค่า References โดยใช้ Lifetime Annotations เพื่อกำหนดความสัมพันธ์ระหว่าง References

## Requirements

### Task 1: Create Longest Function

สร้าง Function ชื่อ `longest` ที่:

- รับ String Slices สองตัว
- ใช้ Lifetime Parameter ชื่อ `'a`
- คืนค่า String Slice ที่มีความยาวมากกว่า
- Return Reference ต้องใช้ Lifetime ที่สัมพันธ์กับ Input References

### Task 2: Test Longest Function

ทดลองใช้ Function ด้วยข้อมูล:

- `"Rust"`
- `"Programming"`

และทดลองเปลี่ยนข้อความเพื่อให้แต่ละ Input เป็นข้อความที่ยาวกว่า

### Task 3: Create First Word Function

สร้าง Function ชื่อ `first_word` ที่:

- รับ `&str`
- คืนค่า `&str`
- คืนค่าคำแรกของข้อความ
- หากไม่มีช่องว่าง ให้คืนค่าข้อความทั้งหมด

ทดลองกับข้อความที่มีหลายคำและข้อความที่มีเพียงคำเดียว

### Task 4: Multiple Lifetime Parameters

สร้าง Function ชื่อ `select_first` ที่:

- รับ Reference สองตัว
- ใช้ Lifetime Parameters แยกกัน เช่น `'a` และ `'b`
- คืนค่า Reference จาก Parameter ตัวแรก
- กำหนด Return Lifetime ให้สัมพันธ์กับ Reference ตัวแรก

### Task 5: Explain Lifetime Relationships

อธิบายความแตกต่างระหว่าง:

- Lifetime ของ Input References
- Lifetime ของ Return Reference
- เหตุผลที่ Return Reference ต้องสัมพันธ์กับข้อมูลต้นทาง

## Constraints

- ต้องใช้ Lifetime Annotations อย่างน้อยสอง Function
- ต้องใช้ Lifetime Parameter `'a`
- ต้องทดลอง Function ที่รับ References มากกว่าหนึ่งตัว
- ต้องมี Function ที่คืนค่า Reference
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Variables
- ต้องไม่คืน Reference ไปยัง Local Variable ที่หมดอายุแล้ว
- โปรแกรมต้อง Compile และ Run ผ่าน

## Expected Output

ตัวอย่างผลลัพธ์:

```text
=== Longest Function ===
First: Rust
Second: Programming
Longest: Programming

=== First Word ===
Sentence: Rust is a systems programming language
First word: Rust

Sentence: Ownership
First word: Ownership

=== Select First ===
Selected: Rust

=== Lifetime Understanding ===
The returned reference must remain valid while it is used.
```

รูปแบบข้อความสามารถแตกต่างได้ แต่ต้องแสดงข้อมูลครบตามโจทย์

## Checklist

- [ ] สร้าง `longest()` ได้
- [ ] ใช้ Lifetime Annotation ใน Function ได้
- [ ] คืนค่า Reference ได้
- [ ] สร้าง `first_word()` ได้
- [ ] ใช้ Lifetime Parameters หลายตัวได้
- [ ] เข้าใจความสัมพันธ์ระหว่าง Input และ Output Lifetimes
- [ ] ไม่มี Dangling References
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 19_lifetimes/exercises/exercise_02/lifetime_functions
cargo run
```
