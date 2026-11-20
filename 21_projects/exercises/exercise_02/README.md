# Exercise 02: Todo CLI

## Project

**Todo CLI — โปรแกรมจัดการรายการสิ่งที่ต้องทำ**

## Objective

สร้างโปรแกรมสำหรับจัดการรายการสิ่งที่ต้องทำผ่าน Command Line โดยผู้ใช้สามารถเพิ่ม ดู ทำเครื่องหมายว่าเสร็จแล้ว และลบรายการได้

## Requirements

โปรแกรมต้องรองรับคำสั่งดังต่อไปนี้:

| Command | Description   |
| ------- | ------------- |
| 1       | Add Task      |
| 2       | View Tasks    |
| 3       | Complete Task |
| 4       | Delete Task   |
| 0       | Exit          |

## Tasks

### Task 1: Design Task Struct

สร้าง Struct สำหรับเก็บข้อมูลรายการสิ่งที่ต้องทำ

ข้อมูลขั้นต่ำ:

- ID
- Title
- Completed Status

### Task 2: Store Tasks

ใช้ `Vec` สำหรับเก็บรายการ Task

### Task 3: Create Menu

สร้างเมนูให้ผู้ใช้เลือกคำสั่ง

### Task 4: Add Task

รับชื่อรายการจากผู้ใช้และเพิ่มเข้าไปในรายการ

### Task 5: View Tasks

แสดงรายการทั้งหมด พร้อมสถานะของแต่ละรายการ

### Task 6: Complete Task

รับ ID จากผู้ใช้และเปลี่ยนสถานะของ Task เป็น Completed

### Task 7: Delete Task

รับ ID จากผู้ใช้และลบ Task ที่ต้องการ

### Task 8: Validate Input

ตรวจสอบข้อมูล เช่น:

- ชื่อ Task ต้องไม่เป็นข้อความว่าง
- ID ต้องมีอยู่จริง
- คำสั่งต้องอยู่ในเมนูที่กำหนด

## Constraints

- ใช้ Struct สำหรับเก็บข้อมูล Task
- ใช้ `Vec` สำหรับเก็บรายการ
- ใช้ Function แยกการทำงานแต่ละคำสั่ง
- ใช้ Loop สำหรับแสดงเมนูซ้ำ
- ใช้ `Result` หรือ `Option` จัดการข้อผิดพลาดตามความเหมาะสม
- ในเวอร์ชันนี้ยังไม่จำเป็นต้องจัดเก็บข้อมูลลงไฟล์

## Expected Output

```text
===== TODO CLI =====
1. Add Task
2. View Tasks
3. Complete Task
4. Delete Task
0. Exit

Select: 1
Enter task title: Learn Rust

Task added successfully.

Select: 2

===== TASK LIST =====
ID: 1
Title: Learn Rust
Status: Pending

Select: 3
Enter task ID: 1

Task completed successfully.

Select: 2

===== TASK LIST =====
ID: 1
Title: Learn Rust
Status: Completed
```

ผลลัพธ์อาจมีรูปแบบต่างจากตัวอย่างได้ แต่ต้องแสดงข้อมูลและสถานะได้ถูกต้อง

## Testing Requirements

ควรทดสอบ:

- เพิ่ม Task
- เพิ่ม Task หลายรายการ
- แสดงรายการเมื่อยังไม่มี Task
- เปลี่ยนสถานะ Task เป็น Completed
- ลบ Task
- ค้นหา ID ที่ไม่มีอยู่
- ป้อนชื่อ Task ว่าง
- ป้อนคำสั่งที่ไม่ถูกต้อง

## Completion Checklist

- [ ] มี Struct สำหรับ Task
- [ ] ใช้ Vec จัดเก็บ Task
- [ ] สามารถเพิ่ม Task ได้
- [ ] สามารถแสดง Task ได้
- [ ] สามารถเปลี่ยนสถานะ Task ได้
- [ ] สามารถลบ Task ได้
- [ ] มีเมนูให้เลือกคำสั่ง
- [ ] ตรวจสอบ Input ได้
- [ ] แยก Function ตามหน้าที่
- [ ] มี Unit Tests สำหรับการจัดการ Task
