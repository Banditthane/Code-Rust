# Lesson 04: Test Attributes and Documentation

## Overview

Rust มี Test Attributes สำหรับกำหนดพฤติกรรมของ Tests และมี Documentation Tests สำหรับทดสอบตัวอย่างโค้ดที่เขียนอยู่ในเอกสารของโปรแกรม

บทนี้จะเรียนรู้วิธีควบคุมการทำงานของ Tests และสร้าง Documentation ที่สามารถทดสอบได้จริง

## Learning Objectives

- เข้าใจ Test Attributes
- ใช้ `#[should_panic]` ได้
- ใช้ `#[ignore]` ได้
- รัน Ignored Tests ได้
- เข้าใจ Expected Panic
- เขียน Documentation Comments ได้
- สร้าง Documentation Tests ได้
- รัน Documentation Tests ด้วย Cargo ได้

## 1. Test Attributes

Test Attributes คือ Attributes ที่ใช้กำหนดพฤติกรรมหรือเงื่อนไขของ Test Functions

ตัวอย่าง:

- `#[test]`
- `#[should_panic]`
- `#[ignore]`

## 2. The should_panic Attribute

`#[should_panic]` ใช้ระบุว่า Test จะผ่านเมื่อเกิด Panic ตามที่คาดหวัง

ตัวอย่างรูปแบบ:

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("Expected panic");
}
```

Test นี้จะผ่านเมื่อเกิด Panic

หาก Function ทำงานจนจบโดยไม่เกิด Panic Test จะ Fail

### Expected Panic Message

สามารถระบุข้อความที่คาดหวังได้:

```rust
#[test]
#[should_panic(expected = "invalid value")]
fn test_invalid_value() {
    panic!("invalid value");
}
```

Test จะผ่านเมื่อเกิด Panic และข้อความ Panic มีข้อความที่กำหนดอยู่

## 3. The ignore Attribute

`#[ignore]` ใช้ข้าม Test บางตัวเมื่อรัน Tests ตามปกติ

ตัวอย่าง:

```rust
#[test]
#[ignore]
fn test_large_operation() {
    // Test implementation
}
```

Test ที่มี `#[ignore]` จะไม่ถูกรันในการทดสอบปกติ

## 4. Running Ignored Tests

รัน Tests ที่ถูก Ignore:

```powershell
cargo test -- --ignored
```

รัน Tests ทั้งหมด รวมทั้ง Tests ที่ถูก Ignore:

```powershell
cargo test -- --include-ignored
```

## 5. Documentation Comments

Rust ใช้ Documentation Comments เพื่อสร้างเอกสารประกอบโค้ด

รูปแบบ:

````rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = 2 + 3;
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

Documentation Comments ใช้ `///` และสามารถใส่ตัวอย่างโค้ดเพื่อให้ Rust นำไปทดสอบได้

## 6. Documentation Tests

Documentation Tests คือการทดสอบโค้ดตัวอย่างที่อยู่ใน Documentation Comments

ตัวอย่างโค้ดภายใน Code Block ที่เขียนใน Documentation สามารถถูกรันเป็น Tests ได้

คำสั่ง:

```powershell
cargo test --doc
```

หรือใช้:

```powershell
cargo test
```

เพื่อรัน Tests ทั้งหมด รวมถึง Documentation Tests ที่เกี่ยวข้อง

## 7. Useful Documentation Sections

Documentation ของ Rust สามารถจัดหัวข้อเพิ่มเติมได้ เช่น:

- `# Examples`
- `# Panics`
- `# Errors`
- `# Safety`

หัวข้อเหล่านี้ช่วยอธิบายวิธีใช้งาน เงื่อนไขการเกิด Panic ข้อผิดพลาด และข้อควรระวังของ Function

## 8. Important Concepts

- `#[test]` ใช้ระบุ Test Function
- `#[should_panic]` ใช้ทดสอบกรณีที่คาดว่าจะเกิด Panic
- `expected` ใช้ตรวจสอบข้อความใน Panic
- `#[ignore]` ใช้ข้าม Test ในการรันปกติ
- `cargo test -- --ignored` ใช้รัน Ignored Tests
- `cargo test -- --include-ignored` ใช้รัน Tests ทั้งหมดรวม Ignored Tests
- `///` ใช้เขียน Documentation Comments
- `cargo test --doc` ใช้รัน Documentation Tests

## Summary

Test Attributes ช่วยควบคุมพฤติกรรมของ Tests ส่วน Documentation Tests ช่วยตรวจสอบว่าโค้ดตัวอย่างในเอกสารสามารถทำงานได้จริง

## Checklist

- [ ] เข้าใจ Test Attributes
- [ ] ใช้ `#[should_panic]` ได้
- [ ] ใช้ `expected` ได้
- [ ] ใช้ `#[ignore]` ได้
- [ ] รัน Ignored Tests ได้
- [ ] เขียน Documentation Comments ได้
- [ ] สร้าง Documentation Tests ได้
- [ ] รัน `cargo test --doc` ได้
