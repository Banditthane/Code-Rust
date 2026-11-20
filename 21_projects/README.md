# Chapter 21: Projects

## Overview

บทนี้เป็นบทสุดท้ายของ Rust Learning Roadmap โดยนำความรู้จากบทที่ 01–20 มาประยุกต์ใช้ในการพัฒนาโปรแกรมจริง

ผู้เรียนจะได้ฝึกตั้งแต่การวางแผนโปรเจกต์ การออกแบบโครงสร้างโปรแกรม การรับข้อมูลจากผู้ใช้ การจัดเก็บข้อมูลลงไฟล์ ไปจนถึงการทดสอบและปรับปรุงโปรแกรม

## Objectives

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- วางแผนและออกแบบโปรแกรม Rust ขนาดเล็กได้
- สร้างโปรแกรม Command Line Application ได้
- รับและตรวจสอบข้อมูลจากผู้ใช้ได้
- ใช้ Struct, Enum, Trait และ Module ในโปรเจกต์ได้
- อ่านและเขียนข้อมูลลงไฟล์ได้
- จัดการข้อผิดพลาดด้วย Result และ Option ได้
- เขียน Unit Tests และ Integration Tests ได้
- แบ่งโค้ดออกเป็นส่วน ๆ เพื่อให้ง่ายต่อการดูแลรักษา

## Lessons

| Lesson | Topic                  | Description                          |
| ------ | ---------------------- | ------------------------------------ |
| 01     | Project Planning       | การวางแผนและออกแบบโปรเจกต์           |
| 02     | CLI Applications       | การสร้างโปรแกรม Command Line         |
| 03     | File Handling Projects | การอ่าน เขียน และจัดเก็บข้อมูลในไฟล์ |
| 04     | Final Project          | การพัฒนาโปรเจกต์แบบครบวงจร           |

## Exercises

| Exercise | Project            | Description                      |
| -------- | ------------------ | -------------------------------- |
| 01       | Calculator CLI     | โปรแกรมเครื่องคิดเลข             |
| 02       | Todo CLI           | โปรแกรมจัดการรายการสิ่งที่ต้องทำ |
| 03       | Expense Tracker    | โปรแกรมบันทึกรายรับรายจ่าย       |
| 04       | Student Management | โปรแกรมจัดการข้อมูลนักเรียน      |

## Folder Structure

```text
21_projects/
├── README.md
├── 01_project_planning/
│   └── README.md
├── 02_cli_applications/
│   └── README.md
├── 03_file_handling_projects/
│   └── README.md
├── 04_final_project/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── calculator_cli/
    ├── exercise_02/
    │   ├── README.md
    │   └── todo_cli/
    ├── exercise_03/
    │   ├── README.md
    │   └── expense_tracker/
    └── exercise_04/
        ├── README.md
        └── student_management/
```

## Knowledge Applied

บทนี้จะนำความรู้จากบทก่อนหน้ามาประยุกต์ใช้ เช่น

- Variables and Data Types
- Conditions and Loops
- Functions
- Strings
- Ownership and Borrowing
- Collections
- Structs and Enums
- Modules
- Error Handling
- Traits and Generics
- Testing

## Recommended Workflow

1. อ่านโจทย์และทำความเข้าใจความต้องการของโปรแกรม
2. ระบุข้อมูลที่ต้องใช้และผลลัพธ์ที่ต้องการ
3. ออกแบบ Struct, Enum และ Function ที่จำเป็น
4. แบ่งโปรแกรมออกเป็น Module ตามหน้าที่
5. พัฒนาโปรแกรมทีละส่วน
6. ตรวจสอบข้อมูลและจัดการข้อผิดพลาด
7. ทดสอบการทำงานทั้งกรณีปกติและกรณีผิดพลาด
8. ปรับปรุงโค้ดให้อ่านง่ายและดูแลรักษาได้

## Useful Commands

```powershell
cargo new project_name --vcs none
cargo run
cargo check
cargo fmt
cargo clippy
cargo test
```

## Completion Checklist

- [ ] เรียน Lesson 01–04 ครบแล้ว
- [ ] ทำ Exercise 01–04 ครบแล้ว
- [ ] สามารถสร้าง CLI Application ได้
- [ ] สามารถอ่านและเขียนไฟล์ได้
- [ ] สามารถจัดการ Error ได้
- [ ] สามารถแบ่งโปรแกรมเป็น Module ได้
- [ ] สามารถเขียน Tests ได้
- [ ] สามารถพัฒนาโปรเจกต์ขนาดเล็กด้วยตนเองได้

## Final Goal

เป้าหมายของบทนี้คือการนำความรู้ Rust ที่เรียนมาทั้งหมดมาสร้างโปรแกรมที่ทำงานได้จริง โดยเน้นการคิด วิเคราะห์ ออกแบบ และแก้ปัญหาด้วยตนเอง
