# Exercise 04: Professional Challenge

## ระดับ

Challenge

## Objective

ประยุกต์ใช้ความรู้ Rust ทั้งหมดเพื่อพัฒนา Application ที่มีโครงสร้างระดับ Professional โดยคำนึงถึง Code Quality, Security, Performance, Testing, Documentation และ Deployment

## Problem Statement

สร้างระบบ **Professional Task Management CLI**

เป็น Application สำหรับจัดการงาน โดยสามารถ:

- เพิ่ม Task
- แสดง Task
- ค้นหา Task
- แก้ไข Task
- ลบ Task
- เปลี่ยนสถานะ Task
- จัดเก็บข้อมูลถาวร
- อ่านและเขียน JSON
- จัดการ Configuration
- บันทึก Logs
- ตรวจสอบ Input
- จัดการ Errors

ต้องออกแบบให้สามารถเพิ่ม REST API หรือ Database ในอนาคตได้

## Part 1: Project Architecture

ออกแบบ Project ให้แบ่งความรับผิดชอบชัดเจน

ตัวอย่างโครงสร้าง:

```text
professional_challenge/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
├── .env.example
├── docs/
│   ├── architecture.md
│   └── usage.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── task.rs
│   ├── services/
│   │   ├── mod.rs
│   │   └── task_service.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   └── json_storage.rs
│   ├── errors.rs
│   └── logging.rs
├── tests/
│   └── integration_tests.rs
└── benches/
    └── performance.rs
```

สามารถปรับโครงสร้างตามความเหมาะสม

## Part 2: Task Model

สร้าง Task Struct:

| Field       | Type ตัวอย่าง                     |
| ----------- | --------------------------------- |
| id          | u64                               |
| title       | String                            |
| description | String                            |
| status      | TaskStatus                        |
| created_at  | DateTime หรือชนิดข้อมูลที่เหมาะสม |
| updated_at  | DateTime หรือชนิดข้อมูลที่เหมาะสม |

กำหนด Task Status:

```text
Pending
InProgress
Completed
```

ต้องรองรับ Serialization และ Deserialization

## Part 3: CLI Commands

สร้างคำสั่งอย่างน้อย:

```text
task-app add
task-app list
task-app show
task-app update
task-app delete
task-app complete
```

ตัวอย่าง:

```text
task-app add --title "Learn Rust" --description "Study Professional Rust"
```

ข้อกำหนด:

- ตรวจสอบ Arguments
- แสดง Help
- แสดง Error Message ที่เข้าใจง่าย
- กำหนด Exit Codes ตามผลการทำงาน
- ไม่ให้ CLI รับ Input ที่ผิดรูปแบบโดยไม่มีการแจ้งเตือน

สามารถเลือกใช้ CLI Library เช่น `clap`

## Part 4: Persistent Storage

จัดเก็บข้อมูลใน JSON File

ตัวอย่าง:

```text
data/
└── tasks.json
```

ความสามารถ:

1. โหลดข้อมูลเมื่อเริ่มโปรแกรม
2. เพิ่มข้อมูล
3. แก้ไขข้อมูล
4. ลบข้อมูล
5. บันทึกข้อมูลกลับลงไฟล์
6. จัดการไฟล์ที่ยังไม่มี
7. จัดการ JSON ที่ไม่ถูกต้อง

ข้อกำหนด:

- ใช้ Serde
- ใช้ JSON
- ตรวจสอบข้อมูลก่อนบันทึก
- ป้องกันการเขียนทับข้อมูลผิดพลาด
- พิจารณาการเขียนแบบ Atomic File Replacement เพื่อลดความเสี่ยงไฟล์เสียหายระหว่างบันทึก
- ห้ามสมมติว่า File I/O สำเร็จเสมอ

## Part 5: Configuration

อ่าน Configuration จาก Environment Variables

ตัวอย่าง:

```text
APP_ENV
APP_DATA_DIR
APP_LOG_LEVEL
```

ข้อกำหนด:

- ตรวจสอบ Configuration ที่จำเป็น
- กำหนด Default Values ตามความเหมาะสม
- แยก Development และ Production Configuration
- ไม่เก็บ Secrets ลง Source Code
- มี `.env.example` ที่ไม่มี Secret จริง

## Part 6: Error Handling

สร้าง Application Error Types

ตัวอย่าง:

```text
InvalidInput
TaskNotFound
DuplicateTask
StorageError
SerializationError
ConfigurationError
```

ข้อกำหนด:

