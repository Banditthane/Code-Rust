# Exercise 04: Operator Challenge

## Objective

ฝึกประยุกต์ใช้ Operators หลายประเภทในโปรแกรมเดียว

## Scenario

สร้างโปรแกรมคำนวณคะแนนและตรวจสอบสถานะของนักเรียน

## Requirements

สร้างตัวแปร:

| Variable        | Value | Type  |
| --------------- | ----: | ----- |
| `math_score`    |    80 | `u32` |
| `science_score` |    70 | `u32` |
| `english_score` |    90 | `u32` |
| `bonus`         |    10 | `u32` |
| `passing_score` |    50 | `u32` |

### Part 1: Arithmetic

1. คำนวณคะแนนรวมทั้งสามวิชา
2. คำนวณคะแนนเฉลี่ย
3. เพิ่มคะแนนโบนัสเข้าไปในคะแนนรวม

### Part 2: Assignment

1. ใช้ `+=` เพื่อเพิ่มโบนัส
2. ทดลองใช้ `-=` เพื่อลดคะแนน
3. แสดงคะแนนหลังปรับค่า

### Part 3: Comparison

ตรวจสอบ:

1. คะแนนรวมมากกว่า 200 หรือไม่
2. คะแนนเฉลี่ยมากกว่าหรือเท่ากับคะแนนผ่านหรือไม่

### Part 4: Logical

ตรวจสอบว่านักเรียนผ่านเกณฑ์หรือไม่ โดยกำหนดให้:

- คะแนนเฉลี่ยอย่างน้อย 50
- คะแนนวิชาคณิตศาสตร์อย่างน้อย 50
- คะแนนวิทยาศาสตร์อย่างน้อย 50

### Part 5: Bitwise

สร้างตัวแปรจำนวนเต็มชนิด `u8` สองตัว แล้วทดลองใช้:

- `&`
- `|`
- `^`
- `<<`
- `>>`

## Requirements Checklist

- [ ] ใช้ Arithmetic Operators
- [ ] ใช้ Assignment Operators
- [ ] ใช้ Comparison Operators
- [ ] ใช้ Logical Operators
- [ ] ใช้ Bitwise Operators
- [ ] แสดงผลลัพธ์ของแต่ละส่วน
- [ ] โปรแกรมทำงานได้โดยไม่มี Compile Error

## Run

```powershell
cargo run
```

## Challenge

ลองเปลี่ยนคะแนนและโบนัส แล้วตรวจสอบว่าผลลัพธ์เปลี่ยนไปอย่างไร
