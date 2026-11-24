# Exercise 04: Web Service Challenge

## ระดับ

Challenge

## Objective

ประยุกต์ใช้ Rust, Axum, Tokio, Serde และ Middleware เพื่อสร้าง Web Service ที่มีโครงสร้างใกล้เคียงกับระบบที่ใช้งานจริง

## Problem Statement

สร้างระบบ **Task Management API** สำหรับจัดการงานของผู้ใช้งาน

ระบบต้องรองรับ:

1. User Authentication
2. Task Management
3. REST API
4. JSON Request/Response
5. Middleware
6. Input Validation
7. Error Handling
8. Logging
9. API Testing

## System Requirements

### Part 1: Project Architecture

ออกแบบโครงสร้าง Project ให้แยกความรับผิดชอบของแต่ละส่วน

ตัวอย่างแนวทาง:

```text
web_service_challenge/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── config.rs
    ├── routes/
    │   ├── mod.rs
    │   ├── auth.rs
    │   └── tasks.rs
    ├── handlers/
    │   ├── mod.rs
    │   ├── auth.rs
    │   └── tasks.rs
    ├── models/
    │   ├── mod.rs
    │   ├── user.rs
    │   └── task.rs
    ├── middleware/
    │   └── mod.rs
    ├── services/
    │   └── mod.rs
    └── errors.rs
```

สามารถปรับโครงสร้างได้ตามความเหมาะสม

### Part 2: Task Model

สร้าง Task Struct:

- ID
- Title
- Description
- Status
- Created At
- Updated At
- Owner ID

กำหนด Task Status:

```text
Pending
InProgress
Completed
```

### Part 3: Task REST API

สร้าง Endpoints:

| Method | Endpoint      | หน้าที่            |
| ------ | ------------- | ------------------ |
| GET    | `/tasks`      | อ่าน Tasks         |
| GET    | `/tasks/{id}` | อ่าน Task ตาม ID   |
| POST   | `/tasks`      | สร้าง Task         |
| PUT    | `/tasks/{id}` | แทนที่ Task        |
| PATCH  | `/tasks/{id}` | แก้ไข Task บางส่วน |
| DELETE | `/tasks/{id}` | ลบ Task            |

ข้อกำหนด:

- Task ต้องมี Title
- Title ต้องไม่เป็น String ว่าง
- Task ต้องมี Owner
- User ต้องเข้าถึงเฉพาะ Task ที่ตนมีสิทธิ์
- ต้องตรวจสอบ Task ID
- ต้องจัดการ Task Not Found

### Part 4: Authentication

สร้าง Authentication API:

```text
POST /auth/register
POST /auth/login
GET  /auth/me
```

ความสามารถ:

1. Register User
2. Login User
3. ตรวจสอบ Credentials
4. ออก Access Token
5. ตรวจสอบ Token
6. ดึงข้อมูล User ปัจจุบัน

ข้อกำหนดด้านความปลอดภัย:

- ห้ามเก็บ Password เป็น Plain Text
- ใช้ Password Hashing Algorithm ที่เหมาะสม เช่น Argon2id
- ห้ามส่ง Password Hash กลับไปใน Response
- ห้ามบันทึก Password หรือ Token ลง Log
- กำหนดอายุและเงื่อนไขการใช้งาน Token
- ตรวจสอบ Token ทุกครั้งที่เรียก Protected Endpoint
- แยก Authentication ออกจาก Authorization

สามารถเลือกใช้ Token แบบ Bearer Token หรือ JWT โดยต้องศึกษาและกำหนดแนวทางตรวจสอบ Token ให้ถูกต้อง

### Part 5: Middleware

เพิ่ม Middleware อย่างน้อย 3 รายการ:

1. Request Logging
2. Request ID
3. Authentication

สามารถเพิ่ม:

- CORS
- Timeout
- Rate Limiting

ข้อกำหนด:

