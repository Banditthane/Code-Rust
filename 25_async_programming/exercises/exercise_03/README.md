# Exercise 03: Async Tasks

## ระดับ

Medium

## Objective

ฝึกจัดการ Future หลายตัวพร้อมกันด้วย `tokio::join!()` และ `tokio::spawn()` รวมถึงการจัดการ Error ด้วย Result

## Problem Statement

สร้างโปรแกรมจำลองระบบรวบรวมข้อมูลจาก 3 แหล่งข้อมูล:

1. User Service
2. Product Service
3. Order Service

แต่ละ Service ต้องเป็น Async Function และคืนค่าข้อมูลของตัวเอง

จากนั้นนำข้อมูลทั้งหมดมาสรุปผล

## Requirements

### Part 1: Async Services

1. สร้าง Async Function `get_user()`
2. สร้าง Async Function `get_products()`
3. สร้าง Async Function `get_orders()`
4. แต่ละ Function ต้องคืนข้อมูลที่เหมาะสม
5. ใช้ Async Delay จำลองการรอข้อมูล

### Part 2: Concurrent Execution

1. ใช้ `tokio::join!()` เรียกทั้ง 3 Services
2. รอให้ Future ทั้งหมดทำงานเสร็จ
3. รับผลลัพธ์จากทั้ง 3 Services
4. แสดงข้อมูลที่ได้รับ

### Part 3: Spawn Tasks

1. สร้าง Async Task เพิ่มเติมด้วย `tokio::spawn()`
2. ให้ Task ทำหน้าที่สรุปข้อมูล
3. ใช้ JoinHandle รับผลลัพธ์
4. รอ Task ให้เสร็จด้วย `.await`

### Part 4: Error Handling

1. กำหนดให้ Service อย่างน้อยหนึ่งตัวสามารถคืนค่า `Result<T, E>`
2. ทดลองกรณีที่ Service ทำงานสำเร็จ
3. ทดลองกรณีที่ Service ส่ง Error
4. ใช้ `match` หรือ `?` จัดการ Error
5. ต้องไม่ใช้ `unwrap()` กับข้อมูลที่อาจเกิด Error โดยไม่ตรวจสอบ

## Constraints

- ต้องใช้ Tokio
- ต้องใช้ `tokio::join!()`
- ต้องใช้ `tokio::spawn()`
- ต้องใช้ JoinHandle
- ต้องมี Async Function อย่างน้อย 3 Function
- ต้องมี Function ที่คืนค่า `Result<T, E>`
- ต้องจัดการ JoinError และ Service Error แยกกัน
- ต้องไม่มีการใช้ `unsafe`

## Expected Output

ตัวอย่างกรณีสำเร็จ:

```text
Fetching user...
Fetching products...
Fetching orders...

User data received
Product data received
Order data received

Summary
User: Alice
Products: 3
Orders: 5

Summary task completed
```

ตัวอย่างกรณี Error:

```text
Fetching user...
Fetching products...
Fetching orders...

Error: Product service unavailable
```

รายละเอียดข้อความสามารถออกแบบได้เอง

## Hints

- ใช้ Struct แยกข้อมูล User, Product และ Order
- ใช้ `tokio::join!()` กับ Future ที่ต้องการรอพร้อมกัน
- `join!()` คืนผลลัพธ์เป็น Tuple
- ใช้ `Result<T, String>` สำหรับฝึกจัดการ Error เบื้องต้น
- ใช้ `tokio::spawn()` เมื่อต้องการแยก Task
- อย่าสับสนระหว่าง Service Error กับ JoinError

## Completion Checklist

- [ ] สร้าง Async Service 3 ตัว
- [ ] ใช้ `tokio::join!()`
- [ ] รับผลลัพธ์จาก Future ทั้งหมด
- [ ] สร้าง Task ด้วย `tokio::spawn()`
- [ ] ใช้ JoinHandle
- [ ] มี Function ที่คืนค่า Result
- [ ] จัดการ Service Error
- [ ] จัดการ JoinError
- [ ] สรุปข้อมูลได้ถูกต้อง
- [ ] ทดลองทั้งกรณีสำเร็จและกรณี Error

## Run

```powershell
cd 25_async_programming/exercises/exercise_03/async_tasks
cargo add tokio --features full
cargo run
```
