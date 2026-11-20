# Lesson 02: Unit Tests

## Overview

Unit Testing คือการทดสอบส่วนย่อยของโปรแกรม เช่น Functions หรือ Modules เพื่อให้มั่นใจว่าแต่ละส่วนทำงานได้ถูกต้อง

ใน Rust สามารถเขียน Unit Tests ภายใน Module โดยใช้ `#[cfg(test)]` และสร้าง Module ชื่อ `tests`

## Learning Objectives

- เข้าใจ Unit Testing
- สร้าง Test Module ได้
- ใช้ `#[cfg(test)]` ได้
- ใช้ `assert!` ได้
- ใช้ `assert_eq!` ได้
- ใช้ `assert_ne!` ได้
- ทดสอบ Functions ที่คืนค่าได้
- ทดสอบ Error Cases ได้
- ทดสอบ Private Functions ภายใน Module ได้

## 1. What is Unit Testing?

Unit Testing คือการทดสอบส่วนย่อยของโปรแกรมแยกจากส่วนอื่น

ตัวอย่างสิ่งที่สามารถทดสอบ:

- Function สำหรับคำนวณ
- Function สำหรับตรวจสอบข้อมูล
- Function สำหรับแปลงข้อมูล
- Function ภายใน Module

## 2. Test Module

Rust นิยมจัด Unit Tests ไว้ใน Module ชื่อ `tests`

ตัวอย่างโครงสร้าง:

```text
src/
├── lib.rs
└── main.rs
```

ภายใน `lib.rs` สามารถเขียน Test Module ได้ดังนี้:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2 + 3, 5);
    }
}
```

## 3. The cfg(test) Attribute

`#[cfg(test)]` ใช้กำหนดให้โค้ดส่วนดังกล่าวถูก Compile เมื่ออยู่ในโหมด Testing

ตัวอย่าง:

```rust
#[cfg(test)]
mod tests {
    // Test functions
}
```

ข้อดี:

- แยก Test Code ออกจากโค้ดปกติ
- ไม่รวม Test Module ในการ Compile Library ตามปกติ
- ช่วยจัดระเบียบโค้ด

## 4. assert! Macro

`assert!` ใช้ตรวจสอบว่า Expression มีค่าเป็น `true` หรือไม่

ตัวอย่างรูปแบบ:

```rust
assert!(5 > 3);
```

หาก Expression เป็น `false` Test จะ Fail

## 5. assert_eq! Macro

`assert_eq!` ใช้ตรวจสอบว่าค่าทั้งสองเท่ากันหรือไม่

ตัวอย่าง:

```rust
assert_eq!(2 + 3, 5);
```

สามารถใช้กับชนิดข้อมูลที่รองรับ PartialEq และ Debug ตามข้อกำหนดของ Macro

## 6. assert_ne! Macro

`assert_ne!` ใช้ตรวจสอบว่าค่าทั้งสองไม่เท่ากัน

ตัวอย่าง:

```rust
assert_ne!(10, 20);
```

## 7. Testing Functions

สามารถทดสอบ Function ที่เขียนไว้ในโปรแกรมได้

ตัวอย่างแนวคิด:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Test ควรตรวจสอบ:

- ค่าบวก
- ค่าลบ
- ค่า 0
- ค่าที่อยู่บริเวณขอบเขตของชนิดข้อมูลตามความเหมาะสม

## 8. Testing Private Functions

Unit Tests ที่อยู่ภายใน Module สามารถเข้าถึง Private Items ของ Module ที่เป็นเจ้าของ Test Module ได้

ตัวอย่างโครงสร้าง:

```rust
mod calculator {
    fn internal_add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_internal_add() {
            assert_eq!(internal_add(2, 3), 5);
        }
    }
}
```

แนวทางนี้ช่วยให้สามารถทดสอบ Private Functions ได้โดยไม่ต้องเปิดเผย Function เหล่านั้นเป็น Public API

## 9. Testing Result

Function ที่คืนค่า `Result<T, E>` สามารถทดสอบได้เช่นกัน

Test Function สามารถคืนค่า `Result<(), E>` ได้ เพื่อใช้ `?` ภายใน Test

ตัวอย่างรูปแบบ:

```rust
#[test]
fn test_result() -> Result<(), String> {
    let value = 10;

    if value == 10 {
        Ok(())
    } else {
        Err("Unexpected value".to_string())
    }
}
```

## 10. Important Concepts

- Unit Tests ใช้ทดสอบส่วนย่อยของโปรแกรม
- `#[cfg(test)]` ใช้ควบคุมการ Compile Test Code
- `assert!` ตรวจสอบ Boolean Expression
- `assert_eq!` ตรวจสอบความเท่ากัน
- `assert_ne!` ตรวจสอบความไม่เท่ากัน
- Unit Tests ภายใน Module สามารถทดสอบ Private Items ของ Module นั้นได้
- Test Function สามารถคืนค่า `Result<(), E>` ได้

## Summary

Unit Testing ช่วยตรวจสอบ Functions และ Modules แยกเป็นส่วน ๆ ทำให้ค้นหาข้อผิดพลาดได้ง่ายขึ้น

## Checklist

- [ ] เข้าใจ Unit Testing
- [ ] สร้าง Test Module ได้
- [ ] ใช้ `#[cfg(test)]` ได้
- [ ] ใช้ `assert!` ได้
- [ ] ใช้ `assert_eq!` ได้
- [ ] ใช้ `assert_ne!` ได้
- [ ] ทดสอบ Functions ได้
- [ ] ทดสอบ Private Functions ได้
- [ ] ทดสอบ Result ได้
