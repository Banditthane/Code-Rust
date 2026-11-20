# 04: String Formatting (การจัดรูปแบบข้อความ)

## ภาพรวม

String Formatting คือการนำข้อความและข้อมูลหลายส่วนมาประกอบกันเป็นข้อความเดียว โดย Rust มี Macro ที่สำคัญ ได้แก่ `format!()`, `println!()` และ `print!()`

บทนี้เน้นการใช้ `format!()` เพื่อสร้างข้อความและนำผลลัพธ์ไปใช้งานต่อ

## 1. format!()

`format!()` ใช้สร้าง String จากข้อความและค่าตัวแปร โดยจะส่ง String กลับมาแทนการแสดงผลออกทางหน้าจอทันที

ตัวอย่าง:

```rust
fn main() {
    let name = "Alex";
    let age = 20;

    let message = format!("Name: {}, Age: {}", name, age);

    println!("{}", message);
}
```

ผลลัพธ์:

```text
Name: Alex, Age: 20
```

## 2. การใช้ Placeholder

Placeholder `{}` ใช้กำหนดตำแหน่งที่ต้องการแสดงค่าตัวแปร

ตัวอย่าง:

```rust
fn main() {
    let name = "Rust";
    let version = 2024;

    println!("Language: {}, Version: {}", name, version);
}
```

ผลลัพธ์:

```text
Language: Rust, Version: 2024
```

## 3. การใช้ Named Arguments

สามารถตั้งชื่อให้กับ Arguments ภายใน Format String ได้

ตัวอย่าง:

```rust
fn main() {
    let message = format!(
        "{language} is a programming language.",
        language = "Rust"
    );

    println!("{}", message);
}
```

ผลลัพธ์:

```text
Rust is a programming language.
```

## 4. การใช้ println!()

`println!()` ใช้แสดงข้อความและขึ้นบรรทัดใหม่เมื่อจบการทำงาน

ตัวอย่าง:

```rust
fn main() {
    let name = "Rust";

    println!("Hello, {}!", name);
    println!("Welcome!");
}
```

ผลลัพธ์:

```text
Hello, Rust!
Welcome!
```

## 5. การใช้ print!()

`print!()` ใช้แสดงข้อความโดยไม่ขึ้นบรรทัดใหม่โดยอัตโนมัติ

ตัวอย่าง:

```rust
fn main() {
    print!("Hello ");
    print!("Rust!");
}
```

ผลลัพธ์:

```text
Hello Rust!
```

## 6. การจัดรูปแบบตัวเลข

สามารถกำหนดรูปแบบการแสดงผลตัวเลขได้

ตัวอย่าง:

```rust
fn main() {
    let price = 1250.5;

    println!("Price: {:.2}", price);
}
```

ผลลัพธ์:

```text
Price: 1250.50
```

`{:.2}` หมายถึงแสดงตัวเลขทศนิยม 2 ตำแหน่ง

## 7. ความแตกต่างระหว่าง format!() และ println!()

| Macro        | หน้าที่                         |
| ------------ | ------------------------------- |
| `format!()`  | สร้าง String และส่งค่ากลับ      |
| `println!()` | แสดงข้อความและขึ้นบรรทัดใหม่    |
| `print!()`   | แสดงข้อความโดยไม่ขึ้นบรรทัดใหม่ |

ตัวอย่าง:

```rust
fn main() {
    let name = "Rust";

    let message = format!("Hello, {}!", name);

    println!("{}", message);
}
```

## สิ่งที่ควรรู้

- `{}` ใช้แสดงค่าตัวแปร
- `format!()` สร้าง String
- `println!()` แสดงข้อความพร้อมขึ้นบรรทัดใหม่
- `print!()` แสดงข้อความโดยไม่ขึ้นบรรทัดใหม่
- `{:.2}` ใช้กำหนดจำนวนตำแหน่งทศนิยม 2 ตำแหน่ง
- String ที่สร้างด้วย `format!()` สามารถเก็บไว้ในตัวแปรและนำไปใช้งานต่อได้

## แบบฝึกหัด

1. สร้างตัวแปร `name` และ `age`
2. ใช้ `format!()` สร้างข้อความแนะนำตัว
3. เก็บข้อความที่ได้ไว้ในตัวแปร `message`
4. แสดงข้อความด้วย `println!()`
5. สร้างข้อความที่มีข้อมูลหลายตัวแปร
6. ทดลองใช้ Named Arguments
7. ทดลองใช้ `{:.2}` กับตัวเลขทศนิยม

## คำถามทบทวน

1. `format!()` ใช้ทำอะไร?
2. `format!()` แตกต่างจาก `println!()` อย่างไร?
3. Placeholder `{}` มีหน้าที่อะไร?
4. `print!()` แตกต่างจาก `println!()` อย่างไร?
5. `{:.2}` ใช้กำหนดรูปแบบการแสดงผลอย่างไร?
