# Exercise 03: Copy and Clone

## Objective

ฝึกแยกความแตกต่างระหว่าง Copy และ Clone โดยทดลองกับ Integer และ String

## Project

`copy_clone`

## Tasks

### Part 1: Copy

1. สร้างตัวแปร `score1` เป็น Integer
2. กำหนดค่า `score1` ให้ `score2`
3. แสดงค่าของทั้งสองตัวแปร
4. ทดลองเปลี่ยนค่าของ `score2`
5. แสดงค่าของทั้งสองตัวแปรอีกครั้ง

### Part 2: Clone

1. สร้าง String ชื่อ `language1`
2. ใช้ `.clone()` สร้าง `language2`
3. แสดงค่าของทั้งสองตัวแปร
4. ทดลองเปลี่ยนแปลง `language2`
5. แสดงค่าของทั้งสองตัวแปรอีกครั้ง

### Part 3: Compare

เปรียบเทียบผลการทดลอง Part 1 และ Part 2 โดยตอบคำถาม:

- ทำไม `score1` ยังใช้งานได้หลังจากกำหนดค่าให้ `score2`?
- ทำไม String จึงใช้ `.clone()` ในการสร้างสำเนา?
- การเปลี่ยนแปลงข้อมูลของตัวแปรที่เป็นสำเนาส่งผลต่อตัวแปรต้นฉบับหรือไม่?

## Requirements

- ต้องมี Integer อย่างน้อย 2 ตัวแปร
- ต้องมี String อย่างน้อย 2 ตัวแปร
- ต้องใช้ `.clone()` อย่างน้อย 1 ครั้ง
- ต้องทดลองเปลี่ยนแปลงค่าของตัวแปร
- ต้องแสดงผลก่อนและหลังการเปลี่ยนแปลง

## Expected Output

รูปแบบผลลัพธ์ตัวอย่าง:

```text
Before Change:
Score 1: 100
Score 2: 100

After Change:
Score 1: 100
Score 2: 200

Language 1: Rust
Language 2: Rust Programming
```

## Constraints

- ใช้ Integer สำหรับทดลอง Copy
- ใช้ String สำหรับทดลอง Clone
- ห้ามใช้ `.clone()` กับ Integer ใน Part 1
- ต้องสังเกตว่าข้อมูลต้นฉบับและสำเนาเป็นอิสระจากกัน

## Checklist

- [ ] ทดลอง Copy ได้
- [ ] ทดลอง Clone ได้
- [ ] ใช้ `.clone()` ได้
- [ ] เปลี่ยนแปลงค่าของตัวแปรได้
- [ ] อธิบายความแตกต่างระหว่าง Copy และ Clone ได้

## Run Project

```powershell
cargo check
cargo run
```
