# Exercise 04: Ownership Challenge

## Objective

ประยุกต์ใช้ Ownership, Move, Clone, Scope และ Function ในโปรแกรมเดียวกัน

## Project

`ownership_challenge`

## Scenario

สร้างโปรแกรมจัดเก็บข้อมูลภาษาที่กำลังเรียน โดยมีข้อมูลชื่อภาษาและคะแนนความสนใจ

## Tasks

### Part 1: Create Data

สร้างตัวแปรดังนี้:

- `language` เป็น String
- `score` เป็น Integer
- `active` เป็น Boolean

แสดงค่าของตัวแปรทั้งหมด

### Part 2: Clone Data

1. ใช้ `.clone()` สร้างตัวแปร `language_copy`
2. แสดงค่าของ String ต้นฉบับและสำเนา
3. ทดลองเปลี่ยนแปลง `language_copy`
4. ตรวจสอบว่า `language` ต้นฉบับยังคงเดิมหรือไม่

### Part 3: Move Data into Function

สร้าง Function:

```rust
fn show_language(language: String) {
    // Display language
}
```

สิ่งที่ต้องทำ:

1. ส่ง String เข้า Function
2. แสดงข้อมูลภายใน Function
3. สังเกตการเปลี่ยนแปลง Ownership
4. ทดลองส่งข้อมูลกลับด้วย Return Value ใน Function อีกชุดหนึ่ง

### Part 4: Scope and Drop

1. สร้าง Scope ด้านใน
2. ประกาศ String ภายใน Scope
3. แสดงค่าของ String
4. ออกจาก Scope
5. อธิบายว่าเกิดอะไรขึ้นกับ String

## Requirements

- ต้องใช้ `String`
- ต้องใช้ Integer และ Boolean
- ต้องใช้ `.clone()`
- ต้องมี Function อย่างน้อย 2 Function
- ต้องมีการ Move String เข้า Function
- ต้องมี Function ที่คืนค่า String
- ต้องมี Scope ด้านใน

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Original Language: Rust
Copied Language: Rust

Updated Copy: Rust Programming
Original Language: Rust

Function received: Python
Returned Language: Java

Inside Scope: C++
Ownership Challenge Completed
```

## Constraints

- ต้องไม่ใช้ตัวแปรหลังจาก Ownership ถูก Move ไปแล้ว
- ใช้ `.clone()` เมื่อต้องการสร้างสำเนา String
- ต้องแยกข้อมูลที่ถูก Move และข้อมูลที่ถูก Clone ให้ชัดเจน
- ต้องไม่เรียกใช้ตัวแปรที่อยู่นอก Scope
- ไม่ใช้ Global Variable

## Final Questions

ตอบคำถามต่อไปนี้ใน README หรือบันทึกการเรียนของตนเอง:

1. Ownership คืออะไร?
2. Move และ Clone ต่างกันอย่างไร?
3. เพราะเหตุใด String จึงไม่ใช่ Copy Type?
4. เมื่อ String ถูกส่งเข้า Function จะเกิดอะไรขึ้น?
5. เมื่อออกจาก Scope จะเกิดอะไรขึ้นกับข้อมูล?
6. เราควรใช้ Clone ในกรณีใด?

## Checklist

- [ ] สร้างข้อมูลได้ครบ
- [ ] ใช้ Clone ได้
- [ ] เข้าใจ Move เข้า Function
- [ ] คืน Ownership จาก Function ได้
- [ ] ใช้ Scope ได้
- [ ] เข้าใจ Drop
- [ ] ตอบ Final Questions ได้

## Run Project

```powershell
cargo check
cargo run
```
