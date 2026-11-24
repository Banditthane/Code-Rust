# Lesson 01: Advanced Cargo

## 1. Cargo คืออะไร?

Cargo คือ Build System และ Package Manager ของ Rust

Cargo ช่วยจัดการ:

- สร้าง Project
- Compile Code
- จัดการ Dependencies
- Run Application
- Run Tests
- สร้าง Documentation
- จัดการ Workspace
- สร้าง Release Builds

คำสั่งพื้นฐาน:

```powershell
cargo new my_project
cargo check
cargo build
cargo run
cargo test
cargo doc
```

## 2. Cargo.toml

`Cargo.toml` เป็น Manifest File สำหรับกำหนดข้อมูลและการตั้งค่าของ Package

ตัวอย่าง:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
```

ส่วนสำคัญ:

| Section                | หน้าที่                                   |
| ---------------------- | ----------------------------------------- |
| `[package]`            | ข้อมูล Package                            |
| `[dependencies]`       | Dependencies สำหรับใช้งาน                 |
| `[dev-dependencies]`   | Dependencies สำหรับ Development และ Tests |
| `[build-dependencies]` | Dependencies สำหรับ Build Script          |
| `[features]`           | กำหนด Feature Flags                       |

## 3. Dependencies

Dependency คือ Library หรือ Package ที่ Project ต้องใช้

เพิ่ม Dependency:

```powershell
cargo add serde
```

เพิ่ม Dependency พร้อม Feature:

```powershell
cargo add tokio --features full
```

ลบ Dependency:

```powershell
cargo remove serde
```

ตรวจสอบ Dependencies:

```powershell
cargo tree
```

ตรวจสอบ Dependencies ที่ใหม่กว่า:

```powershell
cargo update
```

ข้อควรจำ: `cargo update` จะอัปเดตการเลือกเวอร์ชันภายใต้เงื่อนไขใน Manifest และอาจเปลี่ยน `Cargo.lock` แต่ไม่ได้เปลี่ยน Version Requirement ใน `Cargo.toml` โดยอัตโนมัติ

## 4. Semantic Versioning

Rust Dependencies มักใช้ Semantic Versioning

รูปแบบ:

```text
MAJOR.MINOR.PATCH
```

ความหมายโดยทั่วไป:

- MAJOR: การเปลี่ยนแปลงที่ไม่เข้ากันกับเวอร์ชันเดิม
- MINOR: เพิ่มความสามารถที่ยังเข้ากันได้
- PATCH: แก้ไข Bug โดยไม่เปลี่ยน API ที่เข้ากันได้

ตัวอย่าง:

```toml
serde = "1.0"
```

Cargo ใช้ Version Requirement เพื่อเลือกเวอร์ชันที่เข้ากันได้ ไม่ได้หมายความว่าต้องใช้เฉพาะ `1.0.0`

## 5. Cargo.lock

`Cargo.lock` บันทึกเวอร์ชัน Dependencies ที่ Cargo เลือกใช้จริง

ประโยชน์:

- ทำให้การ Build ทำซ้ำได้ง่ายขึ้น
- ลดปัญหา Dependencies เปลี่ยนเวอร์ชันโดยไม่ตั้งใจ
- ช่วยให้สมาชิกทีมใช้ชุด Dependencies เดียวกัน

สำหรับ Application ควร Commit `Cargo.lock` ลง Git

สำหรับ Library ที่เผยแพร่ให้ผู้อื่นนำไปใช้ โดยทั่วไปมักไม่ Commit `Cargo.lock` เพราะผู้ใช้ Library จะเป็นผู้เลือก Dependency Resolution ของตนเอง

## 6. Development Dependencies

ใช้ `[dev-dependencies]` สำหรับ Packages ที่จำเป็นเฉพาะตอนพัฒนาและทดสอบ

ตัวอย่าง:

```toml
[dev-dependencies]
criterion = "0.5"
```

เหมาะกับ:

- Benchmarking
- Test Helpers
- Mock Libraries
- Development Tools

## 7. Cargo Features

Features ใช้เปิดหรือปิดความสามารถบางส่วนของ Crate

ตัวอย่าง:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
```

หรือกำหนด Features ของ Project:

```toml
[features]
default = ["logging"]
logging = []
database = []
```

เลือกเปิด Feature:

```powershell
cargo build --features database
```

ปิด Default Features ของ Dependency:

```toml
[dependencies]
some_crate = {
    version = "1",
    default-features = false
}
```

