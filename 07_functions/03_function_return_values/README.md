# 03: Function Return Values

## ภาพรวม

Return Value คือค่าที่ Function ส่งกลับไปยังจุดที่เรียกใช้งาน

Function สามารถรับข้อมูลผ่าน Parameters ประมวลผลข้อมูล แล้วส่งผลลัพธ์กลับไปให้ผู้เรียกใช้งานได้

## 1. Return Type คืออะไร?

Return Type คือชนิดข้อมูลของค่าที่ Function จะส่งกลับ

Rust ใช้เครื่องหมาย `->` เพื่อระบุ Return Type

รูปแบบคำสั่ง:

```rust
fn function_name() -> data_type {
    // คำสั่ง
    return value;
}
```

## 2. ตัวอย่างการใช้ return

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let result = add(10, 20);

    println!("Result: {}", result);
}
```

ผลลัพธ์:

```text
Result: 30
```

อธิบายการทำงาน:

1. Function `add` รับตัวเลข 2 ค่า
2. นำตัวเลขทั้งสองมาบวกกัน
3. ใช้ `return` ส่งผลลัพธ์กลับ
4. ตัวแปร `result` รับค่าที่ Function ส่งกลับมา
5. แสดงผลลัพธ์ออกทางหน้าจอ

## 3. การส่งค่ากลับโดยไม่ใช้ return

Rust สามารถใช้ Expression ตัวสุดท้ายของ Function เป็นค่าที่ส่งกลับได้ โดยไม่ต้องเขียน `return` และไม่ใส่เครื่องหมาย `;` ท้าย Expression นั้น

ตัวอย่าง:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(10, 20);

    println!("Result: {}", result);
}
```

ผลลัพธ์:

```text
Result: 30
```

ในตัวอย่างนี้:

- `a + b` เป็น Expression ที่คำนวณผลลัพธ์
- ไม่มี `;` ต่อท้าย จึงส่งค่าผลลัพธ์กลับจาก Function
- Return Type ยังคงเป็น `i32`

## 4. ความแตกต่างระหว่าง Expression และ Statement

ตัวอย่าง:

```rust
fn calculate() -> i32 {
    10 + 20
}
```

`10 + 20` เป็น Expression เพราะมีค่าผลลัพธ์ และ Function ส่งค่านั้นกลับมา

แต่ตัวอย่างนี้:

```rust
fn calculate() -> i32 {
    10 + 20;
}
```

เมื่อใส่ `;` นิพจน์จะกลายเป็น Statement และไม่ได้ส่งค่า `i32` กลับจาก Function ตามที่กำหนดไว้ ทำให้เกิดข้อผิดพลาดในการคอมไพล์

## 5. Function ที่ไม่มี Return Value

หากไม่ได้กำหนด Return Type Function จะส่งค่าชนิด `()` หรือ Unit กลับโดยปริยาย

ตัวอย่าง:

```rust
fn greet() {
    println!("Hello!");
}

fn main() {
    greet();
}
```

Function นี้ทำงานโดยแสดงข้อความ แต่ไม่ได้ส่งค่าผลลัพธ์กลับมาให้ผู้เรียกใช้งาน

## 6. Return Value สามารถนำไปใช้งานต่อได้

ตัวอย่าง:

```rust
fn square(number: i32) -> i32 {
    number * number
}

fn main() {
    println!("Result: {}", square(5));
}
```

ผลลัพธ์:

```text
Result: 25
```

สามารถนำ Function Call ไปใช้เป็น Expression ภายใน `println!()` ได้โดยตรง

## สิ่งที่ควรรู้

- ใช้ `->` เพื่อกำหนด Return Type
- ค่าที่ส่งกลับต้องตรงกับ Return Type
- ใช้ `return` เพื่อส่งค่ากลับได้
- สามารถใช้ Expression สุดท้ายโดยไม่ใส่ `;` เพื่อส่งค่ากลับได้
- การใส่ `;` ทำให้ Expression กลายเป็น Statement
- Function ที่ไม่ได้ระบุ Return Type จะส่งค่า Unit `()` กลับโดยปริยาย

## แบบฝึกหัด

1. สร้าง Function ชื่อ `add` รับตัวเลข 2 ค่า ชนิด `i32`
2. กำหนดให้ Function ส่งผลบวกกลับเป็น `i32`
3. สร้าง Function ชื่อ `subtract` เพื่อส่งผลลบกลับ
4. สร้าง Function ชื่อ `multiply` เพื่อส่งผลคูณกลับ
5. สร้าง Function ชื่อ `divide` เพื่อส่งผลหารกลับ โดยตรวจสอบไม่ให้ตัวหารเป็นศูนย์
6. ทดลองนำ Function Call ไปแสดงผลใน `println!()` โดยตรง
7. ทดลองเก็บค่าที่ส่งกลับไว้ในตัวแปรแล้วนำไปใช้งานต่อ

## คำถามทบทวน

1. Return Value คืออะไร?
2. เครื่องหมาย `->` ใช้ทำอะไร?
3. `return` ใช้ทำอะไร?
4. Expression ที่อยู่ท้าย Function โดยไม่มี `;` มีความหมายอย่างไร?
5. การใส่ `;` ท้าย Expression ส่งผลอย่างไร?
6. Function ที่ไม่มี Return Type จะส่งค่าชนิดใดกลับ?
