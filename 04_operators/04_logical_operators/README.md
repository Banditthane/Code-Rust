# Lesson 04: Logical Operators

## Overview

Logical Operators ใช้สำหรับเชื่อมต่อหรือกลับค่าของเงื่อนไข Boolean เพื่อสร้างเงื่อนไขที่ซับซ้อนขึ้น

## Operators

| Operator | Name | Description                                     |
| -------- | ---- | ----------------------------------------------- | --- | ------------------------------------------------ |
| `&&`     | AND  | เป็น true เมื่อทั้งสองเงื่อนไขเป็น true         |
| `        |      | `                                               | OR  | เป็น true เมื่อมีอย่างน้อยหนึ่งเงื่อนไขเป็น true |
| `!`      | NOT  | กลับค่าจาก true เป็น false หรือ false เป็น true |

## AND Operator

ใช้ `&&` เมื่อต้องการให้ทุกเงื่อนไขเป็นจริง

```rust
fn main() {
    let age = 20;
    let has_id = true;

    let can_enter = age >= 18 && has_id;

    println!("{}", can_enter);
}
```

## OR Operator

ใช้ `||` เมื่อต้องการให้มีอย่างน้อยหนึ่งเงื่อนไขเป็นจริง

```rust
fn main() {
    let is_weekend = true;
    let is_holiday = false;

    let can_rest = is_weekend || is_holiday;

    println!("{}", can_rest);
}
```

## NOT Operator

ใช้ `!` เพื่อกลับค่าของ Boolean

```rust
fn main() {
    let is_raining = false;

    let is_not_raining = !is_raining;

    println!("{}", is_not_raining);
}
```

## Truth Table

### AND (`&&`)

| A     | B     | Result |
| ----- | ----- | ------ |
| true  | true  | true   |
| true  | false | false  |
| false | true  | false  |
| false | false | false  |

### OR (`||`)

| A     | B     | Result |
| ----- | ----- | ------ |
| true  | true  | true   |
| true  | false | true   |
| false | true  | true   |
| false | false | false  |

## Important Notes

Logical Operators ใช้กับค่าประเภท Boolean หรือ Expressions ที่ให้ผลลัพธ์เป็น Boolean

`&&` และ `||` เป็น Short-circuit Operators ซึ่งอาจไม่ประเมิน Expression ด้านขวา หากทราบผลลัพธ์จากด้านซ้ายแล้ว

## Practice

1. สร้างตัวแปร `age`
2. สร้างตัวแปร `has_ticket`
3. ตรวจสอบว่าอายุอย่างน้อย 18 ปี และมีตั๋วหรือไม่
4. ทดลองเปลี่ยนเงื่อนไขให้ใช้ `||`
5. ทดลองใช้ `!` กลับค่าของ Boolean

## Knowledge Check

1. `&&` ต้องการให้เงื่อนไขเป็นจริงกี่เงื่อนไข?
2. `||` จะได้ true เมื่อใด?
3. `!true` ได้ผลลัพธ์อะไร?
4. Short-circuit หมายถึงอะไร?
