# Lesson 01: Slices

## 1. What is a Slice?

Slice คือการอ้างอิงไปยังข้อมูลบางส่วนที่อยู่ภายใน Collection หรือข้อมูลประเภท String โดยไม่จำเป็นต้องคัดลอกข้อมูลทั้งหมด

Slice ไม่ได้เป็นเจ้าของข้อมูล แต่เป็นการยืมข้อมูลผ่าน Reference

ตัวอย่าง:

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    let slice = &numbers[1..4];

    println!("{:?}", slice);
}
```

Output:

```text
[20, 30, 40]
```

## 2. Slice Syntax

รูปแบบการเขียน Slice:

```rust
&collection[start..end]
```

- `start` คือ index เริ่มต้น และรวมอยู่ในผลลัพธ์
- `end` คือ index สิ้นสุด แต่ไม่รวมอยู่ในผลลัพธ์

ตัวอย่าง:

```rust
let numbers = [10, 20, 30, 40, 50];

println!("{:?}", &numbers[1..3]);
println!("{:?}", &numbers[0..2]);
println!("{:?}", &numbers[2..]);
println!("{:?}", &numbers[..3]);
println!("{:?}", &numbers[..]);
```

Output:

```text
[20, 30]
[10, 20]
[30, 40, 50]
[10, 20, 30]
[10, 20, 30, 40, 50]
```

## 3. String Slice

เราสามารถสร้าง Slice จาก String ได้เช่นกัน

```rust
fn main() {
    let message = String::from("Hello Rust");

    let part = &message[0..5];

    println!("{}", part);
}
```

Output:

```text
Hello
```

ข้อควรระวัง: String ใน Rust ใช้ UTF-8 ดังนั้นตำแหน่งของ String Slice ต้องอยู่บนขอบเขต byte ที่ถูกต้อง หากตัดกลางตัวอักษร UTF-8 โปรแกรมอาจ panic ได้

## 4. Slice as Function Parameter

สามารถส่ง Slice เข้าไปใน Function ได้

```rust
fn display_numbers(numbers: &[i32]) {
    println!("{:?}", numbers);
}

fn main() {
    let numbers = [10, 20, 30, 40];

    display_numbers(&numbers[1..3]);
}
```

Output:

```text
[20, 30]
```

`&[i32]` หมายถึง Reference ไปยัง Slice ของข้อมูลชนิด `i32`

## 5. Important Notes

- Slice เป็น Reference ไม่ใช่การสร้าง Collection ใหม่
- Slice ไม่ได้เป็นเจ้าของข้อมูล
- Index เริ่มต้นจาก 0
- `start` รวมอยู่ในช่วง แต่ `end` ไม่รวม
- การเข้าถึงช่วงข้อมูลที่เกินขอบเขตจะทำให้เกิด Panic
- String Slice ต้องคำนึงถึง UTF-8 Boundary

## Practice

ทดลองเขียนโปรแกรมเพื่อ:

1. สร้าง Array ที่มีตัวเลข 8 ค่า
2. แสดง Slice ตั้งแต่ index 2 ถึง index 5
3. แสดง Slice ตั้งแต่ index 0 ถึง index 3
4. สร้าง Function ที่รับ `&[i32]` และแสดงผลข้อมูล

## Knowledge Check

1. Slice คืออะไร?
2. `&numbers[1..4]` จะเลือก index ใดบ้าง?
3. Slice เป็นเจ้าของข้อมูลหรือไม่?
4. `&[i32]` ใช้ทำอะไร?
5. เพราะเหตุใด String Slice จึงต้องระวัง UTF-8 Boundary?

## Completion Checklist

- [ ] เข้าใจ Slice
- [ ] เข้าใจ Start และ End Index
- [ ] สามารถสร้าง Array Slice
- [ ] สามารถสร้าง String Slice
- [ ] สามารถส่ง Slice เข้า Function ได้
