# Exercise 04: Final Portfolio Project

## ระดับ

Challenge

## Objective

สร้าง Rust Application ขนาดจริงหนึ่ง Project เพื่อใช้เป็นผลงานหลักใน Rust Portfolio

ต้องประยุกต์ใช้ความรู้จาก Chapter 01–30 ทั้งด้าน Rust Fundamentals, Architecture, Backend, Database, Async, Testing, Security, Performance และ Deployment

## Problem Statement

เลือกพัฒนา Project หนึ่งรายการ:

1. Task Management System
2. Inventory Management System
3. Student Management System
4. Library Management System
5. Expense Tracking System
6. Personal Finance System
7. Booking Management System
8. E-commerce Backend
9. Project Management API
10. Custom Project ที่มีปัญหาและผู้ใช้งานชัดเจน

Project ต้องมี Use Case จริงและสามารถสาธิตการทำงานได้

## Part 1: Project Planning

จัดทำ:

- Project Overview
- Problem Statement
- Target Users
- Functional Requirements
- Non-functional Requirements
- User Stories
- Acceptance Criteria
- MVP Scope
- Future Features

ต้องระบุขอบเขตของ Version แรกให้ชัดเจน

## Part 2: System Architecture

ออกแบบ:

- System Architecture
- Module Structure
- Data Flow
- Database Schema
- API Design
- Error Handling Strategy
- Security Design

จัดทำ Architecture Diagram อย่างน้อย 1 รูป

## Part 3: Rust Application

พัฒนา Application ด้วย Rust

ต้องใช้แนวคิดที่เหมาะสมจาก:

- Structs
- Enums
- Ownership
- Borrowing
- Traits
- Generics
- Error Handling
- Collections
- Modules

ไม่จำเป็นต้องใช้ทุก Feature ในทุก Project แต่ต้องเลือกใช้ให้เหมาะสมกับปัญหา

## Part 4: Backend

หากเลือก Backend Project ต้องมี:

- REST API
- HTTP Methods
- Request Validation
- Response Formatting
- Error Responses
- API Documentation

สามารถเลือกใช้ Axum และ Tokio

หากเลือก CLI หรือ Desktop-Oriented Project ให้กำหนด Interface ที่เหมาะสมแทน REST API

## Part 5: Database

หาก Project ต้องจัดเก็บข้อมูลถาวร ให้ใช้ Database ที่เหมาะสม เช่น:

- SQLite
- PostgreSQL

ต้องมี:

- Database Schema
- Primary Keys
- Foreign Keys ตามความเหมาะสม
- CRUD Operations
- Parameterized Queries
- Transactions เมื่อจำเป็น
- Migrations
- Error Handling

## Part 6: Authentication and Authorization

หาก Project มีผู้ใช้งานหลายคนหรือข้อมูลที่ต้องจำกัดสิทธิ์ ให้พิจารณา:

- User Registration
- Login
- Password Hashing
- Session หรือ Token
- Authentication
- Authorization
- Role-Based Access Control ตามความเหมาะสม

ต้องตรวจสอบสิทธิ์ในทุก Operation ที่เข้าถึง Resource ไม่ใช่เพียงตรวจสอบว่าผู้ใช้ Login แล้ว

## Part 7: Async Processing

หาก Project มี Network หรือ I/O Operations ให้พิจารณาใช้:

- Tokio
- Async/Await
- Futures
- Async Database Operations
- Task Management
- Timeout
- Graceful Shutdown

ไม่ควรใช้ Async เพียงเพื่อให้ Project ดูซับซ้อนขึ้น หากไม่จำเป็นกับงาน

## Part 8: Error Handling

กำหนด Error Types ที่เหมาะสมกับ Project

ต้องรองรับ:

- Invalid Input
- Resource Not Found
- Conflict
- Database Errors
- External Service Errors (ถ้ามี)
- Internal Errors

ไม่ควรใช้ `unwrap()` ใน Main Application Flow โดยไม่มีเหตุผล

## Part 9: Testing

ต้องมี:

### Unit Tests

ทดสอบ Business Logic และ Functions หลัก

### Integration Tests

ทดสอบการทำงานร่วมกันระหว่าง Components

### API Tests

หากมี Backend ให้ทดสอบ HTTP Requests และ Responses

### Documentation Tests

ทดสอบ Code Examples ใน Rustdoc

### Error Tests

