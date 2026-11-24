# Exercise 04: Concurrency Challenge

## ระดับ

Challenge

## Objective

ประยุกต์ใช้ Thread, Channel, Arc, Mutex และ Atomic เพื่อสร้างระบบประมวลผลข้อมูลแบบหลาย Thread

## Problem Statement

สร้างโปรแกรมจำลองระบบประมวลผลคำสั่งซื้อสินค้า

ระบบต้องมี Worker หลายตัวรับผิดชอบการประมวลผลคำสั่งซื้อ โดย Thread หลักส่งงานผ่าน Channel และ Worker แต่ละตัวแจ้งผลการทำงานกลับมา

กำหนดให้มีคำสั่งซื้อ 10 รายการ

แต่ละรายการมีข้อมูล:

- Order ID
- Product Name
- Quantity

ตัวอย่างข้อมูล:

| Order ID | Product      | Quantity |
| -------- | ------------ | -------: |
| 1        | Keyboard     |        2 |
| 2        | Mouse        |        3 |
| 3        | Monitor      |        1 |
| 4        | USB Cable    |        5 |
| 5        | Headset      |        2 |
| 6        | Webcam       |        1 |
| 7        | Microphone   |        2 |
| 8        | SSD          |        1 |
| 9        | RAM          |        4 |
| 10       | Laptop Stand |        2 |

## Requirements

### Part 1: Data Model

1. สร้าง Struct สำหรับ Order
2. กำหนด Field สำหรับ Order ID, Product Name และ Quantity
3. เตรียมข้อมูลคำสั่งซื้ออย่างน้อย 10 รายการ

### Part 2: Worker Threads

1. สร้าง Worker อย่างน้อย 3 Thread
2. Worker แต่ละตัวต้องสามารถรับงานได้
3. ใช้ `thread::spawn()` สร้าง Worker
4. ใช้ `move` เพื่อส่งข้อมูลที่จำเป็นเข้า Thread

### Part 3: Message Passing

1. ใช้ Channel ส่ง Order จาก Thread หลักไปยัง Worker
2. ใช้ Sender สำหรับส่งข้อมูล
3. Worker ต้องรับข้อมูลจาก Receiver
4. Worker ต้องส่งผลการประมวลผลกลับมายัง Thread หลัก
5. Thread หลักต้องรับผลลัพธ์จาก Worker

### Part 4: Shared State

1. ใช้ `Arc<Mutex<usize>>` สำหรับเก็บจำนวน Order ที่ประมวลผลสำเร็จ
2. Worker เพิ่ม Counter เมื่อประมวลผลสำเร็จ
3. ต้อง Lock ก่อนแก้ไขข้อมูล
4. ต้องปลด Lock เมื่อไม่จำเป็นต้องใช้แล้ว

### Part 5: Atomic Counter

1. เพิ่ม Counter อีกตัวโดยใช้ `AtomicUsize`
2. ใช้ Counter นี้นับจำนวนงานที่ Worker เริ่มประมวลผล
3. ใช้ Atomic Operation สำหรับเพิ่มและอ่านค่า
4. เลือก Ordering ให้เหมาะสมกับการใช้งาน

### Part 6: Result Summary

เมื่อประมวลผลเสร็จทั้งหมด ให้แสดง:

- จำนวน Order ทั้งหมด
- จำนวน Order ที่เริ่มประมวลผล
- จำนวน Order ที่ประมวลผลสำเร็จ
- รายการผลลัพธ์ที่ได้รับจาก Worker

## Constraints

- ต้องมี Worker อย่างน้อย 3 Thread
- ต้องใช้ `std::thread`
- ต้องใช้ `std::sync::mpsc`
- ต้องใช้ `Arc<Mutex<T>>`
- ต้องใช้ `AtomicUsize`
- ต้องใช้ `.join()` รอ Worker ทุกตัว
- ต้องจัดการ Error จาก Channel, Mutex และ Thread
- ห้ามใช้ `static mut`
- ห้ามใช้ Global Mutable Variable
- ห้ามใช้ `unsafe`
- ต้องไม่ทำให้ข้อมูล Order สูญหาย
- ต้องไม่มีการนับ Order ซ้ำโดยไม่ได้ตั้งใจ

