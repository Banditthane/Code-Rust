# Lesson 01: Thread Basics

## 1. Thread คืออะไร?

Thread คือหน่วยการทำงานภายใน Process ที่สามารถทำงานพร้อมกับ Thread อื่นได้

ตัวอย่าง:

- Thread หนึ่งอ่านข้อมูลจากไฟล์
- Thread หนึ่งประมวลผลข้อมูล
- Thread หนึ่งแสดงผลให้ผู้ใช้

การใช้หลาย Thread ช่วยให้โปรแกรมทำงานหลายส่วนได้พร้อมกัน แต่ต้องจัดการการเข้าถึงข้อมูลและ Ownership ให้ถูกต้อง

## 2. สร้าง Thread ด้วย thread::spawn()

Rust ใช้ `std::thread::spawn()` เพื่อสร้าง Thread ใหม่

รูปแบบ:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
    });

    println!("Hello from main thread!");

    handle.join().unwrap();
}
```

คำอธิบาย:

- `thread::spawn()` สร้าง Thread ใหม่
- `||` คือ Closure ที่ใช้เป็นงานของ Thread
- `handle` คือ JoinHandle ที่ใช้จัดการ Thread
- `.join()` ใช้รอให้ Thread ทำงานเสร็จ
- `.unwrap()` ดึงผลลัพธ์ หาก Thread ทำงานผิดพลาดจะเกิด Panic

ลำดับข้อความที่แสดงอาจแตกต่างกัน เพราะทั้งสอง Thread ทำงานพร้อมกัน

## 3. JoinHandle

`thread::spawn()` คืนค่าเป็น `JoinHandle<T>`

ใช้ `.join()` เพื่อรอให้ Thread ทำงานเสร็จ และรับค่าที่ Thread ส่งกลับมา

ตัวอย่าง:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        10 + 20
    });

    let result = handle.join().unwrap();

    println!("Result: {}", result);
}
```

ผลลัพธ์:

```text
Result: 30
```

ข้อควรจำ:

- `.join()` เป็นการรอ Thread ให้เสร็จ
- `.join()` ใช้กับ JoinHandle และใช้สิทธิ์ Ownership ของ Handle
- ผลลัพธ์จาก `.join()` เป็น `Result<T, Box<dyn Any + Send>>`
- หาก Thread เกิด Panic การเรียก `.join()` จะได้ `Err`

## 4. Move Closure

Thread ใหม่อาจทำงานนานกว่าขอบเขต Scope ของตัวแปรใน Thread หลัก

Rust จึงต้องแน่ใจว่าข้อมูลที่ Closure ใช้จะยังมีอายุการใช้งานเพียงพอ

สามารถใช้ `move` เพื่อย้าย Ownership ของข้อมูลเข้า Closure ได้

ตัวอย่าง:

```rust
use std::thread;

fn main() {
    let message = String::from("Hello Rust");

    let handle = thread::spawn(move || {
        println!("{}", message);
    });

    handle.join().unwrap();
}
```

`move` ทำให้ Closure รับ Ownership ของ `message`

หลังจากย้าย Ownership แล้ว ไม่สามารถนำตัวแปรเดิมมาใช้งานต่อได้ตามปกติ

## 5. Thread Lifecycle

วงจรการทำงานทั่วไป:

1. สร้าง Thread ด้วย `thread::spawn()`
2. Thread เริ่มทำงาน
3. Thread ทำงานตาม Closure
4. Thread ทำงานเสร็จหรือเกิด Panic
5. Thread หลักสามารถรอผลด้วย `.join()`

หากไม่เก็บ JoinHandle หรือไม่เรียก `.join()` Thread หลักอาจจบก่อน Thread ที่สร้างขึ้นทำงานเสร็จ

## 6. Thread Sleep

ใช้ `thread::sleep()` เพื่อหยุดการทำงานของ Thread ชั่วคราว

ตัวอย่าง:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    for i in 1..=3 {
        thread::spawn(move || {
            println!("Thread {}", i);
            thread::sleep(Duration::from_secs(1));
        });
    }

    thread::sleep(Duration::from_secs(2));
}
```

หมายเหตุ: การใช้ `sleep()` ไม่ใช่วิธีรับประกันว่า Thread ทำงานเสร็จ ควรใช้ `.join()` เมื่อต้องการรอผลอย่างชัดเจน

## 7. ข้อควรระวัง

- Closure ที่ส่งให้ `thread::spawn()` ต้องเป็น `Send` และมี Lifetime แบบ `'static`
- หากต้องใช้ข้อมูลจาก Thread หลัก ต้องพิจารณา Ownership
- ลำดับการทำงานของหลาย Thread ไม่ได้รับประกัน
- Thread อาจเกิด Panic แยกจาก Thread หลักได้

## 8. สรุป

| คำสั่ง            | หน้าที่                     |
| ----------------- | --------------------------- |
| `thread::spawn()` | สร้าง Thread                |
| `JoinHandle<T>`   | Handle สำหรับจัดการ Thread  |
| `.join()`         | รอ Thread และรับผลลัพธ์     |
| `move`            | ย้าย Ownership เข้า Closure |
| `thread::sleep()` | หยุด Thread ชั่วคราว        |

## Completion Checklist

- [ ] เข้าใจ Thread คืออะไร
- [ ] สร้าง Thread ด้วย `thread::spawn()` ได้
- [ ] เข้าใจ JoinHandle
- [ ] ใช้ `.join()` ได้
- [ ] เข้าใจ `move` Closure
- [ ] เข้าใจว่า Thread อาจทำงานไม่เรียงลำดับกัน
