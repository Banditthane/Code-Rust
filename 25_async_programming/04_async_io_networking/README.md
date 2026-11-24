# Lesson 04: Async I/O and Networking

## 1. Async I/O คืออะไร?

Async I/O คือการทำงานกับ Input/Output โดยไม่จำเป็นต้องบล็อก Thread ไว้ระหว่างรอการดำเนินการ

ตัวอย่าง:

- อ่านไฟล์
- เขียนไฟล์
- รับข้อมูลจาก TCP Connection
- ส่งข้อมูลผ่าน Network
- รอข้อความจาก Channel

ในบทนี้ใช้ Tokio เป็นหลัก

## 2. Async File I/O

Tokio มี Module `tokio::fs` สำหรับทำงานกับไฟล์แบบ Async

ตัวอย่างอ่านไฟล์:

```rust
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("data.txt").await?;

    println!("{}", content);

    Ok(())
}
```

ตัวอย่างเขียนไฟล์:

```rust
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::write("data.txt", "Hello Async Rust").await?;

    println!("File saved");

    Ok(())
}
```

## 3. Async TCP Networking

TCP เป็น Protocol สำหรับสื่อสารข้อมูลระหว่าง Client และ Server

Tokio มี API สำหรับ TCP ผ่าน:

```rust
tokio::net::TcpListener
tokio::net::TcpStream
```

ตัวอย่าง Server:

```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on port 8080");

    loop {
        let (socket, address) = listener.accept().await?;

        println!("Client connected: {}", address);

        drop(socket);
    }
}
```

ตัวอย่างนี้รับ Connection แล้วปิด Socket โดยยังไม่ได้รับส่งข้อมูล

## 4. TCP Client

ตัวอย่างการเชื่อมต่อ TCP Server:

```rust
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    println!("Connected to server");

    drop(stream);

    Ok(())
}
```

ต้องเปิด Server ก่อนจึงจะเชื่อมต่อสำเร็จ

## 5. อ่านและเขียน TCP Stream

Tokio มี `AsyncReadExt` และ `AsyncWriteExt` สำหรับอ่านและเขียนข้อมูล

ตัวอย่างแนวคิด:

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    stream.write_all(b"Hello Server").await?;

    let mut buffer = [0u8; 1024];

    let size = stream.read(&mut buffer).await?;

    println!("Received: {:?}", &buffer[..size]);

    Ok(())
}
```

ตัวอย่างนี้ต้องเชื่อมต่อกับ Server ที่อ่านและส่งข้อมูลตอบกลับได้จริง

## 6. Async Channels

Tokio มี Channel สำหรับสื่อสารระหว่าง Async Tasks

ตัวอย่าง MPSC Channel:

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        tx.send(String::from("Hello from task"))
            .await
            .unwrap();
    });

    if let Some(message) = rx.recv().await {
        println!("{}", message);
    }
}
```

ข้อแตกต่างจาก `std::sync::mpsc`:

- Tokio Channel ออกแบบมาสำหรับ Async Tasks
- `send().await` สามารถรอพื้นที่ว่างใน Buffer ได้
- `recv().await` สามารถรอข้อความโดยไม่บล็อก Runtime Worker แบบ Synchronous

## 7. Timeout

ใช้ `tokio::time::timeout()` เพื่อกำหนดเวลาสูงสุดที่ต้องการรอ Future

ตัวอย่าง:

```rust
use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    let result = timeout(
        Duration::from_secs(2),
        sleep(Duration::from_secs(5))
    ).await;

    match result {
        Ok(_) => println!("Completed"),
        Err(_) => println!("Timed out"),
    }
}
```

ผลลัพธ์:

```text
Timed out
```

เมื่อ Timeout เกิดขึ้น Future ภายในจะถูก Drop การ Drop Future ไม่ได้หมายความว่างานภายนอกทุกประเภทถูกย้อนกลับหรือหยุดได้อย่างสมบูรณ์

## 8. Cancellation

Cancellation คือการยกเลิก Async Task หรือหยุดการรอ Future ก่อนทำงานเสร็จ

ตัวอย่างยกเลิก Tokio Task:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(10)).await;
        println!("Task completed");
    });

    handle.abort();

    let result = handle.await;

    println!("Task stopped: {:?}", result);
}
```

การเรียก `abort()` ขอให้ Tokio ยกเลิก Task

การยกเลิกจะเกิดขึ้นเมื่อ Runtime ได้มีโอกาสจัดการ Task นั้น และการ Drop Future ไม่สามารถย้อนกลับ Side Effect ที่เกิดขึ้นไปแล้วได้

## 9. Cancellation Safety

Cancellation Safety คือการออกแบบ Async Operation ให้รับมือได้เมื่อ Future ถูกยกเลิกระหว่างทำงาน

แนวทาง:

- หลีกเลี่ยงการทำให้ข้อมูลอยู่ในสถานะไม่สมบูรณ์
- ใช้การบันทึกสถานะเมื่อมีการประมวลผลหลายขั้นตอน
- ระวังการยกเลิกระหว่างอ่านหรือเขียนข้อมูล
- ตรวจสอบเอกสารของ Async API ว่ามี Cancellation Safety หรือไม่
- อย่าคิดว่าการยกเลิก Task จะย้อนกลับการทำงานที่เกิดขึ้นแล้ว

## 10. ข้อควรระวัง

- Async File I/O ต้องใช้ Tokio และเปิด Feature ที่เกี่ยวข้อง
- TCP Connection อาจถูกปิดหรือเกิด Error ได้
- `read()` อาจอ่านข้อมูลได้น้อยกว่าขนาด Buffer
- ต้องจัดการ Error จาก Network
- Timeout ไม่ได้ย้อนกลับ Side Effect ที่เกิดขึ้นแล้ว
- Cancellation อาจทำให้การทำงานหยุดกลางทาง
- ควรออกแบบ Protocol และการจัดการ Connection ให้ชัดเจน

## Completion Checklist

- [ ] เข้าใจ Async I/O
- [ ] อ่านไฟล์ด้วย `tokio::fs` ได้
- [ ] เขียนไฟล์ด้วย `tokio::fs` ได้
- [ ] เข้าใจ TcpListener
- [ ] เข้าใจ TcpStream
- [ ] ส่งและรับ TCP Data เบื้องต้นได้
- [ ] ใช้ Tokio MPSC Channel ได้
- [ ] ใช้ Timeout ได้
- [ ] ยกเลิก Task ด้วย `abort()` ได้
- [ ] เข้าใจ Cancellation Safety
