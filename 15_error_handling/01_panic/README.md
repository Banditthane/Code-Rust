# Lesson 01: Panic

## 1. What is Panic?

`panic!` เป็น Macro ที่ใช้แจ้งว่าโปรแกรมพบข้อผิดพลาดที่ไม่สามารถดำเนินการต่อได้ตามปกติ

เมื่อเกิด Panic โปรแกรมจะเริ่มกระบวนการหยุดทำงาน (Unwinding หรือ Abort ขึ้นอยู่กับการตั้งค่า)

ตัวอย่าง:

```rust
fn main() {
    println!("Program started");

    panic!("Something went wrong!");

    println!("Program finished");
}
```

ตัวอย่างผลลัพธ์:

```text
Program started
thread 'main' panicked at ...
Something went wrong!
```

บรรทัด `Program finished` จะไม่ทำงาน

## 2. Unrecoverable Errors

Unrecoverable Error คือข้อผิดพลาดที่โปรแกรมไม่สามารถจัดการเพื่อดำเนินการต่อได้ตามปกติ

ตัวอย่างสถานการณ์:

- โปรแกรมพบข้อมูลที่ผิดเงื่อนไขอย่างร้ายแรง
- เกิดข้อผิดพลาดที่ทำให้ไม่สามารถรักษาสถานะที่ถูกต้องได้
- มีการเรียก `panic!` โดยตรง

ไม่ควรใช้ `panic!` กับทุกข้อผิดพลาด เพราะข้อผิดพลาดหลายประเภทสามารถจัดการด้วย `Result` ได้

## 3. Panic from Invalid Array Index

การเข้าถึง Index ที่ไม่มีอยู่ใน Array หรือ Vector จะทำให้เกิด Panic

ตัวอย่าง:

```rust
fn main() {
    let numbers = [10, 20, 30];

    println!("{}", numbers[5]);
}
```

Array มี Index ตั้งแต่ `0` ถึง `2` เท่านั้น

การเข้าถึง Index `5` จึงทำให้โปรแกรม Panic

## 4. The unwrap() Method

`unwrap()` ใช้ดึงค่าภายใน `Result` หรือ `Option`

- หากเป็น `Ok(value)` จะคืนค่า `value`
- หากเป็น `Err(error)` จะทำให้เกิด Panic
- หากเป็น `Some(value)` จะคืนค่า `value`
- หากเป็น `None` จะทำให้เกิด Panic

ตัวอย่าง:

```rust
fn main() {
    let number: Result<i32, _> = "42".parse();

    let value = number.unwrap();

    println!("{}", value);
}
```

ผลลัพธ์:

```text
42
```

หากแปลงข้อความไม่สำเร็จ `unwrap()` จะทำให้เกิด Panic

## 5. The expect() Method

`expect()` ทำงานคล้ายกับ `unwrap()` แต่สามารถกำหนดข้อความประกอบเมื่อเกิด Panic ได้

ตัวอย่าง:

```rust
fn main() {
    let number: Result<i32, _> = "abc".parse();

    let value = number.expect("Failed to convert number");

    println!("{}", value);
}
```

หากแปลงไม่สำเร็จ โปรแกรมจะ Panic พร้อมข้อความที่กำหนด

## 6. Recoverable vs Unrecoverable Errors

| Type          | Description                     | Rust Tool      |
| ------------- | ------------------------------- | -------------- |
| Recoverable   | สามารถตรวจสอบและจัดการได้       | `Result<T, E>` |
| Unrecoverable | ไม่สามารถดำเนินการต่อได้ตามปกติ | `panic!`       |

ตัวอย่าง Recoverable Error:

- เปิดไฟล์ไม่สำเร็จ
- แปลงข้อความเป็นตัวเลขไม่สำเร็จ
- ข้อมูลที่รับเข้ามาไม่ถูกต้อง

ตัวอย่าง Unrecoverable Error:

- เงื่อนไขภายในโปรแกรมผิดพลาดอย่างร้ายแรง
- เกิดสถานการณ์ที่โปรแกรมไม่สามารถรักษาความถูกต้องของสถานะได้

## 7. Important Notes

- `panic!` ใช้หยุดการทำงานเมื่อเกิดข้อผิดพลาดที่ไม่สามารถดำเนินการต่อได้ตามปกติ
- `unwrap()` และ `expect()` อาจทำให้เกิด Panic
- ไม่ควรใช้ `unwrap()` กับข้อมูลที่ไม่มั่นใจว่าจะสำเร็จ
- ควรใช้ `Result` เมื่อข้อผิดพลาดสามารถจัดการได้
- การตั้งค่า Panic Strategy อาจกำหนดให้ Unwinding หรือ Abort

## 8. Knowledge Check

1. `panic!` คืออะไร?
2. Unrecoverable Error หมายถึงอะไร?
3. การเข้าถึง Index ที่ไม่มีอยู่จะเกิดอะไรขึ้น?
4. `unwrap()` ทำงานอย่างไร?
5. `expect()` แตกต่างจาก `unwrap()` อย่างไร?
6. ควรใช้ `panic!` กับข้อผิดพลาดทุกประเภทหรือไม่?
7. Recoverable Error แตกต่างจาก Unrecoverable Error อย่างไร?

## 9. Completion Checklist

- [ ] เข้าใจความหมายของ Panic
- [ ] ใช้ `panic!` ได้
- [ ] เข้าใจ Unrecoverable Errors
- [ ] เข้าใจ Panic จาก Invalid Index
- [ ] ใช้ `unwrap()` ได้
- [ ] ใช้ `expect()` ได้
- [ ] เข้าใจความเสี่ยงของการใช้ `unwrap()`
- [ ] แยก Recoverable และ Unrecoverable Errors ได้
