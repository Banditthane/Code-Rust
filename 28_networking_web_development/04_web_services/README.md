# Lesson 04: Web Services

## 1. Web Service คืออะไร?

Web Service คือ Application ที่เปิดความสามารถให้ Client หรือระบบอื่นเรียกใช้งานผ่าน Network

ตัวอย่าง:

- User Management Service
- Payment Service
- Inventory Service
- Authentication Service
- Notification Service

Web Service ที่ดีควรมี:

- Routing
- Input Validation
- Error Handling
- Authentication
- Authorization
- Logging
- Testing
- Security Controls

## 2. Middleware คืออะไร?

Middleware คือชั้นการประมวลผลที่ทำงานร่วมกับ Request และ Response ก่อนหรือหลัง Handler ตามรูปแบบของ Framework

ตัวอย่างงานที่ Middleware ใช้ทำ:

- Logging
- Request ID
- Authentication
- Timeout
- CORS
- Compression
- Rate Limiting

แนวคิด:

```text
Client
  |
  v
Middleware
  |
  v
Router
  |
  v
Handler
  |
  v
Response
  |
  v
Client
```

ใน Axum สามารถใช้ Middleware จาก Tower และ Tower HTTP ได้

## 3. เพิ่ม Tower HTTP

ติดตั้ง:

```powershell
cargo add tower-http --features trace,cors
cargo add tracing
cargo add tracing-subscriber
```

ตัวอย่าง Middleware สำหรับ Trace:

```rust
use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

async fn home() -> &'static str {
    "Hello"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(home))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
```

## 4. CORS

CORS (Cross-Origin Resource Sharing) เป็นกลไกที่ Browser ใช้ควบคุมการเข้าถึง Resource จาก Origin ที่แตกต่างกัน

ตัวอย่าง Origin:

```text
http://localhost:3000
http://localhost:5173
```

แนวทาง:

- กำหนด Allowed Origins ให้เหมาะสม
- กำหนด Allowed Methods
- กำหนด Allowed Headers
- ไม่เปิดกว้างเกินความจำเป็น
- เข้าใจ Preflight Request

CORS ไม่ใช่ระบบ Authentication และไม่สามารถทดแทน Authorization ได้

## 5. Authentication คืออะไร?

Authentication คือกระบวนการตรวจสอบว่า User เป็นใคร

ตัวอย่าง:

- Username และ Password
- Session
- API Key
- Access Token
- JSON Web Token (JWT)

## 6. Authorization คืออะไร?

Authorization คือการตรวจสอบว่า User มีสิทธิ์ทำอะไรได้บ้าง

ตัวอย่าง:

- Admin สามารถลบ User ได้
- User สามารถดูข้อมูลของตัวเองได้
- Guest สามารถอ่านข้อมูลสาธารณะได้

เปรียบเทียบ:

| รายการ   | Authentication | Authorization      |
| -------- | -------------- | ------------------ |
| คำถาม    | คุณคือใคร?     | คุณมีสิทธิ์ทำอะไร? |
| หน้าที่  | ตรวจสอบตัวตน   | ตรวจสอบสิทธิ์      |
| ตัวอย่าง | Login          | Role / Permission  |

## 7. Password Security

ข้อควรปฏิบัติ:

- ห้ามเก็บ Password เป็น Plain Text
- ใช้ Password Hashing Algorithm ที่ออกแบบมาสำหรับ Password เช่น Argon2id
- ใช้ Salt ตามข้อกำหนดของ Algorithm
- หลีกเลี่ยงการสร้าง Hash Algorithm เอง
- ใช้ HTTPS ในการส่งข้อมูลสำคัญ
- จำกัดจำนวน Login Attempts ตามความเหมาะสม
- ไม่บันทึก Password หรือ Token ลง Log

## 8. Error Response

API ควรส่ง Error Response ที่มีรูปแบบสม่ำเสมอ

ตัวอย่าง:

```json
{
  "error": {
    "code": "USER_NOT_FOUND",
    "message": "User not found"
  }
}
```

แนวทางเลือก Status Code:

