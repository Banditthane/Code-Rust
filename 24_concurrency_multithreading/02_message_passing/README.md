# Lesson 02: Message Passing

## 1. Message Passing คืออะไร?

Message Passing คือวิธีสื่อสารระหว่าง Thread โดยส่งข้อมูลจาก Thread หนึ่งไปยังอีก Thread หนึ่ง

แทนที่หลาย Thread จะเข้าถึงข้อมูลชุดเดียวกันโดยตรง แต่ละ Thread สามารถส่งข้อความผ่านช่องทางที่เรียกว่า Channel

แนวคิด:

```text
Thread A
   |
   | ส่งข้อมูล
   v
 Channel
   |
   | รับข้อมูล
   v
Thread B
```

## 2. Channel ใน Rust

Rust มี Channel ใน Standard Library ผ่าน `std::sync::mpsc`

คำว่า `mpsc` ย่อมาจาก Multiple Producer, Single Consumer

หมายถึง:

- สามารถมี Sender หลายตัว
- มี Receiver หนึ่งตัว
- ใช้ส่งข้อมูลจาก Thread หนึ่งไปยังอีก Thread ได้

นำเข้า Library:

```rust
use std::sync::mpsc;
```

## 3. สร้าง Channel

ใช้ `mpsc::channel()` เพื่อสร้าง Sender และ Receiver

ตัวอย่าง:

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.send(String::from("Hello from Thread"))
        .unwrap();

    let message = rx.recv().unwrap();

    println!("{}", message);
}
```

คำอธิบาย:

- `tx` คือ Sender สำหรับส่งข้อมูล
- `rx` คือ Receiver สำหรับรับข้อมูล
- `send()` ใช้ส่งข้อมูล
- `recv()` ใช้รับข้อมูล
- `recv()` จะรอจนกว่าจะได้รับข้อความ หรือไม่มี Sender เหลืออยู่

ผลลัพธ์:

```text
Hello from Thread
```

## 4. ส่งข้อมูลระหว่าง Thread

ตัวอย่าง:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Hello from spawned thread");
        tx.send(message).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Received: {}", received);
}
```

ข้อสังเกต:

- `move` ย้าย Ownership ของ Sender เข้า Thread
- `send()` ส่ง Ownership ของข้อมูลไปยัง Receiver
- หลังส่ง `String` แล้ว Sender จะไม่สามารถใช้ค่าเดิมต่อได้

## 5. recv() และ try_recv()

| Method       | การทำงาน                                  |
| ------------ | ----------------------------------------- |
| `recv()`     | รอจนได้รับข้อมูลหรือ Channel ถูกปิด       |
| `try_recv()` | ตรวจสอบทันที หากไม่มีข้อมูลจะคืนค่า Error |

ตัวอย่าง `try_recv()`:

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    match rx.try_recv() {
        Ok(message) => println!("{}", message),
        Err(error) => println!("No message: {}", error),
    }

    drop(tx);
}
```

## 6. ส่งข้อมูลหลายข้อความ

สามารถส่งข้อมูลหลายครั้งผ่าน Sender ตัวเดิมได้

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for number in 1..=5 {
            tx
```
