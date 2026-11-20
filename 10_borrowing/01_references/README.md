# Lesson 01: References

## 1. What is a Reference?

Reference คือการอ้างอิงไปยังข้อมูลที่มีอยู่ โดยไม่ต้องย้าย Ownership จากตัวแปรเจ้าของข้อมูล

เครื่องหมาย `&` ใช้สร้าง Reference

ตัวอย่าง:

```rust
fn main() {
    let name = String::from("Rust");

    let reference = &name;

    println!("Name: {}", name);
    println!("Reference: {}", reference);
}
```

คำอธิบาย:

- `name` เป็นเจ้าของ String
- `&name` สร้าง Reference ไปยังข้อมูลของ `name`
- `reference` ยืมข้อมูลมาใช้งาน
- Ownership ยังคงเป็นของ `name`

## 2. Borrowing

Borrowing คือการยืมข้อมูลผ่าน Reference

ข้อดีคือสามารถส่งข้อมูลให้ Function หรือใช้งานข้อมูลจากตัวแปรอื่นได้ โดยไม่จำเป็นต้องย้าย Ownership

ตัวอย่าง:

```rust
fn show_name(name: &String) {
    println!("Name: {}", name);
}

fn main() {
    let name = String::from("Rust");

    show_name(&name);

    println!("Original: {}", name);
}
```

คำอธิบาย:

- `&String` หมายถึง Function รับ Reference ไปยัง String
- `&name` ส่ง Reference เข้า Function
- Function อ่านข้อมูลได้โดยไม่รับ Ownership
- หลังเรียก Function แล้ว `name` ยังคงใช้งานได้

## 3. Reference vs Ownership

| หัวข้อ         | Ownership                 | Reference               |
| -------------- | ------------------------- | ----------------------- |
| ความหมาย       | เป็นเจ้าของข้อมูล         | อ้างอิงหรือยืมข้อมูล    |
| สัญลักษณ์      | ไม่มีสัญลักษณ์พิเศษ       | `&`                     |
| การเป็นเจ้าของ | รับผิดชอบข้อมูล           | ไม่ได้เป็นเจ้าของข้อมูล |
| การใช้งาน      | ใช้ข้อมูลที่ตนเป็นเจ้าของ | ใช้ข้อมูลผ่านการยืม     |

## 4. Reference and Function

Function สามารถรับ Reference เป็น Parameter ได้

ตัวอย่าง:

```rust
fn calculate_length(text: &String) -> usize {
    text.len()
}

fn main() {
    let message = String::from("Hello Rust");

    let length = calculate_length(&message);

    println!("Message: {}", message);
    println!("Length: {}", length);
}
```

คำอธิบาย:

- `text: &String` รับ Reference ไปยัง String
- `text.len()` คืนค่าความยาวเป็นจำนวน Byte
- Function ไม่ได้ย้าย Ownership ของ `message`
- `message` ยังใช้งานได้หลังเรียก Function

ข้อควรรู้: หาก Function เพียงต้องการอ่านข้อความ โดยทั่วไปการรับ `&str` จะยืดหยุ่นกว่าการรับ `&String` ซึ่งจะได้เรียนและฝึกเพิ่มเติมในบทถัดไป

## 5. Dereferencing

Dereferencing คือการเข้าถึงค่าที่ Reference อ้างถึง โดยใช้เครื่องหมาย `*`

ตัวอย่าง:

```rust
fn main() {
    let number = 100;

    let reference = &number;

    println!("Number: {}", *reference);
}
```

คำอธิบาย:

- `reference` อ้างถึง `number`
- `*reference` ใช้เข้าถึงค่าที่ Reference อ้างถึง

ในหลายกรณี Rust สามารถจัดการ Reference ให้โดยอัตโนมัติเมื่อเรียกใช้งาน Method หรือแสดงผล จึงไม่จำเป็นต้องเขียน `*` ทุกครั้ง

## 6. Practice

ทดลองเขียนโปรแกรมที่:

1. สร้าง String ชื่อ `language`
2. สร้าง Reference ชื่อ `reference`
3. แสดง String ผ่านตัวแปรเจ้าของข้อมูล
4. แสดง String ผ่าน Reference
5. สร้าง Function ชื่อ `show_text`
6. กำหนดให้ Function รับ Reference ของ String
7. เรียก Function โดยส่ง Reference เข้าไป
8. ทดลองใช้งาน String หลังเรียก Function

## 7. Knowledge Check

- Reference คืออะไร?
- เครื่องหมายใดใช้สร้าง Reference?
- Borrowing ต่างจาก Move อย่างไร?
- Function สามารถรับ Reference ได้หรือไม่?
- Reference เป็นเจ้าของข้อมูลหรือไม่?
- Dereferencing ใช้เครื่องหมายอะไร?

## 8. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Reference
- [ ] สร้าง Reference ได้
- [ ] เข้าใจ Borrowing
- [ ] ส่ง Reference เข้า Function ได้
- [ ] เข้าใจ Dereferencing
