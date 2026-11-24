# Lesson 03: Async Tasks

## 1. Async Task คืออะไร?

Async Task คือหน่วยงานที่ Runtime จัดการและขับเคลื่อน Future

Task สามารถพักการทำงานระหว่างรอ I/O แล้วเปิดโอกาสให้ Runtime จัดการ Task อื่นได้

## 2. tokio::spawn()

ใช้ `tokio::spawn()` เพื่อสร้าง Task ใหม่ใน Tokio Runtime

ตัวอย่าง:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task completed");
    });

    handle.await.unwrap();

    println!("Main completed");
}
```

คำอธิบาย:

- `tokio::spawn()` สร้าง Task ใหม่
- `handle` เป็น JoinHandle
- `handle.await` รอให้ Task ทำงานเสร็จ
- `.unwrap()` ใช้ดึงผลลัพธ์ หาก Task เกิด Panic จะเกิด Panic ตาม

ข้อกำหนดสำคัญ:

Task ที่สร้างด้วย `tokio::spawn()` ต้องมี Future และผลลัพธ์ที่เป็น `Send + 'static` ตามข้อกำหนดของ API

## 3. JoinHandle

`tokio::spawn()` คืนค่าเป็น `tokio::task::JoinHandle<T>`

สามารถ Await เพื่อรับผลลัพธ์จาก Task ได้

ตัวอย่าง:

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        10 + 20
    });

    let result = handle.await.unwrap();

    println!("Result: {}", result);
}
```

ผลลัพธ์:

```text
Result: 30
```

## 4. สร้างหลาย Tasks

ตัวอย่าง:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task_a = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task A");
    });

    let task_b = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task B");
    });

    task_a.await.unwrap();
    task_b.await.unwrap();
}
```

Task สามารถสลับการทำงานได้ตาม Runtime และเหตุการณ์ที่เกิดขึ้น

ลำดับผลลัพธ์อาจไม่เหมือนกันในทุกสถานการณ์

## 5. tokio::join!

ใช้ `tokio::join!` เพื่อรอ Future หลายตัวให้เสร็จ โดย Future เหล่านั้นถูกขับเคลื่อนร่วมกันภายใน Task ปัจจุบัน

ตัวอย่าง:

```rust
async fn task_a() -> i32 {
    10
}

async fn task_b() -> i32 {
    20
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(
        task_a(),
        task_b()
    );

    println!("{} {}", a, b);
}
```

ผลลัพธ์:

```text
10 20
```

ข้อแตกต่าง:

| `tokio::spawn()`                             | `tokio::join!()`                            |
| -------------------------------------------- | ------------------------------------------- |
| สร้าง Task แยกใน Runtime                     | รวม Future หลายตัวใน Task ปัจจุบัน          |
| คืนค่า JoinHandle                            | คืนผลลัพธ์เป็น Tuple                        |
| Task อาจถูกขับเคลื่อนบน Worker Thread อื่น   | ไม่ได้สร้าง Task แยก                        |
| มีข้อกำหนด Send + 'static สำหรับงานที่ Spawn | ไม่ต้องมีข้อกำหนดแบบ Spawn โดยตัว Macro เอง |

`join!` ช่วยให้ Future หลายตัวมีโอกาสทำงานระหว่างรอ แต่ไม่ได้หมายความว่าทุก Future จะทำงานแบบ Parallel บนหลาย CPU Core

## 6. Error Handling ใน Async

Async Function สามารถคืนค่า `Result<T, E>` ได้เหมือน Function ปกติ

ตัวอย่าง:

```rust
async fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Cannot divide by zero"));
    }

    Ok(a / b)
}

#[tokio::main]
async fn main() {
    match divide(10.0, 2.0).await {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
```

สามารถใช้ `?` ภายใน Async Function เพื่อส่งต่อ Error ได้เช่นเดียวกับ Function ปกติ

## 7. JoinError

การ Await JoinHandle อาจได้ Error หาก Task เกิด Panic หรือถูกยกเลิก

ตัวอย่าง:

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        panic!("Task failed");
    });

    match handle.await {
        Ok(_) => println!("Task completed"),
        Err(error) => println!("Task error: {}", error),
    }
}
```

## 8. Blocking และ Async

ไม่ควรเรียกงาน Blocking ที่ใช้เวลานานบน Async Runtime Worker โดยไม่จำเป็น

ตัวอย่างงาน Blocking:

- คำนวณหนักเป็นเวลานาน
- เรียก Function ที่บล็อก Thread
- ใช้ `std::thread::sleep()` ใน Async Task

Tokio มี `spawn_blocking()` สำหรับส่งงาน Blocking ไปยัง Thread Pool ที่ออกแบบมารองรับงานประเภทนี้

## 9. ข้อควรระวัง

- Task ที่ Spawn อาจทำงานต่อหลัง Function หลักดำเนินการต่อไป
- ต้อง Await JoinHandle หากต้องการรอผลลัพธ์
- ต้องจัดการ JoinError
- `join!` ไม่ได้สร้าง Thread เพิ่ม
- อย่าสับสนระหว่าง Concurrent กับ Parallel
- หลีกเลี่ยงงาน Blocking บน Async Runtime Worker

## Completion Checklist

- [ ] เข้าใจ Async Task
- [ ] ใช้ `tokio::spawn()` ได้
- [ ] ใช้ JoinHandle ได้
- [ ] Await Task เพื่อรับผลลัพธ์ได้
- [ ] ใช้ `tokio::join!()` ได้
- [ ] อธิบายความแตกต่างระหว่าง spawn และ join! ได้
- [ ] ใช้ Result ใน Async Function ได้
- [ ] จัดการ JoinError ได้
- [ ] เข้าใจ Blocking และ `spawn_blocking()`
