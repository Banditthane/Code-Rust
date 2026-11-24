# Exercise 02: Documentation Practice

## ระดับ

Easy-Medium

## Objective

ฝึกเขียน Documentation สำหรับ Rust Library และปรับปรุง Code Quality ด้วย Rustfmt และ Clippy

## Problem Statement

สร้าง Library ชื่อ `student_library` สำหรับจัดการข้อมูลนักเรียน

Library ต้องมี API สำหรับ:

- สร้าง Student
- อ่านข้อมูล Student
- แก้ไข Score
- คำนวณ Grade
- ตรวจสอบ Score
- คำนวณ Average

ทุก Public API ต้องมี Documentation ที่ชัดเจน

## Requirements

### Part 1: Project Setup

สร้าง Library Project:

```powershell
cargo new --lib student_library --vcs none
```

### Part 2: Data Model

สร้าง Struct:

```text
Student
```

กำหนด Fields:

- ID
- Name
- Score

ออกแบบ Visibility ให้เหมาะสม

### Part 3: Public API

สร้าง Public Functions หรือ Methods อย่างน้อย 5 รายการ

ตัวอย่างความสามารถ:

1. Create Student
2. Get Student Information
3. Update Score
4. Calculate Grade
5. Calculate Average

### Part 4: Rustdoc

เขียน Documentation ให้:

- Crate
- Public Struct
- Public Enum
- Public Functions
- Public Methods

Documentation ของ Function ต้องอธิบาย:

- หน้าที่
- Arguments
- Return Value
- Error Conditions
- Examples

หาก Function มีโอกาส Panic หรือมี Safety Requirements ต้องอธิบายตามพฤติกรรมจริง

### Part 5: Documentation Examples

เพิ่ม Code Examples ใน Rustdoc อย่างน้อย 3 ตัวอย่าง

ตัวอย่างต้องสามารถ Compile และ Run ผ่าน Documentation Tests ได้

### Part 6: Rustfmt

จัดรูปแบบ Code:

```powershell
cargo fmt
```

ตรวจสอบ:

```powershell
cargo fmt --check
```

### Part 7: Clippy

ตรวจสอบ:

```powershell
cargo clippy --all-targets --all-features -- -D warnings
```

แก้ไข Warnings ที่เกิดขึ้น หรืออธิบายเหตุผลหากมีกรณีจำเป็นต้องยกเว้น Lint

### Part 8: Documentation Build

สร้าง Documentation:

```powershell
cargo doc
```

เปิด Documentation:

```powershell
cargo doc --open
```

รัน Documentation Tests:

```powershell
cargo test --doc
```

## Constraints

- ต้องสร้างเป็น Library Crate
- ต้องมี Public API อย่างน้อย 5 รายการ
- ต้องมี Rustdoc Documentation สำหรับ Public API
- ต้องมี Documentation Examples อย่างน้อย 3 ตัวอย่าง
- ต้องมี Unit Tests
- ต้องใช้ Rustfmt
- ต้องใช้ Clippy
- ต้องไม่มี Warning ที่ยังไม่ได้ตรวจสอบ
- ต้องไม่มี Documentation ที่ขัดแย้งกับพฤติกรรมจริง
- ต้องไม่เปิดเผย Fields โดยไม่จำเป็น

## Expected Output

```text
Documentation Practice

Checking formatting...
Formatting check passed.

Running Clippy...
Clippy completed successfully.

Running tests...
All tests passed.

Running documentation tests...
3 documentation tests passed.

Generating documentation...
Documentation generated successfully.

Student Library is ready.
```

จำนวน Tests จริงขึ้นอยู่กับจำนวนที่เขียน

## Hints

คำสั่งที่ควรรู้:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo test --doc
cargo doc --open
```

แนวทาง:

- ใช้ `///` สำหรับ Public API
- ใช้ `//!` สำหรับ Crate-level Documentation
- ใช้ `# Examples` สำหรับตัวอย่าง
- ใช้ `# Errors` สำหรับ Function ที่คืนค่า Error
- ใช้ `# Panics` เมื่อ Function อาจ Panic
- อย่าเขียน Documentation ที่รับประกันเกินกว่าพฤติกรรมจริง

## Completion Checklist

- [ ] สร้าง Library Crate ได้
- [ ] สร้าง Student Struct ได้
- [ ] สร้าง Public API อย่างน้อย 5 รายการ
- [ ] เขียน Crate Documentation ได้
- [ ] เขียน Struct Documentation ได้
- [ ] เขียน Function Documentation ได้
- [ ] เพิ่ม Examples อย่างน้อย 3 รายการ
- [ ] `cargo fmt --check` ผ่าน
- [ ] Clippy ผ่าน
- [ ] Unit Tests ผ่าน
- [ ] Documentation Tests ผ่าน
- [ ] สร้าง Documentation ได้
- [ ] ตรวจสอบ Documentation ด้วยตนเอง

## Run

```powershell
cd 30_professional_rust_development/exercises/exercise_02/documentation_practice

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo test --doc
cargo doc --open
```
