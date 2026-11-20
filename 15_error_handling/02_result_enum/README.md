# Lesson 02: Result Enum

## 1. What is Result?

`Result<T, E>` เป็น Enum ใน Standard Library ของ Rust ใช้แทนผลลัพธ์ที่อาจสำเร็จหรือเกิดข้อผิดพลาด

โครงสร้างโดยย่อ:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

ประกอบด้วย:

- `Ok(T)` หมายถึงการทำงานสำเร็จและมีค่าผลลัพธ์
- `Err(E)` หมายถึงการทำงานไม่สำเร็จและมีข้อมูล Error

## 2. Creating a Result

ตัวอย่าง Function ที่คืนค่า Result:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

Function นี้จะ:

- คืน `Ok(result)` เมื่อหารสำเร็จ
- คืน `Err(message)` เมื่อไม่สามารถหารได้

## 3. Handling Result with match

ใช้ `match` เพื่อตรวจสอบว่า Result เป็น `Ok` หรือ `Err`

ตัวอย่าง:

```rust
fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

ผลลัพธ์:

```text
Result: 5
```

## 4. Result with parse()

การแปลงข้อความเป็นตัวเลขอาจไม่สำเร็จ จึงคืนค่าเป็น Result

ตัวอย่าง:

```rust
fn main() {
    let number: Result<i32, _> = "123".parse();

    match number {
        Ok(value) => println!("Number: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
```

ผลลัพธ์:

```text
Number: 123
```

หากเปลี่ยนข้อความเป็น `"abc"` จะได้กรณี `Err`

## 5. The unwrap() Method

`unwrap()` ใช้ดึงค่าจาก `Ok`

ตัวอย่าง:

```rust
fn main() {
    let result: Result<i32, &str> = Ok(100);

    let value = result.unwrap();

    println!("{}", value);
}
```

ผลลัพธ์:

```text
100
```

ข้อควรระวัง: หาก Result เป็น `Err` จะเกิด Panic

## 6. The expect() Method

`expect()` ใช้ดึงค่าเช่นเดียวกับ `unwrap()` แต่กำหนดข้อความประกอบ Panic ได้

ตัวอย่าง:

```rust
fn main() {
    let result: Result<i32, &str> = Ok(100);

    let value = result.expect("Could not get value");

    println!("{}", value);
}
```

## 7. The unwrap_or() Method

`unwrap_or()` ใช้กำหนดค่าเริ่มต้นเมื่อ Result เป็น `Err`

ตัวอย่าง:

```rust
fn main() {
    let result: Result<i32, &str> = Err("Invalid number");

    let value = result.unwrap_or(0);

    println!("{}", value);
}
```

ผลลัพธ์:

```text
0
```

ข้อแตกต่าง:

| Method        | เมื่อเป็น Ok | เมื่อเป็น Err      |
| ------------- | ------------ | ------------------ |
| `unwrap()`    | คืนค่า       | Panic              |
| `expect()`    | คืนค่า       | Panic พร้อมข้อความ |
| `unwrap_or()` | คืนค่า       | คืนค่าเริ่มต้น     |

## 8. Result with if let

หากต้องการตรวจสอบเพียงกรณีเดียว สามารถใช้ `if let`

ตัวอย่าง:

```rust
fn main() {
    let result: Result<i32, &str> = Ok(50);

    if let Ok(value) = result {
        println!("Value: {}", value);
    }
}
```

หากต้องการจัดการทั้ง `Ok` และ `Err` ควรพิจารณาใช้ `match`

## 9. Important Notes

- `Result<T, E>` ใช้แทนผลลัพธ์ที่อาจผิดพลาด
- `Ok(T)` เก็บค่าที่สำเร็จ
- `Err(E)` เก็บข้อมูลข้อผิดพลาด
- `match` ช่วยจัดการทั้งสองกรณี
- `unwrap()` และ `expect()` อาจทำให้เกิด Panic
- `unwrap_or()` ช่วยกำหนดค่าเริ่มต้นเมื่อเกิด Error
- การเลือกใช้ Method ต้องพิจารณาว่าต้องการจัดการ Error อย่างไร

## 10. Knowledge Check

1. `Result<T, E>` คืออะไร?
2. `Ok` ใช้ทำอะไร?
3. `Err` ใช้ทำอะไร?
4. `match` ใช้จัดการ Result อย่างไร?
5. `unwrap()` จะเกิดอะไรขึ้นเมื่อ Result เป็น Err?
6. `expect()` แตกต่างจาก `unwrap()` อย่างไร?
7. `unwrap_or()` ใช้ในสถานการณ์ใด?
8. `parse()` คืนค่าเป็นชนิดใด?

## 11. Completion Checklist

- [ ] เข้าใจ `Result<T, E>`
- [ ] เข้าใจ `Ok` และ `Err`
- [ ] สร้าง Function ที่คืนค่า Result ได้
- [ ] ใช้ `match` จัดการ Result ได้
- [ ] ใช้ `parse()` ได้
- [ ] เข้าใจ `unwrap()`
- [ ] เข้าใจ `expect()`
- [ ] ใช้ `unwrap_or()` ได้
- [ ] ใช้ `if let` กับ Result ได้
