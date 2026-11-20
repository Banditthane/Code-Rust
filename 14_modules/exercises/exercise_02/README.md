# Exercise 02: File Modules

## Objective

ฝึกแยก Module ออกจาก `main.rs` ไปยังไฟล์ `.rs` และเรียนรู้การจัดโครงสร้าง Module Files

## Project

`file_modules`

## Scenario

สร้างโปรแกรมจัดการข้อมูลร้านค้า โดยแบ่งโค้ดออกเป็น Module ตามหน้าที่ของแต่ละส่วน

## Required Structure

จัดโครงสร้างภายใน `src/` ดังนี้:

```text
src/
├── main.rs
├── products.rs
├── customers.rs
└── orders.rs
```

## Requirements

สร้าง Module 3 รายการ:

### 1. products

สร้าง Function:

| Function          | Description      |
| ----------------- | ---------------- |
| `show_products()` | แสดงรายการสินค้า |

### 2. customers

สร้าง Function:

| Function           | Description       |
| ------------------ | ----------------- |
| `show_customers()` | แสดงรายชื่อลูกค้า |

### 3. orders

สร้าง Function:

| Function        | Description          |
| --------------- | -------------------- |
| `show_orders()` | แสดงรายการคำสั่งซื้อ |

ทุก Function ต้องสามารถเรียกใช้งานจาก `main.rs` ได้

## Tasks

1. สร้างไฟล์
