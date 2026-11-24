# Lesson 03: Interior Mutability

## วัตถุประสงค์

เรียนรู้แนวคิด Interior Mutability และวิธีใช้ `Cell<T>` กับ `RefCell<T>` เพื่อเปลี่ยนแปลงข้อมูลผ่านตัวแปรที่ไม่ได้ประกาศเป็น `mut` โดยมีระบบตรวจสอบความปลอดภัยตามรูปแบบของแต่ละชนิด

## 1. Interior Mutability คืออะไร

โดยปกติ Rust ใช้กฎ Borrowing เพื่อควบคุมการเข้าถึงข้อมูล:

- มี Immutable References ได้หลายรายการ
- มี Mutable Reference ได้เพียงหนึ่งรายการในช่วงเวลาหนึ่ง
- ไม่สามารถมี Mutable Reference พร้อมกับ Immutable References ที่ยังใช้งานอยู่ได้

Interior Mutability เป็นรูปแบบการออกแบบที่อนุญาตให้แก้ไขข้อมูลภายในได้ แม้ตัวแปรภายนอกจะไม่ได้ประกาศเป็น `mut`

การตรวจสอบบางส่วนจะย้ายจาก Compile Time ไปเป็น Runtime

## 2. Cell<T>

`Cell<T>` ใช้สำหรับเปลี่ยนค่าภายในโดยไม่ต้องสร้าง Reference ไปยังค่าด้านในโดยตรง

เหมาะกับข้อมูลที่สามารถ Copy ค่าออกมาได้ หรือกรณีที่ต้องการแทนที่ค่าทั้งก้อน

ตัวอย่าง:

```rust
use std::cell::Cell;

fn main() {
    let value = Cell::new(10);

    value.set(20);

    println!("{}", value.get());
}
```

ผลลัพธ์:

```text
20
```

เมธอดที่ควรรู้จัก:

| Method             | หน้าที่                           |
| ------------------ | --------------------------------- |
| `Cell::new(value)` | สร้าง Cell                        |
| `.get()`           | อ่านค่า โดยใช้กับชนิดที่ Copy ได้ |
| `.set(value)`      | แทนที่ค่าภายใน                    |
| `.replace(value)`  | แทนที่ค่าและคืนค่าเดิม            |
| `.take()`          | นำค่าออกและแทนด้วย Default        |

ข้อควรจำ: `Cell<T>` ไม่ได้ให้ Reference ไปยังค่าภายในโดยตรง

## 3. RefCell<T>

`RefCell<T>` ใช้ตรวจสอบกฎ Borrowing ขณะ Runtime แทนการตรวจสอบทั้งหมดใน Compile Time

เมธอดสำคัญ:

| Method                | หน้าที่                                         |
| --------------------- | ----------------------------------------------- |
| `RefCell::new(value)` | สร้าง RefCell                                   |
| `.borrow()`           | ยืมข้อมูลแบบ Immutable                          |
| `.borrow_mut()`       | ยืมข้อมูลแบบ Mutable                            |
| `.try_borrow()`       | พยายามยืมแบบ Immutable และคืน Result            |
| `.try_borrow_mut()`   | พยายามยืมแบบ Mutable และคืน Result              |
| `.get_mut()`          | เข้าถึงข้อมูลแบบ Mutable ผ่าน `&mut RefCell<T>` |

ตัวอย่าง:

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10);

    *value.borrow_mut() = 20;

    println!("{}", value.borrow());
}
```

ผลลัพธ์:

```text
20
```

## 4. Runtime Borrow Checking

`RefCell<T>` ตรวจสอบกฎ Borrowing ขณะโปรแกรมทำงาน

กฎสำคัญ:

- สามารถมี Immutable Borrows หลายรายการพร้อมกันได้
- สามารถมี Mutable Borrow ได้หนึ่งรายการ
- ไม่สามารถมี Mutable Borrow พร้อมกับ Borrow อื่นที่ยังมีชีวิตอยู่ได้

หากละเมิดกฎผ่าน `.borrow()` หรือ `.borrow_mut()` โปรแกรมจะเกิด Panic ขณะ Runtime

หากไม่ต้องการให้โปรแกรม Panic สามารถใช้ `.try_borrow()` หรือ `.try_borrow_mut()` เพื่อจัดการกรณีที่ยืมข้อมูลไม่ได้

## 5. Rc<RefCell<T>>

บางสถานการณ์ต้องการทั้ง:

1. แชร์ Ownership
2. เปลี่ยนแปลงข้อมูลร่วมกัน

สามารถใช้ `Rc<RefCell<T>>` เพื่อรวมความสามารถทั้งสองแบบได้

ตัวอย่างแนวคิด:

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(10));

    let shared_value = Rc::clone(&value);

    *shared_value.borrow_mut() += 5;

    println!("{}", value.borrow());
}
```

ผลลัพธ์:

```text
15
```

ตัวแปรทั้งสองเข้าถึง RefCell ตัวเดียวกัน

## 6. Cell<T> กับ RefCell<T>

| หัวข้อ                               | Cell<T>                                      | RefCell<T>                    |
| ------------------------------------ | -------------------------------------------- | ----------------------------- |
| การตรวจสอบ                           | ใช้รูปแบบการเข้าถึงที่ไม่ให้ Reference ภายใน | Runtime Borrow Checking       |
| อ่านค่า                              | `.get()` สำหรับชนิด Copy                     | `.borrow()`                   |
| เปลี่ยนค่า                           | `.set()`                                     | `.borrow_mut()`               |
| Reference ภายใน                      | ไม่เปิดให้ยืมค่าภายในโดยตรง                  | เปิดให้ยืมผ่าน Ref และ RefMut |
| ความเสี่ยง Panic จาก Borrow Conflict | ไม่มีจากระบบ Borrow Checking แบบ RefCell     | มีได้                         |

## 7. ข้อควรระวัง

- อย่าเก็บ `Ref` หรือ `RefMut` ไว้นานเกินจำเป็น
- ระวังการเรียก `.borrow_mut()` ซ้อนกัน
- ใช้ `.try_borrow_mut()` เมื่อไม่แน่ใจว่าจะยืมแบบ Mutable ได้หรือไม่
- `RefCell<T>` ไม่ได้ทำให้ข้อมูล Thread-safe
- สำหรับการเปลี่ยนข้อมูลร่วมกันระหว่าง Threads ให้ศึกษา `Mutex<T>` และ `Arc<T>` เพิ่มเติม

## 8. สรุป

- Interior Mutability อนุญาตให้แก้ไขข้อมูลภายในผ่านรูปแบบการเข้าถึงที่กำหนดไว้
- `Cell<T>` เหมาะกับการอ่านหรือแทนที่ค่าโดยไม่ให้ Reference ภายใน
- `RefCell<T>` ตรวจสอบ Borrowing ขณะ Runtime
- `Rc<RefCell<T>>` รวม Shared Ownership และ Interior Mutability
- การใช้ Interior Mutability ต้องระวัง Borrow Conflicts

## Completion Checklist

- [ ] อธิบาย Interior Mutability ได้
- [ ] สร้าง `Cell<T>` ได้
- [ ] ใช้ `.get()` และ `.set()` ได้
- [ ] สร้าง `RefCell<T>` ได้
- [ ] ใช้ `.borrow()` และ `.borrow_mut()` ได้
- [ ] อธิบาย Runtime Borrow Checking ได้
- [ ] เข้าใจความเสี่ยงของ Borrow Conflict
- [ ] อธิบายการทำงานของ `Rc<RefCell<T>>` ได้
