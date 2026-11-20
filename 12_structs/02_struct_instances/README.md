# Lesson 02: Struct Instances

## 1. What is a Struct Instance?

หลังจากประกาศ Struct แล้ว เราสามารถสร้างข้อมูลจริงจาก Struct นั้นได้ สิ่งนี้เรียกว่า **Struct Instance**

ตัวอย่างเช่น เราประกาศ `Student` เป็นชนิดข้อมูล แล้วสร้างนักเรียนชื่อ Alice และ Bob เป็น Instances คนละตัวได้

## 2. Creating an Instance

รูปแบบการสร้าง Instance:

```rust
let variable_name = StructName {
    field_name: value,
};
```

ตัวอย่าง:

```rust
struct Student {
    name: String,
    age: u8,
    score: f64,
}

fn main() {
    let student = Student {
        name: String::from("Alice"),
        age: 20,
        score: 85.5,
    };
}
```

สิ่งที่เกิดขึ้น:

- `Student` คือ Struct ที่ประกาศไว้
- `student` คือ Instance ที่สร้างขึ้น
- แต่ละ Field ได้รับค่าตาม Data Type ที่กำหนดไว้

## 3. Accessing Fields

ใช้เครื่องหมายจุด `.` เพื่อเข้าถึง Fields

```rust
println!("Name: {}", student.name);
println!("Age: {}", student.age);
println!("Score: {}", student.score);
```

สามารถเข้าถึง Field เพื่อนำไปแสดงผลหรือใช้คำนวณได้

## 4. Mutable Struct Instance

หากต้องการแก้ไขค่า Field ต้องประกาศ Instance ด้วย `mut`

```rust
let mut student = Student {
    name: String::from("Alice"),
    age: 20,
    score: 85.5,
};

student.age = 21;
student.score = 90.0;
```

ข้อควรจำ:

- `let student` ไม่อนุญาตให้แก้ไข Fields ผ่าน Instance นั้น
- `let mut student` อนุญาตให้แก้ไข Fields ได้
- ความเป็น Mutable อยู่ที่ Instance ทั้งตัว ไม่ได้ประกาศ `mut` แยกเป็นราย Field

## 5. Field Init Shorthand

หากชื่อตัวแปรตรงกับชื่อ Field สามารถเขียนแบบย่อได้

ตัวอย่าง:

```rust
let name = String::from("Alice");
let age = 20;

let student = Student {
    name,
    age,
    score: 85.5,
};
```

Rust จะใช้ค่าจากตัวแปร `name` และ `age` ใส่ใน Fields ที่ชื่อเดียวกัน

## 6. Struct Update Syntax

สามารถสร้าง Instance ใหม่โดยนำค่าบางส่วนจาก Instance เดิมมาใช้ได้

ตัวอย่าง:

```rust
let student1 = Student {
    name: String::from("Alice"),
    age: 20,
    score: 85.5,
};

let student2 = Student {
    name: String::from("Bob"),
    ..student1
};
```

`..student1` หมายถึงนำค่าของ Fields ที่ไม่ได้ระบุใน Instance ใหม่มาจาก `student1`

ข้อควรระวัง:

- Fields ที่ระบุใหม่จะใช้ค่าที่กำหนดไว้
- Fields ที่เหลือจะนำค่าจาก Instance เดิมมาใช้
- การย้ายค่า (Move) ของ Fields ที่ไม่ได้เป็น `Copy` อาจทำให้ไม่สามารถใช้ Instance เดิมได้ครบทุก Field อีกต่อไป

## 7. Important Notes

- Struct Instance คือข้อมูลจริงที่สร้างจาก Struct
- เข้าถึง Fields ด้วย `instance.field`
- ใช้ `mut` เมื่อจำเป็นต้องแก้ไข Fields
- Field Init Shorthand ช่วยลดการเขียนซ้ำ
- Struct Update Syntax ช่วยสร้าง Instance ใหม่จากข้อมูลเดิม
- ต้องคำนึงถึง Ownership เมื่อย้ายค่าจาก Instance เดิม

## 8. Knowledge Check

1. Struct Instance คืออะไร?
2. ใช้เครื่องหมายใดในการเข้าถึง Field?
3. เพราะเหตุใดจึงต้องใช้ `mut` เมื่อต้องการแก้ไข Field?
4. Field Init Shorthand ช่วยลดการเขียนโค้ดอย่างไร?
5. `..student1` ใน Struct Update Syntax มีความหมายอย่างไร?

## 9. Completion Checklist

- [ ] สร้าง Struct Instance ได้
- [ ] กำหนดค่าให้ทุก Field ได้
- [ ] เข้าถึง Fields ด้วยเครื่องหมายจุดได้
- [ ] แก้ไข Fields ผ่าน Mutable Instance ได้
- [ ] ใช้ Field Init Shorthand ได้
- [ ] เข้าใจ Struct Update Syntax
- [ ] เข้าใจผลกระทบของ Ownership เมื่อใช้ Instance เดิม
