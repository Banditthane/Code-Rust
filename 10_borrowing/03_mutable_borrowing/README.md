# Lesson 03: Mutable Borrowing

## 1. What is Mutable Borrowing?

Mutable Borrowing คือการยืมข้อมูลโดยอนุญาตให้แก้ไขข้อมูลผ่าน Reference ได้

ใช้เครื่องหมาย `&mut`

ตัวอย่าง:

```rust
fn main() {
    let mut language = String::from("Rust");

    let reference = &mut language;

    reference.push_str(" Programming");

    println!("{}", reference);
}
```

คำอธิบาย:

- `language` ต้องประกาศด้วย `mut` เพื่อให้แก้ไขค่าได้
- `&mut language` สร้าง Mutable Reference
- `push_str()` เพิ่มข้อความเข้าไปใน String
- สามารถแก้ไขข้อมูลผ่าน `reference` ได้

## 2. Mutable Variable vs Mutable Reference

สองสิ่งนี้เกี่ยวข้องกัน แต่มีหน้าที่ต่างกัน

| หัวข้อ    | ความหมาย                                           |
| --------- | -------------------------------------------------- |
| `let mut` | อนุญาตให้ตัวแปรแก้ไขค่าได้                         |
| `&mut`    | สร้าง Reference ที่อนุญาตให้ยืมข้อมูลเพื่อแก้ไขได้ |

ตัวอย่าง:

```rust
fn main() {
    let mut score = 100;

    let reference = &mut score;

    *reference = 200;

    println!("Score: {}", score);
}
```

คำอธิบาย:

- `score` เป็น Mutable Variable
- `reference` เป็น Mutable Reference
- `*reference = 200` แก้ไขค่าที่ Reference อ้างถึง
- ค่า `score` จึงเปลี่ยนเป็น 200

## 3. Mutable Borrowing with Function

Function สามารถรับ Mutable Reference เพื่อแก้ไขข้อมูลของตัวแปรเจ้าของได้

ตัวอย่าง:

```rust
fn add_score(score: &mut i32) {
    *score += 10;
}

fn main() {
    let mut score = 100;

    add_score(&mut score);

    println!("Score: {}", score);
}
```

คำอธิบาย:

- `score: &mut i32` รับ Mutable Reference
- `&mut score` ส่ง Mutable Reference เข้า Function
- `*score += 10` แก้ไขค่าที่ Reference อ้างถึง
- เมื่อ Function ทำงานเสร็จ ค่าเจ้าของข้อมูลจะเป็น 110

## 4. Only One Active Mutable Reference

ในช่วงเวลาที่มีการยืมแบบ Mutable ใช้งานอยู่ จะไม่สามารถสร้าง Reference อื่นที่ขัดแย้งกับการยืมนั้นได้

ตัวอย่างที่ถูกต้อง:

```rust
fn main() {
    let mut number = 10;

    let reference = &mut number;

    *reference += 5;

    println!("{}", reference);
}
```

หลังจาก Reference ถูกใช้งานเสร็จแล้ว จึงสามารถยืมข้อมูลใหม่ได้ตามกฎ Borrowing

## 5. Mutable Borrowing and String

String สามารถแก้ไขผ่าน Mutable Reference ได้

Method ที่ใช้บ่อย:

| Method       | Description              |
| ------------ | ------------------------ |
| `push()`     | เพิ่ม Character หนึ่งตัว |
| `push_str()` | เพิ่มข้อความ             |
| `clear()`    | ลบข้อความทั้งหมด         |

ตัวอย่าง:

```rust
fn main() {
    let mut message = String::from("Hello");

    let reference = &mut message;

    reference.push_str(" Rust");
    reference.push('!');

    println!("{}", reference);
}
```

## 6. Practice

ทดลองเขียนโปรแกรมที่:

1. สร้าง Mutable String ชื่อ `language`
2. สร้าง Mutable Reference
3. เพิ่มข้อความด้วย `push_str()`
4. เพิ่ม Character ด้วย `push()`
5. สร้าง Function ชื่อ `update_score`
6. ให้ Function รับ Mutable Reference ของ Integer
7. เพิ่มคะแนนภายใน Function
8. แสดงค่าหลังจากแก้ไข

## 7. Knowledge Check

- Mutable Borrowing คืออะไร?
- เครื่องหมาย `&mut` ใช้ทำอะไร?
- เพราะเหตุใดตัวแปรเจ้าของจึงต้องประกาศด้วย `mut`?
- `*reference` ใช้ทำอะไร?
- สามารถมี Mutable References ที่ใช้งานทับซ้อนกันหลายตัวได้หรือไม่?
- Function สามารถแก้ไขข้อมูลผ่าน Mutable Reference ได้หรือไม่?

## 8. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Mutable Borrowing
- [ ] สร้าง Mutable Reference ได้
- [ ] แก้ไขข้อมูลผ่าน Reference ได้
- [ ] ใช้ Mutable Reference กับ Function ได้
- [ ] เข้าใจข้อจำกัดของ Mutable Reference
