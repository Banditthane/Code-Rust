# Lesson 03: Web API Development

## 1. Web API คืออะไร?

Web API คือ Interface ที่เปิดให้ Client ติดต่อกับ Application ผ่าน Network โดยมักใช้ HTTP เป็น Protocol

ตัวอย่าง:

- User API
- Product API
- Order API
- Authentication API

ตัวอย่าง Request:

```http
GET /users/1
```

ตัวอย่าง Response:

```json
{
  "id": 1,
  "name": "Alice"
}
```

## 2. REST API

REST (Representational State Transfer) เป็น Architectural Style สำหรับออกแบบระบบที่สื่อสารผ่าน Resource

แนวคิดสำคัญ:

- ใช้ Resource เป็นศูนย์กลาง
- ใช้ URI ระบุ Resource
- ใช้ HTTP Methods แสดงเจตนา
- ใช้ HTTP Status Codes
- มี Stateless Communication ตามข้อกำหนดของ REST

ตัวอย่าง Resource:

```text
/users
/users/1
/products
/products/10
/orders
/orders/20
```

## 3. REST API Design

ตัวอย่าง User API:

| Method | Endpoint      | หน้าที่            |
| ------ | ------------- | ------------------ |
| GET    | `/users`      | อ่าน Users ทั้งหมด |
| GET    | `/users/{id}` | อ่าน User ตาม ID   |
| POST   | `/users`      | สร้าง User         |
| PUT    | `/users/{id}` | แทนที่ข้อมูล User  |
| PATCH  | `/users/{id}` | แก้ไขข้อมูลบางส่วน |
| DELETE | `/users/{id}` | ลบ User            |

ควรใช้ Nouns สำหรับ Resource และเลือก HTTP Method ให้เหมาะสม

## 4. Axum คืออะไร?

Axum เป็น Web Framework สำหรับ Rust ที่ใช้สร้าง HTTP Server และ API

คุณสมบัติ:

- Routing
- Extractors
- JSON Handling
- Middleware Integration
- Async Handler
- Integration กับ Tokio และ Tower

## 5. สร้าง Axum Project

สร้าง Project:

```powershell
cargo new rest_api
cd rest_api
```

ติดตั้ง Dependencies:

```powershell
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
```

ตรวจสอบ `Cargo.toml` ว่ามี Dependencies ครบถ้วน

## 6. Axum Router

Router ใช้กำหนดเส้นทาง HTTP Request

ตัวอย่าง:

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn home() -> &'static str {
    "Hello from Axum"
}
```

เปิด Server แล้วทดลองเรียก:

```powershell
curl http://127.0.0.1:3000/
```

## 7. Handler

Handler คือ Async Function ที่รับ Request และสร้าง Response

ตัวอย่าง:

```rust
use axum::{
    Json,
    routing::get,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        name: String::from("Alice"),
    })
}

fn create_app() -> Router {
    Router::new()
        .route("/user", get(get_user))
}
```

## 8. Path Parameters

ใช้ Path Extractor เพื่ออ่านค่าจาก URL

ตัวอย่าง Endpoint:

```text
/users/10
```

แนวคิด:

```rust
use axum::{
    extract::Path,
    Json,
};

async fn get_user(Path(id): Path<u32>) -> Json<u32> {
    Json(id)
}
```

กำหนด Route ให้ตรงกับ Path Parameter เช่น `/users/{id}` ใน Axum รุ่นปัจจุบัน

## 9. Query Parameters

ใช้ Query Extractor สำหรับอ่าน Query String

ตัวอย่าง:

```text
/users?page=1&limit=10
```

ตัวอย่าง Struct:

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: u32,
    limit: u32,
}

async fn list_users(Query(params): Query<Pagination>) {
    println!("Page: {}", params.page);
    println!("Limit: {}", params.limit);
}
```

## 10. JSON Request Body

ใช้ `Json<T>` เพื่อรับ JSON จาก Client

ตัวอย่าง:

```rust
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u8,
}

async fn create_user(Json(payload): Json<CreateUser>) -> String {
    format!("Created user: {}", payload.name)
}
```

ข้อมูล JSON ที่ไม่ตรงกับชนิดข้อมูลอาจทำให้เกิด Rejection ซึ่งควรจัดการให้เหมาะสม

## 11. JSON Response

ใช้ `Serialize` เพื่อแปลง Rust Struct เป็น JSON

ตัวอย่าง:

```rust
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    name: String,
}
```

จากนั้นคืนค่า `Json<UserResponse>` จาก Handler

## 12. State Management

ใช้ State เพื่อแชร์ข้อมูลระหว่าง Handler

แนวคิด:

- กำหนด Application State
- ใช้ `Router::with_state()`
- รับ State ผ่าน `State<T>`
- ใช้ `Arc` หรือ Thread-safe Data Structure เมื่อจำเป็น

หากมีหลาย Request เข้าถึงข้อมูลร่วมกัน ต้องออกแบบ Synchronization ให้ถูกต้อง

## 13. Error Handling

Axum รองรับการสร้าง Response ด้วย `IntoResponse`

แนวทาง:

- ใช้ `StatusCode` ระบุ HTTP Status
- สร้าง Error Response ที่มีรูปแบบสม่ำเสมอ
- ไม่ส่ง Internal Error หรือ Secret กลับไปยัง Client
- บันทึกรายละเอียด Error ฝั่ง Server ตามความเหมาะสม

## 14. สรุป

| คำศัพท์   | ความหมาย                            |
| --------- | ----------------------------------- |
| Web API   | Interface สำหรับสื่อสารผ่าน Network |
| REST      | Architectural Style                 |
| Resource  | ข้อมูลหรือ Entity ที่ API จัดการ    |
| Router    | กำหนดเส้นทาง Request                |
| Handler   | ประมวลผล Request                    |
| Extractor | ดึงข้อมูลจาก Request                |
| `Json<T>` | รับหรือส่ง JSON                     |
| State     | ข้อมูลที่แชร์ใน Application         |
| Axum      | Rust Web Framework                  |

## Completion Checklist

- [ ] เข้าใจ Web API
- [ ] เข้าใจ REST
- [ ] ออกแบบ Resource Endpoint ได้
- [ ] ติดตั้ง Axum ได้
- [ ] สร้าง Router ได้
- [ ] สร้าง Handler ได้
- [ ] ใช้ Path Extractor ได้
- [ ] ใช้ Query Extractor ได้
- [ ] รับ JSON ได้
- [ ] ส่ง JSON ได้
- [ ] ใช้ Application State ได้
- [ ] จัดการ HTTP Error ได้
