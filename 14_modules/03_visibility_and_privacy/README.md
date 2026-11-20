# Lesson 03: Visibility and Privacy

## 1. What is Visibility?

Visibility คือการกำหนดว่า Items ภายใน Module สามารถเข้าถึงจากตำแหน่งใดได้บ้าง

Rust ใช้ระบบ Privacy เพื่อควบคุมการเข้าถึงข้อมูลและการทำงานของโปรแกรม

โดยค่าเริ่มต้น Items ภายใน Module จะเป็น Private

ตัวอย่าง Items:

- Functions
- Structs
- Enums
- Constants
- Modules

## 2. Private Items

หากไม่กำหนด `pub` Item จะเป็น Private

ตัวอย่าง:

```rust
mod account {
    fn show_balance() {
        println!("Balance: 1000");
    }
}

fn main() {
    // account::show_balance();
}
```

Function `show_balance()` เป็น Private จึงไม่สามารถเรียกจากภายนอก Module `account` ได้

## 3. Public Items with pub

ใช้ Keyword `pub` เพื่อกำหนดให้ Item สามารถเข้าถึงจากภายนอก Module ได้

ตัวอย่าง:

```rust
mod account {
    pub fn show_balance() {
        println!("Balance: 1000");
    }
}

fn main() {
    account::show_balance();
}
```

ผลลัพธ์:

```text
Balance: 1000
```

## 4. Public Struct and Private Fields

การกำหนด `pub` ให้ Struct ไม่ได้ทำให้ Fields ทุกตัวเป็น Public โดยอัตโนมัติ

ตัวอย่าง:

```rust
mod user {
    pub struct User {
        pub name: String,
        age: u8,
    }
}

fn main() {
    let user = user::User {
        name: String::from("Alice"),
        // age: 20,
    };

    println!("{}", user.name);
}
```

ในตัวอย่าง:

- `User` เป็น Public Struct
- `name` เป็น Public Field
- `age` เป็น Private Field

จึงสามารถเข้าถึง `user.name` จากภายนอก Module ได้ แต่ไม่สามารถเข้าถึง `user.age` โดยตรง

## 5. Accessing Private Fields with Methods

หากต้องการให้ภายนอก Module อ่านหรือแก้ไข Private Fields สามารถสร้าง Public Methods เพื่อควบคุมการเข้าถึงได้

ตัวอย่าง:

```rust
mod user {
    pub struct User {
        name: String,
        age: u8,
    }

    impl User {
        pub fn new(name: String, age: u8) -> Self {
            Self { name, age }
        }

        pub fn show_age(&self) {
            println!("Age: {}", self.age);
        }
    }
}

fn main() {
    let user = user::User::new(
        String::from("Alice"),
        20,
    );

    user.show_age();
}
```

ผลลัพธ์:

```text
Age: 20
```

แนวทางนี้ช่วยให้ Module ควบคุมวิธีเข้าถึงข้อมูลของตัวเองได้

## 6. Privacy Rules

กฎสำคัญของ Rust:

- Private Item สามารถเข้าถึงได้ภายใน Module ที่ประกาศ Item นั้น และภายใน Child Modules ของ Module นั้น
- Parent Module ไม่สามารถเข้าถึง Private Items ภายใน Child Module ได้โดยตรง
- Sibling Module ไม่สามารถเข้าถึง Private Items ของกันและกันโดยตรง
- Public Item สามารถเข้าถึงได้ตามขอบเขตของ Visibility ที่กำหนด
- การเข้าถึง Item จากภายนอก Crate ยังต้องมี Public Path ผ่าน Module ทุกระดับที่เกี่ยวข้อง

## 7. Restricted Visibility

Rust ยังมีรูปแบบการกำหนด Visibility ที่จำกัดขอบเขตได้ เช่น:

| Syntax         | Meaning                        |
| -------------- | ------------------------------ |
| `pub`          | Public                         |
| `pub(crate)`   | เข้าถึงได้ภายใน Crate เดียวกัน |
| `pub(super)`   | เข้าถึงได้จาก Parent Module    |
| `pub(in path)` | เข้าถึงได้ภายใน Module ที่ระบุ |

ตัวอย่าง:

```rust
mod account {
    pub(crate) fn internal_report() {
        println!("Internal report");
    }
}
```

Function นี้สามารถเข้าถึงได้จากส่วนต่าง ๆ ภายใน Crate เดียวกัน แต่ไม่เปิดเป็น Public API ให้ Crate อื่นโดยตรง

## 8. Important Notes

- Items เป็น Private โดยค่าเริ่มต้น
- ใช้ `pub` เพื่อเปิดการเข้าถึง
- Public Struct ไม่ได้ทำให้ Fields เป็น Public ทั้งหมด
- สามารถใช้ Methods เป็นตัวกลางเข้าถึง Private Fields ได้
- Parent Module ไม่สามารถเข้าถึง Private Items ของ Child Module ได้โดยตรง
- `pub(crate)` จำกัดการเข้าถึงภายใน Crate
- การออกแบบ Visibility ช่วยปกป้องข้อมูลและลดการพึ่งพาภายในโปรแกรม

## 9. Knowledge Check

1. Visibility คืออะไร?
2. Item ที่ไม่กำหนด `pub` มีสถานะอย่างไร?
3. `pub` ใช้ทำอะไร?
4. Public Struct ทำให้ทุก Field เป็น Public หรือไม่?
5. Parent Module สามารถเข้าถึง Private Items ใน Child Module ได้หรือไม่?
6. `pub(crate)` แตกต่างจาก `pub` อย่างไร?
7. เหตุใดจึงควรใช้ Methods เพื่อควบคุมการเข้าถึง Private Fields?

## 10. Completion Checklist

- [ ] เข้าใจ Visibility
- [ ] เข้าใจ Private Items
- [ ] ใช้ `pub` กับ Function ได้
- [ ] ใช้ `pub` กับ Struct ได้
- [ ] กำหนด Public และ Private Fields ได้
- [ ] ใช้ Methods เข้าถึง Private Fields ได้
- [ ] เข้าใจ Privacy Rules
- [ ] เข้าใจ `pub(crate)` และ `pub(super)`
