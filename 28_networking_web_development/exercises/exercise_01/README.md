# Exercise 01: TCP Client

## ระดับ

Easy

## Objective

ฝึกสร้าง TCP Client ด้วย Rust เพื่อเชื่อมต่อกับ TCP Server ส่งข้อมูล และรับข้อมูลกลับมา

## Problem Statement

สร้างโปรแกรม TCP Client สำหรับเชื่อมต่อกับ Echo Server

Echo Server คือ Server ที่รับข้อความจาก Client แล้วส่งข้อความเดิมกลับไป

กำหนด Server Address:

```text
127.0.0.1:7878
```

ตัวอย่างข้อความที่ส่ง:

```text
Hello Server
Rust Networking
TCP Client Practice
```

โปรแกรมต้องสามารถ:

1. เชื่อมต่อ TCP Server
2. ส่งข้อความ
3. รับข้อความตอบกลับ
4. แสดงผลลัพธ์
5. จัดการกรณีเชื่อมต่อไม่สำเร็จ

## Requirements

### Part 1: TCP Connection

1. ใช้ `TcpStream::connect()`
2. เชื่อมต่อไปยัง `127.0.0.1:7878`
3. แสดงสถานะการเชื่อมต่อ
4. จัดการ Connection Error

### Part 2: Send Message

1. ใช้ `Write` Trait
2. ส่งข้อความอย่างน้อย 3 ข้อความ
3. ใช้ `write_all()` ส่งข้อมูล
4. กำหนดรูปแบบ Message ให้ Server แยกข้อความได้ เช่น ใช้ Newline Delimiter

### Part 3: Receive Message

1. ใช้ `Read` Trait
2. อ่านข้อมูลจาก Server
3. แสดงข้อความที่ได้รับ
4. จัดการกรณี Server ปิด Connection
5. ไม่สมมติว่า `read()` จะรับข้อมูลได้ครบในครั้งเดียว

### Part 4: Error Handling

ต้องจัดการกรณี:

- Server ไม่ทำงาน
- Connection Refused
- Connection Reset
- Read Error
- Write Error
- Server ปิด Connection

## Constraints

- ต้องใช้ `TcpStream`
- ต้องใช้ `Read` และ `Write`
- ต้องใช้ `Result` หรือ `match` จัดการ Error
- ต้องไม่ใช้ `unwrap()` กับ Network Operation ที่อาจล้มเหลวโดยไม่มีการจัดการ
- ต้องไม่สมมติว่า TCP รักษาขอบเขต Message
- ต้องไม่ทำให้ Client รออ่านข้อมูลอย่างไม่มีกำหนดโดยไม่มีแนวทางจัดการ
- ต้องแสดงสถานะการทำงานอย่างชัดเจน

## Expected Output

```text
TCP Client
Connecting to 127.0.0.1:7878...

Connected successfully!

Sending: Hello Server
Received: Hello Server

Sending: Rust Networking
Received: Rust Networking

Sending: TCP Client Practice
Received: TCP Client Practice

Connection completed.
```

กรณี Server ไม่ทำงาน:

```text
TCP Client
Connecting to 127.0.0.1:7878...

Connection failed!
Error: Connection refused
```

## Hints

- ใช้ `std::net::TcpStream`
- ใช้ `std::io::{Read, Write}`
- ใช้ `write_all()` สำหรับส่งข้อมูล
- ใช้ Buffer สำหรับรับข้อมูล
- พิจารณา `set_read_timeout()` เพื่อจำกัดเวลารอ
- ออกแบบ Delimiter หรือ Message Framing ให้ Client และ Server เข้าใจตรงกัน

## วิธีทดสอบ

ต้องมี Echo Server ทำงานอยู่ที่ `127.0.0.1:7878` ก่อน

สามารถสร้าง Echo Server แยกอีก Project หรือใช้เครื่องมือ TCP Server ที่รองรับการ Echo ได้

## Completion Checklist

- [ ] สร้าง TCP Client ได้
- [ ] เชื่อมต่อ Server ได้
- [ ] ส่งข้อมูลได้
- [ ] รับข้อมูลได้
- [ ] แสดงข้อความตอบกลับได้
- [ ] จัดการ Connection Error ได้
- [ ] จัดการ Read/Write Error ได้
- [ ] เข้าใจ TCP Byte Stream
- [ ] เข้าใจ Message Framing
- [ ] ทดสอบกรณี Server ปิดได้

## Run

```powershell
cd 28_networking_web_development/exercises/exercise_01/tcp_client

cargo check
cargo run
```
