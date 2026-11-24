# Exercise 01: Project Planning

## ระดับ

Easy

## Objective

ฝึกวิเคราะห์ Requirements กำหนด Project Scope และออกแบบ Architecture ก่อนเริ่มเขียน Code

## Problem Statement

เลือกปัญหาหนึ่งอย่างที่ต้องการแก้ไข แล้วออกแบบ Software Project ด้วย Rust

เลือกหัวข้อได้ เช่น:

- Task Management System
- Student Management System
- Inventory Management System
- Library Management System
- Expense Tracker
- Personal Finance Manager
- Book Management System

ต้องจัดทำ Project Plan ที่อธิบายว่าระบบจะทำอะไร และจะพัฒนาอย่างไร

## Requirements

### Part 1: Project Overview

จัดทำข้อมูล:

- Project Name
- Project Description
- Problem Statement
- Target Users
- Main Objectives

### Part 2: Requirements Analysis

กำหนด Functional Requirements อย่างน้อย 5 รายการ

ตัวอย่าง:

- Create Record
- Read Record
- Update Record
- Delete Record
- Search Record

กำหนด Non-functional Requirements อย่างน้อย 3 รายการ เช่น:

- Security
- Performance
- Reliability
- Maintainability

### Part 3: User Stories

เขียน User Stories อย่างน้อย 5 รายการ

ใช้รูปแบบ:

```text
As a [user]
I want [feature]
So that [benefit]
```

กำหนด Acceptance Criteria สำหรับ User Stories อย่างน้อย 3 รายการ

### Part 4: Project Scope

แบ่ง Features ออกเป็น:

- MVP Features
- Future Features
- Out of Scope

ต้องระบุให้ชัดเจนว่า Version แรกจะทำอะไรและจะยังไม่ทำอะไร

### Part 5: System Architecture

ออกแบบ Architecture Diagram

แสดง Components ที่เกี่ยวข้อง เช่น:

- Client
- API
- Service
- Repository
- Database

หากเลือก CLI Project สามารถออกแบบเป็น CLI, Application Logic และ Storage แทนได้

### Part 6: Data Model

กำหนด Data Models อย่างน้อย 2 รายการ

แต่ละ Model ต้องระบุ:

- Name
- Fields
- Data Types
- Validation Rules
- Relationships (ถ้ามี)

### Part 7: API Design หรือ Application Interface

หากเป็น Backend ให้กำหนด API Endpoints อย่างน้อย 5 รายการ

หากเป็น CLI ให้กำหนด Commands อย่างน้อย 5 รายการ

### Part 8: Development Plan

แบ่งการพัฒนาเป็น Milestones อย่างน้อย 4 ระยะ เช่น:

1. Project Setup
2. Core Features
3. Testing and Security
4. Documentation and Release

## Constraints

- ต้องเลือก Project หนึ่งหัวข้อ
- ต้องมี Functional Requirements อย่างน้อย 5 รายการ
- ต้องมี Non-functional Requirements อย่างน้อย 3 รายการ
- ต้องมี User Stories อย่างน้อย 5 รายการ
- ต้องมี Acceptance Criteria อย่างน้อย 3 รายการ
- ต้องกำหนด MVP Scope
- ต้องมี Architecture Diagram
- ต้องมี Data Models อย่างน้อย 2 รายการ
- ต้องมี API Endpoints หรือ CLI Commands อย่างน้อย 5 รายการ
- ต้องมี Development Milestones อย่างน้อย 4 ระยะ
- ยังไม่จำเป็นต้องเขียน Application Code

## Expected Output

สร้างเอกสาร:

```text
project_planning/
├── README.md
├── requirements.md
├── user_stories.md
├── architecture.md
├── data_models.md
└── development_plan.md
```

เอกสารต้องอธิบาย Project ได้ตั้งแต่ปัญหา Requirements ไปจนถึงแผนการพัฒนา

## Hints

ใช้คำถามเหล่านี้ช่วยวิเคราะห์:

1. ปัญหาคืออะไร?
2. ใครเป็นผู้ใช้งาน?
3. ระบบต้องทำอะไร?
4. ข้อมูลอะไรที่ต้องจัดเก็บ?
5. ระบบประกอบด้วย Components อะไร?
6. มี Error Cases อะไรบ้าง?
7. Feature ใดจำเป็นต่อ MVP?
8. จะทดสอบความสำเร็จอย่างไร?

## Completion Checklist

- [ ] เลือก Project ได้
- [ ] เขียน Project Overview ได้
- [ ] ระบุ Problem Statement ได้
- [ ] ระบุ Target Users ได้
- [ ] เขียน Functional Requirements ได้
- [ ] เขียน Non-functional Requirements ได้
- [ ] เขียน User Stories ได้
- [ ] กำหนด Acceptance Criteria ได้
- [ ] กำหนด MVP Scope ได้
- [ ] สร้าง Architecture Diagram ได้
- [ ] ออกแบบ Data Models ได้
- [ ] ออกแบบ API หรือ CLI ได้
- [ ] วาง Development Milestones ได้

## Deliverables

- Requirements Document
- User Stories
- Architecture Diagram
- Data Model Document
- API หรือ CLI Design
- Development Plan

## Final Review

ให้ผู้อื่นอ่าน Project Plan แล้วตอบได้ว่า:

- ระบบนี้แก้ปัญหาอะไร?
- ใครจะใช้ระบบ?
- Version แรกทำอะไรได้?
- ระบบประกอบด้วยส่วนใดบ้าง?
- จะรู้ได้อย่างไรว่าระบบทำงานถูกต้อง?
