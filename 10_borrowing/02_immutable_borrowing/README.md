# Lesson 02: Immutable Borrowing

## 1. What is Immutable Borrowing?

Immutable Borrowing คือการยืมข้อมูลเพื่ออ่าน โดยไม่อนุญาตให้แก้ไขข้อมูลผ่าน Reference นั้น

ใช้เครื่องหมาย `&`

ตัวอย่าง:

```rust
fn main() {
    let language = String::from("Rust");

    let reference1 = &language;
    let reference2 = &language;

    println!("{}", reference1);
    println!("{}", reference2);
}
```

คำอธิบาย:

- `reference1` ยืมข้อมูลจาก `language`
- `reference2` ยืมข้อมูลจาก `language` เช่นกัน
- สามารถมี Immutable References หลายตัวพร้อมกันได้
- Reference ทั้งสองใช้ข้อมูลเพื่ออ่านได้

## 2. Immutable Reference

รูปแบบการประกาศ:

```rust
let variable = &value;
```

ตัวอย่าง:

```rust
fn main() {
    let score = 100;

    let reference = &score;

    println!("Score: {}", reference);
}
```

Immutable Reference ไม่อนุญาตให้แก้ไขข้อมูลผ่าน Reference นั้น

## 3. Immutable Borrowing with Function

Function สามารถรับ Immutable Reference เพื่ออ่านข้อมูลได้

ตัวอย่าง:

```rust
fn show_message(message: &String) {
    println!("Message: {}", message);
}

fn main() {
    let text = String::from("Learning Rust");

    show_message(&text);

    println!("Original: {}", text);
}
```

คำอธิบาย:

- Function รับ `&String`
- Function สามารถอ่าน String ได้
- Function ไม่ได้เป็นเจ้าของ String
- ตัวแปร `text` ยังใช้งานได้หลังเรียก Function

## 4. Multiple Immutable References

สามารถสร้าง Immutable References หลายตัวที่อ้างถึงข้อมูลเดียวกันได้

ตัวอย่าง:

```rust
fn main() {
    let message = String::from("Hello Rust");

    let reference1 = &message;
    let reference2 = &message;
    let reference3 = &message;

    println!("{}", reference1);
    println!("{}", reference2);
    println!("{}", reference3);
}
```

การยืมแบบอ่านอย่างเดียวหลายตัวพร้อมกันสามารถทำได้ เพราะไม่มี Reference ใดแก้ไขข้อมูล

## 5. Immutable vs Mutable

| Immutable Borrowing          | Mutable Borrowing                                        |
| ---------------------------- | -------------------------------------------------------- |
| `&T`                         | `&mut T`                                                 |
| ใช้อ่านข้อมูล                | ใช้อ่านและแก้ไขข้อมูล                                    |
| มีหลาย Reference พร้อมกันได้ | มี Mutable Reference ที่ใช้งานทับซ้อนกันได้เพียงหนึ่งตัว |
| ไม่แก้ไขผ่าน Reference       | แก้ไขผ่าน Reference ได้                                  |

## 6. Practice

ทดลองเขียนโปรแกรมที่:

1. สร้าง String ชื่อ `message`
2. สร้าง Immutable Reference 2 ตัว
3. แสดงข้อมูลผ่าน Reference ทั้งสองตัว
4. สร้าง Function ชื่อ `show_message`
5. ให้ Function รับ `&String`
6. ส่ง Reference เข้า Function
7. แสดงค่า String ต้นฉบับหลังเรียก Function

## 7. Knowledge Check

- Immutable Borrowing คืออะไร?
- เครื่องหมาย `&` ใช้ทำอะไร?
- Immutable Reference สามารถแก้ไขข้อมูลได้หรือไม่?
- สามารถสร้าง Immutable References หลายตัวพร้อมกันได้หรือไม่?
- Function ที่รับ `&String` สามารถย้าย Ownership ของ String ได้หรือไม่?

## 8. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Immutable Borrowing
- [ ] สร้าง Immutable Reference ได้
- [ ] สร้าง Immutable References หลายตัวได้
- [ ] ใช้ Reference กับ Function ได้
- [ ] เข้าใจว่า Immutable Reference ใช้แก้ไขข้อมูลไม่ได้
