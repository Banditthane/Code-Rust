# Lesson 01: Closure Basics

## 1. Closure คืออะไร?

Closure คือฟังก์ชันรูปแบบหนึ่งใน Rust ที่สามารถกำหนดและใช้งานได้ภายในขอบเขตของโค้ด โดยมีความสามารถในการเข้าถึงตัวแปรจากขอบเขตภายนอกได้

Closure มีลักษณะคล้าย Anonymous Function หรือฟังก์ชันที่ไม่มีชื่อ

## 2. รูปแบบการเขียน Closure

รูปแบบพื้นฐาน:

```rust
|parameters| expression
```

ตัวอย่าง:

```rust
let add = |a, b| a + b;

println!("{}", add(10, 20));
```

ผลลัพธ์:

```text
30
```

คำอธิบาย:

- `let add` สร้างตัวแปรเพื่อเก็บ Closure
- `|a, b|` กำหนดพารามิเตอร์
- `a + b` เป็นนิพจน์ที่ Closure ประมวลผล
- `add(10, 20)` เรียกใช้งาน Closure

## 3. Closure ที่มีหลายคำสั่ง

Closure สามารถมีหลายคำสั่งได้ โดยใช้ `{}` ครอบเนื้อหา

```rust
let calculate = |a, b| {
    let result = a + b;
    result * 2
};

println!("{}", calculate(10, 5));
```

ผลลัพธ์:

```text
30
```

นิพจน์สุดท้ายที่ไม่มีเครื่องหมาย `;` จะเป็นค่าที่ Closure คืนกลับ

## 4. Parameters และ Type Inference

Rust สามารถอนุมานชนิดข้อมูลของพารามิเตอร์จากบริบทการใช้งานได้

```rust
let square = |number: i32| number * number;

println!("{}", square(5));
```

สามารถระบุชนิดข้อมูลอย่างชัดเจนได้เช่นกัน

```rust
let multiply = |a: i32, b: i32| -> i32 {
    a * b
};
```

ข้อควรจำ:

- Closure มักใช้ Type Inference
- สามารถระบุชนิดข้อมูลเองได้
- เมื่อ Closure ถูกเรียกใช้งานครั้งแรก Rust จะกำหนดชนิดของพารามิเตอร์ตามบริบท
- Closure ตัวเดิมไม่สามารถใช้กับชนิดพารามิเตอร์หลายชนิดได้อย่างอิสระเหมือน Generic Function ทั่วไป

## 5. Return Value

Closure สามารถคืนค่าได้เหมือนฟังก์ชันทั่วไป

```rust
let get_message = || "Hello Rust";

println!("{}", get_message());
```

หากมีหลายคำสั่ง สามารถใช้คำสั่งสุดท้ายเป็น Return Value ได้

## 6. Capturing the Environment

ความสามารถสำคัญของ Closure คือการเข้าถึงตัวแปรที่อยู่ภายนอก

```rust
let message = String::from("Hello");

let print_message = || {
    println!("{}", message);
};

print_message();
```

Closure สามารถจับค่าจาก Environment ภายนอกได้ตามลักษณะการใช้งาน เช่น:

- ยืมแบบ Immutable Reference
- ยืมแบบ Mutable Reference
- รับ Ownership ของค่า

รายละเอียดเรื่องลักษณะการจับค่าจะเรียนต่อในบท Closure Traits

## 7. Closure กับ Function ต่างกันอย่างไร?

| หัวข้อ      | Function                 | Closure                 |
| ----------- | ------------------------ | ----------------------- | --- | --- |
| การประกาศ   | ใช้ `fn`                 | ใช้ `                   |     | `   |
| ชื่อ        | มีชื่อได้                | มักเก็บไว้ในตัวแปร      |
| Parameters  | ระบุชนิดข้อมูล           | มักอนุมานชนิดข้อมูล     |
| Environment | ไม่จับตัวแปรภายนอกโดยตรง | สามารถจับตัวแปรภายนอก   |
| การใช้งาน   | เรียกผ่านชื่อฟังก์ชัน    | เรียกผ่านตัวแปร Closure |

## 8. ข้อควรระวัง

- Closure ไม่ใช่ Function ที่มีความสามารถเหมือนกันทุกประการ
- Closure แต่ละตัวมีชนิดเฉพาะของตัวเอง
- การจับค่าจาก Environment มีผลต่อ Ownership และ Borrowing
- อย่าสับสนระหว่างการสร้าง Closure กับการเรียกใช้งาน Closure

## 9. สรุป

สิ่งที่ต้องเข้าใจ:

- Closure คือ Anonymous Function
- ใช้ `|parameters|` ในการประกาศ
- สามารถรับพารามิเตอร์และคืนค่าได้
- สามารถเข้าถึงตัวแปรภายนอกได้
- สามารถใช้ Type Inference ได้

## Checklist

- [ ] เข้าใจความหมายของ Closure
- [ ] เขียน Closure แบบไม่มีพารามิเตอร์ได้
- [ ] เขียน Closure แบบมีพารามิเตอร์ได้
- [ ] เขียน Closure ที่คืนค่าได้
- [ ] ใช้ Closure ที่มีหลายคำสั่งได้
- [ ] เข้าใจ Environment Capture เบื้องต้น
