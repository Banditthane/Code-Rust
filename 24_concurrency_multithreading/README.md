# Chapter 24: Concurrency & Multithreading

## เป้าหมายของบทเรียน

เรียนรู้การทำงานหลาย Thread พร้อมกันในภาษา Rust เข้าใจการสื่อสารระหว่าง Thread การแบ่งปันข้อมูล และกลไกที่ Rust ใช้ป้องกันปัญหาจากการทำงานพร้อมกัน

เมื่อเรียนจบบทนี้ ควรสามารถ:

- เข้าใจความหมายของ Concurrency และ Multithreading
- สร้าง Thread ด้วย `thread::spawn()`
- รอผลการทำงานด้วย `JoinHandle` และ `.join()`
- ส่งข้อมูลระหว่าง Thread ด้วย Channel
- ใช้ `Mutex<T>` เพื่อควบคุมการเข้าถึงข้อมูลร่วมกัน
- ใช้ `Arc<T>` เพื่อแบ่งปัน Ownership ระหว่าง Thread
- เข้าใจ `Send` และ `Sync`
- อธิบาย Race Condition และ Data Race ได้
- เลือกใช้เครื่องมือสำหรับจัดการข้อมูลร่วมกันได้เหมาะสม

## บทเรียน

| Lesson | หัวข้อ                   | เนื้อหา                              |
| ------ | ------------------------ | ------------------------------------ |
| 01     | Thread Basics            | Thread, spawn, join, move, lifecycle |
| 02     | Message Passing          | Channel, mpsc, Sender, Receiver      |
| 03     | Shared-State Concurrency | Mutex, Arc, Lock, Deadlock           |
| 04     | Concurrency Safety       | Send, Sync, Race Condition, Atomic   |

## แบบฝึกหัด

| Exercise | ชื่อ                  | ระดับ       |
| -------- | --------------------- | ----------- |
| 01       | Thread Practice       | Easy        |
| 02       | Channel Practice      | Easy-Medium |
| 03       | Shared State Practice | Medium      |
| 04       | Concurrency Challenge | Challenge   |

## โครงสร้างโฟลเดอร์

```text
24_concurrency_multithreading/
├── README.md
├── 01_thread_basics/
│   └── README.md
├── 02_message_passing/
│   └── README.md
├── 03_shared_state/
│   └── README.md
├── 04_concurrency_safety/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── thread_practice/
    ├── exercise_02/
    │   ├── README.md
    │   └── channel_practice/
    ├── exercise_03/
    │   ├── README.md
    │   └── shared_state_practice/
    └── exercise_04/
        ├── README.md
        └── concurrency_challenge/
```

## คำสั่งที่ใช้บ่อย

```powershell
cargo new thread_practice
cargo run
cargo check
cargo test
```

## แนวทางการเรียน

1. เรียนรู้การสร้างและจัดการ Thread
2. ฝึกส่งข้อมูลระหว่าง Thread ด้วย Channel
3. ฝึกแบ่งปันข้อมูลด้วย Mutex และ Arc
4. ทำความเข้าใจ Send, Sync และปัญหาที่เกิดจาก Concurrency
5. ทำแบบฝึกหัดตามลำดับ

## สิ่งที่ควรระวัง

- การทำงานพร้อมกันอาจทำให้ลำดับการแสดงผลไม่แน่นอน
- ต้องจัดการ Ownership และ Lifetime ของข้อมูลที่ส่งเข้า Thread
- ต้องระวังการล็อก Mutex ค้างไว้นานเกินไป
- หลีกเลี่ยง Deadlock ด้วยการออกแบบลำดับการล็อกข้อมูล
- อย่าใช้ `unwrap()` กับงานจริงโดยไม่พิจารณาความผิดพลาดที่อาจเกิดขึ้น

## Completion Checklist

- [ ] เข้าใจ Thread และ Concurrency
- [ ] ใช้ `thread::spawn()` ได้
- [ ] ใช้ `.join()` ได้
- [ ] ใช้ Channel ส่งข้อมูลระหว่าง Thread ได้
- [ ] ใช้ `Arc<Mutex<T>>` ได้
- [ ] เข้าใจ `Send` และ `Sync`
- [ ] เข้าใจ Race Condition และ Deadlock
- [ ] ทำแบบฝึกหัดทั้ง 4 ข้อครบถ้วน

## เกณฑ์ผ่านบทเรียน

สามารถเขียนโปรแกรมที่ใช้หลาย Thread ทำงานร่วมกัน ส่งข้อมูลระหว่าง Thread และจัดการข้อมูลร่วมกันได้อย่างปลอดภัย
