# Lesson 04: Borrowing Rules

## 1. Why Borrowing Rules?

Borrowing Rules คือกฎที่ Rust ใช้ควบคุมการยืมข้อมูล เพื่อป้องกันการเข้าถึงและแก้ไขข้อมูลที่อาจทำให้เกิดปัญหา

กฎเหล่านี้ช่วยให้ Rust ตรวจสอบความปลอดภัยของ Memory ได้ในระหว่าง Compile

## 2. Rule 1: Multiple Immutable References

สามารถมี Immutable References หลายตัวที่อ้างถึงข้อมูลเดียวกันได้

ตัวอย่าง:

```rust
fn main() {
    let message = String::from("Rust");

    let reference1 = &message;
    let reference2 = &message;

    println!("{}", reference1);
    println!("{}", reference2);
}
```

เหตุผล:

- Reference ทั้งสองใช้เพื่ออ่านข้อมูล
- ไม่มีการแก้ไขข้อมูล
- จึงสามารถใช้งานร่วมกันได้

## 3. Rule 2: One Active Mutable Borrow

สามารถมี Mutable Borrow ที่ใช้งานทับซ้อนกันได้เพียงหนึ่งตัวต่อข้อมูลหนึ่งชุด

ตัวอย่างที่ถูกต้อง:

```rust
fn main() {
    let mut message = String::from("Rust");

    let reference = &mut message;

    reference.push_str(" Programming");

    println!("{}", reference);
}
```

ไม่ควรสร้าง Mutable References หลายตัวที่มีช่วงเวลาการยืมทับซ้อนกันกับข้อมูลเดียวกัน

## 4. Rule 3: No Conflicting Mutable and Immutable Borrows

ไม่สามารถมี Immutable Reference และ Mutable Reference ที่ใช้งานทับซ้อนกันกับข้อมูลเดียวกันได้

ตัวอย่างที่ไม่ถูกต้อง:

```rust
fn main() {
    let mut number = 10;

    let reference1 = &number;
    let reference2 = &mut number;

    println!("{}", reference1);
    println!("{}", reference2);
}
```

เหตุผล:

- `reference1` ยืมข้อมูลเพื่ออ่าน
- `reference2` ยืมข้อมูลเพื่อแก้ไข
- การยืมทั้งสองแบบมีช่วงเวลาทับซ้อนกัน
- Rust จึงไม่อนุญาต

หมายเหตุ: ตัวอย่างนี้ตั้งใจให้เกิด Compile Error เพื่อศึกษา Borrowing Rules ไม่ใช่โปรแกรมที่ต้องรันผ่าน

## 5. Rule 4: References Must Not Outlive Their Data

Reference ต้องไม่ถูกใช้งานหลังจากข้อมูลที่มันอ้างถึงหมดอายุไปแล้ว

ตัวอย่างที่ถูกต้อง:

```rust
fn main() {
    let message = String::from("Rust");

    let reference = &message;

    println!("{}", reference);
}
```

ตัวอย่างที่ไม่ถูกต้อง:

```rust
fn main() {
    let reference;

    {
        let message = String::from("Rust");
        reference = &message;
    }

    println!("{}", reference);
}
```

เหตุผล:

- `message` ถูกสร้างใน Scope ด้านใน
- เมื่อออกจาก Scope ด้านใน `message` จะถูก Drop
- `reference` ไม่สามารถใช้งานข้อมูลที่ถูก Drop ไปแล้วได้

แนวคิดนี้เกี่ยวข้องกับ Lifetime ซึ่งจะเรียนอย่างละเอียดในบทที่ 19

## 6. Non-Lexical Lifetimes (NLL)

Rust ใช้ระบบ Non-Lexical Lifetimes (NLL) เพื่อพิจารณาช่วงเวลาการยืมจากการใช้งาน Reference จริง

ตัวอย่าง:

```rust
fn main() {
    let mut number = 10;

    let reference = &number;

    println!("{}", reference);

    let mutable_reference = &mut number;

    *mutable_reference += 5;

    println!("{}", mutable_reference);
}
```

ตัวอย่างนี้ทำงานได้ เพราะการใช้งาน Immutable Reference จบลงก่อนเริ่มใช้งาน Mutable Reference

## 7. Practice

ทดลองเขียนโปรแกรม 4 กรณี:

1. สร้าง Immutable References 2 ตัวจากข้อมูลเดียวกัน
2. สร้าง Mutable Reference 1 ตัวเพื่อแก้ไขข้อมูล
3. ทดลองสร้าง Immutable และ Mutable References ที่มีช่วงเวลาทับซ้อนกัน
4. ทดลองสร้าง Reference ที่อ้างถึงข้อมูลซึ่งออกจาก Scope ไปแล้ว

สังเกตว่าแต่ละกรณี Compile ผ่านหรือไม่ และอธิบายเหตุผล

## 8. Knowledge Check

- สามารถมี Immutable References หลายตัวได้หรือไม่?
- สามารถมี Mutable Reference ที่ใช้งานทับซ้อนกันหลายตัวได้หรือไม่?
- เหตุใด Immutable และ Mutable References จึงไม่สามารถใช้งานทับซ้อนกันได้?
- Dangling Reference คืออะไร?
- NLL คืออะไร?
- Reference สามารถมีอายุยาวกว่าข้อมูลที่อ้างถึงได้หรือไม่?

## 9. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Borrowing Rules
- [ ] ใช้ Immutable References หลายตัวได้
- [ ] เข้าใจข้อจำกัดของ Mutable Reference
- [ ] เข้าใจการยืมแบบที่ขัดแย้งกัน
- [ ] เข้าใจ Dangling Reference
- [ ] เข้าใจ NLL เบื้องต้น
