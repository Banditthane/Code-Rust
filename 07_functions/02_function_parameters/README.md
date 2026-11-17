# 02: Function Parameters และ Arguments

## ภาพรวม

Parameters ช่วยให้ Function รับข้อมูลจากภายนอกเข้ามาใช้งาน ทำให้ Function เดียวสามารถทำงานกับข้อมูลที่แตกต่างกันได้

ตัวอย่างเช่น Function สำหรับแสดงชื่อ สามารถรับชื่อจากผู้เรียกใช้งาน แทนการกำหนดชื่อไว้ตายตัวภายใน Function

## 1. Parameters คืออะไร?

Parameters คือตัวแปรที่กำหนดไว้ในวงเล็บของการประกาศ Function โดยต้องระบุชนิดข้อมูลด้วย

ตัวอย่าง:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

ในตัวอย่างนี้:

- `name` คือ Parameter
- `&str` คือชนิดข้อมูลของ Parameter

## 2. Arguments คืออะไร?

Arguments คือค่าที่ส่งเข้าไปให้ Function เมื่อเรียกใช้งาน

ตัวอย่าง:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alex");
    greet("John");
}
```

ผลลัพธ์:

```text
Hello, Alex!
Hello, John!
```

อธิบาย:

- `"Alex"` และ `"John"` คือ Arguments
- ค่าเหล่านี้ถูกส่งเข้าไปยัง Parameter ชื่อ `name`

## 3. Function ที่มีหลาย Parameters

Function สามารถรับ Parameters ได้มากกว่าหนึ่งตัว โดยคั่นแต่ละตัวด้วยเครื่องหมาย `,`

ตัวอย่าง:

```rust
fn show_info(name: &str, age: u32) {
    println!("Name: {}", name);
    println!("Age: {}", age);
}

fn main() {
    show_info("Alex", 20);
}
```

ผลลัพธ์:

```text
Name: Alex
Age: 20
```

## 4. Parameters ต้องระบุชนิดข้อมูล

Rust ต้องการให้ระบุชนิดข้อมูลของ Parameters อย่างชัดเจน

ตัวอย่าง:

```rust
fn add(a: i32, b: i32) {
    println!("{}", a + b);
}

fn main() {
    add(10, 20);
}
```

ผลลัพธ์:

```text
30
```

ในตัวอย่างนี้:

- `a` มีชนิดข้อมูลเป็น `i32`
- `b` มีชนิดข้อมูลเป็น `i32`
- `10` และ `20` เป็น Arguments ที่ส่งเข้าไป

## สิ่งที่ควรรู้

- Function สามารถรับ Parameters ได้หลายตัว
- Parameters ต้องระบุชนิดข้อมูล
- Arguments ต้องส่งให้ตรงกับจำนวนและลำดับของ Parameters
- ชนิดข้อมูลของ Arguments ต้องสอดคล้องกับ Parameters
- Function สามารถนำ Parameters ไปคำนวณหรือประมวลผลได้

## แบบฝึกหัด

1. สร้าง Function ชื่อ `greet` โดยรับ Parameter ชื่อ `name` ชนิด `&str`
2. ให้ Function แสดงข้อความทักทายพร้อมชื่อ
3. สร้าง Function ชื่อ `show_info` โดยรับ `name` ชนิด `&str` และ `age` ชนิด `u32`
4. ให้ Function แสดงชื่อและอายุ
5. สร้าง Function ชื่อ `multiply` โดยรับตัวเลข 2 ค่า ชนิด `i32`
6. ให้ Function แสดงผลคูณของตัวเลขทั้งสองค่า
7. ทดลองเรียกแต่ละ Function ด้วย Arguments ที่แตกต่างกัน

## คำถามทบทวน

1. Parameter คืออะไร?
2. Argument คืออะไร?
3. Parameters และ Arguments แตกต่างกันอย่างไร?
4. เพราะเหตุใด Rust จึงต้องระบุชนิดข้อมูลของ Parameters?
5. หากส่ง Arguments ไม่ตรงกับจำนวนหรือชนิดข้อมูลที่กำหนด จะเกิดอะไรขึ้น?
