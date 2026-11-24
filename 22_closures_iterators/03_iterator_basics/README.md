# Lesson 03: Iterator Basics

## 1. Iterator คืออะไร?

Iterator คือเครื่องมือใน Rust ที่ใช้เข้าถึงสมาชิกของชุดข้อมูลทีละรายการ เช่น

- Array
- Vector
- String
- Range
- Collection ประเภทต่าง ๆ

Iterator ช่วยให้สามารถประมวลผลข้อมูลโดยไม่จำเป็นต้องเขียน Loop แบบเดิมทุกครั้ง

## 2. Iterator Trait

Iterator ใน Rust ทำงานผ่าน Trait ชื่อ `Iterator`

แนวคิดสำคัญคือ Iterator มีเมธอด `next()` สำหรับดึงสมาชิกถัดไป

`next()` จะคืนค่าเป็น:

```rust
Option<Item>
```

โดยมีความหมายว่า:

- `Some(value)` มีข้อมูลรายการถัดไป
- `None` ไม่มีข้อมูลเหลือแล้ว

## 3. สร้าง Iterator จาก Vector

Vector สามารถสร้าง Iterator ด้วย `iter()` ได้

```rust
let numbers = vec![10, 20, 30];

let mut iterator = numbers.iter();
```

เรียก `next()` เพื่ออ่านข้อมูลทีละรายการ

```rust
println!("{:?}", iterator.next());
println!("{:?}", iterator.next());
println!("{:?}", iterator.next());
println!("{:?}", iterator.next());
```

ผลลัพธ์:

```text
Some(10)
Some(20)
Some(30)
None
```

## 4. iter()

`iter()` ใช้สร้าง Iterator ที่อ่านสมาชิกผ่าน Immutable Reference

ลักษณะสำคัญ:

- ไม่ย้าย Ownership ของสมาชิกออกจาก Collection
- สามารถอ่านข้อมูลได้
- ไม่สามารถแก้ไขสมาชิกผ่าน Iterator นี้โดยตรง

ตัวอย่าง:

```rust
let numbers = vec![10, 20, 30];

for number in numbers.iter() {
    println!("{}", number);
}
```

## 5. iter_mut()

`iter_mut()` ใช้สร้าง Iterator ที่เข้าถึงสมาชิกผ่าน Mutable Reference

ตัวอย่าง:

```rust
let mut numbers = vec![10, 20, 30];

for number in numbers.iter_mut() {
    *number += 5;
}

println!("{:?}", numbers);
```

ผลลัพธ์:

```text
[15, 25, 35]
```

เครื่องหมาย `*` ใช้ Dereference เพื่อเข้าถึงค่าที่ Reference ชี้อยู่

## 6. into_iter()

`into_iter()` ใช้สร้าง Iterator ที่รับสมาชิกออกมาตาม Ownership ของ Collection

ตัวอย่าง:

```rust
let numbers = vec![10, 20, 30];

let iterator = numbers.into_iter();

for number in iterator {
    println!("{}", number);
}
```

หลังจากย้าย Ownership ไปแล้ว จะไม่สามารถใช้งานตัวแปร `numbers` เดิมได้ตามปกติ

## 7. เปรียบเทียบ iter(), iter_mut() และ into_iter()

| Method        | สิ่งที่ได้จาก Iterator   | ผลต่อ Collection                 |
| ------------- | ------------------------ | -------------------------------- |
| `iter()`      | `&T`                     | ยังคงเป็นเจ้าของข้อมูลเดิม       |
| `iter_mut()`  | `&mut T`                 | ยังคงเป็นเจ้าของข้อมูลเดิม       |
| `into_iter()` | `T` สำหรับ Vec ในกรณีนี้ | ย้าย Ownership ออกจาก Collection |

หมายเหตุ: พฤติกรรมของ `into_iter()` ขึ้นอยู่กับชนิด Collection และบริบทการใช้งาน ส่วนตารางนี้อธิบายกรณี Vector

## 8. Iterator กับ for Loop

สามารถใช้ Iterator ร่วมกับ `for` ได้โดยตรง

```rust
let numbers = vec![1, 2, 3];

for number in numbers.iter() {
    println!("{}", number);
}
```

เมื่อใช้ `for` กับ Iterator ระบบจะเรียก `next()` เพื่อดึงข้อมูลแต่ละรายการจนกว่าจะได้ `None`

## 9. Lazy Evaluation

Iterator ใน Rust ใช้แนวคิด Lazy Evaluation

หมายความว่า การสร้าง Iterator หรือ Adapter ส่วนใหญ่ยังไม่ได้ประมวลผลสมาชิกทั้งหมดทันที แต่จะทำงานเมื่อมีการดึงข้อมูลหรือมี Consumer มาใช้งาน

ตัวอย่างแนวคิด:

- สร้าง Iterator
- กำหนดวิธีแปลงข้อมูล
- เรียก Consumer เพื่อเริ่มประมวลผล

## 10. สรุป

- Iterator ใช้ประมวลผลข้อมูลทีละรายการ
- `next()` คืนค่าเป็น `Option`
- `iter()` อ่านผ่าน Immutable Reference
- `iter_mut()` อ่านและแก้ไขผ่าน Mutable Reference
- `into_iter()` รับ Ownership ของสมาชิกตามชนิด Collection
- Iterator ใช้ Lazy Evaluation

## Checklist

- [ ] เข้าใจ Iterator
- [ ] เข้าใจ Iterator Trait
- [ ] เข้าใจการทำงานของ `next()`
- [ ] ใช้ `iter()` ได้
- [ ] ใช้ `iter_mut()` ได้
- [ ] ใช้ `into_iter()` ได้
- [ ] เข้าใจความแตกต่างของทั้งสาม Method
- [ ] เข้าใจ Lazy Evaluation