ทดสอบ Invalid Inputs และ Failure Cases

ต้องแยก Test Data ออกจากข้อมูลจริง

## Part 10: Code Quality

ตรวจสอบ:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo test --doc
```

ต้อง:

- ใช้ Naming ที่สื่อความหมาย
- แยก Modules ตามหน้าที่
- ลด Code Duplication
- หลีกเลี่ยง Abstraction ที่ไม่จำเป็น
- จัดการ Error อย่างเหมาะสม
- เขียน Documentation สำหรับ Public APIs

## Part 11: Security Review

ตรวจสอบ:

- Input Validation
- Authentication
- Authorization
- SQL Injection Prevention
- Password Hashing (ถ้ามี Password)
- Secrets Management
- Dependency Security
- Logging Privacy
- File และ Network Access

ใช้ `cargo audit` ตรวจสอบ Dependency Advisories หากติดตั้งเครื่องมือแล้ว

## Part 12: Performance Review

เลือกวัด Performance ของงานที่สำคัญอย่างน้อย 2 รายการ

ตัวอย่าง:

- API Response Time
- Database Query Time
- Data Processing
- Memory Usage
- File Operations

ต้อง:

1. สร้าง Baseline
2. วัดผล
3. วิเคราะห์ Bottleneck
4. ปรับปรุงอย่างน้อย 1 จุด
5. วัดผลอีกครั้ง
6. บันทึกผล

ไม่จำเป็นต้อง Optimize ทุกส่วน หากไม่มีปัญหาที่วัดได้

## Part 13: Documentation

จัดทำเอกสารอย่างน้อย:

### README.md

- Project Overview
- Problem Statement
- Features
- Technology Stack
- Requirements
- Installation
- Configuration
- Usage
- Testing
- Demo
- Limitations
- Future Improvements
- License

### Architecture Documentation

อธิบาย:

- Components
- Module Structure
- Data Flow
- Database Design
- API Design
- Technical Decisions

### API Documentation

หากมี REST API ให้ระบุ:

- Endpoints
- Methods
- Parameters
- Request Examples
- Response Examples
- Status Codes
- Error Responses

### Development Documentation

อธิบาย:

- วิธี Setup
- วิธี Run
- วิธี Test
- วิธี Build
- วิธี Configure

## Part 14: GitHub Repository

สร้าง GitHub Repository สำหรับ Project

ต้องมี:

- Source Code
- README.md
- Cargo.toml
- Cargo.lock สำหรับ Application
- `.gitignore`
- `.env.example` หากใช้ Environment Configuration
- Documentation
- Tests
- License ตามสิทธิ์และเงื่อนไขของ Project

ห้าม Commit:

- `.env` ที่มี Secrets จริง
- Passwords
- API Tokens
- Private Keys
- Database Credentials

## Part 15: CI/CD

หากทำได้ ให้สร้าง CI Workflow ที่ตรวจสอบ:

1. Formatting
2. Clippy
3. Unit Tests
4. Integration Tests
5. Documentation Tests
6. Release Build

สามารถใช้ GitHub Actions

Deployment Automation เป็นส่วนเพิ่มเติมตามความเหมาะสม

## Part 16: Demo

จัดทำ Demo อย่างน้อยหนึ่งรูปแบบ:

- Screenshots
- Terminal Recording
- Video
- Hosted Application
- API Demonstration

Demo ต้องแสดง Features หลักและวิธีใช้งานจริง

หากเป็น Backend Project ควรแสดง Request และ Response ตัวอย่างด้วย

## Constraints

- ต้องใช้ Rust เป็นภาษาหลัก
- ต้องมี Project Requirements
- ต้องมี Architecture Design
- ต้องมี Data Models
- ต้องมี Error Handling
- ต้องมี Tests
- ต้องมี Documentation
- ต้องมี Security Review
- ต้องมี Performance Review ตามความเหมาะสม
- ต้องผ่าน Rustfmt
- ต้องผ่าน Clippy
- ต้อง Build แบบ Release ได้
- ต้องมี GitHub Repository
- ต้องมี Demo
- ต้องระบุ Limitations
- ต้องระบุ Future Improvements
- ต้องไม่มี Secrets ใน Repository
- ต้องไม่อ้างว่าระบบผ่านการตรวจสอบ หากยังไม่ได้ทดสอบจริง

## Expected Output

ผลงานสุดท้ายควรมีโครงสร้างใกล้เคียง:

```text
final_rust_project/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── .env.example
├── docs/
│   ├── architecture.md
│   ├── development.md
│   └── api.md
├── migrations/
├── src/
├── tests/
├── benches/
└── .github/
    └── workflows/
