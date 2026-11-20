# Lesson 01: Struct Basics

## 1. What is a Struct?

`struct` คือวิธีสร้างชนิดข้อมูลของเราเองในภาษา Rust โดยสามารถรวบรวมข้อมูลหลายรายการที่เกี่ยวข้องกันไว้ในโครงสร้างเดียว

ตัวอย่างเช่น ข้อมูลนักเรียนประกอบด้วย:

- name
- age
- score
- is_active

หากไม่ใช้ Struct เราอาจต้องประกาศตัวแปรแยกกัน แต่ Struct ช่วยให้ข้อมูลเหล่านี้อยู่ภายใต้ชนิดข้อมูลเดียวกัน

## 2. Struct Declaration

รูปแบบการประกาศ Struct:

```rust
struct StructName {
    field_name: DataType,
}
```

ตัวอย่าง:

```rust
struct Student {
    name: String,
    age: u8,
    score: f64,
    is_active: bool,
}
```

อธิบาย:

- `struct` ใช้ประกาศชนิดข้อมูลแบบ Struct
- `Student` คือชื่อ Struct
- `name`, `age`, `score`, `is_active` คือ Fields
- `String`, `u8`, `f64`, `bool` คือชนิดข้อมูลของแต่ละ Field

ข้อควรจำ: การประกาศ Struct เป็นการสร้างชนิดข้อมูล ยังไม่ได้สร้างข้อมูลนักเรียนจริง ๆ

## 3. Struct Fields

Field คือข้อมูลแต่ละรายการที่อยู่ภายใน Struct

ตัวอย่าง:

```rust
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}
```

Struct `Product` มี 3 Fields:

| Field    | Data Type | Purpose     |
| -------- | --------- | ----------- |
| name     | String    | ชื่อสินค้า  |
| price    | f64       | ราคาสินค้า  |
| quantity | u32       | จำนวนสินค้า |

แต่ละ Field สามารถใช้ Data Type แตกต่างกันได้ตามความเหมาะสม

## 4. Naming Convention

แนวทางการตั้งชื่อ Struct ใน Rust:

- ใช้ PascalCase สำหรับชื่อ Struct เช่น `Student`, `Product`
- ใช้ snake_case สำหรับชื่อ Fields เช่น `student_name`, `total_score`
- ตั้งชื่อให้สื่อความหมาย

ตัวอย่าง:

```rust
struct Employee {
    employee_name: String,
    department: String,
    salary: u32,
}
```

## 5. Important Notes

- Struct ใช้สร้าง Custom Data Types
- Struct หนึ่งตัวสามารถมีหลาย Fields
- แต่ละ Field สามารถมี Data Type ต่างกันได้
- การประกาศ Struct ยังไม่ได้สร้าง Instance
- การสร้าง Instance จะเรียนใน Lesson 02

## 6. Knowledge Check

1. `struct` ใช้ทำอะไร?
2. Field คืออะไร?
3. Struct สามารถมี Fields ที่เป็น Data Type ต่างกันได้หรือไม่?
4. `Student` และ `student_name` ใช้รูปแบบการตั้งชื่อแบบใด?
5. การประกาศ Struct หมายถึงการสร้าง Instance แล้วหรือยัง?

## 7. Completion Checklist

- [ ] เข้าใจความหมายของ Struct
- [ ] เข้าใจความแตกต่างระหว่าง Struct และ Field
- [ ] ประกาศ Struct ได้
- [ ] กำหนด Data Type ให้ Fields ได้
- [ ] ตั้งชื่อ Struct และ Fields ตาม Convention ได้
