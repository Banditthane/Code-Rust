# Exercise 03: Visibility Practice

## Objective

ฝึกควบคุมการเข้าถึง Items ภายใน Module ด้วย `pub` และเรียนรู้การจัดการ Private Fields ผ่าน Public Methods

## Project

`visibility_practice`

## Scenario

สร้าง Module สำหรับจัดเก็บข้อมูลบัญชีผู้ใช้ โดยกำหนดให้ข้อมูลบางส่วนสามารถเข้าถึงได้จากภายนอก และบางส่วนต้องถูกควบคุมผ่าน Methods

## Requirements

### Part 1: Create Module

สร้าง Module ชื่อ `account`

### Part 2: Create Struct

ประกาศ Struct ชื่อ `Account` โดยมี Fields ดังนี้:

| Field    | Data Type | Visibility | Description |
| -------- | --------- | ---------- | ----------- |
| username | String    | Public     | ชื่อผู้ใช้  |
| balance  | u32       | Private    | ยอดเงิน     |
| active   | bool      | Private    | สถานะบัญชี  |

### Part 3: Create Methods

สร้าง Public Methods ภายใน `impl Account`:

| Method           | Requirement                                                      |
| ---------------- | ---------------------------------------------------------------- |
| `new()`          | สร้าง Account จาก username และ balance โดยกำหนด active เป็น true |
| `show_balance()` | แสดงยอดเงิน                                                      |
| `deposit()`      | เพิ่มยอดเงิน                                                     |
| `is_active()`    | คืนค่าสถานะบัญชีเป็น bool                                        |

ข้อกำหนด:

- `new()` ต้องคืนค่าเป็น `Self`
- `show_balance()` ใช้ `&self`
- `deposit()` ใช้ `&mut self`
- `is_active()` ใช้ `&self` และคืนค่า `bool`

### Part 4: Test Visibility

จาก `main.rs`:

1. สร้าง Account Instance ผ่าน `Account::new()`
2. อ่าน Public Field `username`
3. เรียก `show_balance()`
4. เพิ่มยอดเงินด้วย `deposit()`
5. เรียก `is_active()`
   6
