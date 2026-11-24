# Exercise 03: Shared State Practice

## ระดับ

Medium

## Objective

ฝึกใช้ `Arc<Mutex<T>>` เพื่อแบ่งปันและแก้ไขข้อมูลร่วมกันระหว่างหลาย Thread

## Problem Statement

สร้างโปรแกรมนับจำนวนงานที่ Worker ทำสำเร็จ

กำหนดให้มี Worker จำนวน 5 ตัว

Worker แต่ละตัวต้องเพิ่มค่า Counter ขึ้น 1 เมื่อทำงานเสร็จ

เมื่อ Worker ทั้งหมดทำงานเสร็จ ต้องแสดงจำนวนงานทั้งหมด

## Requirements

1. สร้าง Counter เริ่มต้นด้วยค่า 0
2. ใช้ `Arc<Mutex<usize>>` สำหรับ Counter
3. สร้าง Worker อย่างน้อย 5 Thread
4. Clone Arc เพื่อส่งข้อมูลให้ Worker แต่ละตัว
5. ใช้ `.lock()` เพื่อเข้าถึง Counter
6. เพิ่มค่า Counter ภายใน Lock
7. เก็บ JoinHandle ของทุก Thread
8. ใช้ `.join()` รอ Worker ทุกตัว
9. แสดงค่า Counter สุดท้าย

## Constraints

- ต้องใช้ `Arc`
- ต้องใช้ `Mutex`
- ต้องใช้ `Arc<Mutex<usize>>`
- ห้ามใช้ `static mut`
- ห้ามใช้ Global Mutable Variable
- ต้องเรียก `.join()` ให้ครบทุก Thread
- ต้องปลด Lock เมื่อไม่จำเป็นต้องใช้แล้ว
- ต้องจัดการผลลัพธ์จาก `.lock()` และ `.join()` อย่างเหมาะสม

## Expected Output

```text
Initial Counter: 0
Worker 1 completed
Worker 2 completed
Worker 3 completed
Worker 4 completed
Worker 5 completed
Final Counter: 5
```

ลำดับข้อความ Worker อาจแตกต่างกัน แต่ Counter สุดท้ายต้องเท่ากับ 5

## Hints

- สร้าง `Arc::new(Mutex::new(0))`
- ใช้ `Arc::clone()` ก่อนสร้างแต่ละ Thread
- ใช้ `let mut counter = shared_counter.lock()` เพื่อขอ Lock
- ใช้ Dereference เพื่อเพิ่มค่าภายใน Mutex
- รอทุก Thread ก่อนอ่านค่า Counter สุดท้าย

## Completion Checklist

- [ ] สร้าง Counter เริ่มต้นเป็น 0
- [ ] ใช้ `Arc<Mutex<usize>>`
- [ ] สร้าง Worker 5 Thread
- [ ] Clone Arc ให้ Worker
- [ ] ใช้ Lock ก่อนแก้ไข Counter
- [ ] เพิ่ม Counter ได้ถูกต้อง
- [ ] รอ Worker ทุกตัวด้วย `.join()`
- [ ] Counter สุดท้ายเท่ากับ 5

## Run

```powershell
cd 24_concurrency_multithreading/exercises/exercise_03/shared_state_practice
cargo run
```
