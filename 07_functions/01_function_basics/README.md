# 01: พื้นฐาน Function

## ภาพรวม

Function คือชุดคำสั่งที่รวมไว้ภายใต้ชื่อเดียวกัน เพื่อให้สามารถเรียกใช้งานได้หลายครั้งโดยไม่ต้องเขียนคำสั่งเดิมซ้ำ

ในภาษา Rust ใช้คำสั่ง `fn` สำหรับประกาศ Function

## รูปแบบการประกาศ Function

```rust
fn function_name() {
    // คำสั่งภายใน Function
}
```

ส่วนประกอบ:

- `fn` ใช้ประกาศ Function
- `function_name` คือชื่อ Function
- `()` คือส่วนกำหนด Parameters ซึ่งตัวอย่างนี้ยังไม่มี
- `{}` คือขอบเขตของคำสั่งภายใน Function

## ตัวอย่างที่ 1: สร้าง Function

```rust
fn say_hello() {
    println!("Hello!");
}

fn main() {
    say_hello();
}
```

ผลลัพธ์:

```text
Hello!
```

## อธิบายการทำงาน

1. ประกาศ Function ชื่อ `say_hello`
2. ภายใน Function มีคำสั่ง `println!()`
3. ภายใน `main()` เรียกใช้งานด้วย `say_hello()`
4. โปรแกรมจึงแสดงข้อความ `Hello!`

การประกาศ Function ไม่ได้หมายความว่า Function จะทำงานทันที ต้องมีการเรียกใช้งานก่อน

## ตัวอย่างที่ 2: เรียก Function หลายครั้ง

```rust
fn greet() {
    println!("Welcome to Rust!");
}

fn main() {
    greet();
    greet();
    greet();
}
```

ผลลัพธ์:

```text
Welcome to Rust!
Welcome to Rust!
Welcome to Rust!
```

Function เดียวสามารถเรียกใช้งานได้หลายครั้ง

## ตัวอย่างที่ 3: Function หลายตัว

```rust
fn say_hello() {
    println!("Hello!");
}

fn say_goodbye() {
    println!("Goodbye!");
}

fn main() {
    say_hello();
    say_goodbye();
}
```

ผลลัพธ์:

```text
Hello!
Goodbye!
```

## สิ่งที่ควรรู้

- ใช้ `fn` เพื่อประกาศ Function
- ชื่อ Function นิยมใช้รูปแบบ `snake_case`
- Function จะทำงานเมื่อถูกเรียกใช้งาน
- Function สามารถเรียกใช้งานซ้ำได้
- `main()` เป็น Function หลักที่โปรแกรมเริ่มต้นทำงาน
- สามารถประกาศ Function ไว้ก่อนหรือหลัง `main()` ก็ได้

## แบบฝึกหัด

1. สร้าง Function ชื่อ `greet` เพื่อแสดงข้อความ `Hello, Rust!`
2. สร้าง Function ชื่อ `show_name` เพื่อแสดงชื่อตนเอง
3. สร้าง Function ชื่อ `show_age` เพื่อแสดงอายุ
4. เรียกใช้งาน Function ทั้ง 3 ตัวจากภายใน `main()`
5. ทดลองเรียก Function `greet()` มากกว่า 1 ครั้ง

## คำถามทบทวน

1. Function คืออะไร?
2. คำสั่งใดใช้ประกาศ Function ใน Rust?
3. การประกาศ Function กับการเรียกใช้งาน Function แตกต่างกันอย่างไร?
4. สามารถเรียก Function เดิมหลายครั้งได้หรือไม่?
5. Function `main()` มีหน้าที่อะไร?