| สถานการณ์                   | Status       |
| --------------------------- | ------------ |
| Request สำเร็จ              | 200          |
| สร้างข้อมูลสำเร็จ           | 201          |
| ลบข้อมูลสำเร็จโดยไม่มี Body | 204          |
| Input ไม่ถูกต้อง            | 400 หรือ 422 |
| ไม่ได้ยืนยันตัวตน           | 401          |
| ไม่มีสิทธิ์                 | 403          |
| ไม่พบข้อมูล                 | 404          |
| ข้อมูลขัดแย้ง               | 409          |
| Server Error                | 500          |

ไม่ควรเปิดเผย Stack Trace, Database Credentials หรือ Internal Details ใน Public Response

## 9. Input Validation

ข้อมูลจาก Client ต้องถือว่าเป็น Untrusted Input

ตัวอย่างสิ่งที่ควรตรวจสอบ:

- String ว่าง
- String ยาวเกินกำหนด
- ตัวเลขติดลบในกรณีที่ไม่อนุญาต
- Email Format
- ID ที่ไม่ถูกต้อง
- Enum Value ที่ไม่รองรับ
- JSON ที่มีโครงสร้างไม่ตรงตามที่ต้องการ

ควรตรวจสอบทั้งชนิดข้อมูลและ Business Rules

## 10. API Testing

API Testing ใช้ตรวจสอบว่า Endpoint ทำงานถูกต้องตามที่ออกแบบไว้

ประเภทการทดสอบ:

| ประเภท           | จุดประสงค์                         |
| ---------------- | ---------------------------------- |
| Unit Test        | ทดสอบ Function หรือ Logic          |
| Integration Test | ทดสอบการทำงานร่วมกันของ Components |
| API Test         | ทดสอบ Request และ Response         |
| End-to-End Test  | ทดสอบ Flow การทำงานทั้งหมด         |

กรณีที่ควรทดสอบ:

- Request ถูกต้อง
- Request ไม่ครบ
- Invalid Input
- Resource ไม่พบ
- ไม่มี Authentication
- ไม่มี Permission
- Server Error
- Boundary Values

## 11. Logging และ Observability

ควรบันทึกข้อมูลที่ช่วยตรวจสอบปัญหา เช่น:

- Request ID
- HTTP Method
- Endpoint
- Status Code
- Request Duration
- Error Category

ไม่ควรบันทึก:

- Password
- Access Token
- Secret Key
- Sensitive Personal Data ที่ไม่จำเป็น

## 12. Timeout และ Rate Limiting

Timeout ช่วยจำกัดเวลารอการทำงานของ Request หรือการเรียก Service ภายนอก

Rate Limiting ช่วยจำกัดจำนวน Request ต่อช่วงเวลา

ประโยชน์:

- ลดการใช้ Resource เกินจำเป็น
- ป้องกันการเรียก API มากเกินไป
- ช่วยรับมือกับ Abuse
- ป้องกัน Request ที่ใช้เวลานานเกินไป

## 13. สรุป

| คำศัพท์        | ความหมาย                           |
| -------------- | ---------------------------------- |
| Middleware     | ชั้นประมวลผล Request/Response      |
| CORS           | กลไกควบคุม Cross-Origin ใน Browser |
| Authentication | ตรวจสอบตัวตน                       |
| Authorization  | ตรวจสอบสิทธิ์                      |
| Validation     | ตรวจสอบ Input                      |
| Error Response | รูปแบบตอบกลับเมื่อเกิด Error       |
| Logging        | บันทึกเหตุการณ์                    |
| Rate Limiting  | จำกัดจำนวน Request                 |
| Timeout        | จำกัดเวลารอ                        |
| API Testing    | ทดสอบ API                          |

## Completion Checklist

- [ ] เข้าใจ Middleware
- [ ] ใช้ Middleware ใน Axum ได้
- [ ] เข้าใจ CORS
- [ ] เข้าใจ Authentication
- [ ] เข้าใจ Authorization
- [ ] เข้าใจ Password Hashing
- [ ] ออกแบบ Error Response ได้
- [ ] ตรวจสอบ Input ได้
- [ ] เข้าใจ API Testing
- [ ] เข้าใจ Logging
- [ ] เข้าใจ Timeout
- [ ] เข้าใจ Rate Limiting
- [ ] เข้าใจ Security เบื้องต้น
