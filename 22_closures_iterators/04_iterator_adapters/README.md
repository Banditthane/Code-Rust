# Lesson 04: Iterator Adapters and Consumers

## 1. Iterator Adapters คืออะไร?

Iterator Adapter คือ Method ที่ใช้แปลงหรือปรับวิธีประมวลผลข้อมูลจาก Iterator เดิมให้เป็น Iterator ใหม่

Adapter ส่วนใหญ่ใช้ Lazy Evaluation หมายความว่ายังไม่ประมวลผลข้อมูลทันทีเมื่อเรียก Method

## 2. map()

`map()` ใช้แปลงสมาชิกแต่ละรายการให้เป็นค่าใหม่

ตัวอย่าง:

```rust
let numbers = vec![1, 2, 3];

let result: Vec<i32> = numbers
    .iter()
    .map(|number| number * 2)
    .collect();

println!("{:?}", result);
```

ผลลัพธ์:

```text
[2, 4, 6]
```

แนวคิด:

- อ่านสมาชิกทีละรายการ
- ส่งสมาชิกเข้า Closure
- สร้างค่าผลลัพธ์ใหม่
- คืน Iterator ใหม่

## 3. filter()

`filter()` ใช้เลือกเฉพาะสมาชิกที่ผ่านเงื่อนไข

ตัวอย่าง:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let result: Vec<i32> = numbers
    .iter()
    .filter(|number| **number % 2 == 0)
    .copied()
    .collect();

println!("{:?}", result);
```

ผลลัพธ์:

```text
[2, 4, 6]
```

หมายเหตุ:

- `filter()` ตรวจสอบเงื่อนไขด้วย Reference ของสมาชิก
- `copied()` คัดลอกค่าที่เป็น Copy ออกจาก Reference
- สำหรับชนิดข้อมูลที่ไม่ใช่ Copy อาจต้องใช้วิธีแปลงหรือจัดการ Ownership ที่เหมาะสม

## 4. filter_map()

`filter_map()` ใช้ทั้งแปลงข้อมูลและคัดกรองข้อมูลในขั้นตอนเดียว

Closure จะคืนค่า `Option<T>`:

- `Some(value)` เก็บค่าที่แปลงแล้ว
- `None` ตัดรายการนั้นออก

เหมาะกับกรณีที่ต้องแปลงข้อมูลซึ่งอาจไม่สำเร็จทุกครั้ง เช่น การแปลงข้อความเป็นตัวเลข

## 5. fold()

`fold()` ใช้รวบรวมข้อมูลหลายรายการให้กลายเป็นผลลัพธ์เดียว โดยใช้ค่าเริ่มต้นและ Closure

รูปแบบแนวคิด:

```rust
iterator.fold(initial_value, |accumulator, item| {
    // คำนวณค่ารอบถัดไป
})
```

ตัวอย่าง:

```rust
let numbers = vec![1, 2, 3, 4];

let total = numbers
    .iter()
    .fold(0, |sum, number| sum + number);

println!("{}", total);
```

ผลลัพธ์:

```text
10
```

คำศัพท์สำคัญ:

- `initial_value` คือค่าเริ่มต้น
- `accumulator` คือค่าที่สะสมจากรอบก่อน
- `item` คือข้อมูลในรอบปัจจุบัน

## 6. Iterator Consumers คืออะไร?

Consumer คือ Method ที่ใช้ Iterator เพื่อประมวลผลจนได้ผลลัพธ์สุดท้าย

Consumer จะทำให้ Iterator เริ่มทำงานจริง

## 7. collect()

`collect()` ใช้รวบรวมสมาชิกจาก Iterator ให้กลายเป็น Collection

ตัวอย่าง:

```rust
let numbers = vec![1, 2, 3];

let result: Vec<i32> = numbers
    .iter()
    .map(|number| number * 10)
    .copied()
    .collect();

