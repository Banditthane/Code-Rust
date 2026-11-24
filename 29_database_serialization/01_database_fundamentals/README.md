# Lesson 01: Database Fundamentals

## 1. Database คืออะไร?

Database คือระบบสำหรับจัดเก็บ จัดการ และค้นหาข้อมูลอย่างเป็นระบบ

ตัวอย่างการใช้งาน:

- ระบบสมาชิก
- ระบบสินค้า
- ระบบคำสั่งซื้อ
- ระบบบัญชี
- ระบบจัดการงาน
- ระบบโรงพยาบาล

Database ช่วยให้ Application สามารถบันทึกและเรียกใช้ข้อมูลได้ แม้โปรแกรมจะปิดและเปิดใหม่

## 2. Database Management System

DBMS (Database Management System) คือซอฟต์แวร์ที่ใช้จัดการ Database

ตัวอย่าง:

| Database   | ประเภท               |
| ---------- | -------------------- |
| SQLite     | Relational Database  |
| PostgreSQL | Relational Database  |
| MySQL      | Relational Database  |
| MongoDB    | Document Database    |
| Redis      | In-memory Data Store |

บทนี้จะเน้น Relational Database โดยใช้ SQLite เป็นตัวเริ่มต้น และแนะนำ PostgreSQL สำหรับระบบที่ใหญ่ขึ้น

## 3. Relational Database

Relational Database จัดเก็บข้อมูลในรูปแบบ Tables และกำหนดความสัมพันธ์ระหว่าง Tables

ตัวอย่าง Table: `users`

| id  | name    | email               |
| --- | ------- | ------------------- |
| 1   | Alice   | alice@example.com   |
| 2   | Bob     | bob@example.com     |
| 3   | Charlie | charlie@example.com |

องค์ประกอบ:

- Table: ตารางข้อมูล
- Row: แถวข้อมูลหนึ่งรายการ
- Column: คอลัมน์หรือคุณสมบัติของข้อมูล
- Schema: โครงสร้างของ Database
- Record: ข้อมูลหนึ่งรายการ

## 4. Primary Key

Primary Key คือ Column หรือชุดของ Columns ที่ใช้ระบุแต่ละ Row อย่างไม่ซ้ำกัน