## Expected Output

ตัวอย่างผลลัพธ์:

```text
Order Processing Started

Worker 1 processing Order 1
Worker 2 processing Order 2
Worker 3 processing Order 3

...

Processing Summary
Total Orders: 10
Started Orders: 10
Completed Orders: 10

All orders processed successfully
```

ลำดับการทำงานและข้อความจาก Worker อาจแตกต่างกันได้

## Hints

- ใช้ `mpsc::channel()` สำหรับส่ง Order
- สามารถ Clone Sender เพื่อส่งข้อมูลจากหลายแหล่งได้
- พิจารณาการกระจายงานให้ Worker หลายตัวรับจาก Receiver เดียวกัน โดยใช้ Receiver ที่ถูกแชร์ผ่าน Mutex หรือออกแบบการส่งงานแยก Channel ให้แต่ละ Worker
- ใช้ Channel อีกชุดสำหรับส่งผลลัพธ์กลับ Thread หลัก
- ใช้ `Arc::clone()` สำหรับข้อมูลที่ต้องแชร์
- ใช้ `AtomicUsize::fetch_add()` สำหรับ Counter
- ใช้ `.join()` ก่อนสรุปผล
- แยกส่วนสร้างงาน ส่วนประมวลผล และส่วนสรุปผลให้ชัดเจน

## Completion Checklist

### Data Model

- [ ] สร้าง Order Struct
- [ ] เตรียมข้อมูล Order อย่างน้อย 10 รายการ

### Thread

- [ ] สร้าง Worker อย่างน้อย 3 Thread
- [ ] ใช้ `thread::spawn()`
- [ ] ใช้ `move` Closure
- [ ] เก็บ JoinHandle

### Message Passing

- [ ] ส่ง Order ผ่าน Channel
- [ ] Worker รับ Order ได้
- [ ] Worker ส่งผลลัพธ์กลับได้
- [ ] Thread หลักรับผลลัพธ์ครบ

### Shared State

- [ ] ใช้ `Arc<Mutex<usize>>`
- [ ] นับจำนวน Order ที่สำเร็จได้ถูกต้อง

### Atomic

- [ ] ใช้ `AtomicUsize`
- [ ] นับจำนวน Order ที่เริ่มทำงานได้ถูกต้อง

### Final Result

- [ ] แสดงจำนวน Order ทั้งหมด
- [ ] แสดงจำนวน Order ที่เริ่มประมวลผล
- [ ] แสดงจำนวน Order ที่สำเร็จ
- [ ] รอ Worker ทุกตัวด้วย `.join()`
- [ ] จัดการ Error อย่างเหมาะสม
- [ ] โปรแกรมทำงานครบโดยไม่มี `unsafe`

## Run

```powershell
cd 24_concurrency_multithreading/exercises/exercise_04/concurrency_challenge
cargo run
```

## Final Challenge

เมื่อทำโจทย์สำเร็จแล้ว ให้ลองปรับระบบเพิ่มเติม:

1. เพิ่มสถานะ Order เช่น Pending, Processing และ Completed
2. เพิ่มการตรวจสอบ Order ที่มี Quantity เป็น 0
3. เพิ่มระบบ Retry เมื่อ Worker ประมวลผลไม่สำเร็จ
4. ทดลองเปรียบเทียบการใช้ Mutex กับ Atomic
5. ทดลองเพิ่มจำนวน Worker และสังเกตผลการทำงาน

**หมายเหตุ:** โจทย์นี้เน้นการฝึกออกแบบ Concurrency ไม่ได้กำหนดให้ต้องมีลำดับการประมวลผลตายตัว
