# Exercise 04: Async Challenge

## ระดับ

Challenge

## Objective

ประยุกต์ใช้ Async Function, Tokio Tasks, Channels, File I/O, TCP Networking, Timeout และ Cancellation เพื่อสร้างระบบรับและประมวลผลข้อความแบบ Asynchronous

## Problem Statement

สร้างระบบจำลอง Message Processing Server

ระบบต้องสามารถ:

1. รับข้อความจาก Client ผ่าน TCP
2. ส่งข้อความเข้า Channel
3. ให้ Worker ประมวลผลข้อความ
4. ส่งผลลัพธ์กลับไปยัง Client
5. บันทึกข้อมูลลงไฟล์แบบ Async
6. กำหนด Timeout สำหรับการรอ
7. จัดการ Error และการยกเลิก Task

กำหนดให้มี Worker อย่างน้อย 3 ตัว

## Requirements

### Part 1: TCP Server

1. ใช้ `tokio::net::TcpListener`
2. Bind Server กับ Local Address
3. รับ Connection จาก Client
4. อ่านข้อมูลจาก TCP Stream
5. จัดการ Client ที่เชื่อมต่อเข้ามาหลายราย
6. ปิด Connection อย่างเหมาะสม

### Part 2: Message Passing

1. สร้าง Tokio MPSC Channel
2. ใช้ Channel ส่งข้อความจากส่วนรับข้อมูลไปยัง Worker
3. ใช้ Receiver รับข้อความ
4. กำหนด Buffer Capacity
5. จัดการกรณี Channel ถูกปิด
6. ป้องกันไม่ให้ Sender ที่ไม่จำเป็นค้างอยู่จน Worker รอข้อความไม่สิ้นสุด

### Part 3: Worker Tasks

1. สร้าง Worker อย่างน้อย 3 Tasks
2. Worker ต้องรับข้อความจาก Channel
3. Worker ประมวลผลข้อความ
4. Worker ส่งผลลัพธ์กลับไปยังส่วนที่รับผิดชอบ
5. ใช้ `tokio::spawn()` หรือโครงสร้าง Task ที่เหมาะสม
6. จัดการ Error ของ Worker

### Part 4: Async File I/O

1. ใช้ `tokio::fs` หรือ Async File API ของ Tokio
2. บันทึกข้อความที่ประมวลผลแล้ว
3. ใช้ Async Write
4. กำหนดรูปแบบข้อมูลในไฟล์
5. จัดการ Error จากการอ่านหรือเขียนไฟล์

### Part 5: Timeout and Cancellation

1. ใช้ `tokio::time::timeout()` กับการรอที่กำหนด
2. ทดลองกรณีที่งานเสร็จภายในเวลา
3. ทดลองกรณีที่งานใช้เวลานานเกินกำหนด
4. ทดลองยกเลิก Task ด้วย `abort()`
5. จัดการผลลัพธ์หลัง Task ถูกยกเลิก
6. พิจารณาผลกระทบต่อข้อมูลที่กำลังประมวลผล

### Part 6: Result Summary

เมื่อระบบทำงานเสร็จ ให้แสดง:

- จำนวน Client ที่เชื่อมต่อ
- จำนวนข้อความที่ได้รับ
- จำนวนข้อความที่ประมวลผลสำเร็จ
- จำนวนข้อความที่เกิด Error
- จำนวนงานที่ Timeout
- สถานะการบันทึกไฟล์

## Constraints

- ต้องใช้ Tokio
- ต้องใช้ Async Function
- ต้องใช้ `tokio::spawn()` หรือ Task Management ที่เหมาะสม
- ต้องใช้ Tokio MPSC Channel
- ต้องใช้ Async File I/O
- ต้องใช้ TCP Networking
- ต้องใช้ Timeout
- ต้องทดลอง Cancellation
- ต้องจัดการ Error ด้วย Result
- ห้ามใช้ `static mut`
- ห้ามใช้ `unsafe`
- ห้ามใช้ `std::thread::sleep()` ภายใน Async Task
- ต้องไม่ใช้ Blocking I/O บน Runtime Worker โดยไม่จำเป็น
- ต้องไม่สมมติว่าการยกเลิก Task จะย้อนกลับการเขียนไฟล์หรือ Side Effect ที่เกิดขึ้นแล้ว

## Expected Output

