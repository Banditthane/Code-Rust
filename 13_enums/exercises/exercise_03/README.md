# Exercise 03: Order Status

## Objective

ฝึกใช้ Enum ร่วมกับ Struct และ Match เพื่อสร้างระบบจัดการสถานะคำสั่งซื้อ

## Project

`order_status`

## Scenario

สร้างโปรแกรมจัดการคำสั่งซื้อ โดยคำสั่งซื้อแต่ละรายการมีหมายเลขคำสั่งซื้อ ชื่อลูกค้า และสถานะของคำสั่งซื้อ

สถานะสามารถเปลี่ยนแปลงได้ตามกระบวนการจัดส่งสินค้า

## Requirements

### Part 1: Create Enum

ประกาศ Enum ชื่อ `OrderStatus` โดยมี Variants ดังนี้:

| Variant    | Meaning          |
| ---------- | ---------------- |
| Pending    | รอดำเนินการ      |
| Processing | กำลังดำเนินการ   |
| Shipped    | จัดส่งแล้ว       |
| Delivered  | ส่งถึงลูกค้าแล้ว |
| Cancelled  | ยกเลิกคำสั่งซื้อ |

### Part 2: Create Struct

ประกาศ Struct ชื่อ `Order` โดยมี Fields ดังนี้:

| Field         | Data Type   | Description       |
| ------------- | ----------- | ----------------- |
| order_id      | u32         | หมายเลขคำสั่งซื้อ |
| customer_name | String      | ชื่อลูกค้า        |
| status        | OrderStatus | สถานะคำสั่งซื้อ   |

### Part 3: Create Instances

สร้างคำสั่งซื้ออย่างน้อย 3 รายการ โดยกำหนดสถานะแตกต่างกัน

### Part 4: Create Methods or Functions

สร้าง Function หรือ Method สำหรับแสดงสถานะคำสั่งซื้อ

ข้อกำหนด:

- รับค่า `OrderStatus`
- ใช้ `match` ตรวจสอบทุก Variant
- แสดงข้อความอธิบายสถานะเป็นภาษาอังกฤษ
- ต้องจัดการทุกสถานะให้ครบถ้วน

### Part 5: Display Order Information

แสดงข้อมูลคำสั่งซื้อแต่ละรายการ ประกอบด้วย:

- Order ID
- Customer Name
- Order Status

## Expected Output

```text
=== Order Management ===

Order ID: 1001
Customer: Alice
Status: Pending
Description: Waiting for processing

Order ID: 1002
Customer: Bob
Status: Shipped
Description: Order has been shipped

Order ID: 1003
Customer: Charlie
Status: Delivered
Description: Order has been delivered
```

## Constraints

- ต้องใช้ Enum ชื่อ `OrderStatus`
- ต้องมี Variants ครบทั้ง 5 รายการ
- ต้องใช้ Struct ชื่อ `Order`
- Struct ต้องมี Fields ครบ 3 รายการ
- ต้องสร้างคำสั่งซื้ออย่างน้อย 3 รายการ
- ต้องใช้ `match` ตรวจสอบสถานะ
- ต้องจัดการทุก Variant
- ต้องแสดงข้อมูลคำสั่งซื้อครบถ้วน

## Checklist

- [ ] ประกาศ Enum `OrderStatus`
- [ ] สร้าง Variants ครบ 5 รายการ
- [ ] ประกาศ Struct `Order`
- [ ] กำหนด Fields ครบถ้วน
- [ ] สร้าง Order Instances อย่างน้อย 3 รายการ
- [ ] สร้าง Function หรือ Method สำหรับแสดงสถานะ
- [ ] ใช้ `match` ตรวจสอบ Enum
- [ ] จัดการสถานะทุกประเภท
- [ ] แสดงข้อมูลคำสั่งซื้อได้ถูกต้อง
- [ ] `cargo check` ผ่าน
- [ ] `cargo run` ทำงานได้ตามต้องการ

## Challenge (Optional)

เพิ่ม Method `update_status()` เพื่อเปลี่ยนสถานะคำสั่งซื้อ โดยใช้ `&mut self` และทดลองเปลี่ยนสถานะจาก Pending เป็น Processing และ Shipped
