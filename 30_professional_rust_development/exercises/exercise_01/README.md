# Exercise 01: Cargo Workspace

## ระดับ

Easy

## Objective

ฝึกสร้าง Cargo Workspace ที่ประกอบด้วยหลาย Packages และเรียนรู้การจัดการ Dependencies ร่วมกัน

## Problem Statement

สร้างระบบ Student Management Workspace ที่ประกอบด้วย 3 Packages:

1. `student_core` — Library สำหรับจัดการข้อมูลนักเรียน
2. `student_utils` — Library สำหรับ Utility Functions
3. `student_app` — Application สำหรับเรียกใช้งาน Libraries

## Requirements

### Part 1: Workspace Structure

สร้างโครงสร้าง:

```text
cargo_workspace/
├── Cargo.toml
├── Cargo.lock
├── student_core/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── student_utils/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── student_app/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

### Part 2: Workspace Configuration

กำหนด:

- Workspace Members
- Resolver
- Shared Dependencies
- Workspace Package Settings ตามความเหมาะสม

### Part 3: Student Core

สร้าง Student Struct:

- ID
- Name
- Score

กำหนดความสามารถ:

- สร้าง Student
- อ่านข้อมูล Student
- คำนวณผลการเรียนเบื้องต้น

### Part 4: Student Utils

สร้าง Utility Functions อย่างน้อย 3 รายการ เช่น:

- ตรวจสอบ Score
- แปลง Score เป็น Grade
- คำนวณ Average

### Part 5: Student App

Application ต้อง:

1. เรียกใช้ `student_core`
2. เรียกใช้ `student_utils`
3. สร้าง Student อย่างน้อย 3 คน
4. แสดงข้อมูลนักเรียน
5. แสดง Grade
6. แสดง Average Score

### Part 6: Workspace Dependencies

กำหนด Dependencies ที่ใช้ร่วมกันไว้ใน Workspace Root

ทดลองใช้ Workspace Dependency ใน Packages ที่ต้องการ

### Part 7: Workspace Commands

ทดลองคำสั่ง:

```powershell
cargo check --workspace
cargo build --workspace
cargo test --workspace
cargo run -p student_app
```

## Constraints

- ต้องมีอย่างน้อย 3 Packages
- ต้องมี Library Crates อย่างน้อย 2 Crates
- ต้องมี Binary Crate อย่างน้อย 1 Crate
- ต้องใช้ Path Dependencies สำหรับ Libraries ภายใน Workspace
- ต้องกำหนด Workspace Members
- ต้องใช้ Workspace Dependencies อย่างน้อย 1 รายการ
- ต้องแยกหน้าที่ของแต่ละ Package
- ต้อง Build และ Test ทั้ง Workspace ได้
- ต้องไม่มี Circular Dependency ระหว่าง Packages

## Expected Output

```text
Student Management Workspace

Student List
--------------------------------
ID: 1
Name: Alice
Score: 85
Grade: B
--------------------------------
ID: 2
Name: Bob
Score: 92
Grade: A
--------------------------------
ID: 3
Name: Charlie
Score: 76
Grade: C
--------------------------------

Average Score: 84.33

Workspace Application Completed.
```

ผลลัพธ์ Grade ต้องเป็นไปตามเกณฑ์ที่กำหนดใน Project

## Hints

คำสั่งสร้าง Workspace และ Packages:

```powershell
mkdir cargo_workspace
cd cargo_workspace

cargo new --lib student_core --vcs none
cargo new --lib student_utils --vcs none
cargo new student_app --vcs none
```

กำหนด Path Dependencies ใน `student_app/Cargo.toml`

ตัวอย่าง:

```toml
[dependencies]
student_core = { path = "../student_core" }
student_utils = { path = "../student_utils" }
```

## Completion Checklist

- [ ] สร้าง Workspace Root ได้
- [ ] สร้าง `student_core` ได้
- [ ] สร้าง `student_utils` ได้
- [ ] สร้าง `student_app` ได้
- [ ] กำหนด Workspace Members ได้
- [ ] กำหนด Workspace Dependencies ได้
- [ ] ใช้ Path Dependencies ได้
- [ ] สร้าง Student Struct ได้
- [ ] สร้าง Utility Functions ได้
- [ ] เรียก Libraries จาก Application ได้
- [ ] Build Workspace ได้
- [ ] Test Workspace ได้
- [ ] Run เฉพาะ Package ได้

## Run

```powershell
cd 30_professional_rust_development/exercises/exercise_01/cargo_workspace

cargo check --workspace
cargo test --workspace
cargo run -p student_app
```
