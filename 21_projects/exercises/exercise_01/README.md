# Exercise 01: Calculator CLI

## Project

**Calculator CLI — โปรแกรมเครื่องคิดเลขผ่าน Command Line**

## Objective

สร้างโปรแกรมเครื่องคิดเลขที่รับตัวเลขและตัวดำเนินการจากผู้ใช้ แล้วคำนวณผลลัพธ์ผ่าน Terminal

## Requirements

โปรแกรมต้องรองรับการคำนวณพื้นฐาน 4 ประเภท:

1. Addition (`+`)
2. Subtraction (`-`)
3. Multiplication (`*`)
4. Division (`/`)

## Tasks

### Task 1: Create Calculator Functions

สร้าง Function สำหรับการคำนวณแต่ละประเภท

- บวกเลขสองจำนวน
- ลบเลขสองจำนวน
- คูณเลขสองจำนวน
- หารเลขสองจำนวน

### Task 2: Receive User Input

รับข้อมูลจากผู้ใช้:

- ตัวเลขตัวที่ 1
- ตัวดำเนินการ
- ตัวเลขตัวที่ 2

### Task 3: Process the Operation

ตรวจสอบตัวดำเนินการและเลือก Function ที่เหมาะสม

### Task 4: Display Result

แสดงผลลัพธ์การคำนวณใน Terminal

### Task 5: Handle Errors

รองรับกรณีต่อไปนี้:

- ผู้ใช้ป้อนข้อความแทนตัวเลข
- ผู้ใช้ป้อนตัวดำเนินการที่ไม่รองรับ
- ผู้ใช้หารด้วยศูนย์

## Constraints

- ใช้ Function แยกการคำนวณแต่ละประเภท
- ใช้ `match` หรือ `if/else` ตรวจสอบตัวดำเนินการ
- ใช้ `Result` หรือวิธีจัดการ Error ที่เหมาะสม
- ห้ามปล่อยให้การหารด้วยศูนย์ทำให้โปรแกรม Panic
- ไม่ควรใช้ `unwrap()` กับข้อมูลจากผู้ใช้โดยไม่มีการตรวจสอบ

## Expected Output

ตัวอย่างกรณีคำนวณสำเร็จ:

```text
===== CALCULATOR =====
Enter first number: 20
Enter operator (+, -, *, /): +
Enter second number: 10

Result: 30
```

ตัวอย่างกรณีหารด้วยศูนย์:

```text
===== CALCULATOR =====
Enter first number: 20
Enter operator (+, -, *, /): /
Enter second number: 0

Error: Cannot divide by zero.
```

ตัวอย่างกรณีตัวดำเนินการไม่ถูกต้อง:

```text
Enter operator (+, -, *, /): %
Error: Unsupported operator.
```

## Testing Requirements

ควรทดสอบอย่างน้อย:

- บวกเลขจำนวนเต็มบวก
- ลบเลขที่มีผลลัพธ์เป็นลบ
- คูณด้วยศูนย์
- หารเลขที่หารลงตัว
- หารเลขที่ได้ผลลัพธ์เป็นทศนิยม
- หารด้วยศูนย์
- ป้อนตัวดำเนินการที่ไม่รองรับ
- ป้อนข้อความแทนตัวเลข

## Completion Checklist

- [ ] มี Function สำหรับการบวก
- [ ] มี Function สำหรับการลบ
- [ ] มี Function สำหรับการคูณ
- [ ] มี Function สำหรับการหาร
- [ ] รับข้อมูลจากผู้ใช้ได้
- [ ] ตรวจสอบตัวดำเนินการได้
- [ ] จัดการการหารด้วยศูนย์ได้
- [ ] จัดการ Input ที่ไม่ถูกต้องได้
- [ ] มี Unit Tests สำหรับ Function คำนวณ
- [ ] โปรแกรมทำงานผ่าน Terminal ได้
