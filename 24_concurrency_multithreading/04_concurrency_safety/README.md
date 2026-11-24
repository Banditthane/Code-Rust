# Lesson 04: Concurrency Safety

## 1. Concurrency Safety คืออะไร?

Concurrency Safety คือการออกแบบโปรแกรมที่ทำงานพร้อมกันหลาย Thread โดยลดความเสี่ยงจากการเข้าถึงข้อมูลที่ไม่ปลอดภัย

Rust มีระบบ Ownership, Borrowing และ Type System เพื่อช่วยตรวจสอบความปลอดภัยตั้งแต่ Compile Time

แนวคิดสำคัญ:

- Ownership
- Borrowing
- Send
- Sync
- Mutex
- Atomic
- Race Condition

## 2. Send Trait

`Send` เป็น Marker Trait ที่บอกว่าค่าชนิดนั้นสามารถย้าย Ownership ไปยัง Thread อื่นได้อย่างปลอดภัย

ตัวอย่าง:

```rust
use std::thread;

fn main() {
    let message = String::from("Hello");

    let handle = thread::spawn(move || {
        println!("{}", message);
    });

    handle.join().unwrap();
}
```

`String` สามารถย้าย Ownership ไปยัง Thread ใหม่ได้

ชนิดข้อมูลจำนวนมากใน Rust เป็น `Send` โดยอัตโนมัติ แต่บางชนิดไม่สามารถส่งข้าม Thread ได้

## 3. Sync Trait

`Sync` เป็น Marker Trait ที่บอกว่าการอ้างอิงแบบ Immutable Reference (`&T`) สามารถแบ่งปันระหว่าง Thread ได้อย่างปลอดภัย

กฎสำคัญ:

`T: Sync` หมายความว่า `&T: Send`

ตัวอย่าง:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let value = Arc::new(100);

    let shared_value = Arc::clone(&value);

    let handle = thread::spawn(move || {
        println!("{}", shared_value);
    });

    handle.join().unwrap();
}
```

`Arc<T>` สามารถใช้แบ่งปันข้อมูลระหว่าง Thread ได้เมื่อ `T` มีคุณสมบัติเหมาะสม โดยทั่วไป `Arc<T>` จะเป็น `Send` และ `Sync` เมื่อ `T` เป็นทั้ง `Send` และ `Sync`

## 4. ความแตกต่างระหว่าง Send และ Sync

| Trait  | ความหมาย                                                |
| ------ | ------------------------------------------------------- |
| `Send` | ย้าย Ownership ไปยัง Thread อื่นได้อย่างปลอดภัย         |
| `Sync` | แชร์ Immutable Reference ระหว่าง Thread ได้อย่างปลอดภัย |

จำง่าย ๆ:

- Send = ส่งค่าไปอีก Thread
- Sync = แชร์ Reference ให้หลาย Thread

ทั้งสองเป็น Marker Trait ที่ Rust ใช้ตรวจสอบคุณสมบัติของชนิดข้อมูล

## 5. Race Condition คืออะไร?

Race Condition คือสถานการณ์ที่ผลลัพธ์ของโปรแกรมขึ้นอยู่กับลำดับหรือจังหวะเวลาการทำงานของหลาย Thread

ตัวอย่าง:

```text
Thread A อ่าน counter = 0
Thread B อ่าน counter = 0
Thread A เพิ่ม counter เป็น 1
Thread B เพิ่ม counter เป็น 1
```

ผลลัพธ์สุดท้ายอาจเป็น 1 แทนที่จะเป็น 2 หากไม่มีการควบคุมการเข้าถึงข้อมูล

สามารถใช้ Mutex เพื่อควบคุมการแก้ไขข้อมูลร่วมกันได้

## 6. Data Race กับ Race Condition

สองคำนี้เกี่ยวข้องกัน แต่ไม่ใช่ความหมายเดียวกัน

| คำ             | ความหมาย                                                                                          |
| -------------- | ------------------------------------------------------------------------------------------------- |
| Data Race      | การเข้าถึงหน่วยความจำพร้อมกันโดยมีการเขียนอย่างน้อยหนึ่งครั้ง โดยไม่มี Synchronization ที่ถูกต้อง |
| Race Condition | ผลลัพธ์ขึ้นอยู่กับลำดับหรือเวลาการทำงาน                                                           |

Rust Safe Code ป้องกัน Data Race ในรูปแบบปกติผ่านระบบ Type และ Ownership

อย่างไรก็ตาม Race Condition ในระดับตรรกะยังสามารถเกิดขึ้นได้ แม้โปรแกรมจะไม่มี Data Race ก็ตาม

## 7. Atomic Types

Atomic Types เป็นชนิดข้อมูลที่รองรับการดำเนินการแบบ Atomic เพื่อให้หลาย Thread ใช้งานข้อมูลบางประเภทได้โดยไม่ต้องใช้ Mutex

ตัวอย่าง:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let counter = AtomicUsize::new(0);

    counter.fetch_add(1, Ordering::SeqCst);

    println!("{}", counter.load(Ordering::SeqCst));
}
```

