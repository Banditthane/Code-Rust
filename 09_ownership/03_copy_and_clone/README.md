# Lesson 03: Copy and Clone

## 1. What is Copy?

Copy เป็น Trait ที่อนุญาตให้ค่าบางประเภทถูกคัดลอกเมื่อกำหนดค่าให้ตัวแปรใหม่

ตัวอย่าง:

```rust
fn main() {
    let number1 = 100;

    let number2 = number1;

    println!("Number 1: {}", number1);
    println!("Number 2: {}", number2);
}
```

คำอธิบาย:

- `number1` เก็บค่า Integer
- `number2` ได้รับสำเนาของค่า
- `number1` ยังสามารถใช้งานได้
- Integer เป็นประเภทข้อมูลที่มี Copy Trait

## 2. Examples of Copy Types

ประเภทข้อมูลที่มักมี Copy ได้แก่:

- Integer เช่น `i32`, `u32`
- Floating-point เช่น `f32`, `f64`
- Boolean เช่น `bool`
- Character เช่น `char`
- Tuple ที่สมาชิกทุกตัวเป็นประเภท Copy

ตัวอย่าง:

```rust
let age = 25;
let age2 = age;

let active = true;
let active2 = active;

let point = (10, 20);
let point2 = point;
```

## 3. What is Clone?

Clone คือการสร้างสำเนาของข้อมูลอย่างชัดเจนด้วย Method `.clone()`

ตัวอย่าง:

```rust
fn main() {
    let name1 = String::from("Rust");

    let name2 = name1.clone();

    println!("Name 1: {}", name1);
    println!("Name 2: {}", name2);
}
```

คำอธิบาย:

- `name1` เป็นเจ้าของ String เดิม
- `.clone()` สร้างสำเนาของ String
- `name2` เป็นเจ้าของ String ที่ถูกคัดลอก
- ทั้งสองตัวแปรสามารถใช้งานได้

## 4. Copy vs Clone

| หัวข้อ    | Copy                            | Clone                        |
| --------- | ------------------------------- | ---------------------------- |
| วิธีทำงาน | คัดลอกค่าโดยอัตโนมัติเมื่อ Copy | เรียก `.clone()` อย่างชัดเจน |
| การใช้งาน | ประเภทข้อมูลที่รองรับ Copy      | ประเภทข้อมูลที่รองรับ Clone  |
| String    | ไม่ใช่ Copy                     | ใช้ Clone ได้                |
| Ownership | ไม่เกิด Move เมื่อ Copy         | ได้ข้อมูลสำเนาแยกจากต้นฉบับ  |

## 5. Why String Does Not Implement Copy

String เป็นประเภทข้อมูลที่จัดการข้อมูลข้อความบน Heap

หาก String ถูก Copy แบบอัตโนมัติ อาจทำให้มีเจ้าของข้อมูลเดียวกันหลายตัวโดยไม่มีการจัดการที่เหมาะสม

Rust จึงใช้ Move เป็นค่าเริ่มต้นเมื่อกำหนด String ให้ตัวแปรใหม่ และให้ผู้ใช้เลือก `.clone()` เมื่อต้องการสร้างสำเนา

## 6. Practice

ทดลองเขียนโปรแกรมที่:

1. สร้างตัวแปร Integer ชื่อ `score1`
2. กำหนดค่าให้ `score2` จาก `score1`
3. ทดลองแสดงค่าของทั้งสองตัวแปร
4. สร้าง String ชื่อ `language1`
5. ใช้ `.clone()` สร้าง `language2`
6. ทดลองแสดงค่าของทั้งสองตัวแปร

## 7. Knowledge Check

- Copy คืออะไร?
- Clone คืออะไร?
- Integer สามารถ Copy ได้หรือไม่?
- String สามารถ Copy ได้หรือไม่?
- `.clone()` ใช้ทำอะไร?
- Copy และ Clone ต่างกันอย่างไร?

## 8. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Copy Trait
- [ ] รู้จักประเภทข้อมูลที่มี Copy
- [ ] เข้าใจ Clone
- [ ] ใช้ `.clone()` ได้
- [ ] อธิบายความแตกต่างระหว่าง Copy และ Clone ได้
