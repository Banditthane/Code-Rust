# Contributing Guide

แนวทางสำหรับการเพิ่ม แก้ไข และปรับปรุงเนื้อหาใน Rust Complete Learning Repository

## 1. General Principles

ทุก Contribution ควรยึดหลักดังนี้:

- เนื้อหาต้องถูกต้องตามภาษา Rust
- เรียงลำดับจากพื้นฐานไปขั้นสูง
- ใช้คำอธิบายที่เข้าใจง่ายสำหรับผู้เริ่มต้น
- ตัวอย่างโค้ดต้องอ่านง่ายและนำไปทดลองได้
- แบบฝึกหัดต้องระบุสิ่งที่ต้องทำอย่างชัดเจน
- หลีกเลี่ยงการให้เฉลยในโจทย์แบบฝึกหัด
- ไม่เพิ่มเนื้อหาซ้ำกับ Chapter อื่นโดยไม่จำเป็น

## 2. Chapter Structure

แต่ละ Chapter ควรใช้โครงสร้างมาตรฐาน:

```text
chapter_name/
├── README.md
├── 01_lesson_one/
│   └── README.md
├── 02_lesson_two/
│   └── README.md
├── 03_lesson_three/
│   └── README.md
├── 04_lesson_four/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── project_name/
    │       ├── Cargo.toml
    │       └── src/
    │           └── main.rs
    ├── exercise_02/
    ├── exercise_03/
    └── exercise_04/
```

แต่ละ Chapter ประกอบด้วย:

- Chapter Overview 1 ไฟล์
- Lesson README 4 ไฟล์
- Exercise README 4 ไฟล์

รวม 9 README files ต่อ Chapter

หาก Exercise เป็นงานวางแผนหรือออกแบบเอกสารโดยไม่ต้องเขียนโปรแกรม ไม่จำเป็นต้องสร้าง Cargo Project

## 3. Lesson README Requirements

แต่ละ Lesson README ควรประกอบด้วย:

1. Title
2. Learning Objectives
3. Concept Explanation
4. Syntax
5. Code Examples
6. Code Explanation
7. Expected Output
8. Common Mistakes
9. Summary
10. Related Exercises

ตัวอย่างโครงสร้าง:

```markdown
# Lesson Title

## Learning Objectives

## Concept

## Syntax

## Examples

## Code Explanation

## Expected Output

## Common Mistakes

## Summary
```

ตัวอย่างโค้ดต้องระบุให้ชัดเจนว่าเป็นโค้ดที่รันได้ทั้งโปรแกรม หรือเป็นเพียงส่วนหนึ่งของโค้ด

## 4. Exercise README Requirements

ทุก Exercise ควรมี:

1. Objective
2. Problem Statement
3. Requirements
4. Constraints
5. Expected Output
6. Hints
7. Completion Checklist

ตัวอย่าง:

```markdown
# Exercise Title

**Difficulty:** Easy / Easy-Medium / Medium / Challenge

## Objective

## Problem Statement

## Requirements

## Constraints

## Expected Output

## Hints

## Completion Checklist
```

### Exercise Difficulty

| Level       | Description                              |
| ----------- | ---------------------------------------- |
| Easy        | ฝึกแนวคิดหลักของบท                       |
| Easy-Medium | ประยุกต์ใช้หลายแนวคิด                    |
| Medium      | แก้ปัญหาที่มีหลายขั้นตอน                 |
| Challenge   | รวมความรู้และออกแบบวิธีแก้ปัญหาด้วยตนเอง |

## 5. Cargo Project Rules

สร้างโปรเจกต์ด้วย Cargo:

```powershell
cargo new project_name --vcs none
```

สำหรับ Library Project:

```powershell
cargo new project_name --lib --vcs none
```

ข้อกำหนด:

