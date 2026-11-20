# Lesson 02: Move Semantics

## 1. What is Move?

Move คือการย้าย Ownership จากตัวแปรหนึ่งไปยังอีกตัวแปรหนึ่ง

เมื่อเกิด Move ตัวแปรเดิมจะไม่สามารถใช้งานข้อมูลที่ถูกย้ายไปแล้วได้อีก

ตัวอย่าง:

```rust
fn main() {
    let name1 = String::from("Rust");

    let name2 = name1;

    println!("{}", name2);
}
```

คำอธิบาย:

- `name1` เป็นเจ้าของ String
- `name2 = name1` ทำให้ Ownership ถูกย้ายไปยัง `name2`
- `name2` กลายเป็นเจ้าของ String
- `name1` ไม่สามารถใช้งาน String เดิมได้อีก

## 2. Why Does Rust Move Ownership?

Rust ใช้ Move เพื่อป้องกันการมีเจ้าของข้อมูลเดียวกันหลายตัวโดยไม่มีกลไกควบคุม

หากตัวแปรหลายตัวสามารถจัดการข้อมูลเดียวกันบน Heap ได้อย่างอิสระ อาจทำให้เกิดปัญหาในการจัดการหน่วยความจำ

Move จึงเป็นกลไกที่ช่วยให้ Rust ควบคุมการเป็นเจ้าของข้อมูลได้อย่างปลอดภัย

## 3. Move with Function

การส่ง String เข้า Function โดยตรงจะเป็นการย้าย Ownership ไปยัง Parameter ของ Function

ตัวอย่าง:

```rust
fn print_name(name: String) {
    println!("Name: {}", name);
}

fn main() {
    let name = String::from("Rust");

    print_name(name);
}
```

คำอธิบาย:

- `name` ใน `main()` เป็นเจ้าของ String ก่อนเรียก Function
- เมื่อส่ง `name` เข้า `print_name()` จะเกิด Move
- Parameter `name` ใน Function จะกลายเป็นเจ้าของข้อมูล
- เมื่อ Function จบ Scope ข้อมูลจะถูก Drop

## 4. Returning Ownership

เราสามารถส่ง Ownership กลับจาก Function ด้วย Return Value ได้

ตัวอย่าง:

```rust
fn create_name() -> String {
    String::from("Rust")
}

fn main() {
    let name = create_name();

    println!("{}", name);
}
```

ค่า String ที่ Function ส่งกลับมาจะมี `name` เป็นเจ้าของ

## 5. Move vs Borrow

| Move                            | Borrow                                     |
| ------------------------------- | ------------------------------------------ |
| ย้าย Ownership                  | ยืมข้อมูล                                  |
| เจ้าของเดิมใช้ข้อมูลไม่ได้      | เจ้าของเดิมยังเป็นเจ้าของ                  |
| ใช้ค่าจากตัวแปรที่ถูกย้ายไม่ได้ | ใช้ข้อมูลผ่าน Reference ได้ตามกฎ Borrowing |

Borrowing จะเรียนอย่างละเอียดในบทที่ 10

## 6. Practice

ทดลองเขียนโปรแกรมที่:

1. สร้าง String ชื่อ `language`
2. กำหนดให้ `language2` รับค่าจาก `language`
3. ทดลองใช้ `language2`
4. สังเกตว่า `language` ยังสามารถใช้งานได้หรือไม่
5. สร้าง Function ที่รับ String เป็น Parameter
6. ทดลองส่ง String เข้า Function

หมายเหตุ: ให้ทดลองกรณีที่เกิด Move โดยแยกเป็นการทดลองคนละชุด หากต้องการรันโปรแกรมโดยไม่เกิด Compile Error

## 7. Knowledge Check

- Move คืออะไร?
- เมื่อเกิด Move ตัวแปรเดิมยังใช้งานข้อมูลได้หรือไม่?
- การส่ง String เข้า Function เกิดอะไรขึ้น?
- Function สามารถส่ง Ownership กลับได้หรือไม่?

## 8. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Move Semantics
- [ ] เข้าใจการย้าย Ownership ระหว่างตัวแปร
- [ ] เข้าใจ Move เมื่อส่งค่าเข้า Function
- [ ] เข้าใจการ Return Ownership
- [ ] สามารถอธิบายได้ว่าทำไมตัวแปรเดิมจึงใช้งานไม่ได้หลัง Move