- Middleware ต้องทำงานตามลำดับที่เหมาะสม
- ต้องไม่บันทึก Secret ลง Log
- ต้องตอบกลับเมื่อ Authentication ไม่ผ่าน
- ต้องไม่เปิด Protected Endpoint ให้ผู้ใช้ที่ไม่มีสิทธิ์

### Part 6: Error Handling

ออกแบบ Error Response ให้เป็นมาตรฐานเดียวกัน

ตัวอย่าง:

```json
{
  "error": {
    "code": "INVALID_TASK_TITLE",
    "message": "Task title is required"
  }
}
```

รองรับ Error:

- Invalid Input
- Authentication Failed
- Authorization Failed
- User Not Found
- Task Not Found
- Duplicate Resource
- Internal Server Error

ต้องกำหนด HTTP Status Code ให้เหมาะสม

### Part 7: Application State

ออกแบบ State สำหรับ:

- Users
- Tasks
- Authentication Data
- Configuration

เริ่มต้นด้วย In-memory Storage ได้ แต่ต้องแยกส่วน Storage ออกจาก Business Logic เพื่อให้สามารถเปลี่ยนไปใช้ Database ในบทถัดไปได้

### Part 8: Logging and Configuration

กำหนด Configuration ผ่าน Environment Variables เช่น:

```text
APP_HOST
APP_PORT
TOKEN_SECRET
TOKEN_EXPIRATION
```

ข้อกำหนด:

- ห้าม Hardcode Secret ลง Source Code
- ต้องตรวจสอบว่า Configuration ที่จำเป็นมีอยู่
- ต้องไม่แสดง Secret ใน Log
- ต้องมี Logging สำหรับ Request และ Error
- ต้องแยก Development Configuration จาก Production Configuration

### Part 9: Testing

เขียน Tests อย่างน้อย:

- Register สำเร็จ
- Register ด้วยข้อมูลผิดพลาด
- Login สำเร็จ
- Login ด้วย Password ผิด
- Request ที่ไม่มี Token
- Request ที่มี Token ไม่ถูกต้อง
- Create Task สำเร็จ
- Get Task สำเร็จ
- Update Task สำเร็จ
- Delete Task สำเร็จ
- อ่าน Task ที่ไม่มีอยู่
- User พยายามเข้าถึง Task ของ User อื่น

## Constraints

- ต้องใช้ Rust
- ต้องใช้ Axum
- ต้องใช้ Tokio
- ต้องใช้ Serde
- ต้องใช้ JSON
- ต้องมี REST API
- ต้องมี Authentication
- ต้องมี Authorization
- ต้องมี Middleware อย่างน้อย 3 รายการ
- ต้องมี Input Validation
- ต้องมี Error Handling
- ต้องมี Logging
- ต้องใช้ Environment Variables สำหรับ Secrets
- ต้องมี Unit Tests
- ต้องมี Integration Tests
- ห้ามเก็บ Password เป็น Plain Text
- ห้ามส่ง Secret กลับไปยัง Client
- ห้ามบันทึก Token ลง Log
- ห้ามใช้ `unwrap()` กับ Request Handling โดยไม่มีเหตุผล
- ต้องจัดการ Concurrent Access อย่างปลอดภัย
- ต้องแยก Business Logic ออกจาก HTTP Handler

## Expected Output

### Server Started

```text
Task Management API

Server: http://127.0.0.1:3000

Routes:
POST   /auth/register
POST   /auth/login
GET    /auth/me

GET    /tasks
GET    /tasks/{id}
POST   /tasks
PUT    /tasks/{id}
PATCH  /tasks/{id}
DELETE /tasks/{id}

Server is running...
```

### Register

Request:

```http
POST /auth/register
Content-Type: application/json
```

```json
{
  "name": "Alice",
  "email": "alice@example.com",
  "password": "example-password"
}
```

Expected:

```http
HTTP/1.1 201 Created
```

Response ต้องไม่ส่ง Password หรือ Password Hash กลับไป

### Login

Request:

```http
POST /auth/login
Content-Type: application/json
```

Expected:

