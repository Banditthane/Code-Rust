# Contributing Guide

ข้อตกลงและแนวทางสำหรับการเพิ่มบทเรียน แบบฝึกหัด และ Mini Projects ใน Repository `proj-code-rust`

## 1. Learning Principles

การเรียนรู้ใน Repository นี้ใช้แนวทาง:

1. Learn — ศึกษาแนวคิดและตัวอย่าง
2. Practice — ทดลองเขียนโค้ดและทำแบบฝึกหัด
3. Test — ตรวจสอบผลลัพธ์และแก้ไขข้อผิดพลาด
4. Review — ทบทวนและอธิบายสิ่งที่เรียนรู้
5. Build — นำความรู้ไปสร้างโปรแกรม

## 2. Chapter Structure

บทเรียนแต่ละบทควรแบ่งหัวข้อย่อยตามแนวคิด เช่น

```text
07_functions/
├── 01_function_basics/
├── 02_parameters/
├── 03_return_values/
└── 04_practical_exercises/
```

## 3. Exercise Requirements

แบบฝึกหัดแต่ละข้อควรมี:

- Objective — เป้าหมายของโจทย์
- Concept — แนวคิดที่ต้องใช้
- Problem — รายละเอียดโจทย์
- Requirements — เงื่อนไขของโปรแกรม
- Expected Output — ผลลัพธ์ที่คาดหวัง
- Acceptance Criteria — เกณฑ์ตรวจสอบคำตอบ

## 4. Coding Standards

- ใช้ชื่อฟังก์ชันและตัวแปรที่สื่อความหมาย
- ใช้ `snake_case` สำหรับ functions และ variables
- จัดรูปแบบโค้ดด้วย `cargo fmt`
- ตรวจสอบโค้ดด้วย `cargo clippy`
- เขียนโค้ดให้อ่านง่ายและหลีกเลี่ยงความซับซ้อนที่ไม่จำเป็น

## 5. Testing

ก่อนส่งหรือบันทึกงาน ควรรันคำสั่ง:

```powershell
cargo check
cargo test
cargo fmt --check
cargo clippy
```

## 6. Commit Convention

ใช้รูปแบบข้อความ Commit ดังนี้:

| Prefix      | Usage                 |
| ----------- | --------------------- |
| `learn:`    | เพิ่มบทเรียน          |
| `exercise:` | เพิ่มหรือแก้แบบฝึกหัด |
| `solution:` | เพิ่มเฉลย             |
| `project:`  | เพิ่ม Mini Project    |
| `docs:`     | แก้ไขเอกสาร           |
| `fix:`      | แก้ไขข้อผิดพลาด       |

ตัวอย่าง:

```text
learn: add rust variables lesson
exercise: add function practice
docs: update learning roadmap
fix: correct loop exercise
```

## 7. Learning Completion

ถือว่าบทเรียนเสร็จสมบูรณ์เมื่อ:

- เข้าใจแนวคิดหลัก
- เขียนตัวอย่างได้ด้วยตนเอง
- ทำแบบฝึกหัดได้
- ทดสอบโปรแกรมผ่าน
- อธิบายวิธีแก้ปัญหาได้

## 8. General Rule

ให้ความสำคัญกับความเข้าใจมากกว่าการทำโจทย์ให้เสร็จเร็ว และควรพยายามเขียนโค้ดด้วยตนเองก่อนดูเฉลย
