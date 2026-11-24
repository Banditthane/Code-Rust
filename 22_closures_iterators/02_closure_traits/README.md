# Lesson 02: Closure Traits

## 1. Closure Traits คืออะไร?

Closure ใน Rust มี Trait หลัก 3 แบบที่กำหนดว่า Closure สามารถใช้งานค่าที่จับมาจาก Environment ได้อย่างไร ได้แก่

- `Fn`
- `FnMut`
- `FnOnce`

Trait เหล่านี้เกี่ยวข้องกับวิธีที่ Closure เข้าถึงและเปลี่ยนแปลงข้อมูลภายนอก

## 2. Fn

`Fn` ใช้กับ Closure ที่สามารถเรียกใช้งานได้โดยไม่ต้องเปลี่ยนแปลงค่าที่จับมาจาก Environment

โดยทั่วไป Closure ประเภทนี้สามารถเรียกซ้ำได้

ตัวอย่างลักษณะการใช้งาน:

```rust
let message = String::from("Rust");

let print_message = || {
    println!("{}", message);
};

print_message();
print_message();
```

แนวคิดสำคัญ:

- อ่านค่าจาก Environment ได้
- ไม่ต้องแก้ไขค่าที่จับมา
- สามารถเรียกซ้ำได้

## 3. FnMut

`FnMut` ใช้กับ Closure ที่ต้องแก้ไขค่าจาก Environment ผ่าน Mutable Reference

ตัวอย่าง:

```rust
let mut count = 0;

let mut increase = || {
    count += 1;
};

increase();
increase();

println!("{}", count);
```

ผลลัพธ์:

```text
2
```

แนวคิดสำคัญ:

- สามารถเปลี่ยนแปลงค่าที่จับมาได้
- ต้องประกาศตัวแปร Closure เป็น `mut` เมื่อเรียกใช้ในกรณีที่ต้องใช้ `FnMut`
- การแก้ไขข้อมูลยังอยู่ภายใต้กฎ Borrowing ของ Rust

## 4. FnOnce

`FnOnce` ใช้กับ Closure ที่อาจเคลื่อนย้าย Ownership ของค่าที่จับมา

Closure ประเภทนี้สามารถเรียกใช้งานได้อย่างน้อยหนึ่งครั้ง แต่บาง Closure จะไม่สามารถเรียกซ้ำได้ เพราะ Ownership ของค่าภายนอกถูกย้ายออกไปแล้ว

ตัวอย่าง:

```rust
let message = String::from("Hello Rust");

let consume = move || {
    println!("{}", message);
};

consume();
```

คำว่า `move` บังคับให้ Closure จับค่าจาก Environment ด้วยการย้ายหรือคัดลอกตามกฎ Ownership

ข้อควรระวัง:

- `move` ไม่ได้หมายความว่า Closure ต้องเป็น `FnOnce` เสมอไป
- Closure ที่ใช้ `move` ยังอาจ implement `Fn` หรือ `FnMut` ได้ ขึ้นอยู่กับวิธีใช้งานค่าที่จับมา
- หาก Closure ย้าย Ownership ของค่าที่ไม่ใช่ Copy ออกไป อาจทำให้เรียกซ้ำไม่ได้

## 5. เปรียบเทียบ Fn, FnMut และ FnOnce

| Trait  | ลักษณะการใช้งาน                  | เรียกซ้ำได้หรือไม่                         |
| ------ | -------------------------------- | ------------------------------------------ |
| Fn     | อ่านค่าผ่าน Immutable Borrow     | ได้                                        |
| FnMut  | แก้ไขค่าผ่าน Mutable Borrow      | ได้ หากยังไม่ละเมิดกฎ Borrowing            |
| FnOnce | อาจย้าย Ownership ของค่าที่จับมา | ขึ้นอยู่กับการใช้งาน อาจเรียกได้ครั้งเดียว |

ความสัมพันธ์ของ Trait:

- Closure ที่ implement `Fn` จะ implement `FnMut` และ `FnOnce` ด้วย
- Closure ที่ implement `FnMut` จะ implement `FnOnce` ด้วย
- Closure ที่ implement เพียง `FnOnce` ไม่จำเป็นต้อง implement `FnMut` หรือ `Fn`

## 6. Move Closures

ใช้คำว่า `move` เพื่อให้ Closure จับค่าจาก Environment ด้วย Ownership หรือ Copy ตามชนิดข้อมูล

ตัวอย่าง:

```rust
let name = String::from("Alice");

let greet = move || {
    println!("Hello {}", name);
};

greet();
```

Move Closure มีประโยชน์เมื่อ:

- ต้องการย้ายข้อมูลเข้าไปใน Closure
- ส่ง Closure ไปทำงานใน Thread
- ต้องการให้ Closure เป็นเจ้าของข้อมูลที่ใช้

อย่างไรก็ตาม การใช้ `move` ไม่ได้ทำให้ข้อมูลมีอายุยืนขึ้นโดยอัตโนมัติ แต่เป็นการเปลี่ยนวิธีจับค่าตามกฎ Ownership

## 7. Closure กับ Function Parameters

Closure สามารถถูกส่งเข้าไปในฟังก์ชันผ่าน Generic Type และ Trait Bounds ได้

แนวคิด:

- `F: Fn()`
- `F: FnMut()`
- `F: FnOnce()`

ชนิด Trait ที่เลือกใช้ขึ้นอยู่กับพฤติกรรมที่ฟังก์ชันต้องการจาก Closure

## 8. สรุป

- `Fn` อ่านค่าจาก Environment
- `FnMut` แก้ไขค่าผ่าน Mutable Borrow
- `FnOnce` อาจย้าย Ownership ของค่าที่จับมา
- `move` กำหนดให้ Closure จับค่าด้วย Ownership หรือ Copy
- Closure ที่ใช้ `move` ไม่จำเป็นต้องเป็น `FnOnce` เสมอไป

## Checklist

- [ ] เข้าใจ Fn
- [ ] เข้าใจ FnMut
- [ ] เข้าใจ FnOnce
- [ ] แยกความแตกต่างของทั้ง 3 Trait ได้
- [ ] เข้าใจการใช้ `move`
- [ ] เข้าใจผลของ Ownership เมื่อ Closure จับค่าจากภายนอก
- [ ] เข้าใจว่า Closure สามารถส่งเป็น Parameter ให้ฟังก์ชันได้
