# Rust Complete — Learning Repository

Repository สำหรับเรียนรู้ภาษา **Rust ตั้งแต่ระดับ Beginner ถึง Advanced** ผ่านบทเรียน แบบฝึกหัด และโปรเจกต์จริง โดยจัดเนื้อหาเป็นลำดับจากพื้นฐานไปสู่การพัฒนา Software ที่พร้อมใช้งานจริง

## Objectives

- เรียนรู้ Syntax และพื้นฐานของภาษา Rust
- เข้าใจ Ownership, Borrowing และ Lifetime
- ใช้งาน Collections, Structs, Enums, Traits และ Generics
- พัฒนาโปรแกรมด้วย Error Handling, Testing และ Modules
- เรียนรู้ Smart Pointers, Concurrency และ Async Programming
- พัฒนา CLI, Networking, Web API และ Database Applications
- ฝึกใช้เครื่องมือสำหรับ Professional Rust Development
- สร้างผลงานจริงและ Portfolio

## Learning Roadmap

หลักสูตรแบ่งออกเป็น 5 Phases รวม 31 Chapters

### Phase 1 — Rust Fundamentals

| Chapter | Topic        |
| ------- | ------------ |
| 00      | Setup        |
| 01      | Fundamentals |
| 02      | Variables    |
| 03      | Data Types   |
| 04      | Operators    |
| 05      | Conditions   |
| 06      | Loops        |
| 07      | Functions    |
| 08      | Strings      |
| 09      | Ownership    |
| 10      | Borrowing    |

### Phase 2 — Rust Core Concepts

| Chapter | Topic           |
| ------- | --------------- |
| 11      | Data Structures |
| 12      | Structs         |
| 13      | Enums           |
| 14      | Modules         |
| 15      | Error Handling  |
| 16      | Collections     |
| 17      | Traits          |
| 18      | Generics        |
| 19      | Lifetimes       |
| 20      | Testing         |

### Phase 3 — Advanced Rust

| Chapter | Topic                        |
| ------- | ---------------------------- |
| 21      | Projects                     |
| 22      | Closures & Iterators         |
| 23      | Smart Pointers               |
| 24      | Concurrency & Multithreading |
| 25      | Async Programming            |
| 26      | Macros                       |
| 27      | Unsafe Rust                  |
| 28      | Networking & Web Development |
| 29      | Database & Serialization     |

### Phase 4 — Professional Rust Development

| Chapter | Topic                         |
| ------- | ----------------------------- |
| 30      | Professional Rust Development |

### Phase 5 — Real-world Projects & Portfolio

| Chapter | Topic                           |
| ------- | ------------------------------- |
| 31      | Real-world Projects & Portfolio |

## Repository Structure

```text
proj-code-rust/
├── README.md
├── ROADMAP.md
├── PROGRESS.md
├── CONTRIBUTING.md
├── LICENSE
├── docs/
│   ├── learning-guide.md
│   ├── coding-standard.md
│   ├── exercise-rules.md
│   └── glossary.md
├── 00_setup/
├── 01_fundamentals/
├── 02_variables/
├── 03_data_types/
├── 04_operators/
├── 05_conditions/
├── 06_loops/
├── 07_functions/
├── 08_strings/
├── 09_ownership/
├── 10_borrowing/
├── 11_data_structures/
├── 12_structs/
├── 13_enums/
├── 14_modules/
├── 15_error_handling/
├── 16_collections/
├── 17_traits/
├── 18_generics/
├── 19_lifetimes/
├── 20_testing/
├── 21_projects/
├── 22_closures_iterators/
├── 23_smart_pointers/
├── 24_concurrency_multithreading/
├── 25_async_programming/
├── 26_macros/
├── 27_unsafe_rust/
├── 28_networking_web_development/
├── 29_database_serialization/
├── 30_professional_rust_development/
├── 31_real_world_projects_portfolio/
└── resources/
    ├── references.md
    └── useful-links.md
```

> หมายเหตุ: ชื่อโฟลเดอร์ของแต่ละ Chapter ให้ยึดตามชื่อที่ใช้อยู่จริงใน Repository หากต่างจากตัวอย่างนี้ ไม่จำเป็นต้องเปลี่ยนชื่อโฟลเดอร์เพื่อให้ตรงกับ README

## Learning Method

แต่ละ Chapter ประกอบด้วย:

- `README.md` — ภาพรวมของบท
- `01_...` ถึง `04_...` — บทเรียน 4 หัวข้อ
- `exercises/` — แบบฝึกหัด 4 ระดับ

รูปแบบแบบฝึกหัด:

1. Easy
2. Easy–Medium
3. Medium
4. Challenge

แต่ละ Exercise ควรมี README ระบุ:

- Objective
- Problem Statement
- Requirements
- Constraints
- Expected Output
- Hints
- Completion Checklist

แบบฝึกหัดเน้นให้ผู้เรียนลงมือเขียนโค้ดด้วยตนเอง โดยไม่แสดงเฉลยล่วงหน้า

## Development Environment

เครื่องมือหลัก:

- Rustup
- Rust Compiler (`rustc`)
- Cargo
- Visual Studio Code
- rust-analyzer
- rustfmt
- Clippy

ตรวจสอบการติดตั้ง:

```powershell
rustc --version
cargo --version
rustup --version
rustup show
```

ติดตั้ง Components ที่จำเป็น:

```powershell
rustup component add rustfmt clippy
```

## Common Cargo Commands

สร้างโปรเจกต์:

```powershell
cargo new my_project --vcs none
```

รันโปรแกรม:

```powershell
cargo run
```

ตรวจสอบ Compile:

```powershell
cargo check
```

จัดรูปแบบโค้ด:

```powershell
cargo fmt
```

ตรวจสอบคุณภาพโค้ด:

```powershell
cargo clippy
```

รันทดสอบ:

```powershell
cargo test
```

สร้าง Release Build:

```powershell
cargo build --release
```

## Learning Principles

- เรียนตามลำดับจากพื้นฐานไปขั้นสูง
- อ่านบทเรียนก่อนเริ่มแบบฝึกหัด
- ทดลองเขียนโค้ดด้วยตนเอง
- อ่าน Compiler Errors และทำความเข้าใจสาเหตุ
- ใช้ `cargo fmt`, `cargo clippy` และ `cargo test` อย่างสม่ำเสมอ
- ไม่ข้ามหัวข้อสำคัญ เช่น Ownership, Borrowing และ Lifetimes
- สร้างโปรเจกต์จริงเพื่อทบทวนความรู้

## Completion Criteria

ถือว่าเรียนจบหลักสูตรเมื่อ:

- [ ] ผ่านเนื้อหาทั้ง 31 Chapters
- [ ] ทำแบบฝึกหัดตามลำดับ
- [ ] เข้าใจ Ownership และ Borrowing
- [ ] ใช้ Cargo และเครื่องมือพื้นฐานได้
- [ ] เขียน Unit Tests และ Integration Tests ได้
- [ ] พัฒนา CLI Application ได้
- [ ] พัฒนา Web API และเชื่อมต่อ Database ได้
- [ ] เข้าใจ Async และ Concurrency
- [ ] ทำ Final Project สำเร็จ
- [ ] มี Repository และ Documentation สำหรับ Portfolio

## License

โปรเจกต์นี้ใช้ License ตามที่ระบุในไฟล์ `LICENSE`

---

**Learn Rust step by step. Build projects. Become a Rust developer.**
