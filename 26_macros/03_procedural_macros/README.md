# Lesson 03: Procedural Macros

## 1. Procedural Macro คืออะไร?

Procedural Macro เป็น Macro ที่รับ TokenStream แล้วประมวลผลเพื่อสร้าง TokenStream ใหม่

ต่างจาก Declarative Macro ที่ใช้ Pattern Matching ผ่าน `macro_rules!`

Procedural Macro สามารถตรวจสอบโครงสร้างของโค้ดที่รับเข้ามา และสร้างโค้ดใหม่ตามข้อมูลนั้นได้

## 2. TokenStream

TokenStream คือข้อมูลที่ประกอบด้วย Token ของ Rust Source Code

Procedural Macro ใช้ TokenStream เป็น Input และ Output

นำเข้า:

```rust
use proc_macro::TokenStream;
```

แนวคิด:

```text
Rust Source Code
       |
       v
   TokenStream
       |
       v
Procedural Macro
       |
       v
Generated TokenStream
       |
       v
 Rust Compilation
```

## 3. ประเภทของ Procedural Macro

Procedural Macro มี 3 ประเภทหลัก

| ประเภท              | หน้าที่                                     |
| ------------------- | ------------------------------------------- |
| Derive Macro        | สร้าง Implementation ให้ Struct หรือ Enum   |
| Attribute Macro     | รับ Attribute แล้วแปลง Item ที่กำหนด        |
| Function-like Macro | ทำงานในรูปแบบคล้าย Function-like Invocation |

## 4. Derive Macro

Derive Macro ใช้สร้าง Trait Implementation ให้ Struct หรือ Enum

ตัวอย่างการใช้งาน:

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
}
```

`Debug`, `Clone` และ `PartialEq` เป็น Derive Macro ที่มีให้ใช้ใน Rust

สามารถสร้าง Derive Macro ของตัวเองได้ด้วย `#[proc_macro_derive(...)]`

ตัวอย่างโครงสร้าง Procedural Macro Crate:

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    // Process input and generate code
    todo!()
}
```

ตัวอย่างนี้แสดงโครงสร้างเท่านั้น ยังไม่ใช่ Implementation ที่สมบูรณ์

## 5. Attribute Macro

Attribute Macro ใช้ Attribute เพื่อประมวลผล Item เช่น Function หรือ Module

ตัวอย่างรูปแบบ:

```rust
#[my_attribute]
fn say_hello() {
    println!("Hello");
}
```

ในฝั่ง Procedural Macro ต้องประกาศด้วย:

```rust
#[proc_macro_attribute]
pub fn my_attribute(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    // Process attribute and item
    todo!()
}
```

- `attr` คือข้อมูลภายใน Attribute
- `item` คือโค้ด Item ที่ถูกนำมาใช้กับ Attribute
- Return เป็น TokenStream ที่สร้างขึ้น

## 6. Function-like Procedural Macro

Function-like Procedural Macro มีรูปแบบการเรียกคล้าย Declarative Macro

ตัวอย่าง:

```rust
my_macro!(some input);
```

ประกาศด้วย:

```rust
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Process input
    todo!()
}
```

## 7. สร้าง Procedural Macro Crate

Procedural Macro ต้องอยู่ใน Library Crate ที่กำหนด:

```toml
[lib]
proc-macro = true
```

ตัวอย่าง `Cargo.toml`:

```toml
[package]
name = "my_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = "2"
quote = "1"
```

หมายเหตุ:

- `proc-macro = true` กำหนดให้ Crate เป็น Procedural Macro Crate
- `syn` ช่วย Parse Rust Syntax
- `quote` ช่วยสร้าง TokenStream จากโครงสร้างโค้ด
- เวอร์ชันของ Dependency สามารถเปลี่ยนตามความเหมาะสมของโปรเจกต์

## 8. syn และ quote

### syn

ใช้ Parse TokenStream ให้เป็นโครงสร้างที่เข้าใจได้ เช่น:

- Struct
- Enum
- Function
- Type
- Attribute

### quote

ใช้สร้าง TokenStream จากโค้ดที่เขียนในรูปแบบ Quote

ตัวอย่างแนวคิด:

```rust
quote::quote! {
    println!("Generated code");
}
```

โดยทั่วไปจะใช้ร่วมกับ `proc_macro2` เพื่อจัดการ TokenStream ในกระบวนการสร้างโค้ด

## 9. ความแตกต่างของ Procedural Macro ทั้ง 3 ประเภท

| ประเภท        | Attribute            | Input              | ผลลัพธ์                    |
| ------------- | -------------------- | ------------------ | -------------------------- |
| Derive        | `#[derive(MyTrait)]` | Struct หรือ Enum   | เพิ่ม Trait Implementation |
| Attribute     | `#[my_attribute]`    | Attribute และ Item | แปลง Item                  |
| Function-like | `my_macro!(...)`     | TokenStream        | สร้างโค้ดจาก Input         |

## 10. ข้อควรระวัง

- Procedural Macro ต้องอยู่ใน Crate ที่กำหนด `proc-macro = true`
- Procedural Macro Crate ไม่ใช่ Application Binary Crate ทั่วไป
- ต้อง Parse และสร้าง TokenStream อย่างถูกต้อง
- ต้องคำนึงถึงชื่อที่สร้างขึ้นไม่ให้ชนกับโค้ดผู้ใช้
- Error Message ควรชัดเจนและระบุตำแหน่งที่เกี่ยวข้อง
- Macro ควรสร้างโค้ดที่มีพฤติกรรมคาดเดาได้

## Completion Checklist

- [ ] เข้าใจ Procedural Macro
- [ ] เข้าใจ TokenStream
- [ ] เข้าใจ Derive Macro
- [ ] เข้าใจ Attribute Macro
- [ ] เข้าใจ Function-like Macro
- [ ] สร้าง Procedural Macro Crate ได้
- [ ] เข้าใจ `proc-macro = true`
- [ ] เข้าใจบทบาทของ `syn`
- [ ] เข้าใจบทบาทของ `quote`
- [ ] เข้าใจโครงสร้างของ Procedural Macro
