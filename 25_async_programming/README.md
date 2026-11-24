# Chapter 25: Async Programming

## เป้าหมายของบทเรียน

เรียนรู้การเขียนโปรแกรมแบบ Asynchronous ด้วย Rust เข้าใจการทำงานของ `async`, `await`, `Future` และ Async Runtime รวมถึงการใช้ Tokio เพื่อจัดการหลายงานที่ต้องรอให้เสร็จ

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบายความแตกต่างระหว่าง Synchronous และ Asynchronous ได้
- สร้าง Async Function ด้วย `async fn` ได้
- ใช้ `.await` เพื่อรอผลลัพธ์จาก Future ได้
- เข้าใจ Future และ Async Runtime
- ใช้ Tokio Runtime เพื่อรัน Async Code ได้
- สร้างและจัดการ Async Tasks ได้
- ใช้ `tokio::spawn()` และ `tokio::join!()` ได้
- จัดการ Error ใน Async Function ได้
- ใช้ Async File I/O และ TCP เบื้องต้นได้
- ใช้ Timeout และ Cancellation ได้

## บทเรียน

| Lesson | หัวข้อ                   | เนื้อหา                                        |
| ------ | ------------------------ | ---------------------------------------------- |
| 01     | Async Fundamentals       | Sync, Async, async, await, Future              |
| 02     | Future and Runtime       | Future Trait, Runtime, Executor, Tokio         |
| 03     | Async Tasks              | Spawn, JoinHandle, join!, Error Handling       |
| 04     | Async I/O and Networking | File I/O, TCP, Channels, Timeout, Cancellation |

## แบบฝึกหัด

| Exercise | ชื่อ            | ระดับ       |
| -------- | --------------- | ----------- |
| 01       | Async Basics    | Easy        |
| 02       | Tokio Practice  | Easy-Medium |
| 03       | Async Tasks     | Medium      |
| 04       | Async Challenge | Challenge   |

## โครงสร้างโฟลเดอร์

```text
25_async_programming/
├── README.md
├── 01_async_fundamentals/
│   └── README.md
├── 02_future_and_runtime/
│   └── README.md
├── 03_async_tasks/
│   └── README.md
├── 04_async_io_networking/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── async_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── tokio_practice/
    ├── exercise_03/
    │   ├── README.md
    │   └── async_tasks/
    └── exercise_04/
        ├── README.md
        └── async_challenge/
```

## แนวคิดสำคัญ

### Synchronous

การทำงานแบบปกติ โดยคำสั่งถัดไปจะรอคำสั่งก่อนหน้าทำงานเสร็จ

### Asynchronous

การเขียนโปรแกรมที่สามารถพักงานซึ่งกำลังรอผล แล้วเปิดโอกาสให้ Runtime จัดการงานอื่นต่อได้

### Future

ตัวแทนของผลลัพธ์ที่อาจพร้อมใช้งานในอนาคต

### Async Runtime

ระบบที่ทำหน้าที่จัดตารางและขับเคลื่อน Future ให้ทำงาน เช่น Tokio

## เครื่องมือที่ใช้

- Rust Standard Library
- Tokio
- Tokio Macros
- Tokio Time
- Tokio Sync
- Tokio Filesystem และ Networking

## คำสั่งที่ใช้บ่อย

```powershell
cargo new async_basics
cd async_basics

cargo add tokio --features full

cargo run
cargo check
cargo test
```

หมายเหตุ: `cargo add` ใช้เพิ่ม Dependency ใน `Cargo.toml` หากยังไม่มีคำสั่งนี้ สามารถเพิ่ม Dependency ในไฟล์ `Cargo.toml` ด้วยตนเองได้

## แนวทางการเรียน

1. ทำความเข้าใจ Sync และ Async
2. เรียนรู้ Future และ Runtime
3. ทดลองสร้างและจัดการ Async Tasks
4. ฝึก Async I/O และ Networking
5. ทำแบบฝึกหัดตามลำดับ

## ข้อควรระวัง

- การประกาศ `async fn` ไม่ได้หมายความว่า Function จะเริ่มทำงานทันที
- Future ต้องถูก Poll โดย Executor จึงจะขับเคลื่อนการทำงาน
- `.await` อาจพัก Task เพื่อให้ Runtime ไปทำงานอื่น
- `tokio::spawn()` สร้าง Task ไม่ใช่การรับประกันว่าจะสร้าง OS Thread ใหม่
- Task ที่ Spawn ด้วย Tokio ต้องมีคุณสมบัติ `Send + 'static` ตามข้อกำหนดของ API ที่ใช้
- อย่าปิดกั้น Runtime Worker ด้วยงาน Blocking ที่ใช้เวลานาน
- การยกเลิก Task ต้องคำนึงถึงการทำงานที่เกิดขึ้นไปแล้วและการทำความสะอาดทรัพยากร

## Completion Checklist

- [ ] เข้าใจ Synchronous และ Asynchronous
- [ ] สร้าง `async fn` ได้
- [ ] ใช้ `.await` ได้
- [ ] เข้าใจ Future
- [ ] เข้าใจ Async Runtime
- [ ] ใช้ Tokio Runtime ได้
- [ ] สร้าง Async Tasks ได้
- [ ] ใช้ `tokio::spawn()` และ `tokio::join!()` ได้
- [ ] ใช้ Async File I/O และ Networking เบื้องต้นได้
- [ ] ใช้ Timeout และ Cancellation ได้
- [ ] ทำแบบฝึกหัดทั้ง 4 ข้อครบถ้วน

## เกณฑ์ผ่านบทเรียน

สามารถเขียนโปรแกรม Async ด้วย Tokio จัดการหลาย Tasks และใช้ Async I/O เบื้องต้นได้อย่างถูกต้อง
