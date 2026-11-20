# 02: การสร้าง String

## ภาพรวม

บทนี้เรียนรู้วิธีสร้าง String และเพิ่มข้อความใน String โดยใช้คำสั่งและ Methods ที่ภาษา Rust เตรียมไว้ให้

## 1. สร้าง String ด้วย String::from()

ใช้ `String::from()` เพื่อสร้าง String จากข้อความที่กำหนด

ตัวอย่าง:

```rust
fn main() {
    let name = String::from("Rust");

    println!("{}", name);
}
```

ผลลัพธ์:

```text
Rust
```

## 2. สร้าง String ด้วย String::new()

ใช้ `String::new()` เพื่อสร้าง String ว่างที่ยังไม่มีข้อความ

ตัวอย่าง:

```rust
fn main() {
    let message = String::new();

    println!("Message: {}", message);
}
```

ผลลัพธ์:

```text
Message:
```

## 3. เพิ่มข้อความด้วย push_str()

ใช้ `push_str()` เพื่อเพิ่มข้อความชนิด `&str` ต่อท้าย String

ตัวอย่าง:

```rust
fn main() {
    let mut message = String::from("Hello");

    message.push_str(", Rust!");

    println!("{}", message);
}
```

ผลลัพธ์:

```text
Hello, Rust!
```

ข้อควรรู้: ต้องใช้ `mut` เพราะมีการเปลี่ยนแปลงเนื้อหาของ String

## 4. เพิ่มตัวอักษรด้วย push()

ใช้ `push()` เพื่อเพิ่มตัวอักษรชนิด `char` ต่อท้าย String

ตัวอย่าง:

```rust
fn main() {
    let mut message = String::from("Rust");

    message.push('!');

    println!("{}", message);
}
```

ผลลัพธ์:

```text
Rust!
```

ข้อแตกต่าง:

- `push_str()` รับข้อความชนิด `&str`
- `push()` รับตัวอักษรชนิด `char`

## 5. ต่อ String ด้วยเครื่องหมาย +

สามารถใช้เครื่องหมาย `+` เพื่อต่อ String กับข้อความชนิด `&str` ได้

ตัวอย่าง:

```rust
fn main() {
    let first = String::from("Hello");
    let second = " Rust";

    let result = first + second;

    println!("{}", result);
}
```

ผลลัพธ์:

```text
Hello Rust
```

ข้อควรระวัง: การใช้ `+` จะย้าย Ownership ของ String ตัวแรก จึงไม่สามารถนำตัวแปร `first` มาใช้งานต่อในรูปแบบเดิมได้หลังจากนำไปใช้กับ `+`

## สิ่งที่ควรรู้

- `String::from()` ใช้สร้าง String จากข้อความ
- `String::new()` ใช้สร้าง String ว่าง
- `push_str()` ใช้เพิ่มข้อความ
- `push()` ใช้เพิ่มตัวอักษร
- String ที่ต้องการแก้ไขต้องประกาศด้วย `mut`
- เครื่องหมาย `+` ใช้ต่อ String แต่มีเรื่อง Ownership เข้ามาเกี่ยวข้อง

## แบบฝึกหัด

1. สร้าง String ชื่อ `first_name` โดยกำหนดค่าเป็นชื่อของตนเอง
2. สร้าง String ว่างชื่อ `message`
3. ใช้ `push_str()` เพิ่มข้อความลงใน `message`
4. ใช้ `push()` เพิ่มเครื่องหมาย `!`
5. แสดงข้อความทั้งหมด
6. ทดลองสร้าง String อีกตัวแล้วต่อข้อความด้วยเครื่องหมาย `+`

## คำถามทบทวน

1. `String::from()` ใช้ทำอะไร?
2. `String::new()` ใช้ทำอะไร?
3. `push_str()` แตกต่างจาก `push()` อย่างไร?
4. เพราะเหตุใด String ที่ต้องการแก้ไขต้องใช้ `mut`?
5. การใช้เครื่องหมาย `+` กับ String มีผลต่อ Ownership อย่างไร?
