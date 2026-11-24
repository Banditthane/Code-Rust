# Lesson 01: Project Architecture

## 1. Project Architecture คืออะไร?

Project Architecture คือการออกแบบโครงสร้างของ Software ก่อนหรือระหว่างการพัฒนา

Architecture ช่วยกำหนด:

- ระบบต้องทำอะไร
- ระบบประกอบด้วยส่วนใดบ้าง
- แต่ละส่วนรับผิดชอบอะไร
- ข้อมูลไหลผ่านระบบอย่างไร
- แต่ละ Module ติดต่อกันอย่างไร
- ระบบจัดการ Error อย่างไร
- ระบบจะขยายเพิ่มเติมได้อย่างไร

## 2. Requirements Analysis

Requirements คือข้อกำหนดของระบบ

แบ่งได้เป็น 2 ประเภทหลัก:

### Functional Requirements

กำหนดว่าระบบต้องทำอะไร

ตัวอย่างระบบ Task Management:

- เพิ่ม Task
- แสดง Task
- แก้ไข Task
- ลบ Task
- เปลี่ยนสถานะ Task
- ค้นหา Task

### Non-functional Requirements

กำหนดคุณภาพหรือข้อจำกัดของระบบ

ตัวอย่าง:

- ความปลอดภัย
- Performance
- Reliability
- Maintainability
- Scalability
- Usability
- Availability

## 3. Define Project Scope

Project Scope คือขอบเขตของงานที่จะพัฒนา

ควรกำหนด:

- Features ที่ต้องมี
- Features ที่ยังไม่ทำ
- ผู้ใช้งานเป้าหมาย
- Platform ที่รองรับ
- ข้อจำกัดด้านเวลา
- ข้อจำกัดด้านทรัพยากร

ตัวอย่าง:

Project: Task Management API

MVP Features:

1. Create Task
2. List Tasks
3. Update Task
4. Delete Task
5. Update Task Status

Future Features:

- User Authentication
- Team Collaboration
- Notifications
- File Attachments

ไม่ควรเพิ่มทุก Feature ตั้งแต่เริ่มต้น

## 4. User Stories

User Story ช่วยอธิบายความต้องการจากมุมมองผู้ใช้งาน

รูปแบบ:

```text
As a [user]
I want [feature]
So that [benefit]
```

ตัวอย่าง:

```text
As a user
I want to create a task
So that I can manage my work.
```

ควรกำหนด Acceptance Criteria เพื่อระบุเงื่อนไขที่ใช้ตัดสินว่า Feature ทำงานถูกต้อง

ตัวอย่าง:

- ผู้ใช้สามารถส่งชื่อ Task ได้
- ชื่อ Task ต้องไม่เป็นค่าว่าง
- เมื่อสร้างสำเร็จ ระบบต้องคืน Task ID
- หากข้อมูลไม่ถูกต้อง ระบบต้องแจ้ง Error

## 5. System Design

System Design คือการกำหนดองค์ประกอบและการทำงานร่วมกันของระบบ

ตัวอย่าง:

```text
Client
   |
   v
HTTP API
   |
   v
Application Service
   |
   v
Repository Layer
   |
   v
Database
```

แต่ละส่วนมีหน้าที่ต่างกัน:

| Layer      | หน้าที่                 |
| ---------- | ----------------------- |
| Client     | ส่ง Request             |
| API        | รับและตอบ HTTP Requests |
| Service    | จัดการ Business Logic   |
| Repository | จัดการ Data Access      |
| Database   | จัดเก็บข้อมูล           |

## 6. Module Architecture

Rust ใช้ Modules เพื่อแบ่ง Code ตามหน้าที่

ตัวอย่าง:

```text
src/
├── main.rs
├── lib.rs
├── models/
│   ├── mod.rs
│   └── task.rs
├── services/
│   ├── mod.rs
│   └── task_service.rs
├── repositories/
│   ├── mod.rs
│   └── task_repository.rs
├── routes/
│   ├── mod.rs
│   └── task_routes.rs
└── errors.rs
```

แนวทาง:

- Models เก็บ Data Structures
- Services เก็บ Business Logic
- Repositories จัดการ Data Access
- Routes จัดการ API Endpoints
- Errors รวม Error Types ที่เกี่ยวข้อง

