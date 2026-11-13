# Lesson 04: Rust Comments

**Chapter:** 01 - Rust Fundamentals  
**Level:** L1 - Beginner  
**Status:** Learning

## 1. Overview

Comments คือข้อความที่เขียนไว้เพื่ออธิบายโค้ด โดย Compiler จะไม่ประมวลผลข้อความ Comment เป็นคำสั่งโปรแกรม

Comments ช่วยให้โค้ดอ่านง่ายและเข้าใจได้มากขึ้น

## 2. Learning Objectives

- เข้าใจความหมายของ Comments
- เขียน Single-line Comment ได้
- เขียน Multi-line Comment ได้
- ใช้ Comments อธิบายโค้ดได้

## 3. Single-line Comments

ใช้เครื่องหมาย `//`

ตัวอย่าง:

```rust
fn main() {
    // This is a comment
    println!("Hello Rust!");
}
```

ข้อความหลัง `//` ในบรรทัดนั้นจะเป็น Comment

## 4. Multi-line Comments

ใช้เครื่องหมาย `/*` และ `*/`

ตัวอย่าง:

```rust
fn main() {
    /*
       This is a multi-line comment.
       We are learning Rust.
    */

    println!("Hello Rust!");
}
```

## 5. Inline Comments

สามารถเขียน Comment ต่อท้ายคำสั่งได้

ตัวอย่าง:

```rust
fn main() {
    println!("Hello Rust!"); // Display message
}
```

## 6. Comment Comparison

| Type        | Syntax  | Description         |
| ----------- | ------- | ------------------- |
| Single-line | `//`    | Comment หนึ่งบรรทัด |
| Multi-line  | `/* */` | Comment หลายบรรทัด  |

## 7. Practice

ทดลองเขียนโปรแกรม:

```rust
fn main() {
    // Display program title
    println!("Rust Learning");

    /*
       Display learning information
       for beginner students.
    */
    println!("Level: Beginner");
    println!("Status: Learning");
}
```

## 8. Completion Checklist

- [ ] เข้าใจ Comments
- [ ] ใช้ `//` ได้
- [ ] ใช้ `/* */` ได้
- [ ] เขียน Comment อธิบายโค้ดได้

## 9. References

- https://www.w3schools.com/rust/rust_comments.php

## 10. Next Step

เมื่อเรียนครบทั้ง 4 Lessons แล้ว ให้ทำแบบฝึกหัดในโฟลเดอร์ `exercises/`
