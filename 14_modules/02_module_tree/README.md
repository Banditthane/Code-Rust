# Lesson 02: Module Tree

## 1. What is a Module Tree?

Module Tree คือโครงสร้างความสัมพันธ์ของ Modules ภายใน Crate

Module แต่ละตัวสามารถมี:

- Parent Module
- Child Modules
- Items เช่น Functions, Structs และ Enums

ตัวอย่าง:

```rust
mod company {
    pub mod employees {
        pub fn show() {
            println!("Employee");
        }
    }

    pub mod departments {
        pub fn show() {
            println!("Department");
        }
    }
}
```

โครงสร้าง:

```text
crate
└── company
    ├── employees
    │   └── show()
    └── departments
        └── show()
```

## 2. Root Module

ใน Binary Crate โดยทั่วไป `src/main.rs` เป็นจุดเริ่มต้นของ Module Tree

ใน Library Crate โดยทั่วไป `src/lib.rs` เป็น Root Module

ตัวอย่าง:

```rust
fn main() {
    println!("Root module");
}
```

Function `main()` อยู่ภายใน Root Module ของ Binary Crate

## 3. Parent and Child Modules

Module ที่ประกาศอยู่ภายใน Module อื่นจะเรียกว่า Child Module

ตัวอย่าง:

```rust
mod parent {
    pub mod child {
        pub fn hello() {
            println!("Hello from child");
        }
    }
}

fn main() {
    parent::child::hello();
}
```

Path:

```rust
parent::child::hello()
```

หมายถึง:

1. เริ่มจาก `parent`
2. เข้าไปยัง `child`
3. เรียก Function `hello()`

## 4. Separating Modules into Files

เมื่อโปรแกรมมีขนาดใหญ่ขึ้น เราสามารถแยก Module ออกจาก `main.rs` ไปยังไฟล์อื่นได้

ตัวอย่างโครงสร้าง:

```text
my_project/
└── src/
    ├── main.rs
    └── greetings.rs
```

ภายใน `main.rs`:

```rust
mod greetings;

fn main() {
    greetings::hello();
}
```

ภายใน `greetings.rs`:

```rust
pub fn hello() {
    println!("Hello from another file!");
}
```

ผลลัพธ์:

```text
Hello from another file!
```

ข้อควรจำ:

- `mod greetings;` ประกาศ Module จากไฟล์
- Rust จะค้นหาไฟล์ `src/greetings.rs`
- ไม่จำเป็นต้องเขียนเนื้อหา Module ซ้ำใน `main.rs`

## 5. Nested Module Files

หากต้องการแยก Module ที่ซ้อนกันเป็นไฟล์ สามารถจัดโครงสร้างได้ดังนี้:

```text
src/
├── main.rs
└── front_of_house/
    ├── mod.rs
    └── hosting.rs
```

ตัวอย่าง `main.rs`:

```rust
mod front_of_house;

fn main() {
    front_of_house::hosting::hello();
}
```

ตัวอย่าง `front_of_house/mod.rs`:

```rust
pub mod hosting;
```

ตัวอย่าง `front_of_house/hosting.rs`:

```rust
pub fn hello() {
    println!("Hello from hosting!");
}
```

หมายเหตุ: Rust ยังรองรับรูปแบบ `front_of_house.rs` ร่วมกับโฟลเดอร์ `front_of_house/` สำหรับเก็บ Module ย่อยด้วย

## 6. Important Notes

- Module Tree แสดงความสัมพันธ์ของ Modules
- Root Module เป็นจุดเริ่มต้นของ Module Tree
- Child Module อยู่ภายใน Parent Module
- `mod name;` ใช้ประกาศ Module ที่แยกไปยังไฟล์
- ชื่อไฟล์และตำแหน่งต้องตรงกับโครงสร้าง Module
- สามารถแบ่ง Module ซ้อนกันหลายระดับได้

## 7. Knowledge Check

1. Module Tree คืออะไร?
2. Root Module ของ Binary Crate โดยทั่วไปคือไฟล์ใด?
3. Child Module คืออะไร?
4. `mod greetings;` ทำหน้าที่อะไร?
5. หากประกาศ `mod greetings;` ใน `src/main.rs` Rust จะค้นหาไฟล์ใด?
6. สามารถแยก Nested Module เป็นหลายไฟล์ได้หรือไม่?

## 8. Completion Checklist

- [ ] เข้าใจ Module Tree
- [ ] เข้าใจ Root Module
- [ ] เข้าใจ Parent และ Child Modules
- [ ] ประกาศ Module ภายใน Module อื่นได้
- [ ] แยก Module ไปยังไฟล์ `.rs` ได้
- [ ] จัดโครงสร้าง Nested Module Files ได้
- [ ] เรียก Items จาก Module ที่แยกไฟล์ได้
