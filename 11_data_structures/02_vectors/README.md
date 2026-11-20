# Lesson 02: Vectors

## 1. What is a Vector?

Vector หรือ `Vec<T>` คือ Collection ที่ใช้เก็บข้อมูลหลายค่า โดยข้อมูลทุกตัวภายใน Vector ต้องเป็นชนิดเดียวกัน

ข้อแตกต่างจาก Array คือ Vector สามารถเพิ่มหรือลบจำนวนสมาชิกได้ระหว่างการทำงานของโปรแกรม

ตัวอย่าง:

```rust
fn main() {
    let numbers = vec![10, 20, 30, 40];

    println!("{:?}", numbers);
}
```

Output:

```text
[10, 20, 30, 40]
```

## 2. Creating a Vector

มีหลายวิธีในการสร้าง Vector

### Using vec! Macro

```rust
let numbers = vec![10, 20, 30];
```

### Using Vec::new()

```rust
let numbers: Vec<i32> = Vec::new();
```

`Vec::new()` สร้าง Vector ว่าง โดยตัวอย่างนี้ระบุชนิดข้อมูลเป็น `i32`

## 3. Accessing Vector Elements

เข้าถึงข้อมูลด้วย Index ได้เหมือน Array

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    println!("{}", numbers[0]);
    println!("{}", numbers[1]);
    println!("{}", numbers[2]);
}
```

Output:

```text
10
20
30
```

อีกวิธีหนึ่งคือใช้ `.get()`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    println!("{:?}", numbers.get(1));
    println!("{:?}", numbers.get(5));
}
```

Output:

```text
Some(20)
None
```

ความแตกต่าง:

- `numbers[index]` จะ Panic หาก Index อยู่นอกขอบเขต
- `numbers.get(index)` จะคืนค่า `Option<&T>` ซึ่งเป็น `Some` เมื่อพบข้อมูล และ `None` เมื่อไม่พบ

## 4. Adding Elements

ใช้ `.push()` เพื่อเพิ่มข้อมูลท้าย Vector

```rust
fn main() {
    let mut numbers = vec![10, 20, 30];

    numbers.push(40);
    numbers.push(50);

    println!("{:?}", numbers);
}
```

Output:

```text
[10, 20, 30, 40, 50]
```

ต้องใช้ `mut` เพราะกำลังแก้ไข Vector

## 5. Removing Elements

ใช้ `.pop()` เพื่อลบและคืนค่าสมาชิกตัวสุดท้าย

```rust
fn main() {
    let mut numbers = vec![10, 20, 30];

    let removed = numbers.pop();

    println!("{:?}", numbers);
    println!("{:?}", removed);
}
```

Output:

```text
[10, 20]
Some(30)
```

`.pop()` คืนค่า `Option<T>` เพราะ Vector อาจไม่มีข้อมูลให้ลบ

## 6. Useful Vector Methods

| Method             | Description                   |
| ------------------ | ----------------------------- |
| `push(value)`      | เพิ่มข้อมูลท้าย Vector        |
| `pop()`            | ลบและคืนค่าตัวสุดท้าย         |
| `len()`            | นับจำนวนสมาชิก                |
| `is_empty()`       | ตรวจสอบว่า Vector ว่างหรือไม่ |
| `get(index)`       | เข้าถึงข้อมูลอย่างปลอดภัย     |
| `remove(index)`    | ลบข้อมูลตาม Index             |
| `contains(&value)` | ตรวจสอบว่ามีข้อมูลหรือไม่     |
| `clear()`          | ลบสมาชิกทั้งหมด               |

ตัวอย่าง:

```rust
fn main() {
    let mut numbers = vec![10, 20, 30];

    numbers.push(40);

    println!("Length: {}", numbers.len());
    println!("Contains 20: {}", numbers.contains(&20));

    numbers.remove(0);

    println!("{:?}", numbers);
}
```

## 7. Loop Through a Vector

ใช้ `for` เพื่ออ่านสมาชิกทุกตัว

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    for number in &numbers {
        println!("{}", number);
    }
}
```

การใช้ `&numbers` เป็นการยืม Vector เพื่ออ่านข้อมูล โดยไม่ย้าย Ownership ออกจากตัวแปร `numbers`

## 8. Important Notes

- Vector เก็บข้อมูลชนิดเดียวกัน
- Vector สามารถเพิ่มและลบสมาชิกได้
- ใช้ `mut` เมื่อจำเป็นต้องแก้ไข Vector
- `get()` เหมาะกับกรณีที่ Index อาจไม่มีข้อมูล
- การวนลูปด้วย `&numbers` เป็นการยืมข้อมูลมาอ่าน
- การแก้ไขสมาชิกผ่าน Reference ต้องใช้ Mutable Borrow ให้ถูกต้อง

## Practice

ทดลองเขียนโปรแกรมเพื่อ:

1. สร้าง Vector ที่เก็บคะแนน 5 ค่า
2. เพิ่มคะแนนอีก 1 ค่า
3. ลบสมาชิกตัวสุดท้ายด้วย `.pop()`
4. แสดงจำนวนสมาชิกด้วย `.len()`
5. ตรวจสอบว่ามีคะแนน 80 หรือไม่
6. วนลูปแสดงคะแนนทั้งหมด

## Knowledge Check

1. Vector แตกต่างจาก Array อย่างไร?
2. `vec![]` ใช้ทำอะไร?
3. เพราะเหตุใดจึงต้องใช้ `mut` เมื่อเรียก `.push()`?
4. `.pop()` คืนค่าประเภทใด?
5. `get()` แตกต่างจากการใช้ Index อย่างไร?

## Completion Checklist

- [ ] สร้าง Vector ด้วย `vec![]`
- [ ] สร้าง Vector ด้วย `Vec::new()`
- [ ] เข้าถึงข้อมูลด้วย Index และ `.get()`
- [ ] เพิ่มข้อมูลด้วย `.push()`
- [ ] ลบข้อมูลด้วย `.pop()` และ `.remove()`
- [ ] วนลูปอ่านข้อมูลใน Vector ได้