ตัวอย่างผลลัพธ์:

```text
Message Server Started
Listening on 127.0.0.1:8080

Client connected

Message received: Hello Rust
Message processed: HELLO RUST
Message saved

Message received: Async Programming
Message processed: ASYNC PROGRAMMING
Message saved

Processing Summary
Clients Connected: 1
Messages Received: 2
Messages Completed: 2
Messages Failed: 0
Timeout Tasks: 0

Server stopped
```

ผลลัพธ์จริงขึ้นอยู่กับการออกแบบ Server และ Client ของผู้เรียน

## Hints

- ใช้ `TcpListener::bind()` เพื่อสร้าง Server
- ใช้ `accept().await` เพื่อรับ Connection
- ใช้ `AsyncReadExt` และ `AsyncWriteExt` อ่านและเขียน TCP Stream
- ใช้ `tokio::sync::mpsc::channel()` สำหรับกระจายงาน
- พิจารณาการสร้าง Receiver แยกสำหรับ Worker แต่ละตัว หรือใช้ Receiver ร่วมผ่านกลไกที่เหมาะสม
- ใช้ `tokio::fs::OpenOptions` เมื่อต้องการเปิดไฟล์เพื่อเขียนต่อท้าย
- ใช้ `tokio::time::timeout()` ควบคุมเวลารอ
- ใช้ `JoinHandle::abort()` ทดลองยกเลิก Task
- แยกส่วน TCP Server, Worker, Message Processing และ File Logging ออกจากกัน
- ควรออกแบบให้ Worker ปิดตัวได้อย่างเป็นระเบียบเมื่อไม่มีงานเหลือ

## Completion Checklist

### TCP Server

- [ ] สร้าง TCP Listener
- [ ] รับ Client Connection ได้
- [ ] อ่านข้อความจาก Client ได้
- [ ] ส่งผลลัพธ์กลับ Client ได้

### Message Passing

- [ ] สร้าง Tokio MPSC Channel
- [ ] ส่งข้อความผ่าน Channel ได้
- [ ] Worker รับข้อความได้
- [ ] จัดการ Channel ปิดได้

### Worker Tasks

- [ ] สร้าง Worker อย่างน้อย 3 Tasks
- [ ] ประมวลผลข้อความได้
- [ ] ส่งผลลัพธ์กลับได้
- [ ] จัดการ Error ได้

### File I/O

- [ ] บันทึกข้อความด้วย Async File I/O
- [ ] จัดการ File Error ได้
- [ ] ตรวจสอบไฟล์ผลลัพธ์ได้

### Timeout and Cancellation

- [ ] ใช้ Timeout ได้
- [ ] ทดลองงานที่ Timeout
- [ ] ทดลองยกเลิก Task
- [ ] จัดการผลลัพธ์จาก Task ที่ถูกยกเลิกได้
- [ ] เข้าใจผลกระทบของ Cancellation

### Final Result

- [ ] แสดงจำนวน Client
- [ ] แสดงจำนวนข้อความที่ได้รับ
- [ ] แสดงจำนวนข้อความที่สำเร็จ
- [ ] แสดงจำนวนข้อความที่ผิดพลาด
- [ ] แสดงจำนวน Timeout
- [ ] ปิด Server และ Worker อย่างเหมาะสม
- [ ] โปรแกรมทำงานโดยไม่มี `unsafe`

## Run

```powershell
cd 25_async_programming/exercises/exercise_04/async_challenge
cargo add tokio --features full
cargo run
```

## Final Challenge

เมื่อทำโจทย์สำเร็จแล้ว ให้ทดลองเพิ่มความสามารถ:

1. เพิ่ม Message ID ให้แต่ละข้อความ
2. เพิ่มระบบ Retry สำหรับข้อความที่ประมวลผลไม่สำเร็จ
3. เพิ่มการจำกัดจำนวน Client ที่เชื่อมต่อพร้อมกัน
4. เพิ่มระบบ Graceful Shutdown
5. เพิ่มระบบ Logging พร้อม Timestamp
6. เปรียบเทียบการใช้ `tokio::join!()` กับ `tokio::spawn()`

**หมายเหตุ:** โจทย์นี้เป็นการฝึกออกแบบระบบ Async แบบหลายส่วน ไม่ได้กำหนดให้ต้องสร้าง Production Server ที่รองรับทุกกรณี
