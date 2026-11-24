# Lesson 02: Declarative Macros

## 1. Declarative Macro คืออะไร?

Declarative Macro เป็น Macro ที่กำหนดรูปแบบ Input และรูปแบบโค้ดที่ต้องการสร้าง

ใน Rust ใช้ `macro_rules!` ในการประกาศ

รูปแบบ:

```rust
macro_rules! macro_name {
    (pattern) => {
        // generated code
    };
}
```

## 2. สร้าง Macro เบื้องต้น

ตัวอย่าง:

```rust
macro_rules! say_hello {
    () => {
        println!("Hello Rust");
    };
}

fn main() {
    say_hello!();
}
```

ผลลัพธ์:

```text
Hello Rust
```

คำอธิบาย:

- `macro_rules!` ใช้ประกาศ Declarative Macro
- `say_hello` คือชื่อ Macro
- `()` คือ Pattern ที่ไม่รับ Input
- `=>` แยก Pattern ออกจากโค้ดที่สร้าง
- `println!()` คือโค้ดที่ Macro สร้างขึ้น

## 3. รับ Arguments ด้วย Macro

ใช้ `$` เพื่อประกาศตัวแปรรับ Input

ตัวอย่าง:

```rust
macro_rules! show_value {
    ($value:expr) => {
        println!("Value: {}", $value);
    };
}

fn main() {
    show_value!(100);
    show_value!(10 + 20);
}
```

ผลลัพธ์:

```text
Value: 100
Value: 30
```

`expr` เป็น Fragment Specifier ที่รับ Expression

## 4. Fragment Specifiers

Fragment Specifier ใช้ระบุประเภทของ Input ที่ Macro ยอมรับ

| Specifier | ความหมาย           |
| --------- | ------------------ |
| `expr`    | Expression         |
| `ident`   | Identifier         |
| `ty`      | Type               |
| `pat`     | Pattern            |
| `path`    | Path               |
| `stmt`    | Statement          |
| `block`   | Block              |
| `item`    | Item               |
| `literal` | Literal            |
| `tt`      | Token Tree         |
| `meta`    | Attribute Metadata |

ตัวอย่าง:

```rust
macro_rules! create_variable {
    ($name:ident, $value:expr) => {
        let $name = $value;
    };
}

fn main() {
    create_variable!(score, 100);

    println!("{}", score);
}
```

ผลลัพธ์:

```text
100
```

## 5. รับ Arguments หลายรายการ

สามารถกำหนด Pattern ให้รับ Arguments หลายรายการได้

ตัวอย่าง:

```rust
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    let result = add!(10, 20);

    println!("{}", result);
}
```

ผลลัพธ์:

```text
30
```

## 6. Repetition

ใช้ `$()` ร่วมกับ Repetition Operator เพื่อรับ Input หลายรายการ

รูปแบบที่ใช้บ่อย:

| Operator | ความหมาย            |
| -------- | ------------------- |
| `*`      | รับ 0 รายการขึ้นไป  |
| `+`      | รับ 1 รายการขึ้นไป  |
| `?`      | รับ 0 หรือ 1 รายการ |

ตัวอย่าง:

```rust
macro_rules! show_numbers {
    ($($number:expr),*) => {
        $(
            println!("{}", $number);
        )*
    };
}

fn main() {
    show_numbers!(10, 20, 30);
}
```

ผลลัพธ์:

```text
10
20
30
```

## 7. Repetition แบบมี Separator

สามารถกำหนดเครื่องหมายคั่นระหว่าง Arguments ได้

ตัวอย่าง:

```rust
macro_rules! create_list {
    ($($value:expr),*) => {
        vec![$($value),*]
    };
}

fn main() {
    let numbers = create_list!(10, 20, 30);

    println!("{:?}", numbers);
}
```

ผลลัพธ์:

```text
[10, 20, 30]
```

ในตัวอย่างนี้:

- `$(...)` คือส่วนที่ทำซ้ำ
- `,` คือ Separator
- `*` หมายถึงรับได้ตั้งแต่ 0 รายการขึ้นไป

## 8. Multiple Patterns

Macro สามารถกำหนดหลาย Pattern ได้

ตัวอย่าง:

```rust
macro_rules! calculate {
    ($a:expr, $b:expr) => {
        $a + $b
    };

    ($a:expr) => {
        $a * $a
    };
}

fn main() {
    println!("{}", calculate!(10, 20));
    println!("{}", calculate!(5));
}
```

ผลลัพธ์:

```text
30
25
```

Compiler จะเลือก Pattern ที่ตรงกับรูปแบบ Input

## 9. Macro สามารถคืนค่า Expression ได้

โค้ดที่ Macro สร้างสามารถเป็น Expression และนำไปกำหนดค่าให้ตัวแปรได้

ตัวอย่าง:

```rust
macro_rules! square {
    ($value:expr) => {
        ($value) * ($value)
    };
}

fn main() {
    let result = square!(5 + 1);

    println!("{}", result);
}
```

ผลลัพธ์:

```text
36
```

ข้อควรระวัง: หาก Expression ถูกนำไปใช้ซ้ำ อาจถูกประเมินหลายครั้ง ควรพิจารณาเรื่อง Side Effect และการประเมินค่า

## 10. ข้อควรระวัง

- ใช้ Parentheses เพื่อควบคุมลำดับการคำนวณ
- ระวังการประเมิน Expression ซ้ำ
- Pattern ต้องตรงกับ Input
- Repetition ต้องจัดรูปแบบให้ถูกต้อง
- อย่าใช้ Macro เมื่อ Function ธรรมดาทำงานได้ชัดเจนกว่า

## Completion Checklist

- [ ] ประกาศ Macro ด้วย `macro_rules!` ได้
- [ ] สร้าง Macro ที่ไม่รับ Arguments ได้
- [ ] ใช้ `$value:expr` ได้
- [ ] เข้าใจ Fragment Specifiers
- [ ] รับ Arguments หลายรายการได้
- [ ] ใช้ Repetition `*` ได้
- [ ] เข้าใจ Repetition `+` และ `?`
- [ ] ใช้ Separator ได้
- [ ] สร้าง Multiple Patterns ได้
- [ ] สร้าง Macro ที่คืนค่า Expression ได้
