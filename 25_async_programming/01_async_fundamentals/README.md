# Lesson 01: Async Fundamentals

## 1. Synchronous คืออะไร?

Synchronous คือรูปแบบการทำงานที่คำสั่งทำงานตามลำดับ โดยคำสั่งถัดไปจะรอคำสั่งก่อนหน้าให้เสร็จก่อน

ตัวอย่าง:

```rust
fn main() {
    let first = calculate_first();
    let second = calculate_second();

    println!("{} {}", first, second);
}

fn calculate_first() -> i32 {
    10
}

fn calculate_second() -> i32 {
    20
}
```

โปรแกรมเรียก `calculate_first()` ก่อน แล้วจึงเรียก `calculate_second()`

## 2. Asynchronous คืออะไร?

Asynchronous คือรูปแบบการทำงานที่เปิดโอกาสให้โปรแกรมพักงานที่กำลังรอ แล้วให้ Runtime จัดการงานอื่นต่อได้

เหมาะกับงานประเภท:

- รอข้อมูลจาก Network
- เรียก Web API
- อ่านหรือเขียนไฟล์แบบ Async
- รอข้อความจาก Channel
- จัดการ Connection หลายรายการ

Async ไม่ได้หมายความว่าทุกงานจะทำงานบนหลาย CPU Core เสมอไป

## 3. เปรียบเทียบ Synchronous และ Asynchronous

| Synchronous                    | Asynchronous                      |
| ------------------------------ | --------------------------------- |
| ทำงานตามลำดับ                  | สามารถสลับไปทำงานอื่นระหว่างรอได้ |
| การรออาจหยุดการทำงานของ Thread | การรอ Async อาจพักเฉพาะ Task      |
| เข้าใจง่ายสำหรับงานทั่วไป      | เหมาะกับงาน I/O ที่ต้องรอ         |
| ใช้ Function ปกติ              | ใช้ Async Function และ Future     |

## 4. Async Function

Rust ใช้ `async fn` เพื่อประกาศ Async Function

ตัวอย่าง:

```rust
async fn say_hello() {
    println!("Hello from async function");
}
```

สิ่งสำคัญ:

การเรียก Async Function จะได้ค่า Future กลับมา ไม่ได้หมายความว่า Function เริ่มทำงานทันที

ตัวอย่าง:

```rust
async fn get_number() -> i32 {
    100
}
```

การเรียก `get_number()` จะสร้าง Future ที่เมื่อถูกขับเคลื่อนจนเสร็จ จะให้ผลลัพธ์เป็น `i32`

## 5. Await

ใช้ `.await` เพื่อรอให้ Future ให้ผลลัพธ์

ตัวอย่างเมื่อใช้ Tokio:

```rust
#[tokio::main]
async fn main() {
    let number = get_number().await;

    println!("Number: {}", number);
}

async fn get_number() -> i32 {
    100
}
```

ผลลัพธ์:

```text
Number: 100
```

คำอธิบาย:

- `#[tokio::main]` สร้าง Runtime เพื่อรัน Async Main
- `async fn main()` เป็น Async Entry Point ที่ Macro จัดการให้
- `get_number()` คืนค่า Future
- `.await` รอผลลัพธ์จาก Future

## 6. Future คืออะไร?

`Future` เป็น Trait ใน Rust Standard Library ที่แทนการคำนวณซึ่งอาจยังไม่เสร็จ

เมื่อ Future ถูก Poll จะมีผลลัพธ์ได้ 2 แบบ:

- `Poll::Pending` หมายถึงยังไม่เสร็จ ต้องรอการ Poll อีกครั้ง
- `Poll::Ready(value)` หมายถึงเสร็จแล้วและมีผลลัพธ์

แนวคิด:

```text
Future
   |
   v
 Poll
   |
   +---- Pending
   |       |
   |       v
   |   รอและ Poll อีกครั้ง
   |
   +---- Ready(value)
           |
           v
       ได้ผลลัพธ์
```

โดยทั่วไปผู้เรียนใช้ `.await` แทนการ Poll Future ด้วยตนเอง

## 7. Async Delay

ตัวอย่างการรอเวลาแบบ Async ด้วย Tokio:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start");

    sleep(Duration::from_secs(2)).await;

    println!("Finished");
}
```

ระหว่างที่ Task นี้รอเวลา Runtime สามารถไปทำงานกับ Task อื่นได้

แตกต่างจาก `std::thread::sleep()` ซึ่งจะหยุด Thread ที่เรียกใช้งานไว้โดยตรง

## 8. Async Function เรียก Async Function

ตัวอย่าง:

```rust
#[tokio::main]
async fn main() {
    let result = calculate().await;

    println!("Result: {}", result);
}

async fn calculate() -> i32 {
    let first = get_first().await;
    let second = get_second().await;

    first + second
}

async fn get_first() -> i32 {
    10
}

async fn get_second() -> i32 {
    20
}
```

ผลลัพธ์:

```text
Result: 30
```

## 9. ข้อควรจำ

- `async fn` คืนค่า Future
- การสร้าง Future ไม่ได้ทำให้มันเริ่มทำงานทันที
- `.await` ใช้รอผลลัพธ์จาก Future
- Async Function ต้องมี Executor หรือ Runtime มาขับเคลื่อน
- `.await` ไม่ได้สร้าง Thread ใหม่
- Async เหมาะกับงานที่ต้องรอ I/O มากกว่างานคำนวณหนักบน CPU เพียงอย่างเดียว

## Completion Checklist

- [ ] เข้าใจ Synchronous
- [ ] เข้าใจ Asynchronous
- [ ] อธิบายความแตกต่างระหว่าง Sync และ Async ได้
- [ ] ประกาศ `async fn` ได้
- [ ] เข้าใจว่า Async Function คืนค่า Future
- [ ] ใช้ `.await` ได้
- [ ] เข้าใจ `Poll::Pending` และ `Poll::Ready`
- [ ] ใช้ Async Delay ด้วย Tokio ได้
