# Exercise 02: HTTP Practice

## ระดับ

Easy-Medium

## Objective

ฝึกส่ง HTTP Request จาก Rust ด้วย HTTP Client และเรียนรู้การอ่าน HTTP Response, Status Code, Headers และ JSON

## Problem Statement

สร้างโปรแกรม HTTP Client สำหรับเรียกใช้งาน Public API หรือ API ทดสอบ

เลือก API ที่สามารถเรียกใช้งานได้ เช่น:

```text
https://jsonplaceholder.typicode.com/users/1
```

โปรแกรมต้องสามารถ:

1. ส่ง GET Request
2. อ่าน HTTP Status Code
3. อ่าน Response Headers
4. อ่าน Response Body
5. Deserialize JSON เป็น Rust Struct
6. จัดการ HTTP Error และ Network Error

## Requirements

### Part 1: HTTP GET

1. ใช้ `reqwest`
2. ส่ง GET Request
3. ระบุ URL
4. แสดง HTTP Status
5. แสดงผลลัพธ์จาก Server

### Part 2: Response Headers

1. อ่าน `Content-Type`
2. อ่าน `Content-Length` หากมี
3. แสดง Response Headers ที่สนใจ
4. ตรวจสอบชนิดข้อมูลที่ Server ส่งกลับ

### Part 3: JSON Deserialization

1. สร้าง Struct สำหรับข้อมูล User
2. ใช้ `#[derive(Deserialize)]`
3. Deserialize JSON เป็น Struct
4. แสดงข้อมูล User

ตัวอย่างโครงสร้างข้อมูล:

```json
{
  "id": 1,
  "name": "Leanne Graham",
  "username": "Bret",
  "email": "example@example.com"
}
```

### Part 4: HTTP Methods

ทดลองเรียกใช้งาน:

| Method | Endpoint   |
| ------ | ---------- |
| GET    | `/users/1` |
| POST   | `/posts`   |
| PUT    | `/posts/1` |
| DELETE | `/posts/1` |

บันทึก Status Code ที่ได้รับจากแต่ละ Request

### Part 5: Error Handling

จัดการกรณี:

- URL ไม่ถูกต้อง
- Network ไม่พร้อมใช้งาน
- Request Timeout
- HTTP 4xx
- HTTP 5xx
- JSON ไม่ตรงกับ Struct
- Response Body ไม่สามารถอ่านได้

## Constraints

- ต้องใช้ `reqwest`
- ต้องใช้ Tokio Runtime
- ต้องใช้ Serde
- ต้อง Deserialize JSON เป็น Struct
- ต้องแสดง Status Code
- ต้องจัดการ Network Error
- ต้องแยก HTTP Error ออกจาก JSON Parsing Error
- ห้ามสมมติว่า HTTP 200 เป็นคำตอบของทุก Request
- ห้ามใส่ API Key หรือ Secret ลงใน Source Code
- ต้องกำหนด Timeout อย่างเหมาะสม

## Expected Output

```text
HTTP Client Practice

Request:
GET https://jsonplaceholder.typicode.com/users/1

Response:
Status: 200 OK
Content-Type: application/json

User Information
ID: 1
Name: Leanne Graham
Username: Bret
Email: example@example.com

Request completed.
```

ผลลัพธ์จริงอาจเปลี่ยนแปลงตาม API ที่เลือก

## Hints

ติดตั้ง Dependencies:

```powershell
cargo add reqwest --features json
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
```

แนวทาง:

- ใช้ `reqwest::Client` สำหรับสร้าง Client
- ใช้ `.get()` หรือ `.post()` สร้าง Request
- ใช้ `.send().await` ส่ง Request
- ใช้ `.status()` อ่าน Status Code
- ใช้ `.headers()` อ่าน Headers
- ใช้ `.json::<YourStruct>().await` Deserialize JSON
- พิจารณา `.error_for_status()` สำหรับเปลี่ยน HTTP Error Status ให้เป็น Error

ข้อควรจำ: `send()` ที่สำเร็จไม่ได้แปลว่า HTTP Status เป็น 2xx เสมอไป

## Completion Checklist

- [ ] ติดตั้ง reqwest ได้
- [ ] ใช้ Tokio Runtime ได้
- [ ] ส่ง GET Request ได้
- [ ] อ่าน Status Code ได้
- [ ] อ่าน Headers ได้
- [ ] อ่าน Response Body ได้
- [ ] Deserialize JSON ได้
- [ ] ทดลอง POST ได้
- [ ] ทดลอง PUT ได้
- [ ] ทดลอง DELETE ได้
- [ ] จัดการ Network Error ได้
- [ ] จัดการ HTTP Error ได้
- [ ] จัดการ JSON Error ได้
- [ ] กำหนด Timeout ได้

## Run

```powershell
cd 28_networking_web_development/exercises/exercise_02/http_practice

cargo check
cargo run
```
