# Exercise 03: Production Readiness

## ระดับ

Medium

## Objective

ฝึกตรวจสอบ Backend Application ที่พัฒนาแล้ว และเตรียมระบบให้พร้อมสำหรับ Production Environment

## Problem Statement

นำ Inventory Management API จาก Exercise 02 มาปรับปรุงให้มีคุณภาพและพร้อมสำหรับการส่งมอบ

ไม่จำเป็นต้องเพิ่ม Features จำนวนมาก แต่ต้องเน้นความถูกต้อง ความปลอดภัย การทดสอบ และการดูแลระบบ

## Requirements

### Part 1: Code Quality

ตรวจสอบ:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo test --doc
```

แก้ไขปัญหาที่พบ

### Part 2: Error Handling

ตรวจสอบ Error Cases:

- Invalid Request
- Missing Product
- Database Connection Failure
- Database Query Failure
- Invalid Configuration
- Unexpected Internal Error

กำหนด Error Responses ให้สม่ำเสมอ

ห้ามส่ง Stack Trace หรือ Database Details ให้ Client โดยไม่จำเป็น

### Part 3: Security Review

ตรวจสอบ:

- Input Validation
- SQL Injection Prevention
- Authentication (หากมี)
- Authorization (หากมี)
- Secrets Management
- Dependency Security
- File และ Network Configuration

ใช้ Parameterized Queries สำหรับ Database Operations

### Part 4: Dependency Audit

ติดตั้งเครื่องมือ:

```powershell
cargo install cargo-audit
```

ตรวจสอบ:

```powershell
cargo audit
```

บันทึกผล:

- Dependencies ที่ตรวจสอบ
- Advisory ที่พบ (ถ้ามี)
- ผลกระทบ
- แนวทางแก้ไข
- Dependencies ที่ต้องติดตาม

### Part 5: Configuration

แยก Configuration สำหรับ:

- Development
- Testing
- Production

ใช้ Environment Variables เช่น:

```text
APP_ENV
APP_PORT
DATABASE_URL
APP_LOG_LEVEL
```

ข้อกำหนด:

- ไม่ Hardcode Secrets
- มี `.env.example`
- ตรวจสอบค่าที่จำเป็นตอนเริ่มระบบ
- ไม่ Commit `.env` ที่มีข้อมูลลับจริง

### Part 6: Logging and Tracing

เพิ่ม Logging หรือ Tracing สำหรับ:

- Application Startup
- Incoming Requests
- Request Completion
- Database Errors
- Validation Failures
- Unexpected Errors
- Application Shutdown

ไม่บันทึก Password, Token หรือ Database Credentials

### Part 7: Health Checks

เพิ่ม Endpoint:

```text
GET /health
GET /ready
```

กำหนดความหมายให้ชัดเจน:

- `/health` ใช้ตรวจสอบว่า Application ยังทำงานอยู่
- `/ready` ใช้ตรวจสอบว่า Application พร้อมให้บริการหรือไม่

หาก Readiness ขึ้นอยู่กับ Database ต้องตรวจสอบการเชื่อมต่ออย่างเหมาะสมโดยไม่สร้างภาระเกินจำเป็น

### Part 8: Performance Review

วัด:

- API Response Time
- Database Query Time
- Memory Usage
- CPU Usage

เลือกปรับปรุงอย่างน้อย 1 จุด

บันทึกผลก่อนและหลังปรับปรุง

### Part 9: Graceful Shutdown

ออกแบบให้ Application:

- หยุดรับ Requests ใหม่
- จัดการงานที่กำลังทำอยู่
- ปิด Database Connections อย่างเหมาะสม
- จบ Process อย่างเป็นระเบียบ

กำหนด Shutdown Timeout

### Part 10: Backup and Recovery

จัดทำแผน:

- Database Backup
- Backup Frequency
- Backup Retention
- Restore Procedure
- Data Recovery Verification

ต้องทดสอบ Restore จริงใน Environment แยกต่างหาก

### Part 11: Deployment Preparation

สร้าง Release Build:

```powershell
cargo build --release
```

จัดทำ Deployment Instructions:

- Requirements
- Environment Variables
- Database Setup
- Migration Steps
- Start Application
- Health Check
- Shutdown
- Rollback

## Constraints

- ต้องใช้ Project จาก Exercise 02 หรือ Backend Project ที่เทียบเท่า
- ต้องผ่าน Rustfmt
- ต้องผ่าน Clippy
- ต้องผ่าน Tests
- ต้องตรวจสอบ Dependencies
- ต้องมี Configuration แยก Environment
- ต้องไม่มี Secrets ใน Git
- ต้องมี Error Handling
- ต้องมี Logging หรือ Tracing
- ต้องมี Health Checks
- ต้องมี Performance Review
- ต้องมี Backup และ Restore Plan
- ต้องมี Release Build
- ต้องมี Deployment Instructions

## Expected Output

```text
Production Readiness Review