ตัวอย่าง:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL
);
```

ใน SQLite การประกาศ `INTEGER PRIMARY KEY` มีพฤติกรรมพิเศษที่เชื่อมโยงกับ RowID ของตาราง

ข้อควรจำ:

- Primary Key ต้องระบุ Row ได้อย่างไม่ซ้ำ
- ตารางมี Primary Key ได้หนึ่ง Constraint ซึ่งอาจประกอบด้วยหลาย Columns
- Primary Key ไม่ควรใช้ค่าที่เปลี่ยนแปลงบ่อย

## 5. Foreign Key

Foreign Key ใช้กำหนดความสัมพันธ์ระหว่าง Tables

ตัวอย่าง:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE orders (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    total REAL NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

`orders.user_id` อ้างอิง `users.id`

ใน SQLite ต้องเปิดใช้งาน Foreign Key Enforcement สำหรับ Connection นั้น เช่น:

```sql
PRAGMA foreign_keys = ON;
```

## 6. Relationships

ความสัมพันธ์ระหว่าง Tables มีหลายรูปแบบ

### One-to-One

หนึ่ง User มี Profile หนึ่งรายการ

### One-to-Many

หนึ่ง User มี Orders ได้หลายรายการ

### Many-to-Many

หนึ่ง Student เรียนได้หลาย Courses และหนึ่ง Course มี Students ได้หลายคน

กรณี Many-to-Many มักใช้ Junction Table เชื่อมความสัมพันธ์

ตัวอย่าง:

```sql
CREATE TABLE students (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE courses (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE student_courses (
    student_id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    PRIMARY KEY (student_id, course_id),
    FOREIGN KEY (student_id) REFERENCES students(id),
    FOREIGN KEY (course_id) REFERENCES courses(id)
);
```

## 7. SQL คืออะไร?

SQL (Structured Query Language) ใช้จัดการข้อมูลใน Relational Database

คำสั่งหลัก:

| คำสั่ง | หน้าที่               |
| ------ | --------------------- |
| CREATE | สร้าง Database Object |
| INSERT | เพิ่มข้อมูล           |
| SELECT | อ่านข้อมูล            |
| UPDATE | แก้ไขข้อมูล           |
| DELETE | ลบข้อมูล              |
| ALTER  | เปลี่ยนโครงสร้าง      |
| DROP   | ลบ Database Object    |

## 8. CREATE TABLE

สร้าง Table:

```sql
CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    stock INTEGER NOT NULL DEFAULT 0
);
```

คำอธิบาย:

- `id`: Primary Key
- `name`: ชื่อสินค้า
- `price`: ราคาสินค้า
- `stock`: จำนวนสินค้า
- `NOT NULL`: ไม่อนุญาตให้เป็น NULL
- `DEFAULT 0`: ใช้ค่าเริ่มต้นเป็น 0

## 9. INSERT

เพิ่มข้อมูล:

```sql
INSERT INTO products (name, price, stock)
VALUES ('Keyboard', 1200, 10);
```

เพิ่มหลายรายการ:

```sql
INSERT INTO products (name, price, stock)
VALUES
    ('Mouse', 500, 20),
    ('Monitor', 4500, 5);
```

## 10. SELECT

อ่านข้อมูล:

```sql
SELECT * FROM products;
```

เลือกเฉพาะ Columns:

```sql
SELECT name, price
FROM products;
```

ใช้ WHERE:

```sql
SELECT *
FROM products
WHERE price > 1000;
```

เรียงลำดับ:

```sql
SELECT *
FROM products
ORDER BY price DESC;
```

## 11. UPDATE

แก้ไขข้อมูล:

```sql
UPDATE products
SET price = 1300
WHERE id = 1;
```

ควรตรวจสอบ WHERE ให้ถูกต้อง เพราะ UPDATE ที่ไม่มี WHERE อาจแก้ไขทุก Row

## 12. DELETE

ลบข้อมูล:

```sql
DELETE FROM products
WHERE id = 1;
```

ควรตรวจสอบเงื่อนไขก่อนลบข้อมูลเสมอ

## 13. SQLite

SQLite เป็น Relational Database ที่เก็บข้อมูลในไฟล์

คุณสมบัติ:

- ไม่จำเป็นต้องเปิด Database Server แยก
- เหมาะสำหรับการเรียนรู้และ Application ขนาดเล็กถึงปานกลางในหลายกรณี
- ใช้ไฟล์ Database
- รองรับ SQL
- ใช้งานง่ายสำหรับ Local Application

ตัวอย่างไฟล์:

```text
app.db
```

SQLite เหมาะสำหรับเริ่มต้นเรียนรู้ Database ก่อนเปลี่ยนไปใช้ PostgreSQL

## 14. PostgreSQL

PostgreSQL เป็น Relational Database Server ที่รองรับการใช้งานในระบบหลายรูปแบบ

คุณสมบัติ:

- รองรับ Concurrent Connections
- มีระบบ Transaction
- รองรับ Index และ Constraints
- มีชนิดข้อมูลหลากหลาย
- เหมาะกับ Backend และ Web Application

## 15. SQLite กับ PostgreSQL

| รายการ      | SQLite                         | PostgreSQL                        |
| ----------- | ------------------------------ | --------------------------------- |
| รูปแบบ      | Embedded Database              | Database Server                   |
| การติดตั้ง  | เริ่มใช้งานง่าย                | ต้องติดตั้งหรือใช้ Service        |
| การจัดเก็บ  | ไฟล์ Database                  | Database Server Storage           |
| การใช้งาน   | Local Apps, Prototypes         | Backend, Multi-user Systems       |
| Concurrency | มีข้อจำกัดด้านการเขียนพร้อมกัน | รองรับ Concurrent Workloads ได้ดี |
| การจัดการ   | เรียบง่าย                      | มีระบบจัดการ Server               |

## 16. สรุป

บทนี้ได้เรียนรู้:

- Database และ DBMS
- Relational Database
- Table, Row และ Column
- Primary Key
- Foreign Key
- Relationships
- SQL
- SQLite
- PostgreSQL

## Completion Checklist

- [ ] อธิบาย Database ได้
- [ ] เข้าใจ DBMS
- [ ] เข้าใจ Relational Database
- [ ] เข้าใจ Table, Row และ Column
- [ ] เข้าใจ Primary Key
- [ ] เข้าใจ Foreign Key
- [ ] เข้าใจ One-to-One
- [ ] เข้าใจ One-to-Many
- [ ] เข้าใจ Many-to-Many
- [ ] ใช้ CREATE TABLE ได้
- [ ] ใช้ INSERT ได้
- [ ] ใช้ SELECT ได้
- [ ] ใช้ UPDATE ได้
- [ ] ใช้ DELETE ได้
- [ ] เข้าใจ SQLite และ PostgreSQL
