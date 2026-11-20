# Exercise 03: Expense Tracker

## Project

**Expense Tracker — โปรแกรมบันทึกรายรับรายจ่าย**

## Objective

สร้างโปรแกรมสำหรับบันทึกและติดตามรายรับรายจ่าย โดยสามารถเพิ่มรายการ แสดงประวัติ คำนวณยอดรวม และบันทึกข้อมูลลงไฟล์ได้

## Requirements

โปรแกรมต้องรองรับความสามารถดังต่อไปนี้:

1. Add Income
2. Add Expense
3. View Transactions
4. Show Summary
5. Save Data
6. Load Data
7. Exit

## Tasks

### Task 1: Design Transaction Struct

สร้าง Struct สำหรับเก็บข้อมูลรายการธุรกรรม

ข้อมูลขั้นต่ำ:

- ID
- Description
- Amount
- Transaction Type

### Task 2: Design Transaction Type

สร้าง Enum เพื่อแยกประเภทธุรกรรม:

- Income
- Expense

### Task 3: Store Transactions

ใช้ `Vec` สำหรับเก็บรายการธุรกรรมทั้งหมด

### Task 4: Add Income and Expense

รับข้อมูลจากผู้ใช้:

- รายละเอียดรายการ
- จำนวนเงิน
- ประเภทธุรกรรม

ตรวจสอบข้อมูลก่อนเพิ่มรายการ

### Task 5: View Transactions

แสดงรายการธุรกรรมทั้งหมด โดยแสดงประเภทและจำนวนเงิน

### Task 6: Calculate Summary

คำนวณ:

- Total Income
- Total Expense
- Balance

กำหนดให้:

`Balance = Total Income - Total Expense`

### Task 7: Save and Load Data

บันทึกข้อมูลลงไฟล์และอ่านข้อมูลกลับมาเมื่อเปิดโปรแกรม

สามารถกำหนดรูปแบบจัดเก็บข้อมูลเป็นข้อความธรรมดา เช่น CSV แบบง่าย โดยต้องกำหนดรูปแบบข้อมูลให้ชัดเจน

### Task 8: Handle Errors

รองรับกรณี:

- จำนวนเงินไม่ใช่ตัวเลข
- จำนวนเงินไม่ถูกต้องตามเงื่อนไขที่กำหนด
- รายละเอียดรายการว่าง
- ไฟล์ไม่มีอยู่
- ไม่สามารถอ่านหรือเขียนไฟล์ได้
- ข้อมูลในไฟล์ไม่ถูกต้อง

## Constraints

- ใช้ Struct สำหรับ Transaction
- ใช้ Enum สำหรับประเภท Income และ Expense
- ใช้ Vec สำหรับจัดเก็บข้อมูล
- ใช้ Function แยกการทำงาน
- ใช้ Module แยกส่วนที่เหมาะสม
- ใช้ `Result` และ `Option` จัดการข้อผิดพลาด
- ต้องสามารถบันทึกและโหลดข้อมูลได้
- ห้ามใช้ `unwrap()` กับข้อมูลหรือไฟล์ที่อาจผิดพลาดโดยไม่มีการตรวจสอบ

สำหรับการฝึกเบื้องต้น สามารถใช้ `f64` แทนจำนวนเงินได้ แต่ในการใช้งานด้านการเงินจริงควรพิจารณาเก็บเงินเป็นจำนวนหน่วยย่อยแบบจำนวนเต็ม เพื่อหลีกเลี่ยงปัญหาความคลาดเคลื่อนของเลขทศนิยม

## Expected Output

```text
===== EXPENSE TRACKER =====
1. Add Income
2. Add Expense
3. View Transactions
4. Show Summary
5. Save Data
6. Load Data
0. Exit

Select: 1
Description: Salary
Amount: 25000

Income added successfully.

Select: 2
Description: Food
Amount: 150

Expense added successfully.

Select: 4

===== SUMMARY =====
Total Income: 25000
Total Expense: 150
Balance: 24850
```

## Testing Requirements

ควรทดสอบ:

- เพิ่มรายรับ
- เพิ่มรายจ่าย
- คำนวณยอดรายรับรวม
- คำนวณยอดรายจ่ายรวม
- คำนวณยอดคงเหลือ
- ไม่มีรายการธุรกรรม
- บันทึกข้อมูลลงไฟล์
- โหลดข้อมูลจากไฟล์
- อ่านไฟล์ที่ไม่มีอยู่
- จัดการข้อมูลในไฟล์ที่ไม่ถูกต้อง

## Completion Checklist

- [ ] มี Struct สำหรับ Transaction
- [ ] มี Enum สำหรับ Transaction Type
- [ ] เพิ่มรายรับได้
- [ ] เพิ่มรายจ่ายได้
- [ ] แสดงรายการธุรกรรมได้
- [ ] คำนวณยอดรวมได้
- [ ] คำนวณยอดคงเหลือได้
- [ ] บันทึกข้อมูลลงไฟล์ได้
- [ ] โหลดข้อมูลจากไฟล์ได้
- [ ] จัดการ File Error ได้
- [ ] มี Unit Tests
- [ ] มี Integration Tests สำหรับการทำงานสำคัญ
