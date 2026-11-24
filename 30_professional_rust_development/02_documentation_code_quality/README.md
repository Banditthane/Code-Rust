# Lesson 02: Documentation and Code Quality

## 1. Code Quality คืออะไร?

Code Quality คือคุณภาพของ Source Code ในด้านต่าง ๆ เช่น:

- Readability: อ่านเข้าใจง่าย
- Maintainability: ดูแลรักษาง่าย
- Reliability: ทำงานได้อย่างน่าเชื่อถือ
- Testability: ทดสอบได้
- Consistency: มีรูปแบบสม่ำเสมอ
- Documentation: มีเอกสารอธิบายการใช้งาน

Code ที่ทำงานได้ไม่ได้หมายความว่าจะมีคุณภาพดีเสมอไป

## 2. Rust Documentation

Rust มีระบบ Documentation ในตัวผ่าน Rustdoc

Documentation ช่วยอธิบาย:

- Crate
- Module
- Struct
- Enum
- Function
- Trait
- Method
- Examples

ใช้ `///` สำหรับ Documentation Comments ที่ผูกกับ Item

ตัวอย่าง:

````rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = add(10, 20);
/// assert_eq!(result, 30);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

## 3. Documentation Comment Types

### `///`

ใช้เขียน Documentation ให้ Item ถัดไป

```rust
/// Represents a user.
pub struct User {
    pub id: u32,
    pub name: String,
}
```

### `//!`

ใช้เขียน Documentation ให้ Crate หรือ Module ปัจจุบัน

```rust
//! User management library.
//!
//! This crate provides basic user operations.
```

### `//`

เป็น Comment ปกติ ไม่ได้ถูกนำไปแสดงเป็น Rustdoc Documentation โดยตรง

## 4. Documentation Sections

สามารถจัดหัวข้อใน Documentation ได้ เช่น:

````rust
/// Divides two numbers.
///
/// # Arguments
///
/// * `a` - Numerator
/// * `b` - Denominator
///
/// # Returns
///
/// The division result.
///
/// # Errors
///
/// Returns an error if the denominator is zero.
///
/// # Examples
///
/// ```
/// assert_eq!(divide(10.0, 2.0), Ok(5.0));
/// ```
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Cannot divide by zero"));
    }

    Ok(a / b)
}
````

หัวข้อที่นิยมใช้:

- `# Examples`
- `# Panics`
- `# Errors`
- `# Safety`
- `# Arguments`
- `# Returns`

ควรเขียนเฉพาะข้อมูลที่ตรงกับพฤติกรรมจริงของ Function

## 5. Generate Documentation

สร้าง Documentation:

```powershell
cargo doc
```

เปิด Documentation หลังสร้าง:

```powershell
cargo doc --open
```

สร้าง Documentation รวม Dependencies:

```powershell
cargo doc --no-deps
```

หมายเหตุ: `--no-deps` ใช้สร้าง Documentation ของ Package ปัจจุบันโดยไม่สร้าง Documentation ของ Dependencies

## 6. Documentation Tests

Code Examples ใน Rustdoc สามารถกลายเป็น Tests ได้

ตัวอย่าง:

````rust
/// Adds two numbers.
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

รัน Documentation Tests:

```powershell
cargo test --doc
```

ข้อดี:

- ตัวอย่างสามารถตรวจสอบได้จริง
- ลดโอกาสที่ Documentation ล้าสมัย
- ช่วยให้ผู้ใช้เข้าใจ API

## 7. Rustfmt

Rustfmt ใช้จัดรูปแบบ Rust Code ให้เป็นมาตรฐาน

Format Project:

```powershell
cargo fmt
```

ตรวจสอบโดยไม่แก้ไขไฟล์:

```powershell
cargo fmt --check
```

ข้อดี:

- รูปแบบ Code สม่ำเสมอ
- ลดการถกเถียงเรื่อง Formatting
- ทำให้ Code Review มุ่งเน้นที่ Logic มากขึ้น

## 8. Clippy

Clippy เป็น Linter สำหรับตรวจสอบรูปแบบ Code และแนะนำแนวทางเขียน Rust

ตรวจสอบ:

```powershell
cargo clippy
```

ตรวจสอบทั้ง Workspace:

```powershell
cargo clippy --workspace --all-targets
```

ให้ Warning กลายเป็น Error:

