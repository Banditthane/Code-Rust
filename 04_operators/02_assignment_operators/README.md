# Lesson 02: Assignment Operators

## Overview

Assignment Operators ใช้สำหรับกำหนดค่าให้ตัวแปร และปรับค่าของตัวแปรด้วยการคำนวณ

## Operators

| Operator | Example  | Equivalent        |
| -------- | -------- | ----------------- |
| `=`      | `x = 10` | กำหนดค่า 10 ให้ x |
| `+=`     | `x += 5` | `x = x + 5`       |
| `-=`     | `x -= 5` | `x = x - 5`       |
| `*=`     | `x *= 5` | `x = x * 5`       |
| `/=`     | `x /= 5` | `x = x / 5`       |
| `%=`     | `x %= 5` | `x = x % 5`       |

## Examples

```rust
fn main() {
    let mut score = 100;

    score += 10;
    println!("After += : {}", score);

    score -= 20;
    println!("After -= : {}", score);

    score *= 2;
    println!("After *= : {}", score);

    score /= 3;
    println!("After /= : {}", score);

    score %= 5;
    println!("After %= : {}", score);
}
```

## Important Notes

ตัวแปรที่ต้องการปรับค่าต้องประกาศด้วย `mut`

ตัวอย่าง:

```rust
let mut number = 10;
number += 5;
```

หากประกาศด้วย `let number = 10;` จะไม่สามารถเปลี่ยนค่าเดิมได้

## Practice

1. สร้างตัวแปร `score` โดยกำหนดค่าเริ่มต้นเป็น 100
2. เพิ่มคะแนน 20
3. ลดคะแนน 10
4. คูณคะแนนด้วย 2
5. หารคะแนนด้วย 2

## Knowledge Check

1. `+=` มีความหมายอย่างไร?
2. ทำไมต้องใช้ `mut`?
3. `x *= 2` เทียบเท่ากับคำสั่งใด?
4. `x %= 3` ใช้ทำอะไร?
