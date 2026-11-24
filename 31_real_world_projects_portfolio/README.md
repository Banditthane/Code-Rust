# Chapter 31: Real-world Projects & Portfolio

## เป้าหมายของบทเรียน

นำความรู้ Rust จาก Chapter 01–30 มาประยุกต์ใช้ในการสร้าง Software Projects ที่ใกล้เคียงกับการพัฒนาระบบจริง

เมื่อเรียนจบบทนี้ ควรสามารถ:

- วิเคราะห์ Project Requirements ได้
- กำหนด Project Scope ได้
- ออกแบบ System Architecture ได้
- แบ่งระบบออกเป็น Modules และ Components ได้
- เลือก Data Structures ให้เหมาะสมได้
- ออกแบบ Error Handling ได้
- พัฒนา REST API ได้
- เชื่อมต่อ Database ได้
- ใช้ Async Processing ได้
- ออกแบบ Authentication และ Authorization เบื้องต้นได้
- เขียน Unit Tests และ Integration Tests ได้
- ตรวจสอบ Security ได้
- ตรวจสอบ Performance ได้
- เตรียม Application สำหรับ Production ได้
- จัดทำ GitHub Repository ได้
- เขียน Project Documentation ได้
- สร้าง Architecture Diagram ได้
- จัดทำ Demo และ Portfolio ได้
- ประเมินคุณภาพผลงานของตนเองได้

## บทเรียน

| Lesson | หัวข้อ                           | เนื้อหา                            |
| ------ | -------------------------------- | ---------------------------------- |
| 01     | Project Architecture             | Requirements, Scope, System Design |
| 02     | Backend and Data Projects        | REST API, Database, Authentication |
| 03     | Testing and Production Readiness | Testing, Security, Deployment      |
| 04     | Portfolio and Final Assessment   | GitHub, Documentation, Demo        |

## แบบฝึกหัด

| Exercise | ชื่อ                     | ระดับ       |
| -------- | ------------------------ | ----------- |
| 01       | Project Planning         | Easy        |
| 02       | Backend and Data Project | Easy-Medium |
| 03       | Production Readiness     | Medium      |
| 04       | Final Portfolio Project  | Challenge   |

## โครงสร้างโฟลเดอร์

```text
31_real_world_projects_portfolio/
├── README.md
├── 01_project_architecture/
│   └── README.md
├── 02_backend_data_projects/
│   └── README.md
├── 03_testing_production_readiness/
│   └── README.md
├── 04_portfolio_final_assessment/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   └── README.md
    ├── exercise_02/
    │   └── README.md
    ├── exercise_03/
    │   └── README.md
    └── exercise_04/
        └── README.md
```

## เทคโนโลยีที่อาจนำมาใช้

### Rust Core

- Ownership
- Borrowing
- Lifetimes
- Structs
- Enums
- Traits
- Generics
- Error Handling
- Collections
- Smart Pointers

### Backend

- Axum
- Tokio
- Serde
- REST API
- HTTP

### Database

- SQLite
- PostgreSQL
- SQLx
- Database Migrations

### Quality and Security

- Rustfmt
- Clippy
- Unit Tests
- Integration Tests
- Cargo Audit
- Logging
- Tracing

### Development Tools

- Cargo
- Git
- GitHub
- VS Code
- Docker (Optional)
- GitHub Actions (Optional)

## แนวทางการเรียน

1. เลือกปัญหาที่ต้องการแก้ไข
2. วิเคราะห์ Requirements
3. กำหนด Project Scope
4. ออกแบบ Architecture
5. ออกแบบ Database และ Data Models
6. วางแผน API
7. พัฒนา Core Features
8. เพิ่ม Error Handling
9. เพิ่ม Tests
10. ตรวจสอบ Security
11. ตรวจสอบ Performance
12. เตรียม Production Configuration
13. เขียน Documentation
14. สร้าง GitHub Repository
15. จัดทำ Demo และ Portfolio

## หลักการทำ Real-world Projects

### 1. Solve a Real Problem

Project ควรมีปัญหาหรือความต้องการที่ชัดเจน

### 2. Keep the Scope Manageable

เริ่มจาก MVP (Minimum Viable Product) ก่อน แล้วค่อยเพิ่ม Features

### 3. Design Before Coding

ควรวิเคราะห์ Requirements และออกแบบระบบก่อนเริ่มเขียน Code

### 4. Test Continuously

ทดสอบระหว่างพัฒนา ไม่ใช่รอจนเสร็จทั้งหมด

### 5. Secure by Design

คำนึงถึง Security ตั้งแต่เริ่มออกแบบ

### 6. Document the Project

ผู้อื่นควรเข้าใจวิธีติดตั้ง วิธีใช้งาน และโครงสร้างของระบบได้

### 7. Demonstrate Your Work

Portfolio ต้องแสดงให้เห็นว่า Project ทำอะไร ใช้เทคโนโลยีอะไร และแก้ปัญหาอย่างไร

## Completion Checklist

- [ ] วิเคราะห์ Requirements ได้
- [ ] กำหนด Project Scope ได้
- [ ] ออกแบบ System Architecture ได้
- [ ] ออกแบบ Module Structure ได้
- [ ] ออกแบบ Data Models ได้
- [ ] พัฒนา REST API ได้
- [ ] เชื่อมต่อ Database ได้
- [ ] ใช้ Async Processing ได้
- [ ] ออกแบบ Authentication เบื้องต้นได้
- [ ] เขียน Unit Tests ได้
- [ ] เขียน Integration Tests ได้
- [ ] ตรวจสอบ Security ได้
- [ ] ตรวจสอบ Performance ได้
- [ ] เตรียม Production Configuration ได้
- [ ] จัดทำ GitHub Repository ได้
- [ ] เขียน README ได้
- [ ] สร้าง Architecture Diagram ได้
- [ ] จัดทำ Demo ได้
- [ ] สร้าง Portfolio Project ได้

## เกณฑ์ผ่านบทเรียน

สามารถสร้าง Rust Project ที่มี Requirements ชัดเจน มี Architecture เหมาะสม มี Tests มี Documentation ผ่านการตรวจสอบคุณภาพ และสามารถนำเสนอผลงานให้ผู้อื่นเข้าใจได้

**หลักสำคัญ:** Portfolio ที่ดีไม่ได้วัดจากจำนวนบรรทัด Code แต่วัดจากความสามารถในการแก้ปัญหา คุณภาพของระบบ และความสามารถในการอธิบายการตัดสินใจทางเทคนิค