- แยก Validation Error
- แยก Business Error
- แยก File I/O Error
- แยก Serialization Error
- แสดง Error Message ที่เหมาะสม
- ไม่ใช้ `unwrap()` ในเส้นทางการทำงานหลักโดยไม่มีเหตุผล
- ไม่เปิดเผย Internal Details ที่ไม่จำเป็น

## Part 7: Logging and Tracing

ใช้ Logging หรือ Tracing เพื่อบันทึก:

- Application Started
- Task Created
- Task Updated
- Task Deleted
- Storage Loaded
- Storage Saved
- Operation Failed

ข้อกำหนด:

- กำหนด Log Level
- ไม่บันทึก Secrets
- ไม่บันทึกข้อมูลส่วนบุคคลที่ไม่จำเป็น
- แยก Log Message ตามระดับความสำคัญ

## Part 8: Documentation

ต้องมี:

### README.md

- Project Overview
- Features
- Requirements
- Installation
- Usage
- Examples
- Testing
- Configuration

### Rustdoc

Documentation สำหรับ Public APIs

### Architecture Documentation

อธิบาย:

- Module Structure
- Data Flow
- Storage Design
- Error Handling Design

## Part 9: Code Quality

ต้องผ่านคำสั่ง:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo test --doc
```

ต้องตรวจสอบ:

- Code Readability
- Naming
- Module Responsibility
- Error Handling
- Ownership
- Unnecessary Clone
- Input Validation

## Part 10: Performance

เพิ่ม Benchmark สำหรับการทำงานอย่างน้อย 2 รายการ เช่น:

- การค้นหา Task
- การประมวลผล Task จำนวนมาก
- Serialization
- Deserialization

ต้อง:

1. สร้าง Baseline
2. Benchmark
3. วิเคราะห์ผล
4. ปรับปรุงอย่างน้อย 1 จุด
5. Benchmark ซ้ำ
6. บันทึกผล

ไม่จำเป็นต้อง Optimize ทุก Function หากไม่มีปัญหาด้าน Performance ที่วัดได้

## Part 11: Security

ตรวจสอบ:

- Input Validation
- File Path Handling
- Dependency Security
- Secrets Management
- Error Information
- Data Integrity
- Logging Privacy

ใช้:

```powershell
cargo audit
```

หากติดตั้ง Cargo Audit แล้ว

ตรวจสอบ `.gitignore` ให้ไม่ Commit:

- `.env`
- Runtime Data ที่ไม่ควรเผยแพร่
- Temporary Files
- Build Artifacts

## Part 12: Testing

เขียน Tests อย่างน้อย:

### Unit Tests

- Create Task
- Update Task
- Delete Task
- Change Status
- Validate Input
- Find Task
- Handle Duplicate Task

### Integration Tests

- Add Task
- List Tasks
- Update Task
- Delete Task
- Save Data
- Load Data
- Handle Invalid JSON
- Handle Missing File

### Documentation Tests

ทดสอบ Code Examples ใน Rustdoc

### Performance Tests

สร้าง Benchmark สำหรับการทำงานที่เลือก

ควรใช้ Temporary Directory หรือแยก Test Data เพื่อป้องกัน Tests กระทบข้อมูลจริง

## Part 13: Release and Deployment

เตรียม Application สำหรับ Release:

```powershell
cargo build --release
```

ตรวจสอบ:

- Binary ทำงานได้
- Configuration ถูกต้อง
- Data Directory เข้าถึงได้
- Logs ทำงาน
- Tests ผ่าน
- Documentation พร้อม
- ไม่มี Secret ใน Repository

ทดลอง Build สำหรับ Target Platform เพิ่มเติม หากมี Toolchain รองรับ

## Constraints

- ต้องใช้ Rust
- ต้องใช้ Cargo
- ต้องมี Project Structure ที่ชัดเจน
- ต้องมี CLI
- ต้องมี Persistent Storage
- ต้องใช้ Serde และ JSON
- ต้องมี Configuration
- ต้องมี Error Handling
- ต้องมี Logging หรือ Tracing
- ต้องมี Unit Tests
- ต้องมี Integration Tests
- ต้องมี Documentation Tests
- ต้องมี Benchmark อย่างน้อย 2 รายการ
- ต้องผ่าน Rustfmt
- ต้องผ่าน Clippy
- ต้องตรวจสอบ Dependencies
- ต้องไม่ Commit Secrets
- ต้องตรวจสอบ Input
- ต้องไม่ใช้ `unwrap()` ใน Main Application Flow โดยไม่มีเหตุผล
- ต้องไม่มี Unsafe Code ที่ไม่จำเป็น
- ต้องสร้าง Release Build ได้

## Expected Output

```text
Professional Task Management CLI

