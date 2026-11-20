# Lesson 03: Propagating Errors

## 1. What is Error Propagation?

Error Propagation คือการส่งต่อข้อผิดพลาดจาก Function หนึ่งไปยัง Function ที่เรียกใช้งาน แทนการจัดการ Error ทั้งหมดภายใน Function เดิม

แนวทางนี้ช่วยให้:

- ลดการเขียน `match` ซ้ำหลายจุด
- แยกหน้าที่ของ Functions
- ให้ Function ชั้นบนตัดสินใจว่าจะจัดการ Error อย่างไร

## 2. Returning Result from a Function

Function สามารถคืนค่า `Result<T, E>` เพื่อแจ้งว่าการทำงานสำเร็จหรือเกิด Error

ตัวอย่าง:

```rust
fn parse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    match text.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(error) => Err(error),
    }
}

fn main() {
    match parse_number("123") {
        Ok(number) => println!("Number: {}", number),
        Err(error) => println!("Error: {}", error),
    }
}
```

## 3. The ? Operator

เครื่องหมาย `?` ใช้ส่งต่อ Error จาก Expression ที่คืนค่าเป็น `Result` หรือ `Option` ตามบริบทของ Function

สำหรับ Result:

- หากเป็น `Ok(value)` จะได้ค่า `value` ไปทำงานต่อ
- หากเป็น `Err(error)` จะส่ง Error ออกจาก Function ทันที

ตัวอย่าง:

```rust
fn parse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    let number = text.parse::<i32>()?;

    Ok(number)
}

fn main() {
    match parse_number("123") {
        Ok(number) => println!("Number: {}", number),
        Err(error) => println!("Error: {}", error),
    }
}
```

## 4. Comparing match and ?

แบบใช้ `match`:

```rust
fn parse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    match text.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(error) => Err(error),
    }
}
```

แบบใช้ `?`:

```rust
fn parse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    let number = text.parse::<i32>()?;

    Ok(number)
}
```

ทั้งสองแบบมีจุดประสงค์เดียวกัน แต่แบบใช้ `?` กระชับกว่า

## 5. Error Propagation with File Operations

การอ่านไฟล์เป็นตัวอย่างของงานที่อาจเกิด Error เช่น:

- ไม่พบไฟล์
- ไม่มีสิทธิ์อ่านไฟล์
- เกิดข้อผิดพลาดระหว่างอ่านข้อมูล

ตัวอย่าง:

```rust
use std::fs;
use std::io;

fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;

    Ok(content)
}

fn main() {
    match read_file("message.txt") {
        Ok(content) => println!("{}", content),
        Err(error) => println!("Failed to read file: {}", error),
    }
}
```

## 6. Requirements for ?

ข้อควรจำ:

- Function ต้องมี Return Type ที่รองรับ Error ที่ถูกส่งต่อ
- สำหรับ Result ต้องสามารถแปลง Error ให้ตรงกับ Error Type ที่ Function คืนค่าได้
- `?` ไม่ได้จัดการ Error ให้หายไป แต่ส่งต่อ Error ไปยังผู้เรียก
- ใช้ `?` ได้ภายใน Function หรือ Block ที่รองรับการส่งต่อ Error ตามชนิดของมัน

ตัวอย่างนี้ไม่สามารถใช้ `?` ใน `main()` ที่คืนค่า `()` ได้โดยตรง:

```rust
fn main() {
    // let number = "abc".parse::<i32>()?;
}
```

หากต้องการใช้ `?` ใน `main` สามารถกำหนด Return Type ที่เหมาะสม เช่น:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let number = "123".parse::<i32>()?;

    println!("{}", number);

    Ok(())
}
```

## 7. Important Notes

- Error Propagation คือการส่งต่อ Error
- `?` ช่วยลดการเขียน `match` ซ้ำ
- `?` ส่ง Error ออกจาก Function เมื่อพบ `Err`
- Function ต้องรองรับ Error Type ที่ส่งต่อ
- `?` ไม่ได้ทำให้ Error หายไป
- ผู้เรียก Function สามารถเลือกจัดการ Error ต่อได้

## 8. Knowledge Check

1. Error Propagation คืออะไร?
2. เหตุใดจึงควรส่งต่อ Error?
3. `?` ทำงานอย่างไรเมื่อพบ `Ok`?
4. `?` ทำงานอย่างไรเมื่อพบ `Err`?
5. Function ที่ใช้ `?` กับ Result ต้องคืนค่าอย่างไร?
6. `?` แตกต่างจาก `unwrap()` อย่างไร?
7. การอ่านไฟล์สามารถเกิด Error ประเภทใดได้บ้าง?
8. สามารถใช้ `?` ใน `main()` ที่คืนค่า `()` ได้โดยตรงหรือไม่?

## 9. Completion Checklist

- [ ] เข้าใจ Error Propagation
- [ ] สร้าง Function ที่คืนค่า Result ได้
- [ ] ส่งต่อ Error ด้วย `match` ได้
- [ ] ใช้ `?` Operator ได้
- [ ] เข้าใจการทำงานของ `?` เมื่อเป็น Ok
- [ ] เข้าใจการทำงานของ `?` เมื่อเป็น Err
- [ ] ใช้ `?` กับ `parse()` ได้
- [ ] ใช้ `?` กับ File Operations ได้
- [ ] เข้าใจข้อกำหนด Return Type ของ Function