ผลลัพธ์:

```text
1
```

คำอธิบาย:

- `AtomicUsize` เก็บค่าจำนวนเต็มแบบ Atomic
- `fetch_add()` เพิ่มค่าและคืนค่าเดิม
- `load()` อ่านค่า
- `Ordering::SeqCst` กำหนดลำดับการมองเห็นของการดำเนินการ Atomic แบบ Sequentially Consistent

Atomic เหมาะกับข้อมูลพื้นฐานบางประเภท เช่น Counter หรือ Flag

ไม่ใช่เครื่องมือทดแทน Mutex สำหรับข้อมูลที่มีโครงสร้างซับซ้อนทุกกรณี

## 8. เครื่องมือสำหรับความปลอดภัยของ Concurrency

| เครื่องมือ  | การใช้งาน                                            |
| ----------- | ---------------------------------------------------- |
| Ownership   | ควบคุมเจ้าของข้อมูล                                  |
| Borrowing   | ควบคุมการอ้างอิงข้อมูล                               |
| `Send`      | ตรวจสอบการย้ายข้อมูลข้าม Thread                      |
| `Sync`      | ตรวจสอบการแชร์ Reference ข้าม Thread                 |
| `Mutex<T>`  | ควบคุมการเข้าถึงข้อมูล                               |
| `RwLock<T>` | รองรับหลาย Reader หรือ Writer หนึ่งรายในเวลาเดียวกัน |
| `Atomic`    | จัดการค่าพื้นฐานแบบ Atomic                           |

## 9. แนวทางเลือกใช้

- ใช้ Channel เมื่อ Thread ต้องการส่งข้อมูลให้กัน
- ใช้ `Arc<T>` เมื่อจำเป็นต้องแบ่งปัน Ownership
- ใช้ `Mutex<T>` เมื่อจำเป็นต้องแก้ไขข้อมูลร่วมกัน
- ใช้ `RwLock<T>` เมื่อต้องการแยกการอ่านและการเขียน
- ใช้ Atomic เมื่อข้อมูลเป็นค่าพื้นฐานและการดำเนินการเหมาะสม
- ใช้ Ownership และ Borrowing เป็นหลักก่อนเลือกเครื่องมือที่ซับซ้อน

## 10. ข้อควรระวัง

- อย่าคิดว่า `Arc<T>` ทำให้ข้อมูลภายในแก้ไขได้อย่างปลอดภัยโดยอัตโนมัติ
- อย่าคิดว่าโปรแกรมที่ Compile ผ่านจะไม่มี Race Condition ทางตรรกะ
- ระวัง Deadlock และ Lock Contention
- เลือก Atomic Ordering ให้เหมาะกับการประสานงานระหว่าง Thread
- หลีกเลี่ยง `unsafe` หากยังสามารถใช้ Safe Rust ได้

## Completion Checklist

- [ ] เข้าใจ Send
- [ ] เข้าใจ Sync
- [ ] อธิบายความแตกต่างระหว่าง Send และ Sync ได้
- [ ] เข้าใจ Race Condition
- [ ] แยก Data Race กับ Race Condition ได้
- [ ] เข้าใจ Atomic Types
- [ ] ใช้ Atomic Counter เบื้องต้นได้
- [ ] เลือกเครื่องมือจัดการ Concurrency ได้เหมาะสม
