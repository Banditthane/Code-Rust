# 03: String Methods

## ภาพรวม

Rust มี Methods สำหรับตรวจสอบ ค้นหา และจัดการข้อความ ช่วยให้สามารถทำงานกับ String ได้สะดวกขึ้น

## 1. len()

ใช้ตรวจสอบความยาวของ String โดยผลลัพธ์เป็นจำนวนไบต์ (Bytes)

ตัวอย่าง:

```rust
fn main() {
    let message = String::from("Hello");

    println!("Length: {}", message.len());
}
```

ผลลัพธ์:

```text
Length: 5
```

ข้อควรรู้:

`len()` นับจำนวนไบต์ ไม่ใช่จำนวนตัวอักษรเสมอไป เพราะ Rust ใช้การเข้ารหัส UTF-8

ตัวอย่าง:

```rust
fn main() {
    let message = String::from("สวัสดี");

    println!("Length: {}", message.len());
    println!("Characters: {}", message.chars().count());
}
```

ค่า `len()` และ `chars().count()` อาจแตกต่างกัน เพราะตัวอักษรภาษาไทยหนึ่งตัวอาจใช้มากกว่าหนึ่งไบต์

## 2. is_empty()

ใช้ตรวจสอบว่า String ว่างหรือไม่

ตัวอย่าง:

```rust
fn main() {
    let message = String::new();

    println!("Is empty: {}", message.is_empty());
}
```

ผลลัพธ์:

```text
Is empty: true
```

## 3. contains()

ใช้ตรวจสอบว่าข้อความมีคำหรือข้อความที่กำหนดอยู่หรือไม่

ตัวอย่าง:

```rust
fn main() {
    let message = String::from("Hello Rust");

    println!("{}", message.contains("Rust"));
    println!("{}", message.contains("Java"));
}
```

ผลลัพธ์:

```text
true
false
```

## 4. replace()

ใช้แทนที่ข้อความเดิมด้วยข้อความใหม่

ตัวอย่าง:

```rust
fn main() {
    let message = String::from("Hello Java");

    let new_message = message.replace("Java", "Rust");

    println!("{}", new_message);
}
```

ผลลัพธ์:

```text
Hello Rust
```

`replace()` จะสร้าง String ใหม่ โดยไม่ได้เปลี่ยน String ต้นฉบับ

## 5. trim()

ใช้ตัดช่องว่างและอักขระเว้นวรรคที่อยู่บริเวณต้นและท้ายข้อความ

ตัวอย่าง:

```rust
fn main() {
    let message = "   Hello Rust   ";

    let result = message.trim();

    println!("{}", result);
}
```

ผลลัพธ์:

```text
Hello Rust
```

## 6. to_uppercase()

ใช้แปลงตัวอักษรเป็นตัวพิมพ์ใหญ่

```rust
fn main() {
    let message = "hello rust";

    println!("{}", message.to_uppercase());
}
```

ผลลัพธ์:

```text
HELLO RUST
```

## 7. to_lowercase()

ใช้แปลงตัวอักษรเป็นตัวพิมพ์เล็ก

```rust
fn main() {
    let message = "HELLO RUST";

    println!("{}", message.to_lowercase());
}
```

ผลลัพธ์:

```text
hello rust
```

## ตารางสรุป String Methods

| Method            | หน้าที่                              |
| ----------------- | ------------------------------------ |
| `len()`           | นับจำนวนไบต์                         |
| `is_empty()`      | ตรวจสอบข้อความว่าง                   |
| `contains()`      | ตรวจสอบว่ามีข้อความที่ต้องการหรือไม่ |
| `replace()`       | แทนที่ข้อความ                        |
| `trim()`          | ตัดช่องว่างบริเวณต้นและท้าย          |
| `to_uppercase()`  | แปลงเป็นตัวพิมพ์ใหญ่                 |
| `to_lowercase()`  | แปลงเป็นตัวพิมพ์เล็ก                 |
| `chars().count()` | นับจำนวน Unicode Scalar Values       |

## สิ่งที่ควรรู้

- `len()` นับจำนวนไบต์ ไม่ใช่จำนวนตัวอักษรที่มองเห็น
- `chars().count()` นับ Unicode Scalar Values ไม่จำเป็นต้องเท่ากับจำนวนตัวอักษรที่มองเห็นเสมอไป
- `contains()` คืนค่าเป็น `bool`
- `replace()` สร้าง String ใหม่
- `trim()` คืนค่าเป็น String Slice (`&str`)
- `to_uppercase()` และ `to_lowercase()` สร้าง String ใหม่

## แบบฝึกหัด

1. สร้าง String ชื่อ `message` โดยกำหนดข้อความเป็น `Hello Rust`
2. ใช้ `len()` แสดงความยาวข้อความ
3. ใช้ `contains()` ตรวจสอบว่ามีคำว่า `Rust` หรือไม่
4. ใช้ `replace()` เปลี่ยนคำว่า `Rust` เป็น `Programming`
5. ใช้ `to_uppercase()` แปลงข้อความเป็นตัวพิมพ์ใหญ่
6. ใช้ `to_lowercase()` แปลงข้อความเป็นตัวพิมพ์เล็ก
7. สร้าง String ว่างและตรวจสอบด้วย `is_empty()`
8. ทดลองใช้ `trim()` กับข้อความที่มีช่องว่างด้านหน้าและด้านหลัง

## คำถามทบทวน

1. `len()` ใช้นับอะไร?
2. `chars().count()` แตกต่างจาก `len()` อย่างไร?
3. `contains()` ส่งค่ากลับเป็นชนิดข้อมูลอะไร?
4. `replace()` เปลี่ยน String ต้นฉบับโดยตรงหรือไม่?
5. `trim()` ทำงานอย่างไร?
