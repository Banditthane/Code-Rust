# Lesson 04: Nested Data

## 1. What is Nested Data?

Nested Data คือการจัดเก็บข้อมูลที่มีโครงสร้างข้อมูลอื่นอยู่ภายในอีกชั้นหนึ่ง

ตัวอย่างเช่น:

- Vector ที่เก็บ Vector อีกชุดหนึ่ง
- HashMap ที่มี Value เป็น Vector
- ข้อมูลนักเรียนที่มีรายการคะแนนหลายวิชา

แนวคิดนี้ช่วยจัดกลุ่มข้อมูลที่เกี่ยวข้องกัน

## 2. Nested Vector

สามารถสร้าง Vector ที่เก็บ Vector ได้

```rust
fn main() {
    let numbers = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    println!("{:?}", numbers);
}
```

Output:

```text
[[1, 2, 3], [4, 5, 6], [7, 8, 9]]
```

ข้อมูลชุดนี้มี Vector ชั้นนอก 1 ชุด และมี Vector ชั้นใน 3 ชุด

## 3. Accessing Nested Vector

การเข้าถึงข้อมูลใช้ Index หลายชั้น

```rust
fn main() {
    let numbers = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    println!("{}", numbers[0][0]);
    println!("{}", numbers[1][2]);
    println!("{}", numbers[2][1]);
}
```

Output:

```text
1
6
8
```

อธิบาย:

- `numbers[0]` เลือก Vector ชั้นในตัวแรก
- `numbers[0][0]` เลือกสมาชิกตัวแรกของ Vector ชั้นในตัวแรก
- `numbers[1][2]` เลือกสมาชิก index 2 ของ Vector ชั้นในตัวที่สอง

ข้อควรระวัง: การใช้ Index ที่ไม่มีอยู่จริงจะทำให้เกิด Panic

## 4. Loop Through Nested Vector

ใช้ Loop ซ้อนกันเพื่ออ่านข้อมูลทุกชั้น

```rust
fn main() {
    let numbers = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    for row in &numbers {
        for number in row {
            print!("{} ", number);
        }

        println!();
    }
}
```

Output:

```text
1 2 3
4 5 6
7 8 9
```

แนวคิด:

- Loop ชั้นนอกอ่าน Vector แต่ละชุด
- Loop ชั้นในอ่านสมาชิกของ Vector ชุดนั้น

## 5. HashMap with Vector

HashMap สามารถเก็บ Vector เป็น Value ได้

ตัวอย่าง: เก็บคะแนนของนักเรียนแต่ละคน

```rust
use std::collections::HashMap;

fn main() {
    let mut students = HashMap::new();

    students.insert("Alice", vec![80, 90, 85]);
    students.insert("Bob", vec![70, 75, 80]);

    println!("{:?}", students.get("Alice"));
}
```

Output:

```text
Some([80, 90, 85])
```

ในตัวอย่างนี้:

- Key คือชื่อนักเรียน
- Value คือ Vector ที่เก็บคะแนน

## 6. Reading Nested HashMap Values

สามารถอ่านข้อมูลชั้นในผ่านค่าที่ได้จาก `.get()`

```rust
use std::collections::HashMap;

fn main() {
    let mut students = HashMap::new();

    students.insert("Alice", vec![80, 90, 85]);

    if let Some(scores) = students.get("Alice") {
        println!("{:?}", scores);
        println!("First score: {}", scores[0]);
    }
}
```

Output:

```text
[80, 90, 85]
First score: 80
```

`if let Some(scores)` ใช้ตรวจสอบว่าค้นหาข้อมูลพบก่อนนำ Vector มาใช้งาน

## 7. Important Notes

- Nested Data คือการวางโครงสร้างข้อมูลซ้อนกัน
- สามารถใช้ Vector ซ้อน Vector ได้
- สามารถใช้ HashMap เก็บ Vector เป็น Value ได้
- การเข้าถึงข้อมูลหลายชั้นต้องระวัง Index
- การใช้ Loop ซ้อนกันช่วยอ่านข้อมูลหลายระดับ
- ควรเลือกโครงสร้างให้สอดคล้องกับลักษณะข้อมูล

## Practice

ทดลองเขียนโปรแกรมเพื่อ:

1. สร้าง Vector ที่เก็บคะแนนนักเรียน 3 คน
2. นักเรียนแต่ละคนมีคะแนน 3 วิชา
3. ใช้ Loop ซ้อนกันเพื่อแสดงคะแนนทั้งหมด
4. สร้าง HashMap ที่ใช้ชื่อนักเรียนเป็น Key
5. เก็บ Vector คะแนนเป็น Value
6. ค้นหาคะแนนของนักเรียนหนึ่งคน

## Knowledge Check

1. Nested Data คืออะไร?
2. `numbers[1][2]` หมายถึงอะไร?
3. ทำไม Nested Vector จึงใช้ Loop ซ้อนกันได้?
4. HashMap สามารถเก็บ Vector เป็น Value ได้หรือไม่?
5. ทำไมควรตรวจสอบผลลัพธ์จาก `.get()` ก่อนนำข้อมูลไปใช้?

## Completion Checklist

- [ ] เข้าใจ Nested Data
- [ ] สร้าง Nested Vector ได้
- [ ] เข้าถึงข้อมูลหลายชั้นได้
- [ ] ใช้ Nested Loop ได้
- [ ] ใช้ HashMap เก็บ Vector ได้
- [ ] อ่านข้อมูลจาก Nested Structure ได้
