# Lesson 02: Raw Pointers

## 1. Raw Pointer คืออะไร?

Raw Pointer คือ Pointer ที่เก็บ Address ของข้อมูลใน Memory โดยไม่อยู่ภายใต้กฎ Borrowing แบบ Reference ปกติ

Rust มี Raw Pointer 2 ประเภท:

| ประเภท     | ความหมาย                         |
| ---------- | -------------------------------- |
| `*const T` | Pointer สำหรับอ่านข้อมูล         |
| `*mut T`   | Pointer สำหรับอ่านและเขียนข้อมูล |

Raw Pointer สามารถเป็น Null ได้ และไม่รับประกันว่า Address ที่เก็บอยู่จะเป็น Memory ที่ใช้งานได้

## 2. สร้าง Raw Pointer

ใช้ Raw Borrow Operator เพื่อสร้าง Raw Pointer:

```rust
fn main() {
    let value = 100;

    let const_pointer = &raw const value;

    println!("{:p}", const_pointer);
}
```

ตัวอย่าง Mutable Raw Pointer:

```rust
fn main() {
    let mut value = 100;

    let mut_pointer = &raw mut value;

    println!("{:p}", mut_pointer);
}
```

การสร้าง Raw Pointer ไม่จำเป็นต้องใช้ `unsafe` แต่การ Dereference Raw Pointer ต้องใช้ Unsafe Context

## 3. Dereference Raw Pointer

Dereference คือการเข้าถึงค่าที่ Pointer ชี้อยู่

ตัวอย่าง:

```rust
fn main() {
    let value = 100;
    let pointer = &raw const value;

    let result = unsafe {
        *pointer
    };

    println!("{}", result);
}
```

ผลลัพธ์:

```text
100
```

ต้องใช้ `unsafe` เพราะ Compiler ไม่สามารถรับประกันความถูกต้องของ Raw Pointer ได้ทั้งหมด

## 4. Mutable Raw Pointer

`*mut T` สามารถใช้แก้ไขข้อมูลที่ Pointer ชี้อยู่ได้ หากรักษาเงื่อนไขความปลอดภัยครบถ้วน

ตัวอย่าง:

```rust
fn main() {
    let mut value = 100;

    let pointer = &raw mut value;

    unsafe {
        *pointer = 200;
    }

    println!("{}", value);
}
```

ผลลัพธ์:

```text
200
```

เงื่อนไขสำคัญ:

- Pointer ต้องชี้ไปยัง Valid Memory
- Memory ต้องยังมีชีวิตอยู่
- ต้องมี Alignment ถูกต้อง
- ต้องไม่มีการอ่านหรือเขียนที่ละเมิด Aliasing Rules
- ต้องไม่มี Data Race

## 5. Raw Pointer กับ Reference

| Reference                   | Raw Pointer                         |
| --------------------------- | ----------------------------------- |
| `&T`                        | `*const T`                          |
| `&mut T`                    | `*mut T`                            |
| Compiler ตรวจสอบ Borrowing  | ผู้เขียนต้องรักษา Safety Invariants |
| ไม่อนุญาต Null Reference    | สามารถเป็น Null ได้                 |
| Dereference ได้ใน Safe Rust | Dereference ต้องใช้ Unsafe Context  |

## 6. Null Pointer

สามารถสร้าง Null Pointer ได้ด้วย:

```rust
use std::ptr;

fn main() {
    let pointer: *const i32 = ptr::null();

    println!("Is null: {}", pointer.is_null());
}
```

ผลลัพธ์:

```text
Is null: true
```

สามารถตรวจสอบ Pointer ได้ด้วย:

```rust
pointer.is_null()
```

ห้าม Dereference Null Pointer

## 7. Pointer Arithmetic

Raw Pointer สามารถเลื่อนไปยังตำแหน่ง Memory อื่นได้ด้วย Pointer Arithmetic

ตัวอย่าง:

```rust
fn main() {
    let numbers = [10, 20, 30];

    let pointer = numbers.as_ptr();

    unsafe {
        println!("{}", *pointer.add(1));
    }
}
```

ผลลัพธ์:

```text
20
```

ข้อควรระวัง:

- Pointer ต้องอยู่ใน Allocation ที่ถูกต้อง
- Offset ต้องไม่เกินขอบเขตที่อนุญาต
- ต้องไม่ Dereference Pointer ที่ชี้ไปยังตำแหน่งที่ไม่ถูกต้อง
- Pointer Arithmetic ต้องรักษาเงื่อนไขของ Provenance และ Memory Validity

## 8. Pointer Safety

ก่อน Dereference Raw Pointer ต้องตรวจสอบ:

1. Pointer ไม่เป็น Null
2. Pointer ชี้ไปยัง Memory ที่ยังมีชีวิตอยู่
3. Address มี Alignment ถูกต้อง
4. Memory มีชนิดข้อมูลที่ถูกต้อง
5. ขอบเขตการเข้าถึงถูกต้อง
6. ไม่มีการเข้าถึงที่ละเมิด Aliasing Rules
7. ไม่มี Data Race

การตรวจสอบเพียง `is_null()` ไม่เพียงพอที่จะรับประกันว่า Pointer ปลอดภัย

## 9. สรุป

| คำสั่ง                 | หน้าที่                    |
| ---------------------- | -------------------------- |
| `&raw const value`     | สร้าง Raw Const Pointer    |
| `&raw mut value`       | สร้าง Raw Mutable Pointer  |
| `*pointer`             | Dereference Pointer        |
| `pointer.is_null()`    | ตรวจสอบ Null               |
| `pointer.add(n)`       | เลื่อน Pointer ไปข้างหน้า  |
| `std::ptr::null()`     | สร้าง Null Const Pointer   |
| `std::ptr::null_mut()` | สร้าง Null Mutable Pointer |

## Completion Checklist

- [ ] เข้าใจ Raw Pointer
- [ ] เข้าใจ `*const T`
- [ ] เข้าใจ `*mut T`
- [ ] สร้าง Raw Pointer ได้
- [ ] Dereference Raw Pointer ได้
- [ ] แก้ไขข้อมูลผ่าน Mutable Pointer ได้
- [ ] ตรวจสอบ Null Pointer ได้
- [ ] เข้าใจ Pointer Arithmetic
- [ ] เข้าใจ Pointer Safety
- [ ] อธิบายความแตกต่างระหว่าง Reference และ Raw Pointer ได้