```powershell
cargo clippy -- -D warnings
```

Clippy สามารถตรวจจับ:

- Code ที่อาจผิดพลาด
- รูปแบบที่ไม่เป็น Idiomatic Rust
- Unnecessary Clones
- Inefficient Patterns
- Redundant Expressions
- API Usage ที่ควรปรับปรุง

ไม่ควรแก้ Warning ทุกข้อโดยไม่ทำความเข้าใจคำแนะนำก่อน

## 9. Lint Levels

Rust รองรับระดับการแจ้งเตือน:

| Level  | ความหมาย                                      |
| ------ | --------------------------------------------- |
| allow  | ไม่แจ้งเตือน                                  |
| warn   | แสดง Warning                                  |
| deny   | ทำให้การ Compile ล้มเหลวเมื่อพบ               |
| forbid | ป้องกันการเปลี่ยนระดับ Lint ใน Scope ที่กำหนด |

ตัวอย่าง:

```rust
#![deny(warnings)]
```

ควรใช้การตั้งค่า Lint อย่างระมัดระวัง เพราะ Warning บางประเภทอาจแตกต่างตาม Rust Version และ Dependencies

## 10. Code Review

Code Review คือการตรวจสอบ Code โดยผู้พัฒนาหรือสมาชิกทีม

สิ่งที่ควรตรวจสอบ:

### Correctness

- Logic ถูกต้องหรือไม่
- มี Edge Cases หรือไม่
- Error Handling ครบหรือไม่

### Readability

- ชื่อ Variables เข้าใจง่ายหรือไม่
- Function มีหน้าที่ชัดเจนหรือไม่
- Code ซับซ้อนเกินจำเป็นหรือไม่

### Safety

- มีการใช้ `unsafe` อย่างเหมาะสมหรือไม่
- Ownership และ Borrowing ถูกต้องหรือไม่
- Input ได้รับการตรวจสอบหรือไม่

### Performance

- มีการ Clone โดยไม่จำเป็นหรือไม่
- มีการจัดสรร Memory มากเกินไปหรือไม่
- มี Algorithm ที่ไม่มีประสิทธิภาพหรือไม่

### Testing

- มี Unit Tests หรือไม่
- มี Integration Tests หรือไม่
- มีการทดสอบ Error Cases หรือไม่

## 11. Continuous Integration

Continuous Integration (CI) ช่วยให้ระบบตรวจสอบ Code โดยอัตโนมัติเมื่อมีการเปลี่ยนแปลง

ตัวอย่างขั้นตอน CI:

```text
Push Code
    |
    v
cargo fmt --check
    |
    v
cargo clippy
    |
    v
cargo test
    |
    v
Build
```

สามารถใช้ GitHub Actions หรือ CI Platform อื่นได้

## 12. Quality Checklist

ก่อนส่ง Code:

- Format ผ่าน
- Clippy ผ่าน
- Tests ผ่าน
- Documentation ถูกต้อง
- ไม่มี Secrets ใน Source Code
- ไม่มี Debug Code ที่ไม่จำเป็น
- Error Handling เหมาะสม
- Dependencies ผ่านการตรวจสอบ

## 13. สรุป

| Tool               | หน้าที่                      |
| ------------------ | ---------------------------- |
| Rustdoc            | สร้าง Documentation          |
| `cargo doc`        | Generate Documentation       |
| `cargo test --doc` | ทดสอบ Documentation Examples |
| Rustfmt            | จัดรูปแบบ Code               |
| Clippy             | ตรวจสอบ Code                 |
| Code Review        | ตรวจสอบคุณภาพโดยมนุษย์       |
| CI                 | ตรวจสอบอัตโนมัติ             |

## Completion Checklist

- [ ] เข้าใจ Rustdoc
- [ ] ใช้ `///` ได้
- [ ] ใช้ `//!` ได้
- [ ] เขียน Documentation Sections ได้
- [ ] สร้าง Documentation ด้วย `cargo doc` ได้
- [ ] ใช้ Documentation Tests ได้
- [ ] ใช้ Rustfmt ได้
- [ ] ใช้ Clippy ได้
- [ ] เข้าใจ Lint Levels
- [ ] ทำ Code Review ได้
- [ ] เข้าใจ Continuous Integration
- [ ] สร้าง Quality Checklist ได้