```

โครงสร้างนี้เป็นตัวอย่าง สามารถปรับตามประเภท Project ได้

## Portfolio Presentation

เตรียมข้อมูลสำหรับนำเสนอ:

### 1. Project Name

ชื่อ Project ที่สื่อความหมาย

### 2. Problem

Project นี้แก้ปัญหาอะไร?

### 3. Solution

ระบบทำงานอย่างไร?

### 4. Technology Stack

ใช้ Rust Crates และ Tools อะไร?

### 5. Architecture

ระบบแบ่งเป็น Components อย่างไร?

### 6. Technical Decisions

ทำไมจึงเลือก Architecture และ Technologies เหล่านั้น?

### 7. Testing

ทดสอบอะไรบ้าง และผลเป็นอย่างไร?

### 8. Security

ตรวจสอบความปลอดภัยอย่างไร?

### 9. Performance

วัดและปรับปรุง Performance อย่างไร?

### 10. Limitations

ระบบยังมีข้อจำกัดอะไร?

### 11. Future Improvements

หากพัฒนาต่อจะเพิ่มอะไร?

## Final Deliverables

- [ ] Project Source Code
- [ ] Cargo.toml
- [ ] Cargo.lock
- [ ] README.md
- [ ] Architecture Documentation
- [ ] Requirements Document
- [ ] Database Schema (ถ้ามี)
- [ ] API Documentation (ถ้ามี)
- [ ] Unit Tests
- [ ] Integration Tests
- [ ] Documentation Tests
- [ ] Security Review
- [ ] Performance Report
- [ ] GitHub Repository
- [ ] Release Build
- [ ] Demo
- [ ] Portfolio Presentation

## Final Completion Checklist

### Planning

- [ ] เลือก Project ได้
- [ ] วิเคราะห์ Requirements ได้
- [ ] กำหนด MVP ได้
- [ ] เขียน User Stories ได้
- [ ] กำหนด Acceptance Criteria ได้

### Architecture

- [ ] ออกแบบ Architecture ได้
- [ ] แบ่ง Modules ได้
- [ ] ออกแบบ Data Models ได้
- [ ] ออกแบบ Data Flow ได้
- [ ] ออกแบบ Error Handling ได้

### Development

- [ ] พัฒนา Core Features ได้
- [ ] พัฒนา Backend หรือ Interface ได้
- [ ] เชื่อมต่อ Database ได้ตามความเหมาะสม
- [ ] จัดการ Async Operations ได้ตามความเหมาะสม
- [ ] จัดการ Errors ได้

### Quality

- [ ] Unit Tests ผ่าน
- [ ] Integration Tests ผ่าน
- [ ] Documentation Tests ผ่าน
- [ ] Rustfmt ผ่าน
- [ ] Clippy ผ่าน
- [ ] Release Build ผ่าน

### Security and Performance

- [ ] ตรวจสอบ Input
- [ ] ตรวจสอบ Authorization ตามความเหมาะสม
- [ ] ตรวจสอบ Dependencies
- [ ] ตรวจสอบ Secrets
- [ ] บันทึก Performance Baseline
- [ ] วิเคราะห์ Bottlenecks
- [ ] บันทึกผลการปรับปรุง

### Portfolio

- [ ] สร้าง GitHub Repository
- [ ] เขียน README
- [ ] เขียน Architecture Documentation
- [ ] จัดทำ Demo
- [ ] ระบุ Limitations
- [ ] ระบุ Future Improvements
- [ ] เตรียม Portfolio Presentation

## Final Challenge

หลังจากทำ Project เสร็จ ให้ทดลองให้คนอื่น Clone Repository แล้วทำตาม README เพื่อ Setup และ Run Application

ถ้าผู้อื่นสามารถทำตามขั้นตอนได้โดยไม่ต้องถามวิธีใช้งานเพิ่มเติม แสดงว่า Project มี Documentation ที่ช่วยให้เริ่มต้นได้ดี

**เป้าหมายสุดท้าย:** สร้าง Rust Project ที่สามารถอธิบายได้ ทดสอบได้ Build ได้ และนำเสนอเป็นผลงานจริงบน GitHub
