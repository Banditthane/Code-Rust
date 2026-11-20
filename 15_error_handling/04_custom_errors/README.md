# Lesson 04: Custom Errors

## 1. What are Custom Errors?

Custom Errors คือการสร้างชนิดข้อมูล Error ของตัวเอง เพื่ออธิบายข้อผิดพลาดที่เกิดขึ้นภายในโปรแกรมให้ชัดเจน

ตัวอย่างสถานการณ์:

- จำนวนเงินไม่เพียงพอ
- คะแนนอยู่นอกช่วงที่กำหนด
- ผู้ใช้ไม่มีสิทธิ์ดำเนินการ
- ไม่พบข้อมูลที่ต้องการ

การสร้าง Custom Errors ช่วยให้จัดการข้อผิดพลาดแต่ละประเภทได้เป็นระบบ

## 2. Creating a Custom Error Enum

สามารถใช้ Enum เพื่อกำหนดประเภทของ Error

ตัวอย่าง:

```rust
enum BankError {
    InsufficientBalance,
    InvalidAmount,
    AccountNotFound,
}
```

Enum นี้กำหนด Error ไว้ 3 ประเภท

## 3. Returning Custom Errors

Function สามารถคืนค่า Custom Error ผ่าน `Result<T, E>`

ตัวอย่าง:

```rust
enum BankError {
    InsufficientBalance,
    InvalidAmount,
}

fn withdraw(balance: u32, amount: u32) -> Result<u32, BankError> {
    if amount == 0 {
        return Err(BankError::InvalidAmount);
    }

    if amount > balance {
        return Err(BankError::InsufficientBalance);
    }

    Ok(balance - amount)
}
```

## 4. Handling Custom Errors with match

ใช้ `match` เพื่อตรวจสอบ Error แต่ละประเภท

ตัวอย่าง:

```rust
fn main() {
    match withdraw(1000, 1500) {
        Ok(balance) => println!("Balance: {}", balance),
        Err(BankError::InsufficientBalance) => {
            println!("Error: Insufficient balance");
        }
        Err(BankError::InvalidAmount) => {
            println!("Error: Invalid amount");
        }
    }
}
```

ผลลัพธ์:

```text
Error: Insufficient balance
```

## 5. Implementing Display

หากต้องการให้ Custom Error แสดงข้อความผ่าน `{}` สามารถ Implement Trait `Display`

ตัวอย่าง:

```rust
use std::fmt;

enum BankError {
    InsufficientBalance,
    InvalidAmount,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientBalance => {
                write!(f, "Insufficient balance")
            }
            BankError::InvalidAmount => {
                write!(f, "Invalid amount")
            }
        }
    }
}
```

เมื่อ Implement `Display` แล้ว สามารถแสดง Error ได้:

```rust
println!("{}", error);
```

## 6. Implementing std::error::Error

Rust มี Trait `std::error::Error` สำหรับชนิดข้อมูลที่ต้องการทำหน้าที่เป็น Error

โดยทั่วไป Custom Error ที่ต้องการใช้กับระบบ Error มาตรฐานสามารถ Implement Trait นี้เพิ่มเติมได้

ตัวอย่าง:

```rust
impl std::error::Error for BankError {}
```

หากต้องการใช้งาน Trait นี้ ควร Implement `Display` ให้เรียบร้อยก่อน

## 7. Combining Custom Errors with Result

ตัวอย่างการรวมแนวคิด:

```

```