println!("{:?}", result);
```

ผลลัพธ์:

```text
[10, 20, 30]
```

Rust อาจต้องการข้อมูลชนิดผลลัพธ์เพื่ออนุมานว่าจะรวบรวมเป็น Collection ประเภทใด จึงสามารถระบุชนิดข้อมูลให้ชัดเจนได้

## 8. sum()

`sum()` ใช้หาผลรวมของสมาชิกใน Iterator

ตัวอย่าง:

```rust
let numbers = vec![10, 20, 30];

let total: i32 = numbers.iter().sum();

println!("{}", total);
```

ผลลัพธ์:

```text
60
```

## 9. count()

`count()` ใช้นับจำนวนสมาชิกที่เหลืออยู่ใน Iterator

ตัวอย่าง:

```rust
let numbers = vec![10, 20, 30, 40];

let total = numbers.iter().count();

println!("{}", total);
```

ผลลัพธ์:

```text
4
```

ข้อควรระวัง: Consumer บางตัว เช่น `count()` จะใช้ Iterator จนหมด ทำให้ไม่สามารถนำ Iterator ตัวเดิมมาอ่านต่อได้

## 10. Method ที่ควรรู้เพิ่มเติม

| Method         | หน้าที่                                 |
| -------------- | --------------------------------------- |
| `map()`        | แปลงสมาชิก                              |
| `filter()`     | เลือกสมาชิกตามเงื่อนไข                  |
| `filter_map()` | แปลงและกรองข้อมูล                       |
| `fold()`       | สะสมผลลัพธ์                             |
| `collect()`    | รวบรวมเป็น Collection                   |
| `sum()`        | หาผลรวม                                 |
| `count()`      | นับจำนวน                                |
| `any()`        | ตรวจว่ามีสมาชิกที่ผ่านเงื่อนไขหรือไม่   |
| `all()`        | ตรวจว่าสมาชิกทั้งหมดผ่านเงื่อนไขหรือไม่ |
| `find()`       | ค้นหาสมาชิกตัวแรกที่ผ่านเงื่อนไข        |

## 11. การต่อ Iterator Methods

สามารถนำ Adapter หลายตัวมาต่อกันเป็น Chain ได้

แนวคิด:

1. เริ่มจาก Collection
2. สร้าง Iterator
3. ใช้ Adapter เพื่อแปลงหรือกรองข้อมูล
4. ใช้ Consumer เพื่อรับผลลัพธ์

ตัวอย่างลักษณะการประมวลผล:

```text
Vector
  ↓
iter()
  ↓
filter()
  ↓
map()
  ↓
collect()
  ↓
Result Collection
```

## 12. ข้อควรระวัง

- Adapter ส่วนใหญ่ยังไม่ประมวลผลทันที
- Consumer เป็นตัวกระตุ้นให้ Iterator ทำงาน
- ต้องเข้าใจชนิดข้อมูลที่ Iterator ส่งออกมา
- ระวัง Reference และ Ownership
- `collect()` ต้องทราบชนิด Collection ที่ต้องการ
- Iterator ที่ถูกใช้จนหมดแล้วไม่สามารถอ่านข้อมูลเดิมต่อได้

## 13. สรุป

- `map()` แปลงข้อมูล
- `filter()` คัดกรองข้อมูล
- `filter_map()` แปลงและกรองข้อมูล
- `fold()` สะสมผลลัพธ์
- `collect()` สร้าง Collection
- `sum()` หาผลรวม
- `count()` นับจำนวน
- Iterator Adapters ใช้ร่วมกันเป็น Chain ได้

## Checklist

- [ ] เข้าใจ Iterator Adapter
- [ ] ใช้ `map()` ได้
- [ ] ใช้ `filter()` ได้
- [ ] เข้าใจ `filter_map()`
- [ ] ใช้ `fold()` ได้
- [ ] ใช้ `collect()` ได้
- [ ] ใช้ `sum()` และ `count()` ได้
- [ ] เข้าใจ Lazy Evaluation
- [ ] สามารถต่อ Iterator Methods หลายตัวได้