Application Started

Commands:
1. Add Task
2. List Tasks
3. Show Task
4. Update Task
5. Complete Task
6. Delete Task
0. Exit

Select: 1

Task Title: Learn Rust
Description: Study Professional Rust

Task created successfully!

Task ID: 1
Status: Pending

Data saved successfully.
```

ตัวอย่าง List:

```text
Task List
--------------------------------
ID: 1
Title: Learn Rust
Status: Pending
Description: Study Professional Rust
--------------------------------
```

ตัวอย่าง Error:

```text
Error: Task not found.

Code: TASK_NOT_FOUND
```

ผลลัพธ์และรูปแบบหน้าจอสามารถออกแบบได้เอง ตราบใดที่ความสามารถครบตาม Requirements

## Hints

Dependencies ที่อาจใช้:

```powershell
cargo add clap --features derive
cargo add serde --features derive
cargo add serde_json
cargo add thiserror
cargo add tracing
cargo add tracing-subscriber
cargo add chrono --features serde
cargo add criterion --dev
```

แนวทาง:

1. ออกแบบ Data Model
2. ออกแบบ Error Types
3. สร้าง Storage Layer
4. สร้าง Service Layer
5. สร้าง CLI
6. เพิ่ม Configuration
7. เพิ่ม Logging
8. เขียน Tests
9. เขียน Documentation
10. Benchmark
11. ตรวจสอบ Security
12. สร้าง Release Build

## Final Deliverables

ส่งมอบ Project ให้ครบ:

- [ ] Source Code
- [ ] Cargo.toml
- [ ] Cargo.lock
- [ ] README.md
- [ ] Architecture Documentation
- [ ] Usage Documentation
- [ ] Unit Tests
- [ ] Integration Tests
- [ ] Documentation Tests
- [ ] Benchmark
- [ ] Performance Report
- [ ] Security Checklist
- [ ] Release Build

## Completion Checklist

### Architecture

- [ ] ออกแบบ Project Structure
- [ ] แยก Models
- [ ] แยก Services
- [ ] แยก Storage
- [ ] แยก Errors
- [ ] แยก Configuration

### Application

- [ ] เพิ่ม Task
- [ ] แสดง Tasks
- [ ] ค้นหา Task
- [ ] แก้ไข Task
- [ ] ลบ Task
- [ ] เปลี่ยนสถานะ Task
- [ ] จัดเก็บข้อมูลถาวร

### Quality

- [ ] Rustfmt ผ่าน
- [ ] Clippy ผ่าน
- [ ] Unit Tests ผ่าน
- [ ] Integration Tests ผ่าน
- [ ] Documentation Tests ผ่าน
- [ ] Documentation ครบ

### Performance

- [ ] Benchmark อย่างน้อย 2 รายการ
- [ ] บันทึก Baseline
- [ ] วิเคราะห์ Bottlenecks
- [ ] Optimize อย่างน้อย 1 จุด
- [ ] Benchmark หลังปรับปรุง

### Security

- [ ] ตรวจสอบ Input
- [ ] ตรวจสอบ File Paths
- [ ] ตรวจสอบ Dependencies
- [ ] ไม่ Commit Secrets
- [ ] ตรวจสอบ Logging
- [ ] ตรวจสอบ Data Integrity

### Deployment

- [ ] Release Build ผ่าน
- [ ] Configuration พร้อม
- [ ] Data Directory พร้อม
- [ ] Logs ทำงาน
- [ ] ทดสอบ Application หลัง Build

## Run

```powershell
cd 30_professional_rust_development/exercises/exercise_04/professional_challenge

cargo check
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo test --doc
cargo build --release
cargo run --release
```

## Final Challenge

เมื่อระบบพื้นฐานสมบูรณ์แล้ว ให้เลือกพัฒนาเพิ่มเติมอย่างน้อย 2 รายการ:

1. เพิ่ม REST API ด้วย Axum
2. เปลี่ยน JSON File Storage เป็น SQLite หรือ PostgreSQL
3. เพิ่ม Authentication
4. เพิ่ม Docker Support
5. เพิ่ม CI ด้วย GitHub Actions
6. เพิ่ม Automated Release
7. เพิ่ม Cross Compilation
8. เพิ่ม Monitoring
9. เพิ่ม Backup และ Restore
10. เผยแพร่ Project ลง GitHub พร้อม Documentation

**หลักสำคัญ:** Application ระดับ Professional ต้องไม่ได้แค่ทำงานถูกต้อง แต่ต้องตรวจสอบได้
