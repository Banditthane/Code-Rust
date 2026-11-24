# Exercise 02: Channel Practice

## ระดับ

Easy-Medium

## Objective

ฝึกใช้ Channel เพื่อส่งข้อมูลจากหลาย Thread ไปยัง Thread หลัก

## Problem Statement

สร้างโปรแกรมจำลองระบบรับข้อมูลจาก Worker 3 ตัว

Worker แต่ละตัวส่งข้อความผ่าน Channel กลับมายัง Thread หลัก

กำหนดให้ Worker ส่งข้อความดังนี้:

| Worker   | Message          |
| -------- | ---------------- |
| Worker 1 | Task A completed |
| Worker 2 | Task B completed |
| Worker 3 | Task C completed |

Thread หลักต้องรับข้อความทั้งหมดและแสดงผล

## Requirements

1. สร้าง Channel ด้วย `mpsc::channel()`
2. สร้าง Worker อย่างน้อย 3 Thread
3. Clone Sender เพื่อให้แต่ละ Worker ส่งข้อมูลได้
4. ใช้ `move` Closure
5. Worker แต่ละตัวส่งข้อความของตัวเองด้วย `.send()`
6. Thread หลักรับข้อมูลด้วย `.recv()` หรือวนอ่าน Receiver
7. แสดงข้อความที่ได้รับ
8. จัดการการปิด Channel ให้ถูกต้อง

## Constraints

- ต้องใช้ `std::sync::mpsc`
- ต้องมี Sender อย่างน้อย 3 ตัวที่มาจากการ Clone
- ต้องมี Receiver เพียงตัวเดียว
- ห้ามใช้ Global Mutable Variable
- ต้องไม่ปล่อย Sender ตัวหลักค้างไว้ หากต้องการให้ Receiver จบการวนอ่าน
- ต้องจัดการ Error จากการส่งหรือรับข้อมูลอย่างเหมาะสม

## Expected Output

```text
Received: Task A completed
Received: Task B completed
Received: Task C completed
All messages received
```

ลำดับของข้อความอาจแตกต่างกัน

## Hints

- สร้าง `(tx, rx)` จาก `mpsc::channel()`
- ใช้ `tx.clone()` ก่อนสร้าง Worker แต่ละตัว
- ส่ง String หรือข้อมูลที่เป็นเจ้าของค่าได้
- ใช้ `drop(tx)` เมื่อไม่ต้องการให้ Sender ตัวหลักค้างอยู่
- ใช้ `for message in rx` เพื่อรับข้อมูลจนกว่า Channel จะปิด

## Completion Checklist

- [ ] สร้าง Channel ได้
- [ ] สร้าง Worker 3 Thread
- [ ] Clone Sender ได้
- [ ] ส่งข้อความจากแต่ละ Worker ได้
- [ ] รับข้อความจาก Receiver ได้
- [ ] รับข้อความครบทุก Worker
- [ ] จัดการ Sender และ Receiver ได้ถูกต้อง
- [ ] โปรแกรมไม่รอรับข้อความอย่างไม่สิ้นสุด

## Run

```powershell
cd 24_concurrency_multithreading/exercises/exercise_02/channel_practice
cargo run
```
