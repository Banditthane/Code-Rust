# Lesson 04: Scope and Drop

## 1. What is Scope?

Scope คือขอบเขตที่กำหนดว่าตัวแปรสามารถใช้งานได้ในบริเวณใดของโปรแกรม

ตัวอย่าง:

```rust
fn main() {
    let name = String::from("Rust");

    {
        let age = 25;

        println!("Age: {}", age);
    }

    println!("Name: {}", name);
}
```

คำอธิบาย:

- `name` ถูกประกาศใน Scope ของ `main()`
- `age` ถูกประกาศใน Scope ด้านใน
- `age` ใช้งานได้เฉพาะใน Scope ที่ประกาศและบริเวณด้านใน
- เมื่อออกจาก Scope ด้านใน `age` จะไม่สามารถใช้งานได้อีก
- `name` ยังคงใช้งานได้ใน Scope ของ `main()`

## 2. What is Drop?

Drop คือกระบวนการที่ Rust ใช้จัดการค่าที่หมดอายุการใช้งาน

เมื่อเจ้าของข้อมูลออกจาก Scope Rust จะเรียกกระบวนการ Drop สำหรับค่านั้น

ตัวอย่าง:

```rust
fn main() {
    let language = String::from("Rust");

    println!("{}", language);
}
```

เมื่อ `main()` จบการทำงาน ตัวแปร `language` จะออกจาก Scope และ String ที่มันเป็นเจ้าของจะถูก Drop

## 3. Drop Order

เมื่อมีตัวแปรหลายตัวอยู่ใน Scope เดียวกัน Rust จะ Drop ตัวแปรตามลำดับย้อนกลับจากลำดับที่ประกาศ

ตัวอย่าง:

```rust
fn main() {
    let first = String::from("First");
    let second = String::from("Second");
    let third = String::from("Third");
}
```

ลำดับการ Drop:

1. `third`
2. `second`
3. `first`

## 4. Manual Drop

Rust มี Function `drop()` สำหรับสั่งให้ค่าที่เป็นเจ้าของข้อมูลถูก Drop ก่อนออกจาก Scope ตามปกติ

ตัวอย่าง:

```rust
fn main() {
    let language = String::from("Rust");

    drop(language);

    // language ถูกย้ายเข้าไปใน drop()
}
```

หลังจากเรียก `drop(language)` แล้ว จะไม่สามารถใช้ `language` ต่อได้ เพราะ Ownership ถูกย้ายเข้าไปใน Function `drop()`

## 5. Scope and Ownership

ความสัมพันธ์ระหว่าง Scope และ Ownership:

- ตัวแปรมีขอบเขตการใช้งานตาม Scope
- เจ้าของข้อมูลจะรับผิดชอบค่าที่ตนเป็นเจ้าของ
- เมื่อเจ้าของออกจาก Scope ค่าจะถูก Drop
- เมื่อ Ownership ถูกย้าย เจ้าของเดิมจะไม่สามารถใช้ค่านั้นได้อีก

## 6. Practice

ทดลองเขียนโปรแกรมที่:

1. สร้าง String ชื่อ `first`
2. สร้าง String ชื่อ `second`
3. สร้าง Scope ด้านใน
4. ประกาศ String ชื่อ `third` ภายใน Scope
5. ทดลองแสดงค่าตัวแปรภายในและภายนอก Scope
6. ทดลองใช้ `drop()` กับ String
7. สังเกตว่าตัวแปรใดสามารถใช้งานต่อได้

## 7. Knowledge Check

- Scope คืออะไร?
- Drop คืออะไร?
- ตัวแปรจะถูก Drop เมื่อใด?
- ลำดับการ Drop ของตัวแปรใน Scope เดียวกันเป็นอย่างไร?
- `drop()` ใช้ทำอะไร?
- หลังเรียก `drop()` แล้ว สามารถใช้ตัวแปรเดิมได้หรือไม่?

## 8. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจ Scope
- [ ] เข้าใจ Drop
- [ ] เข้าใจลำดับการ Drop
- [ ] เข้าใจการใช้ `drop()`
- [ ] เข้าใจความสัมพันธ์ระหว่าง Scope และ Ownership