- ไม่สร้าง `Cargo.toml` ด้วยตนเองหากใช้ `cargo new` ได้
- ใช้ชื่อโปรเจกต์ตัวพิมพ์เล็กและ snake_case
- ระบุ Dependencies ที่จำเป็น
- หลีกเลี่ยง Dependencies ที่ไม่เกี่ยวข้อง
- ไม่ Commit Build Artifacts เช่น `target/`
- ไม่ Commit Secrets หรือ Credentials

## 6. Rust Coding Standards

### Naming

| Item      | Convention           | Example           |
| --------- | -------------------- | ----------------- |
| Variables | snake_case           | `user_name`       |
| Functions | snake_case           | `calculate_total` |
| Modules   | snake_case           | `user_service`    |
| Structs   | PascalCase           | `UserAccount`     |
| Enums     | PascalCase           | `OrderStatus`     |
| Traits    | PascalCase           | `Printable`       |
| Constants | SCREAMING_SNAKE_CASE | `MAX_SIZE`        |

### Code Quality

- ใช้ชื่อตัวแปรที่สื่อความหมาย
- แบ่ง Functions ตามหน้าที่
- หลีกเลี่ยงโค้ดซ้ำ
- จัดการ Errors อย่างเหมาะสม
- ใช้ References เมื่อไม่จำเป็นต้องโอน Ownership
- ใช้ `Result` และ `Option` ตามลักษณะของข้อมูล
- หลีกเลี่ยง `unwrap()` ในกรณีที่อาจเกิด Error จาก Input หรือ External Data
- จำกัดการใช้ `unsafe` เฉพาะกรณีที่จำเป็นและมี Safety Contract ชัดเจน

## 7. Required Checks

ก่อนส่ง Contribution ให้ตรวจสอบ:

```powershell
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

หากโปรเจกต์ไม่มี Tests หรือมีข้อจำกัดของ Platform ให้ระบุไว้ใน Contribution

สำหรับโปรเจกต์ที่ใช้ Workspace ให้รันคำสั่งจาก Workspace Root เมื่อเหมาะสม

## 8. Documentation Standards

- ใช้ Markdown ที่อ่านง่าย
- ใช้ Heading ตามลำดับ
- ใช้ Code Fence พร้อมระบุภาษา เช่น `rust`, `toml`, `powershell`
- ระบุ Expected Output เมื่อมีตัวอย่างโปรแกรม
- ระบุ Version หรือข้อจำกัดของ Dependencies เมื่อมีผลต่อการใช้งาน
- ตรวจสอบลิงก์ภายใน Repository ก่อนส่ง

## 9. Pull Request / Change Checklist

- [ ] เนื้อหาตรงกับ Chapter
- [ ] ไม่ซ้ำซ้อนกับบทอื่นโดยไม่จำเป็น
- [ ] ตัวอย่าง Rust Code ถูกต้อง
- [ ] Expected Output สอดคล้องกับโค้ด
- [ ] Exercise มี Requirements และ Constraints
- [ ] ไม่มีเฉลยในโจทย์ที่ตั้งใจให้ผู้เรียนฝึกเอง
- [ ] ผ่าน `cargo fmt --check`
- [ ] ผ่าน `cargo check`
- [ ] ผ่าน `cargo clippy` เมื่อเหมาะสม
- [ ] ผ่าน `cargo test` เมื่อมี Tests
- [ ] อัปเดต README หรือ Progress ที่เกี่ยวข้อง

## 10. Contribution Scope

Contribution สามารถเป็น:

- แก้ไขคำอธิบายที่ไม่ชัดเจน
- แก้ไข Rust Code ที่ผิดพลาด
- เพิ่มตัวอย่างประกอบ
- เพิ่มแบบฝึกหัด
- ปรับปรุง Expected Output
- เพิ่ม References
- ปรับปรุง Documentation
- แก้ไขปัญหาใน Project Structure

---

**Goal:** รักษา Repository ให้เป็นแหล่งเรียนรู้ Rust ที่ถูกต้อง เป็นระบบ และสามารถนำไปฝึกปฏิบัติได้จริง
