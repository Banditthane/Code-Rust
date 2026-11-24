# Chapter 29: Database & Serialization

## เป้าหมายของบทเรียน

เรียนรู้การจัดเก็บข้อมูลด้วย Database และการแปลงข้อมูลระหว่าง Rust Data Structures กับรูปแบบข้อมูลที่ใช้แลกเปลี่ยนระหว่างระบบ

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบาย Database และ Relational Database ได้
- เข้าใจ Table, Row, Column และ Primary Key
- เข้าใจ Foreign Key และ Relationships
- เขียน SQL พื้นฐานได้
- ใช้ SQLite หรือ PostgreSQL ได้
- เชื่อมต่อ Database จาก Rust ได้
- ทำ CRUD Operations ได้
- ใช้ Parameterized Queries ได้
- เข้าใจ Database Transactions
- ใช้ Serde Serialize และ Deserialize ได้
- จัดการ JSON และ TOML ได้
- ใช้ SQLx เชื่อมต่อ Database แบบ Async ได้
- ออกแบบ Data Models สำหรับ Database ได้
- จัดการ Database Errors ได้

## บทเรียน

| Lesson | หัวข้อ                    | เนื้อหา                                         |
| ------ | ------------------------- | ----------------------------------------------- |
| 01     | Database Fundamentals     | Relational Database, SQL, Tables, Relationships |
| 02     | Database Operations       | Connection, CRUD, Parameters, Transactions      |
| 03     | Serialization             | Serde, JSON, TOML, Serialization                |
| 04     | Rust Database Integration | SQLx, Async Queries, Models, Errors             |

## แบบฝึกหัด

| Exercise | ชื่อ               | ระดับ       |
| -------- | ------------------ | ----------- |
| 01       | SQLite Basics      | Easy        |
| 02       | Database CRUD      | Easy-Medium |
| 03       | Serde Practice     | Medium      |
| 04       | Database Challenge | Challenge   |

## โครงสร้างโฟลเดอร์

```text
29_database_serialization/
├── README.md
├── 01_database_fundamentals/
│   └── README.md
├── 02_database_operations/
│   └── README.md
├── 03_serialization/
│   └── README.md
├── 04_rust_database_integration/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── sqlite_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── database_crud/
    ├── exercise_03/
    │   ├── README.md
    │   └── serde_practice/
    └── exercise_04/
        ├── README.md
        └── database_challenge/
```

## เครื่องมือและ Libraries

- Rust
- Cargo
- SQL
- SQLite
- PostgreSQL
- SQLx
- Serde
- Serde JSON
- TOML

## แนวทางการเรียน

1. เข้าใจพื้นฐาน Database
2. เรียนรู้ SQL และ Relational Database
3. ทดลองใช้ SQLite
4. เชื่อมต่อ Database จาก Rust
5. ทำ CRUD Operations
6. เรียนรู้ Transactions
7. ใช้ Serde กับ Rust Struct
8. Serialize และ Deserialize JSON
9. ทดลองใช้ TOML
10. สร้างระบบ Rust ที่เชื่อมต่อ Database

## ข้อควรระวัง

- ใช้ Parameterized Queries แทนการต่อ String เพื่อสร้าง SQL
- ห้ามนำ User Input ไปต่อใน SQL โดยตรง
- ต้องจัดการ Connection และ Database Errors
- ใช้ Transactions เมื่อมีหลาย Operations ที่ต้องสำเร็จหรือยกเลิกร่วมกัน
- หลีกเลี่ยงการเก็บ Password เป็น Plain Text
- ไม่ควรเปิดเผย Database Credentials
- ต้องพิจารณา Connection Pool สำหรับ Application ที่มีหลาย Requests
- ต้องออกแบบ Migration และ Backup สำหรับระบบที่ใช้งานจริง
- ต้องจัดการข้อมูลที่ไม่สามารถ Deserialize ได้อย่างเหมาะสม

## Completion Checklist

- [ ] เข้าใจ Database
- [ ] เข้าใจ Relational Database
- [ ] เข้าใจ Table, Row และ Column
- [ ] เข้าใจ Primary Key และ Foreign Key
- [ ] เขียน SQL พื้นฐานได้
- [ ] เชื่อมต่อ SQLite ได้
- [ ] เชื่อมต่อ Database จาก Rust ได้
- [ ] ทำ CRUD ได้
- [ ] ใช้ Parameterized Queries ได้
- [ ] เข้าใจ Transactions
- [ ] ใช้ Serde ได้
- [ ] Serialize JSON ได้
- [ ] Deserialize JSON ได้
- [ ] อ่านและเขียน TOML ได้
- [ ] ใช้ SQLx ได้
- [ ] จัดการ Database Errors ได้

## เกณฑ์ผ่านบทเรียน

สามารถสร้าง Rust Application ที่เชื่อมต่อ Database จัดการข้อมูลด้วย CRUD ใช้ Transactions ตามความเหมาะสม และรับส่งข้อมูล JSON ผ่าน Serde ได้

**หลักสำคัญ:** Database ใช้จัดเก็บข้อมูลอย่างมีโครงสร้าง ส่วน Serialization ใช้แปลงข้อมูลเพื่อให้โปรแกรมสามารถแลกเปลี่ยนและจัดเก็บข้อมูลในรูปแบบที่เหมาะสม