- ตรวจสอบ Credentials
- ออก Access Token
- ส่ง Token กลับตามรูปแบบที่กำหนด

### Create Task

Request:

```http
POST /tasks
Authorization: Bearer YOUR_TOKEN
Content-Type: application/json
```

```json
{
  "title": "Learn Rust",
  "description": "Practice Rust Web Development",
  "status": "Pending"
}
```

Expected:

```http
HTTP/1.1 201 Created
```

### Get Tasks

Request:

```http
GET /tasks
Authorization: Bearer YOUR_TOKEN
```

Expected:

- แสดง Tasks ของ User ที่ผ่าน Authentication
- ไม่เปิดเผย Tasks ของ User อื่น

## Hints

Dependencies ที่อาจต้องใช้:

```powershell
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
cargo add tower-http --features trace,cors
cargo add tracing
cargo add tracing-subscriber
```

สำหรับ Authentication และ Password Hashing ให้เลือก Crate ที่เหมาะสมและตรวจสอบเอกสารการใช้งานก่อนเพิ่ม Dependency

แนวทางการออกแบบ:

1. แยก Router ออกจาก Handler
2. แยก Model ออกจาก Business Logic
3. สร้าง Application State
4. ใช้ Middleware ตรวจสอบ Request
5. ใช้ Extractor อ่านข้อมูล
6. ตรวจสอบ Input ก่อนทำงาน
7. สร้าง Error Type กลาง
8. เขียน Unit Tests สำหรับ Business Logic
9. เขียน Integration Tests สำหรับ API
10. ทบทวน Security ก่อนจบโครงการ

## Completion Checklist

### Architecture

- [ ] ออกแบบ Project Structure
- [ ] แยก Routes
- [ ] แยก Handlers
- [ ] แยก Models
- [ ] แยก Services
- [ ] แยก Error Handling

### REST API

- [ ] สร้าง Task
- [ ] อ่าน Tasks
- [ ] อ่าน Task ตาม ID
- [ ] แก้ไข Task
- [ ] ลบ Task
- [ ] ตรวจสอบ Input

### Authentication

- [ ] Register User
- [ ] Login User
- [ ] Hash Password
- [ ] ออก Access Token
- [ ] ตรวจสอบ Token
- [ ] ดึง Current User

### Security

- [ ] แยก Authentication และ Authorization
- [ ] ป้องกันการเข้าถึงข้อมูลของ User อื่น
- [ ] ไม่เก็บ Password เป็น Plain Text
- [ ] ไม่เปิดเผย Secret
- [ ] ไม่บันทึก Token ลง Log
- [ ] ใช้ Environment Variables

### Middleware

- [ ] Request Logging
- [ ] Request ID
- [ ] Authentication
- [ ] CORS หรือ Timeout
- [ ] Rate Limiting ตามความเหมาะสม

### Testing

- [ ] Unit Tests
- [ ] Integration Tests
- [ ] ทดสอบ Authentication
- [ ] ทดสอบ Authorization
- [ ] ทดสอบ Invalid Input
- [ ] ทดสอบ Error Responses

## Run

```powershell
cd 28_networking_web_development/exercises/exercise_04/web_service_challenge

cargo check
cargo test
cargo run
```

## Final Challenge

เมื่อทำระบบพื้นฐานสำเร็จแล้ว ให้เพิ่มความสามารถ:

1. เพิ่ม Database แทน In-memory Storage
2. เพิ่ม Pagination
3. เพิ่ม Search และ Filtering
4. เพิ่ม API Versioning เช่น `/api/v1`
5. เพิ่ม Docker Support
6. เพิ่ม Health Check Endpoint
7. เพิ่ม API Documentation
8. เพิ่ม Deployment Configuration

**หลักสำคัญ:** Web Service ที่ดีต้องคำนึงถึงความปลอดภัย การจัดการข้อผิดพลาด และการดูแลระบบ ไม่ใช่แค่ทำให้ Endpoint ตอบกลับได้
