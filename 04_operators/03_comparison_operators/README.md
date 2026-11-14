# Lesson 03: Comparison Operators

## Overview

Comparison Operators ใช้สำหรับเปรียบเทียบค่าของข้อมูล ผลลัพธ์ที่ได้จะเป็น Boolean คือ `true` หรือ `false`

## Operators

| Operator | Name                  | Example    |
| -------- | --------------------- | ---------- |
| `==`     | Equal                 | `10 == 10` |
| `!=`     | Not Equal             | `10 != 5`  |
| `>`      | Greater Than          | `10 > 5`   |
| `<`      | Less Than             | `5 < 10`   |
| `>=`     | Greater Than or Equal | `10 >= 10` |
| `<=`     | Less Than or Equal    | `5 <= 10`  |

## Examples

```rust
fn main() {
    let a = 10;
    let b = 5;

    println!("Equal: {}", a == b);
    println!("Not Equal: {}", a != b);
    println!("Greater: {}", a > b);
    println!("Less: {}", a < b);
    println!("Greater or Equal: {}", a >= b);
    println!("Less or Equal: {}", a <= b);
}
```

## Boolean Result

ตัวอย่าง:

```rust
let age = 20;

let is_adult = age >= 18;

println!("{}", is_adult);
```

ผลลัพธ์คือ `true`

## Important Notes

- `=` ใช้กำหนดค่าให้ตัวแปร
- `==` ใช้ตรวจสอบว่าค่าสองค่ามีค่าเท่ากันหรือไม่
- Comparison Operators คืนค่าเป็น `bool`

## Practice

1. สร้างตัวแปร `age` และ `score`
2. ตรวจสอบว่าอายุเท่ากับ 18 หรือไม่
3. ตรวจสอบว่าคะแนนมากกว่า 50 หรือไม่
4. ตรวจสอบว่าคะแนนไม่เท่ากับ 0 หรือไม่
5. แสดงผลลัพธ์ทุกเงื่อนไข

## Knowledge Check

1. `==` และ `=` แตกต่างกันอย่างไร?
2. ผลลัพธ์ของ `10 > 5` คืออะไร?
3. Comparison Operators คืนค่าชนิดข้อมูลใด?
4. `!=` ใช้ตรวจสอบอะไร?
