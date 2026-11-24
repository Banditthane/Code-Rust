# Chapter 28: Networking & Web Development

## เป้าหมายของบทเรียน

เรียนรู้การเขียนโปรแกรม Network และ Web Development ด้วยภาษา Rust ตั้งแต่การสื่อสารระหว่างเครื่องไปจนถึงการสร้าง REST API

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบายพื้นฐาน Computer Networking ได้
- เข้าใจ IP Address และ Port
- เข้าใจ TCP และ UDP
- สร้าง TCP Client และ Server เบื้องต้นได้
- เข้าใจ HTTP Request และ Response
- เข้าใจ HTTP Methods และ Status Codes
- จัดการ HTTP Headers ได้
- เข้าใจ REST API
- รับและส่งข้อมูล JSON ได้
- สร้าง Routing ด้วย Axum ได้
- สร้าง Web API ด้วย Rust ได้
- เข้าใจ Middleware
- เข้าใจ Authentication เบื้องต้น
- จัดการ Error Response ได้
- ทดสอบ API ได้

## บทเรียน

| Lesson | หัวข้อ                    | เนื้อหา                                             |
| ------ | ------------------------- | --------------------------------------------------- |
| 01     | Networking Fundamentals   | Network, TCP/UDP, IP, Port, Client/Server           |
| 02     | HTTP and Web Fundamentals | HTTP, Request, Response, Methods, Status Codes      |
| 03     | Web API Development       | REST API, JSON, Routing, Axum                       |
| 04     | Web Services              | Middleware, Authentication, Error Handling, Testing |

## แบบฝึกหัด

| Exercise | ชื่อ                  | ระดับ       |
| -------- | --------------------- | ----------- |
| 01       | TCP Client            | Easy        |
| 02       | HTTP Practice         | Easy-Medium |
| 03       | REST API              | Medium      |
| 04       | Web Service Challenge | Challenge   |

## โครงสร้างโฟลเดอร์

```text
28_networking_web_development/
├── README.md
├── 01_networking_fundamentals/
│   └── README.md
├── 02_http_web_fundamentals/
│   └── README.md
├── 03_web_api_development/
│   └── README.md
├── 04_web_services/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── tcp_client/
    ├── exercise_02/
    │   ├── README.md
    │   └── http_practice/
    ├── exercise_03/
    │   ├── README.md
    │   └── rest_api/
    └── exercise_04/
        ├── README.md
        └── web_service_challenge/
```

## แนวคิดสำคัญ

### Networking

การสื่อสารระหว่างอุปกรณ์หรือโปรแกรมผ่านเครือข่าย

### TCP

โปรโตคอลที่ให้การส่งข้อมูลแบบ Connection-oriented และมีการจัดลำดับข้อมูลให้ผู้รับ

### UDP

โปรโตคอลแบบ Connectionless ที่ไม่รับประกันการส่งถึงหรือการจัดลำดับข้อมูล

### HTTP

โปรโตคอลสำหรับการสื่อสารระหว่าง HTTP Client และ HTTP Server

### REST API

แนวทางการออกแบบ API โดยใช้ Resource และ HTTP Methods เพื่อจัดการข้อมูล

### Axum

Web Framework สำหรับ Rust ที่ใช้สร้าง HTTP Server และ Web API

## เครื่องมือที่ใช้

- Rust Compiler
- Cargo
- `std::net`
- Tokio
- Axum
- Serde
- Serde JSON
- HTTP Client เช่น curl หรือ Postman

## คำสั่งที่ใช้บ่อย

```powershell
cargo check
cargo run
cargo test
cargo clippy
```

ตัวอย่างคำสั่งทดสอบ HTTP API:

```powershell
curl http://127.0.0.1:3000/
```

## แนวทางการเรียน

1. เข้าใจพื้นฐาน Network และ TCP/UDP
2. ทดลองสร้าง TCP Client และ Server
3. เรียนรู้ HTTP Request และ Response
4. ทดลองส่งข้อมูลผ่าน HTTP
5. สร้าง REST API ด้วย Axum
6. จัดการ JSON และ Error Response
7. เพิ่ม Middleware และ Authentication
8. ทดสอบ Web API

## ข้อควรระวัง

- อย่าเปิด Server สู่ Internet โดยไม่เข้าใจเรื่อง Security
- อย่าเก็บ Password เป็น Plain Text
- อย่าส่ง Secret หรือ API Key กลับไปใน Response
- ต้องตรวจสอบ Input จาก Client
- ต้องจัดการ Error อย่างเหมาะสม
- ต้องกำหนด Timeout สำหรับงาน Network ที่เหมาะสม
- ต้องระวังการเปิด Port และ Firewall
- หลีกเลี่ยงการใช้ Blocking I/O ภายใน Async Runtime โดยไม่จำเป็น
- Authentication และ Authorization เป็นคนละเรื่องกัน

## Completion Checklist

- [ ] เข้าใจ Networking Fundamentals
- [ ] เข้าใจ IP Address และ Port
- [ ] เข้าใจ TCP
- [ ] เข้าใจ UDP
- [ ] สร้าง TCP Client ได้
- [ ] สร้าง TCP Server ได้
- [ ] เข้าใจ HTTP Request และ Response
- [ ] เข้าใจ HTTP Methods
- [ ] เข้าใจ HTTP Status Codes
- [ ] เข้าใจ REST API
- [ ] รับและส่ง JSON ได้
- [ ] สร้าง API ด้วย Axum ได้
- [ ] เข้าใจ Middleware
- [ ] เข้าใจ Authentication เบื้องต้น
- [ ] ทดสอบ API ได้

## เกณฑ์ผ่านบทเรียน

สามารถสร้าง HTTP API ด้วย Rust รับ Request ตรวจสอบข้อมูล จัดการ Response และ Error ได้ พร้อมเข้าใจพื้นฐานการสื่อสารผ่าน TCP/IP

**หลักสำคัญ:** Web Service ที่ดีต้องไม่ได้แค่ทำงานได้ แต่ต้องจัดการ Input, Error, Security และ Network Failure อย่างเหมาะสม