Code Quality
--------------------------------
Formatting: Passed
Clippy: Passed
Unit Tests: Passed
Integration Tests: Passed
--------------------------------

Security
--------------------------------
Dependency Audit: Reviewed
Input Validation: Reviewed
Secrets Management: Reviewed
--------------------------------

Operations
--------------------------------
Logging: Ready
Health Checks: Ready
Configuration: Ready
Graceful Shutdown: Reviewed
Backup Plan: Documented
--------------------------------

Performance
--------------------------------
Baseline: Recorded
Optimization: Completed
Results: Documented
--------------------------------

Release Build: Completed
Production Readiness Review: Completed
```

ผลลัพธ์ต้องมาจากการตรวจสอบจริง ส่วนที่ยังไม่ผ่านต้องระบุไว้ ไม่ควรทำเครื่องหมาย Ready โดยไม่มีหลักฐาน

## Hints

จัดทำเอกสาร:

```text
production_readiness/
├── README.md
├── security_review.md
├── performance_report.md
├── deployment_guide.md
├── backup_restore.md
└── readiness_checklist.md
```

ควรแยก:

- สิ่งที่ตรวจสอบแล้ว
- สิ่งที่ผ่าน
- สิ่งที่ยังไม่ผ่าน
- ความเสี่ยงที่เหลืออยู่
- แนวทางแก้ไข

## Completion Checklist

- [ ] Rustfmt ผ่าน
- [ ] Clippy ผ่าน
- [ ] Unit Tests ผ่าน
- [ ] Integration Tests ผ่าน
- [ ] ตรวจสอบ Error Handling
- [ ] ตรวจสอบ Input Validation
- [ ] ตรวจสอบ SQL Injection Prevention
- [ ] ตรวจสอบ Dependencies
- [ ] จัดการ Environment Configuration
- [ ] ตรวจสอบ Secrets Management
- [ ] เพิ่ม Logging หรือ Tracing
- [ ] เพิ่ม Health Checks
- [ ] วัด Performance
- [ ] Optimize อย่างน้อย 1 จุด
- [ ] ออกแบบ Graceful Shutdown
- [ ] จัดทำ Backup Plan
- [ ] ทดสอบ Restore
- [ ] สร้าง Release Build
- [ ] เขียน Deployment Guide
- [ ] เขียน Readiness Report

## Deliverables

- Security Review
- Performance Report
- Deployment Guide
- Backup and Restore Plan
- Production Readiness Checklist
- Release Build

## Final Review

ก่อนถือว่าพร้อม Production ต้องตอบคำถามได้ว่า:

1. หาก Database หยุดทำงาน ระบบจะจัดการอย่างไร?
2. หาก Input ไม่ถูกต้อง ระบบจะตอบอย่างไร?
3. หาก Application หยุดทำงาน ข้อมูลจะเสียหายหรือไม่?
4. จะทราบได้อย่างไรว่าระบบมีปัญหา?
5. จะ Restore ข้อมูลได้อย่างไร?
6. จะ Rollback Version ได้อย่างไร?
