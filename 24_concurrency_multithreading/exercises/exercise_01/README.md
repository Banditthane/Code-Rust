# Exercise 01: Thread Practice

## ระดับ

Easy

## Objective

ฝึกสร้าง Thread หลายตัวด้วย `thread::spawn()` และรอให้ทุก Thread ทำงานเสร็จด้วย `.join()`

## Problem Statement

สร้างโปรแกรมที่จำลองการทำงานของ Worker จำนวน 3 ตัว

Worker แต่ละตัวต้องทำงานแยกกันและแสดงหมายเลขของตัวเอง

ตัวอย่าง:

```text
Worker 1 started
Worker 2 started
Worker 3 started

Worker 1 finished
Worker 2 finished
Worker 3 finished

All workers completed
```

หมายเหตุ: ลำดับการแสดงผลจริงอาจแตกต่างกันได้

## Requirements

1. สร้าง Thread อย่างน้อย 3 ตัว
2. Worker แต่ละตัวต้องมีหมายเลขประจำตัว
3. ใช้ `thread::spawn()` สร้าง Thread
4. ใช้ `move` Closure เมื่อจำเป็น
5. เก็บ JoinHandle ของทุก Thread
6. ใช้ `.join()` รอให้ทุก Thread ทำงานเสร็จ
7. แสดงข้อความเมื่อ Worker ทำงานเสร็จ
8. แสดงข้อความเมื่อ Worker ทั้งหมดทำงานเสร็จแล้ว

## Constraints

- ต้องสร้าง Thread อย่างน้อย 3 ตัว
- ห้ามใช้ `async` หรือ Tokio
- ต้องใช้ `JoinHandle`
- ต้องรอทุก Thread ด้วย `.join()`
- ไม่กำหนดให้ผลลัพธ์ต้องแสดงตามลำดับ Worker
- ไม่ควรใช้ `sleep()` แทนการรอด้วย `.join()`

## Expected Output

```text
Worker 1 started
Worker 2 started
Worker 3 started
Worker 1 finished
Worker 2 finished
Worker 3 finished
All workers completed
```

ลำดับจริงสามารถแตกต่างได้

## Hints

- ใช้ `Vec` เก็บ JoinHandle
- ใช้ `for` Loop สร้าง Thread
- ใช้ `move` เพื่อส่งหมายเลข Worker เข้า Closure
- วนเรียก `.join()` กับ Handle ทุกตัว

## Completion Checklist

- [ ] สร้าง Thread ได้อย่างน้อย 3 ตัว
- [ ] แต่ละ Thread มีหมายเลขของตัวเอง
- [ ] ใช้ `thread::spawn()`
- [ ] ใช้ `move` Closure
- [ ] เก็บ JoinHandle ใน Vec
- [ ] เรียก `.join()` ครบทุก Thread
- [ ] แสดงข้อความเมื่อทำงานเสร็จ
- [ ] โปรแกรมทำงานได้โดยไม่มี Panic ที่ไม่ได้ตั้งใจ

## Run

```powershell
cd 24_concurrency_multithreading/exercises/exercise_01/thread_practice
cargo run
```
