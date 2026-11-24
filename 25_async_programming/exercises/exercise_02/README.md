# Exercise 02: Tokio Practice

## ระดับ

Easy-Medium

## Objective

ฝึกสร้างและจัดการ Async Tasks ด้วย Tokio รวมถึงการรอผลลัพธ์จากหลาย Tasks

## Problem Statement

สร้างโปรแกรมจำลองระบบทำงานของ Worker 3 ตัว

Worker แต่ละตัวมีหน้าที่ประมวลผลข้อมูลของตัวเอง และส่งผลลัพธ์กลับมายัง Main Task

กำหนดข้อมูล:

| Worker   | Input |
| -------- | ----: |
| Worker 1 |    10 |
| Worker 2 |    20 |
| Worker 3 |    30 |

แต่ละ Worker ต้องคำนวณค่า Input คูณ 2

## Requirements

1. สร้าง Async Function สำหรับประมวลผลข้อมูล
2. สร้าง Task อย่างน้อย 3 ตัวด้วย `tokio::spawn()`
3. ส่ง Input เข้าแต่ละ Task
4. แต่ละ Task คำนวณ Input คูณ 2
5. ใช้ Tokio Delay จำลองเวลาประมวลผล
6. ใช้ JoinHandle รอผลลัพธ์จากทุก Task
7. แสดงผลลัพธ์ของ Worker แต่ละตัว
8. แสดงข้อความเมื่อทุก Task ทำงานเสร็จ

## Constraints

- ต้องใช้ Tokio
- ต้องใช้ `tokio::spawn()`
- ต้องใช้ JoinHandle
- ต้อง Await ทุก Task ที่ต้องการรับผลลัพธ์
- ต้องไม่ใช้ `std::thread::sleep()`
- ต้องไม่มีการใช้ `unsafe`
- ต้องจัดการ JoinError อย่างเหมาะสม

## Expected Output

```text
Starting workers...

Worker 1 result: 20
Worker 2 result: 40
Worker 3 result: 60

All workers completed
```

ลำดับข้อความจาก Worker อาจแตกต่างกันได้

## Hints

- สร้าง Async Function เช่น `process_data(value: i32) -> i32`
- ใช้ `tokio::spawn(async move { ... })`
- เก็บ JoinHandle ทั้งหมดไว้ใน Vec
- วน Await JoinHandle เพื่อรับผลลัพธ์
- ใช้ `match` จัดการผลลัพธ์จาก JoinHandle

## Completion Checklist

- [ ] สร้าง Async Function สำหรับประมวลผล
- [ ] สร้าง Task อย่างน้อย 3 ตัว
- [ ] ใช้ `tokio::spawn()`
- [ ] ส่งข้อมูลเข้าแต่ละ Task ได้
- [ ] คำนวณผลลัพธ์ถูกต้อง
- [ ] ใช้ Async Delay
- [ ] Await ทุก JoinHandle
- [ ] จัดการ JoinError
- [ ] แสดงผลลัพธ์ครบทุก Worker

## Run

```powershell
cd 25_async_programming/exercises/exercise_02/tokio_practice
cargo add tokio --features full
cargo run
```