ข้อควรระวัง:

- Feature อาจเปิด Dependencies เพิ่ม
- Feature บางรายการอาจเปิดพร้อมกันแล้วเกิดผลกระทบต่อกัน
- ต้องทดสอบชุด Feature ที่ Project รองรับ
- ไม่ควรสร้าง Feature มากเกินความจำเป็น

## 8. Cargo Workspace

Workspace คือกลุ่ม Packages ที่จัดการร่วมกัน

เหมาะกับ Project ที่มีหลาย Crates เช่น:

- Application
- Library
- Database Layer
- Shared Types
- CLI Tool

ตัวอย่างโครงสร้าง:

```text
my_workspace/
├── Cargo.toml
├── Cargo.lock
├── app/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── core/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── shared/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

Workspace Root `Cargo.toml`:

```toml
[workspace]
members = [
    "app",
    "core",
    "shared"
]
resolver = "3"
```

ค่า Resolver ต้องเลือกให้เหมาะกับ Edition และความต้องการของ Workspace

## 9. Workspace Dependencies

สามารถกำหนด Dependencies ที่ใช้ร่วมกันไว้ที่ Root

ตัวอย่าง:

```toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

ใน Package ที่ต้องการใช้:

```toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

ประโยชน์:

- จัดการเวอร์ชันจากจุดเดียว
- ลดการกำหนด Dependencies ซ้ำ
- ทำให้หลาย Crates ใช้ Dependency Requirement สอดคล้องกัน

## 10. Workspace Commands

Build ทั้ง Workspace:

```powershell
cargo build --workspace
```

Test ทั้ง Workspace:

```powershell
cargo test --workspace
```

ตรวจสอบทุก Package:

```powershell
cargo check --workspace
```

เลือก Package:

```powershell
cargo test -p app
```

Run Package:

```powershell
cargo run -p app
```

## 11. Package และ Crate

คำสองคำนี้มีความหมายต่างกัน:

| คำ            | ความหมาย                              |
| ------------- | ------------------------------------- |
| Package       | หน่วยที่ Cargo จัดการ มี `Cargo.toml` |
| Crate         | หน่วยการ Compile ของ Rust             |
| Workspace     | กลุ่ม Packages ที่จัดการร่วมกัน       |
| Library Crate | Crate ที่สร้าง Library                |
| Binary Crate  | Crate ที่สร้าง Executable             |

Package หนึ่งสามารถมี Library Crate และ Binary Crate หลายตัวได้ตามข้อกำหนดของ Cargo

## 12. Dependency Management Best Practices

แนวทาง:

1. เพิ่ม Dependencies เท่าที่จำเป็น
2. ตรวจสอบ License
3. ตรวจสอบความน่าเชื่อถือของ Crate
4. ตรวจสอบ Release และ Maintenance
5. ใช้ Cargo.lock ใน Application
6. ตรวจสอบ Dependency Tree
7. อัปเดต Dependencies อย่างมีแผน
8. ทดสอบหลังอัปเดต
9. ตรวจสอบ Security Advisories
10. หลีกเลี่ยง Dependencies ที่ไม่มีความจำเป็น

## 13. สรุป

- `Cargo.toml`: กำหนด Package และ Dependencies
- `Cargo.lock`: บันทึก Dependency Resolution
- `cargo add`: เพิ่ม Dependency
- `cargo tree`: ตรวจสอบ Dependency Tree
- Features: เปิดความสามารถเพิ่มเติม
- Workspace: จัดการหลาย Packages ร่วมกัน
- Workspace Dependencies: กำหนด Dependencies ร่วมกัน

## Completion Checklist

- [ ] เข้าใจ Cargo.toml
- [ ] เข้าใจ Cargo.lock
- [ ] เพิ่ม Dependency ได้
- [ ] ลบ Dependency ได้
- [ ] ใช้ `cargo tree` ได้
- [ ] เข้าใจ Semantic Versioning
- [ ] เข้าใจ Dev Dependencies
- [ ] ใช้ Cargo Features ได้
- [ ] สร้าง Cargo Workspace ได้
- [ ] กำหนด Workspace Members ได้
- [ ] ใช้ Workspace Dependencies ได้
- [ ] Build ทั้ง Workspace ได้
- [ ] Test ทั้ง Workspace ได้
- [ ] เลือก Build Package ได้
- [ ] เข้าใจ Dependency Management
