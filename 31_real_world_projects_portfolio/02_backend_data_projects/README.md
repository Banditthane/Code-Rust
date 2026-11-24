# Lesson 02: Backend and Data Projects

## 1. Backend Development คืออะไร?

Backend Development คือการพัฒนาส่วนการทำงานเบื้องหลังของ Application

ตัวอย่างหน้าที่:

- รับ HTTP Requests
- ตรวจสอบข้อมูล
- ประมวลผล Business Logic
- ติดต่อ Database
- ตรวจสอบ Authentication
- จัดการ Authorization
- ส่ง HTTP Responses

## 2. Rust Backend Ecosystem

Crates ที่นิยมใช้ใน Rust Backend:

| Crate     | หน้าที่                             |
| --------- | ----------------------------------- |
| Axum      | Web Framework                       |
| Tokio     | Async Runtime                       |
| Serde     | Serialization และ Deserialization   |
| SQLx      | Database Access                     |
| Tower     | Middleware และ Service Abstractions |
| Tracing   | Logging และ Diagnostics             |
| Thiserror | สร้าง Error Types                   |

เลือกใช้เฉพาะ Crates ที่จำเป็นต่อ Project

## 3. REST API

REST API เป็นแนวทางออกแบบ API โดยใช้ Resources และ HTTP Methods

ตัวอย่าง:

```text
GET    /api/tasks
GET    /api/tasks/{id}
POST   /api/tasks
PUT    /api/tasks/{id}
PATCH  /api/tasks/{id}
DELETE /api/tasks/{id}
```

ควรกำหนด:

- Resource Names
- Request Body
- Response Body
- Status Codes
- Validation Rules
- Error Responses
- Pagination
- Filtering
- Sorting

## 4. JSON Serialization

Serde ช่วยแปลง Rust Data Structures เป็น JSON และแปลง JSON กลับเป็น Rust Types

ตัวอย่างแนวคิด:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
}
```

ข้อควรระวัง:

- ตรวจสอบข้อมูลจาก Client
- ไม่เปิดเผย Fields ภายในโดยไม่จำเป็น
- แยก API Request/Response Models ออกจาก Database Models เมื่อมีเหตุผล
- จัดการข้อมูลที่ขาดหายหรือมีรูปแบบผิด

## 5. Database Integration

Backend มักต้องใช้ Database เพื่อเก็บข้อมูลถาวร

ตัวอย่าง Database:

- SQLite
- PostgreSQL
- MySQL

SQLx ช่วยเชื่อมต่อ Database แบบ Async

งานที่ต้องทำ:

- Create Connection Pool
- Execute Queries
- Insert Records
- Select Records
- Update Records
- Delete Records
- Run Transactions
- Manage Migrations

ควรใช้ Parameterized Queries เพื่อป้องกัน SQL Injection

## 6. Repository Pattern

Repository Pattern ช่วยแยก Data Access ออกจาก Business Logic

ตัวอย่างแนวคิด:

```text
Task Service
     |
     v
Task Repository
     |
     v
Database
```

Service รับผิดชอบ Business Rules ส่วน Repository รับผิดชอบการอ่านและเขียนข้อมูล

ไม่จำเป็นต้องสร้าง Abstraction ซับซ้อนเกินความจำเป็นสำหรับ Project ขนาดเล็ก

## 7. Authentication

Authentication คือการตรวจสอบว่า User เป็นใคร

ตัวอย่าง:

- Username and Password
- Session
- Access Token
- JWT

กระบวนการทั่วไป:

1. รับ Credentials
2. ตรวจสอบข้อมูล
3. ตรวจสอบ Password Hash
4. สร้าง Session หรือ Token
5. ส่งผลการ Authentication กลับ

ข้อควรระวัง:

- ห้ามเก็บ Password แบบ Plain Text
- ใช้ Password Hashing Algorithm ที่ออกแบบมาสำหรับ Password เช่น Argon2
- ห้ามส่ง Secret Key ให้ Client
- ใช้ HTTPS ในการส่งข้อมูลจริง
- กำหนดอายุและวิธีเพิกถอน Token ให้เหมาะสม

## 8. Authorization

Authorization คือการตรวจสอบว่า User มีสิทธิ์ทำสิ่งที่ร้องขอหรือไม่

ตัวอย่าง:

- Admin สามารถจัดการ Users
- User สามารถจัดการข้อมูลของตนเอง
- Guest สามารถดูข้อมูลสาธารณะได้

Authentication และ Authorization เป็นคนละเรื่องกัน

การเข้าสู่ระบบสำเร็จไม่ได้หมายความว่า User จะมีสิทธิ์เข้าถึงทุก Resource

## 9. Async Processing

Async Processing ช่วยให้ระบบจัดการงาน I/O-bound หลายงานได้โดยไม่ต้องรอแต่ละงานเสร็จทีละงานเสมอไป

ตัวอย่าง:

- Database Queries
- HTTP Requests
- File I/O
- Network Connections

เครื่องมือ:

- Tokio
- async/await
- Futures
- Tokio Tasks
- Async Channels

ควรแยกความเข้าใจระหว่าง Concurrent และ Parallel:

- Concurrent: งานหลายงานมีความคืบหน้าสลับกัน
- Parallel: งานหลายงานทำงานพร้อมกันจริงบนหลาย Execution Resources

## 10. API Pagination

Pagination ช่วยแบ่งผลลัพธ์เป็นหลายหน้า

ตัวอย่าง:

```text
GET /api/tasks?page=1&limit=20
```

ควรกำหนด:

- Default Page
- Maximum Page Size
- Sorting
- Filtering
- Total Count ตามความจำเป็น

อย่าอนุญาตให้ Client ขอข้อมูลจำนวนมหาศาลโดยไม่มีขีดจำกัด

## 11. API Error Responses

ออกแบบ Error Response ให้สม่ำเสมอ

ตัวอย่าง:

```json
{
  "error": {
    "code": "TASK_NOT_FOUND",
    "message": "Task not found"
  }
}
```

ควรหลีกเลี่ยงการส่ง:

- SQL Statements
- Database Credentials
- Stack Traces
- Internal File Paths
- Secret Values

## 12. Backend Testing

ควรทดสอบ:

- API Request Validation
- API Response
- HTTP Status Codes
- Database Operations
- Authentication
- Authorization
- Error Cases
- Concurrent Requests

ใช้ Test Database หรือ Isolated Test Environment เพื่อป้องกันการแก้ไขข้อมูลจริง

## 13. สรุป

Backend Project ที่ดีควรมี:

- API Design
- Business Logic
- Database Layer
- Error Handling
- Authentication
- Authorization
- Async Processing
- Testing
- Security

## Completion Checklist

- [ ] เข้าใจ Backend Architecture
- [ ] เข้าใจ REST API
- [ ] ใช้ Axum ได้
- [ ] ใช้ Serde ได้
- [ ] เชื่อมต่อ Database ได้
- [ ] ใช้ SQLx ได้
- [ ] เข้าใจ Repository Pattern
- [ ] เข้าใจ Authentication
- [ ] เข้าใจ Authorization
- [ ] เข้าใจ Async Processing
- [ ] ออกแบบ Pagination ได้
- [ ] ออกแบบ Error Responses ได้
- [ ] ทดสอบ Backend API ได้
