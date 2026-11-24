# 03_shared_state

# Lesson 03: Shared-State Concurrency

## 1. Shared-State Concurrency คืออะไร?

Shared-State Concurrency คือการให้หลาย Thread เข้าถึงข้อมูลร่วมกัน

ตัวอย่าง:

- หลาย Thread เพิ่มยอดผู้ใช้งาน
- หลาย Thread ปรับปรุงยอดสินค้า
- หลาย Thread บันทึกจำนวนงานที่ประมวลผลแล้ว

การเข้าถึงข้อมูลร่วมกันต้องมีการควบคุม เพื่อไม่ให้หลาย Thread แก้ไขข้อมูลพร้อมกันจนเกิดปัญหา

## 2. Mutex คืออะไร?

`Mutex<T>` ย่อมาจาก Mutual Exclusion

เป็นเครื่องมือที่อนุญาตให้มี Thread เข้าถึงข้อมูลที่ถูกล็อกได้ทีละหนึ่ง Thread ในช่วงเวลาที่ถือ Lock

นำเข้า:

```rust
use std::sync::Mutex;
```

ตัวอย่าง:

```rust
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    {
        let mut value = counter.lock().unwrap();
        *value += 1;
    }

    println!("Counter: {:?}", counter);
}
```

คำอธิบาย:

- `Mutex::new(0)` สร้าง Mutex ที่เก็บค่า 0
- `.lock()` ขอสิทธิ์เข้าถึงข้อมูล
- `MutexGuard` ที่ได้จาก `.lock()` ใช้เข้าถึงข้อมูลภายใน
- `*value` ใช้ Dereference เพื่อแก้ไขค่าภายใน
- เมื่อ Guard ถูก Drop จะปลดล็อก Mutex

## 3. Arc คืออะไร?

`Arc<T>` ย่อมาจาก Atomically Reference Counted

ใช้แบ่งปัน Ownership ของข้อมูลระหว่าง Thread

นำเข้า:

```rust
use std::sync::Arc;
```

ตัวอย่าง:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let value = Arc::new(100);

    let shared_value = Arc::clone(&value);

    let handle = thread::spawn(move || {
        println!("Value: {}", shared_value);
    });

    handle.join().unwrap();

    println!("Main: {}", value);
}
```

`Arc` ช่วยให้หลาย Thread เป็นเจ้าของข้อมูลเดียวกันได้อย่างปลอดภัยในแง่ของ Reference Counting

แต่ `Arc` เพียงอย่างเดียวไม่ได้ทำให้ข้อมูลภายในสามารถแก้ไขร่วมกันได้

## 4. Arc<Mutex<T>>

เมื่อต้องการแบ่งปัน Ownership และแก้ไขข้อมูลร่วมกัน สามารถใช้ `Arc<Mutex<T>>`

ตัวอย่าง:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            *value += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}
```

ผลลัพธ์:

```text
Counter: 5
```

อธิบาย:

- `Arc` แบ่งปัน Ownership ระหว่าง Thread
- `Mutex` ควบคุมการเข้าถึงข้อมูล
- `lock()` ขอสิทธิ์แก้ไขข้อมูล
- `join()` รอให้ทุก Thread ทำงานเสร็จ

## 5. MutexGuard

ค่าที่ได้จาก `Mutex::lock()` คือ `LockResult<MutexGuard<T>>`

`MutexGuard<T>` เป็นตัวแทนของสิทธิ์การเข้าถึงข้อมูลภายใน Mutex

เมื่อ Guard ถูก Drop ระบบจะปลดล็อก Mutex

ตัวอย่าง:

```rust
let mut value = counter.lock().unwrap();
*value += 1;
```

หากต้องการปลดล็อกก่อนจบ Scope สามารถใช้ `drop(value)` ได้

```rust
let mut value = counter.lock().unwrap();
*value += 1;

drop(value);
```

ควรถือ Lock เฉพาะช่วงเวลาที่จำเป็น

## 6. Poisoned Mutex

หาก Thread เกิด Panic ขณะที่ถือ Mutex อยู่ Mutex อาจถูกทำเครื่องหมายว่า Poisoned

การเรียก `.lock()` ในภายหลังจึงอาจได้ `Err(PoisonError)`

การใช้ `.unwrap()` จะทำให้เกิด Panic เมื่อ Mutex ถูก Poisoned

โปรแกรมสามารถจัดการ Error นี้อย่างเหมาะสมตามเงื่อนไขของงานได้

## 7. Deadlock คืออะไร?

Deadlock คือสถานการณ์ที่ Thread หลายตัวรอทรัพยากรซึ่งกันและกัน จนไม่มี Thread ใดสามารถทำงานต่อได้

ตัวอย่างแนวคิด:

```text
Thread A
  Lock A
  รอ Lock B

Thread B
  Lock B
  รอ Lock A
```

แนวทางป้องกัน:

- กำหนดลำดับการ Lock ให้เหมือนกัน
- หลีกเลี่ยงการถือ Lock หลายตัวพร้อมกันโดยไม่จำเป็น
- ปลด Lock ให้เร็วที่สุด
- หลีกเลี่ยงการเรียกงานที่ใช้เวลานานขณะถือ Lock

## 8. เปรียบเทียบ Arc และ Mutex

| เครื่องมือ      | จุดประสงค์                              |
| --------------- | --------------------------------------- |
| `Arc<T>`        | แบ่งปัน Ownership ระหว่าง Thread        |
| `Mutex<T>`      | ควบคุมการเข้าถึงข้อมูลแบบ Exclusive     |
| `Arc<Mutex<T>>` | แบ่งปัน Ownership และแก้ไขข้อมูลร่วมกัน |

## 9. ข้อควรระวัง

- `Arc` ไม่ได้ทำให้ข้อมูลทุกชนิดแก้ไขร่วมกันได้
- `Mutex` ต้อง Lock ก่อนเข้าถึงข้อมูล
- อย่าถือ Lock นานเกินจำเป็น
- ระวัง Deadlock
- ระวัง Poisoned Mutex
- หลีกเลี่ยงการใช้ `.unwrap()` กับงานจริงโดยไม่จัดการ Error

## Completion Checklist

- [ ] เข้าใจ Shared-State Concurrency
- [ ] เข้าใจ Mutex
- [ ] ใช้ `.lock()` ได้
- [ ] เข้าใจ MutexGuard
- [ ] เข้าใจ Arc
- [ ] ใช้ `Arc<Mutex<T>>` ได้
- [ ] เข้าใจ Poisoned Mutex
- [ ] เข้าใจ Deadlock และวิธีป้องกัน
