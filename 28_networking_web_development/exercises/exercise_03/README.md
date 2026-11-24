# Exercise 03: REST API

## ระดับ

Medium

## Objective

ฝึกสร้าง REST API ด้วย Axum และ Rust โดยจัดการข้อมูลผ่าน HTTP Methods และ JSON

## Problem Statement

สร้างระบบ User Management API สำหรับจัดการข้อมูลผู้ใช้งาน

กำหนด User:

- ID
- Name
- Email
- Age

ตัวอย่างข้อมูล:

```json
{
  "id": 1,
  "name": "Alice",
  "email": "alice@example.com",
  "age": 25
}
```

ระบบต้องสามารถ:

1. แสดง Users ทั้งหมด
2. ค้นหา User ตาม ID
3. สร้าง User ใหม่
4. แก้ไข User
5. ลบ User
6. ตรวจสอบ Input
7. จัดการ Error Response

## Requirements

### Part 1: Project Setup

สร้าง Axum Application

ติดตั้ง Dependencies:

```powershell
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
```

### Part 2: Data Model

สร้าง Struct:

```text
User
```

กำหนด Fields:

- `id`
- `name`
- `email`
- `age`

ใช้ Serde สำหรับ Serialize และ Deserialize

### Part 3: GET Endpoints

สร้าง Endpoint:

| Method | Endpoint      | หน้าที่            |
| ------ | ------------- | ------------------ |
| GET    | `/users`      | อ่าน Users ทั้งหมด |
| GET    | `/users/{id}` | อ่าน User ตาม ID   |

### Part 4: POST Endpoint

สร้าง Endpoint:

```text
POST /users
```

รับ JSON จาก Client และสร้าง User ใหม่

ตรวจสอบ:

- Name ต้องไม่ว่าง
- Email ต้องไม่ว่าง
- Age ต้องอยู่ในช่วงที่ระบบกำหนด
- ID ต้องไม่ซ้ำกัน

### Part 5: PUT Endpoint

สร้าง Endpoint:

```text
PUT /users/{id}
```

ใช้แก้ไขข้อมูล User ตาม ID

หากไม่พบ User ให้ส่ง Error Response ที่เหมาะสม

### Part 6: DELETE Endpoint

สร้าง Endpoint:

```text
DELETE /users/{id}
```

ลบ User ตาม ID

หากไม่มี User ให้ส่ง Status Code ที่เหมาะสม

### Part 7: Application State

ใช้ State สำหรับจัดเก็บข้อมูล Users

ข้อกำหนด:

- แชร์ข้อมูลระหว่าง Handler
- จัดการการเข้าถึงข้อมูลร่วมกันอย่างปลอดภัย
- ป้องกันการแก้ไขข้อมูลพร้อมกันอย่างไม่ถูกต้อง
- แยก Data Model ออกจาก Handler ตามความเหมาะสม

สำหรับแบบฝึกหัดนี้สามารถใช้ In-memory Storage ได้ โดยยังไม่ต้องเชื่อมต่อ Database

### Part 8: Error Handling

กำหนด Error Response ให้มีรูปแบบสม่ำเสมอ

ตัวอย่าง:

```json
{
  "error": {
    "code": "USER_NOT_FOUND",
    "message": "User not found"
  }
}
```

กำหนด Status Code:

| สถานการณ์         | Status       |
| ----------------- | ------------ |
| อ่านข้อมูลสำเร็จ  | 200          |
| สร้างข้อมูลสำเร็จ | 201          |
| ลบข้อมูลสำเร็จ    | 204          |
| Input ไม่ถูกต้อง  | 400 หรือ 422 |
| ไม่พบ User        | 404          |
| ID ซ้ำ            | 409          |
| Server Error      | 500          |

## Constraints

- ต้องใช้ Axum
- ต้องใช้ Tokio
- ต้องใช้ Serde
- ต้องใช้ JSON
- ต้องมี GET, POST, PUT และ DELETE
- ต้องใช้ Path Parameters
- ต้องมี Application State
- ต้องตรวจสอบ Input
- ต้องจัดการ Error Response
- ต้องไม่ใช้ `unwrap()` ในเส้นทางจัดการ Request โดยไม่มีเหตุผล
- ต้องไม่เปิดเผย Internal Error ให้ Client
- ต้องไม่มีข้อมูล User ซ้ำตาม ID
- ต้องแยก Handler และ Data Model ให้เหมาะสม

## Expected Output

### 1. Get All Users

Request:

```http
GET /users
```

Response:

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "id": 1,
    "name": "Alice",
    "email": "alice@example.com",
    "age": 25
  }
]
```

### 2. Get User

Request:

```http
GET /users/1
```

Response:

```json
{
  "id": 1,
  "name": "Alice",
  "email": "alice@example.com",
  "age": 25
}
```

### 3. Create User

Request:

```http
POST /users
Content-Type: application/json
```

```json
{
  "name": "Bob",
  "email": "bob@example.com",
  "age": 30
}
```

Expected:

```http
HTTP/1.1 201 Created
```

### 4. Update User

Request:

```http
PUT /users/1
Content-Type: application/json
```

```json
{
  "name": "Alice Smith",
  "email": "alice.smith@example.com",
  "age": 26
}
```

Expected: User ข้อมูลใหม่ถูกบันทึก

### 5. Delete User

Request:

```http
DELETE /users/1
```

Expected:

```http
HTTP/1.1 204 No Content
```

## Hints

- ใช้ `Router::new()` กำหนด Routes
- ใช้ `State<T>` แชร์ Application State
- ใช้ `Path<T>` อ่าน ID
- ใช้ `Json<T>` รับและส่ง JSON
- ใช้ `Arc<Mutex<T>>` หรือ `Arc<RwLock<T>>` สำหรับ In-memory State ที่ต้องแชร์และแก้ไข
- ใช้ `StatusCode` กำหนด HTTP Status
- ใช้ `IntoResponse` สำหรับสร้าง Error Response
- ใช้ `Result` จัดการข้อผิดพลาด

## Completion Checklist

- [ ] สร้าง Axum Application ได้
- [ ] สร้าง User Struct ได้
- [ ] สร้าง GET `/users` ได้
- [ ] สร้าง GET `/users/{id}` ได้
- [ ] สร้าง POST `/users` ได้
- [ ] สร้าง PUT `/users/{id}` ได้
- [ ] สร้าง DELETE `/users/{id}` ได้
- [ ] รับ JSON ได้
- [ ] ส่ง JSON ได้
- [ ] ใช้ Application State ได้
- [ ] ตรวจสอบ Input ได้
- [ ] จัดการ Duplicate ID ได้
- [ ] จัดการ User Not Found ได้
- [ ] จัดการ Error Response ได้
- [ ] ทดสอบทุก Endpoint ได้

## Run

```powershell
cd 28_networking_web_development/exercises/exercise_03/rest_api

cargo check
cargo run
```

ทดสอบ Endpoint ด้วย curl, Postman หรือ HTTP Client อื่น ๆ
