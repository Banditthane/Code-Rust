# Chapter 04: Operators

## Overview

เรียนรู้การใช้ Operators ในภาษา Rust สำหรับคำนวณ กำหนดค่า เปรียบเทียบ และตรวจสอบเงื่อนไข

## Learning Objectives

เมื่อเรียนจบบทนี้ คุณจะสามารถ:

- ใช้ Arithmetic Operators ในการคำนวณได้
- ใช้ Assignment Operators ในการกำหนดและปรับค่าได้
- ใช้ Comparison Operators เพื่อเปรียบเทียบข้อมูลได้
- ใช้ Logical Operators เพื่อเชื่อมเงื่อนไขได้
- เข้าใจการทำงานพื้นฐานของ Bitwise Operators ได้

## Lessons

| Lesson | Topic                | Description              |
| ------ | -------------------- | ------------------------ |
| 01     | Arithmetic Operators | การคำนวณทางคณิตศาสตร์    |
| 02     | Assignment Operators | การกำหนดและปรับค่าตัวแปร |
| 03     | Comparison Operators | การเปรียบเทียบข้อมูล     |
| 04     | Logical Operators    | การเชื่อมเงื่อนไข        |
| 05     | Bitwise Operators    | การดำเนินการระดับบิต     |

## Operators Overview

### 1. Arithmetic Operators

| Operator | Description    |
| -------- | -------------- |
| `+`      | Addition       |
| `-`      | Subtraction    |
| `*`      | Multiplication |
| `/`      | Division       |
| `%`      | Remainder      |

### 2. Assignment Operators

| Operator | Description          |
| -------- | -------------------- |
| `=`      | Assign               |
| `+=`     | Add and assign       |
| `-=`     | Subtract and assign  |
| `*=`     | Multiply and assign  |
| `/=`     | Divide and assign    |
| `%=`     | Remainder and assign |

### 3. Comparison Operators

| Operator | Description           |
| -------- | --------------------- |
| `==`     | Equal                 |
| `!=`     | Not equal             |
| `>`      | Greater than          |
| `<`      | Less than             |
| `>=`     | Greater than or equal |
| `<=`     | Less than or equal    |

Comparison expressions return a Boolean value: `true` or `false`.

### 4. Logical Operators

| Operator | Description |
| -------- | ----------- | --- | ---------- |
| `&&`     | Logical AND |
| `        |             | `   | Logical OR |
| `!`      | Logical NOT |

### 5. Bitwise Operators

| Operator | Description |
| -------- | ----------- | ---------- |
| `&`      | Bitwise AND |
| `        | `           | Bitwise OR |
| `^`      | Bitwise XOR |
| `!`      | Bitwise NOT |
| `<<`     | Left shift  |
| `>>`     | Right shift |

## Exercises

| Exercise | Topic                | Description                         |
| -------- | -------------------- | ----------------------------------- |
| 01       | Basic Calculator     | ฝึกใช้ Arithmetic Operators         |
| 02       | Comparison Challenge | ฝึกเปรียบเทียบค่าต่าง ๆ             |
| 03       | Logical Conditions   | ฝึกใช้ Logical Operators            |
| 04       | Operator Challenge   | ฝึกประยุกต์ใช้ Operators หลายประเภท |

## Practice Commands

ตรวจสอบโค้ด:

```powershell
cargo check
```

รันโปรแกรม:

```powershell
cargo run
```

ตรวจสอบเพิ่มเติมด้วย Clippy:

```powershell
cargo clippy
```

## Completion Checklist

- [ ] เข้าใจ Arithmetic Operators
- [ ] เข้าใจ Assignment Operators
- [ ] เข้าใจ Comparison Operators
- [ ] เข้าใจ Logical Operators
- [ ] เข้าใจ Bitwise Operators เบื้องต้น
- [ ] ทำ Exercises ครบทั้ง 4 ข้อ
- [ ] โปรแกรมทุกข้อผ่าน `cargo check`
- [ ] อธิบายการทำงานของ Operators ที่ใช้ได้
