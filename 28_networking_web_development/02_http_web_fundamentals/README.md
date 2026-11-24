# Lesson 02: HTTP and Web Fundamentals

## 1. HTTP คืออะไร?

HTTP (Hypertext Transfer Protocol) เป็น Application Layer Protocol สำหรับแลกเปลี่ยนข้อมูลระหว่าง HTTP Client และ HTTP Server

ตัวอย่าง:

- Browser เรียกหน้าเว็บไซต์
- Mobile App เรียก Backend API
- โปรแกรมส่งข้อมูล JSON ไปยัง Server

HTTPS คือ HTTP ที่ทำงานผ่าน TLS เพื่อช่วยปกป้องข้อมูลระหว่างการสื่อสาร

## 2. HTTP Request

HTTP Request คือข้อความที่ Client ส่งไปยัง Server

องค์ประกอบหลัก:

1. Request Method
2. Request Target
3. HTTP Version
4. Headers
5. Body (ถ้ามี)

ตัวอย่าง:

```http
POST /users HTTP/1.1
Host: localhost:3000
Content-Type: application/json

{
  "name": "Alice",
  "age": 25
}
```

ความหมาย:

- `POST` คือ Method
- `/users` คือ Request Target
- `Content-Type` ระบุรูปแบบข้อมูลใน Body
- JSON เป็นข้อมูลที่ส่งไปยัง Server

## 3. HTTP Methods

HTTP Methods ระบุเจตนาของ Request

| Method  | การใช้งานทั่วไป                           |
| ------- | ----------------------------------------- |
| GET     | อ่านข้อมูล                                |
| POST    | สร้าง Resource หรือส่งข้อมูลเพื่อประมวลผล |
| PUT     | แทนที่ Resource                           |
| PATCH   | แก้ไข Resource บางส่วน                    |
| DELETE  | ลบ Resource                               |
| HEAD    | ขอ Headers โดยไม่มี Response Body         |
| OPTIONS | ตรวจสอบ Communication Options             |

ตัวอย่าง:

```http
GET /users
POST /users
GET /users/1
PUT /users/1
PATCH /users/1
DELETE /users/1
```

## 4. HTTP Response

HTTP Response คือข้อมูลที่ Server ส่งกลับไปยัง Client

องค์ประกอบหลัก:

1. HTTP Version
2. Status Code
3. Headers
4. Body (ถ้ามี)

ตัวอย่าง:

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "id": 1,
  "name": "Alice",
  "age": 25
}
```

## 5. HTTP Status Codes

แบ่งออกเป็น 5 กลุ่ม:

| กลุ่ม | ความหมาย      |
| ----- | ------------- |
| 1xx   | Informational |
| 2xx   | Success       |
| 3xx   | Redirection   |
| 4xx   | Client Error  |
| 5xx   | Server Error  |

Status Codes ที่พบบ่อย:

| Code | ความหมาย              |
| ---- | --------------------- |
| 200  | OK                    |
| 201  | Created               |
| 204  | No Content            |
| 301  | Moved Permanently     |
| 302  | Found                 |
| 400  | Bad Request           |
| 401  | Unauthorized          |
| 403  | Forbidden             |
| 404  | Not Found             |
| 409  | Conflict              |
| 422  | Unprocessable Content |
| 429  | Too Many Requests     |
| 500  | Internal Server Error |
| 503  | Service Unavailable   |

## 6. HTTP Headers

Headers ใช้ส่ง Metadata เกี่ยวกับ Request หรือ Response

ตัวอย่าง Request Headers:

```http
Content-Type: application/json
Accept: application/json
Authorization: Bearer TOKEN
```

ตัวอย่าง Response Headers:

```http
Content-Type: application/json
Cache-Control: no-store
```

Headers ที่ควรรู้:

| Header        | หน้าที่                              |
| ------------- | ------------------------------------ |
| Content-Type  | ระบุชนิดข้อมูลใน Body                |
| Accept        | ระบุชนิดข้อมูลที่ Client ต้องการ     |
| Authorization | ส่งข้อมูลสำหรับยืนยันตัวตน           |
| Cache-Control | กำหนดพฤติกรรม Cache                  |
| User-Agent    | ระบุ Client                          |
| Location      | ระบุปลายทางที่เกี่ยวข้องกับ Redirect |

## 7. JSON

JSON (JavaScript Object Notation) เป็นรูปแบบข้อมูลที่นิยมใช้กับ Web API

ตัวอย่าง:

```json
{
  "id": 1,
  "name": "Alice",
  "active": true,
  "scores": [80, 90, 95]
}
```

Rust นิยมใช้ Serde และ Serde JSON สำหรับ Serialize และ Deserialize

แนวคิด:

- Serialize: แปลง Rust Data เป็น JSON
- Deserialize: แปลง JSON เป็น Rust Data

## 8. HTTP Client ใน Rust

สามารถใช้ Crate เช่น `reqwest` เพื่อส่ง HTTP Request

ติดตั้ง:

```powershell
cargo add reqwest --features json
cargo add tokio --features full
```

ตัวอย่างแนวคิด:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://example.com").await?;

    println!("Status: {}", response.status());

    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
```

หมายเหตุ: ตัวอย่างนี้เป็น Async HTTP Client ต้องมี Tokio Runtime และต้องมีการเชื่อมต่อเครือข่าย

## 9. HTTP กับ TCP

HTTP ทำงานบน Application Layer ส่วน TCP เป็น Transport Layer ที่ HTTP/1.1 และ HTTP/2 มักใช้ผ่าน TCP

HTTP/3 ใช้ QUIC ซึ่งทำงานบน UDP

ดังนั้น HTTP ไม่ได้เท่ากับ TCP และไม่ควรเข้าใจว่า HTTP ทุกเวอร์ชันใช้ TCP

## 10. สรุป

| คำศัพท์     | ความหมาย                      |
| ----------- | ----------------------------- |
| HTTP        | Application Protocol          |
| HTTPS       | HTTP ผ่าน TLS                 |
| Request     | ข้อมูลที่ Client ส่ง          |
| Response    | ข้อมูลที่ Server ตอบกลับ      |
| Method      | ระบุเจตนาของ Request          |
| Status Code | ระบุผลลัพธ์ของ Request        |
| Header      | Metadata ของ Request/Response |
| Body        | เนื้อหาข้อมูล                 |
| JSON        | รูปแบบข้อมูลสำหรับแลกเปลี่ยน  |

## Completion Checklist

- [ ] เข้าใจ HTTP
- [ ] เข้าใจ HTTPS
- [ ] เข้าใจ Request
- [ ] เข้าใจ Response
- [ ] เข้าใจ HTTP Methods
- [ ] เข้าใจ Status Codes
- [ ] เข้าใจ HTTP Headers
- [ ] เข้าใจ JSON
- [ ] เข้าใจ Serialize และ Deserialize
- [ ] ส่ง HTTP Request จาก Rust ได้
- [ ] เข้าใจความสัมพันธ์ระหว่าง HTTP และ TCP
