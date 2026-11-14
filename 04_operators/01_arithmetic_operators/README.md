# Lesson 01: Arithmetic Operators

## Overview

Arithmetic Operators ใช้สำหรับดำเนินการคำนวณทางคณิตศาสตร์ เช่น การบวก ลบ คูณ หาร และหารเอาเศษ

## Operators

| Operator | Name           | Example  |
| -------- | -------------- | -------- |
| `+`      | Addition       | `10 + 5` |
| `-`      | Subtraction    | `10 - 5` |
| `*`      | Multiplication | `10 * 5` |
| `/`      | Division       | `10 / 5` |
| `%`      | Remainder      | `10 % 3` |

## Examples

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);
    println!("Remainder: {}", a % b);
}
```

## Important Notes

### Integer Division

เมื่อใช้จำนวนเต็มหารกัน ผลลัพธ์จะเป็นจำนวนเต็ม โดยเศษทศนิยมจะถูกตัดออก

```rust
let result = 10 / 3;
println!("{}", result); // 3
```

### Floating-Point Division

หากต้องการผลลัพธ์ทศนิยม ให้ใช้ Floating-Point เช่น `f64`

```rust
let result = 10.0 / 3.0;
println!("{}", result);
```

### Remainder

เครื่องหมาย `%` ใช้หาค่าเศษจากการหาร

```rust
let result = 10 % 3;
println!("{}", result); // 1
```

## Practice

1. สร้างตัวแปร `a` และ `b`
2. ทดลองใช้ Operators ทั้ง 5 แบบ
3. ทดลองหารจำนวนเต็มและจำนวนทศนิยม
4. ทดลองหาค่าเศษด้วย `%`

## Knowledge Check

1. เครื่องหมายใดใช้สำหรับคูณ?
2. `10 / 3` เมื่อเป็น Integer จะได้ผลลัพธ์เท่าไร?
3. `10 % 3` ได้ผลลัพธ์เท่าไร?
4. ถ้าต้องการผลลัพธ์ทศนิยม ควรใช้ชนิดข้อมูลใด?
