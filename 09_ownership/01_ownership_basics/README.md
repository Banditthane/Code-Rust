# Lesson 01: Ownership Basics

## 1. What is Ownership?

Ownership คือระบบที่ Rust ใช้จัดการการเป็นเจ้าของข้อมูลและหน่วยความจำ

ในภาษา Rust ค่าทุกค่าจะมีเจ้าของ (Owner) ซึ่งเป็นตัวแปรที่รับผิดชอบข้อมูลนั้น

ตัวอย่าง:

```rust
fn main() {
    let name = String::from("Rust");

    println!("{}", name);
}
```

คำอธิบาย:

- `String::from("Rust")` สร้าง String ขึ้นมา
- `name` เป็นเจ้าของ String นี้
- สามารถใช้ `name` เพื่อเข้าถึงข้อมูลได้

## 2. The Three Ownership Rules

Ownership มีกฎหลัก 3 ข้อ:

1. แต่ละค่าจะมีเจ้าของหนึ่งตัวในแต่ละช่วงเวลา
2. สามารถมีเจ้าของข้อมูลได้เพียงหนึ่งตัวในเวลาเดียวกัน
3. เมื่อเจ้าของออกจาก Scope ค่าที่เป็นเจ้าของจะถูก Drop

## 3. What is Scope?

Scope คือขอบเขตที่ตัวแปรสามารถใช้งานได้

ตัวอย่าง:

```rust
fn main() {
    let name = String::from("Rust");

    {
        let age = 20;
        println!("Age: {}", age);
    }

    println!("Name: {}", name);
}
```

คำอธิบาย:

- `name` ถูกประกาศใน Scope ของ `main()`
- `age` ถูกประกาศภายใน Scope ด้านใน
- เมื่อออกจาก Scope ด้านใน จะไม่สามารถใช้ `age` ได้อีก
- `name` ยังใช้งานได้ภายใน Scope ของ `main()`

## 4. Ownership and String

String เป็นตัวอย่างที่ดีสำหรับการเรียน Ownership เพราะ String สามารถจัดการข้อมูลบน Heap ได้

```rust
let language = String::from("Rust");
```

ตัวแปร `language` เป็นเจ้าของ String ที่สร้างขึ้น

## 5. Stack and Heap (Basic)

Rust ใช้หน่วยความจำหลายส่วนในการจัดเก็บข้อมูล โดยแนวคิดพื้นฐานที่ควรรู้คือ:

- Stack: ใช้จัดเก็บข้อมูลที่มีขนาดและรูปแบบแน่นอน
- Heap: ใช้จัดเก็บข้อมูลที่ต้องจองพื้นที่ในระหว่างการทำงาน

ตัวอย่าง:

```rust
let age = 20;
let language = String::from("Rust");
```

`age` เป็น Integer ส่วน `language` เป็น String ที่มีข้อมูลข้อความจัดเก็บบน Heap

## 6. Practice

ทดลองเขียนโปรแกรมที่:

1. สร้างตัวแปร `name` ด้วย String
2. สร้างตัวแปร `age`
3. สร้าง Scope ด้านใน
4. ประกาศตัวแปร `country` ภายใน Scope ด้านใน
5. ทดลองใช้งานตัวแปรก่อนและหลังออกจาก Scope

## 7. Knowledge Check

- Ownership คืออะไร?
- Owner หมายถึงอะไร?
- Scope คืออะไร?
- Ownership Rules มีกี่ข้อ?
- เมื่อเจ้าของข้อมูลออกจาก Scope จะเกิดอะไรขึ้น?

## 8. Commands

```powershell
cargo check
cargo run
```

## Completion Checklist

- [ ] เข้าใจความหมายของ Ownership
- [ ] อธิบาย Ownership Rules ได้
- [ ] เข้าใจ Scope
- [ ] เข้าใจความสัมพันธ์ระหว่าง Owner และข้อมูล
- [ ] ทำ Practice ได้ด้วยตนเอง
