# Lesson 04: Use and Paths

## 1. What is a Path?

Path คือเส้นทางที่ใช้ระบุ Items ภายใน Module เช่น Functions, Structs และ Enums

ตัวอย่าง:

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = calculator::add(10, 5);

    println!("{}", result);
}
```

Path ที่ใช้คือ:

```rust
calculator::add
```

## 2. Absolute Paths

Absolute Path คือ Path ที่เริ่มต้นจาก Root ของ Crate

ใช้ Keyword `crate` เพื่ออ้างอิงจาก Root Module

ตัวอย่าง:

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = crate::calculator::add(10, 5);

    println!("{}", result);
}
```

ผลลัพธ์:

```text
15
```

## 3. Relative Paths

Relative Path คือ Path ที่เริ่มต้นจาก Module ปัจจุบัน

ตัวอย่าง:

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = calculator::add(10, 5);

    println!("{}", result);
}
```

Path `calculator::add` เริ่มจากตำแหน่งปัจจุบัน ซึ่งในตัวอย่างคือ Root Module

## 4. The self Keyword

`self` ใช้อ้างอิง Module ปัจจุบัน

ตัวอย่าง:

```rust
mod tools {
    pub mod calculator {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }

    pub fn calculate() {
        let result = self::calculator::add(10, 5);
        println!("{}", result);
    }
}

fn main() {
    tools::calculate();
}
```

## 5. The super Keyword

`super` ใช้อ้างอิง Parent Module ของ Module ปัจจุบัน

ตัวอย่าง:

```rust
mod parent {
    pub fn hello() {
        println!("Hello from parent");
    }

    pub mod child {
        pub fn call_parent() {
            super::hello();
        }
    }
}

fn main() {
    parent::child::call_parent();
}
```

ผลลัพธ์:

```text
Hello from parent
```

## 6. Using the use Keyword

`use` ใช้นำ Path เข้ามาอยู่ใน Scope เพื่อให้เรียกใช้งานได้สั้นลง

ตัวอย่าง:

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

use crate::calculator::add;

fn main() {
    let result = add(10, 5);

    println!("{}", result);
}
```

เมื่อใช้ `use` แล้ว สามารถเรียก `add()` ได้โดยไม่ต้องเขียน Path เต็มทุกครั้ง

## 7. Bringing a Module into Scope

สามารถนำ Module เข้ามาใน Scope ได้เช่นกัน

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

use crate::calculator;

fn main() {
    println!("{}", calculator::add(10, 5));
}
```

## 8. The as Keyword

ใช้ `as` ตั้งชื่อใหม่ให้ Item ที่นำเข้าด้วย `use`

ตัวอย่าง:

```rust
use std::fmt::Result as FormatResult;
```

ช่วยป้องกันความสับสนเมื่อมีชื่อซ้ำกัน และทำให้โค้ดอ่านง่ายขึ้น

## 9. Re-exporting with pub use

`pub use` ใช้นำ Item เข้ามาใน Module แล้วเปิดให้ผู้ใช้งานภายนอกเข้าถึงผ่าน Path ของ Module นั้นได้

ตัวอย่าง:

```rust
mod internal {
    pub mod calculator {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }
}

pub use internal::calculator;
```

ภายนอก Module สามารถอ้างอิงผ่าน Path:

```rust
calculator::add(10, 5);
```

โดยไม่จำเป็นต้องระบุ `internal` ใน Path ที่เปิดให้ใช้งาน

## 10. Summary

| Keyword   | Purpose                                    |
| --------- | ------------------------------------------ |
| `crate`   | อ้างอิงจาก Root Module                     |
| `self`    | อ้างอิง Module ปัจจุบัน                    |
| `super`   | อ้างอิง Parent Module                      |
| `use`     | นำ Path เข้ามาใน Scope                     |
| `as`      | ตั้งชื่อใหม่ให้ Item ที่นำเข้า             |
| `pub use` | Re-export Item ให้เข้าถึงผ่าน Path ใหม่ได้ |

## 11. Important Notes

- Path ใช้ระบุที่อยู่ของ Items
- Absolute Path เริ่มจาก Root ของ Crate
- Relative Path เริ่มจากตำแหน่งปัจจุบัน
- `crate`, `self` และ `super` ใช้ระบุจุดเริ่มต้นของ Path
- `use` ช่วยลดการเขียน Path ซ้ำ
- `as` ใช้ตั้งชื่อใหม่ให้ Item ที่นำเข้า
- `pub use` ใช้เปิดเผย