ไม่จำเป็นต้องใช้โครงสร้างนี้กับทุก Project สามารถปรับตามขนาดและความซับซ้อนของระบบ

## 7. Separation of Concerns

Separation of Concerns คือการแยกความรับผิดชอบของแต่ละส่วนให้ชัดเจน

ตัวอย่าง:

ไม่ควรให้ HTTP Handler ทำทุกอย่าง เช่น:

- Validate Input
- คำนวณ Business Logic
- เขียน SQL
- จัดรูปแบบ Error
- จัดการ Response

ควรแยกหน้าที่ให้เหมาะสม เพื่อให้ Test และ Maintain ได้ง่ายขึ้น

## 8. Data Modeling

Data Model ใช้อธิบายโครงสร้างข้อมูลของระบบ

ตัวอย่าง:

```rust
struct Task {
    id: u64,
    title: String,
    completed: bool,
}
```

ก่อนออกแบบ Data Model ควรพิจารณา:

- Fields
- Data Types
- Relationships
- Validation Rules
- Optional Values
- Data Ownership
- Serialization Format

## 9. Database Design

ก่อนสร้าง Database ควรกำหนด:

- Tables
- Columns
- Primary Keys
- Foreign Keys
- Relationships
- Constraints
- Indexes

ตัวอย่าง:

```text
users
-----
id
name
email

tasks
-----
id
user_id
title
status
created_at
```

`user_id` ในตาราง tasks สามารถใช้เป็น Foreign Key ที่อ้างอิง `users.id`

ควรกำหนด Constraints ให้สอดคล้องกับ Business Rules

## 10. API Design

ควรออกแบบ API ให้สม่ำเสมอ

ตัวอย่าง REST API:

| Method | Endpoint    | ความหมาย              |
| ------ | ----------- | --------------------- |
| GET    | /tasks      | แสดง Tasks            |
| GET    | /tasks/{id} | แสดง Task รายการเดียว |
| POST   | /tasks      | สร้าง Task            |
| PUT    | /tasks/{id} | แทนที่ข้อมูล Task     |
| PATCH  | /tasks/{id} | แก้ไขข้อมูลบางส่วน    |
| DELETE | /tasks/{id} | ลบ Task               |

ควรเลือก HTTP Status Codes ให้เหมาะสม เช่น:

- 200 OK
- 201 Created
- 204 No Content
- 400 Bad Request
- 401 Unauthorized
- 403 Forbidden
- 404 Not Found
- 500 Internal Server Error

## 11. Error Design

ควรออกแบบ Error Types ตั้งแต่ต้น

ตัวอย่าง:

```text
ValidationError
NotFoundError
ConflictError
DatabaseError
InternalError
```

แนวทาง:

- แยก Error ตามสาเหตุ
- ไม่ส่ง Internal Details ให้ Client โดยไม่จำเป็น
- บันทึก Error ที่จำเป็นใน Logs
- แปลง Error ภายในเป็น HTTP Response อย่างเหมาะสม

## 12. Architecture Documentation

ควรบันทึก:

- Project Overview
- System Components
- Module Responsibilities
- Data Flow
- API Structure
- Database Schema
- Error Handling Strategy
- Security Considerations

สามารถใช้ Diagram เช่น Mermaid เพื่ออธิบาย Architecture

## 13. สรุป

การออกแบบ Project Architecture ช่วยให้:

- เข้าใจปัญหาก่อนเริ่มพัฒนา
- ควบคุม Scope
- แบ่งหน้าที่ของ Modules
- ลดการผูกติดกันระหว่าง Components
- ทดสอบระบบได้ง่ายขึ้น
- รองรับการเปลี่ยนแปลงในอนาคต

## Completion Checklist

- [ ] เข้าใจ Functional Requirements
- [ ] เข้าใจ Non-functional Requirements
- [ ] เขียน User Stories ได้
- [ ] กำหนด Acceptance Criteria ได้
- [ ] กำหนด Project Scope ได้
- [ ] ออกแบบ System Flow ได้
- [ ] แบ่ง Module Responsibilities ได้
- [ ] เข้าใจ Separation of Concerns
- [ ] ออกแบบ Data Models ได้
- [ ] ออกแบบ Database Schema ได้
- [ ] ออกแบบ REST API ได้
- [ ] ออกแบบ Error Types ได้
- [ ] เขียน Architecture Documentation ได้
