# Lesson 02: Future and Runtime

## 1. Future คืออะไร?

Future คือ Trait ที่ใช้แทนงานซึ่งอาจให้ผลลัพธ์ในอนาคต

Rust กำหนด Future ไว้ใน Standard Library:

```rust
use std::future::Future;
```

แนวคิดของ Future:

```rust
Future<Output = T>
```

หมายถึง Future ที่เมื่อทำงานเสร็จจะให้ผลลัพธ์ชนิด `T`

ตัวอย่าง:

```rust
async fn get_message() -> String {
    String::from("Hello")
}
```

Function นี้มี Future ที่ให้ผลลัพธ์เป็น `String`

## 2. Poll

Future มี Method ชื่อ `poll()` ซึ่ง Executor ใช้ขับเคลื่อน Future

ผลลัพธ์เป็น:

```rust
Poll::Pending
Poll::Ready(value)
```

ความหมาย:

| สถานะ          | ความหมาย                |
| -------------- | ----------------------- |
| `Pending`      | Future ยังทำงานไม่เสร็จ |
| `Ready(value)` | Future ทำงานเสร็จแล้ว   |

การ Poll ต้องใช้ `Context` และ `Waker` เพื่อให้ Future แจ้ง Executor เมื่อพร้อมทำงานต่อ

โดยปกติผู้พัฒนา Application ไม่จำเป็นต้องเขียน Poll เอง

## 3. Async Runtime คืออะไร?

Async Runtime เป็นระบบที่ทำหน้าที่:

- สร้างและจัดการ Executor
- Poll Future
- จัดตาราง Async Tasks
- จัดการการรอ I/O
- ปลุก Task เมื่อมีเหตุการณ์ที่ทำให้ทำงานต่อได้

Rust Language มี Syntax สำหรับ Async/Await และ Future แต่ไม่ได้กำหนด Runtime มาตรฐานเพียงตัวเดียว

ตัวอย่าง Runtime ที่ใช้กัน:

- Tokio
- async-std

ในบทนี้จะใช้ Tokio เป็นหลัก

## 4. Tokio คืออะไร?

Tokio เป็น Async Runtime สำหรับ Rust ที่มีเครื่องมือสำหรับ:

- Async Tasks
- Timers
- Networking
- Channels
- Synchronization
- Async File I/O

เพิ่ม Tokio:

```powershell
cargo add tokio --features full
```

หรือเพิ่มใน `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## 5. Tokio Main Macro

ใช้ `#[tokio::main]` เพื่อสร้าง Runtime และรัน Async Main Function

ตัวอย่าง:

```rust
#[tokio::main]
async fn main() {
    println!("Hello Tokio");
}
```

Macro นี้ช่วยสร้าง Runtime ให้โดยอัตโนมัติ

จึงไม่ต้องสร้าง Runtime เองในโปรแกรมตัวอย่างทั่วไป

## 6. สร้าง Runtime ด้วยตนเอง

สามารถสร้าง Runtime เองได้เมื่อจำเป็นต้องควบคุมการทำงานมากขึ้น

ตัวอย่าง:

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async {
        println!("Running async task");
    });
}
```

คำอธิบาย:

- `Runtime::new()` สร้าง Tokio Runtime
- `block_on()` ขับเคลื่อน Future จนได้ผลลัพธ์
- `async { ... }` สร้าง Async Block

การใช้ `block_on()` จะบล็อก Thread ที่เรียกจนกว่า Future จะเสร็จ จึงควรใช้ในบริบทที่เหมาะสม ไม่ควรเรียกซ้อนอย่างไม่ระมัดระวังภายใน Async Context

## 7. Executor และ Task

Executor คือส่วนที่ทำหน้าที่ Poll Future

Task คือหน่วยงาน Async ที่ Runtime จัดการ

ตัวอย่างแนวคิด:

```text
Application
    |
    v
Tokio Runtime
    |
    v
Executor
    |
    +---- Task A
    |
    +---- Task B
    |
    +---- Task C
```

Task หลายตัวสามารถถูกสลับการทำงานระหว่างรอ I/O ได้

## 8. Multi-thread Runtime และ Current-thread Runtime

Tokio มี Runtime หลายรูปแบบ

### Multi-thread Runtime

ใช้ Worker Threads หลายตัวเพื่อขับเคลื่อน Tasks

### Current-thread Runtime

ขับเคลื่อน Tasks บน Thread เดียว

ทั้งสองแบบยังคงเป็น Async Runtime และมีรูปแบบการทำงานที่เหมาะกับสถานการณ์ต่างกัน

## 9. ข้อควรระวัง

- Future จะไม่ทำงานต่อเองหากไม่มี Executor ขับเคลื่อน
- Runtime เป็นผู้ Poll Future
- `block_on()` บล็อก Thread ที่เรียก
- ไม่ควรสร้าง Runtime ใหม่โดยไม่จำเป็น
- งาน Blocking ที่ใช้เวลานานอาจทำให้ Runtime Worker ทำงานอื่นได้ช้าลง
- หากต้องทำงาน Blocking ใน Tokio ให้ศึกษาการใช้ `spawn_blocking()`

## Completion Checklist

- [ ] เข้าใจ Future
- [ ] เข้าใจ Future Output
- [ ] เข้าใจ Poll
- [ ] เข้าใจ Pending และ Ready
- [ ] เข้าใจ Async Runtime
- [ ] เข้าใจ Executor และ Task
- [ ] ติดตั้ง Tokio ได้
- [ ] ใช้ `#[tokio::main]` ได้
- [ ] เข้าใจการสร้าง Runtime ด้วยตนเอง
- [ ] เข้าใจความแตกต่างระหว่าง Multi-thread และ Current-thread Runtime
