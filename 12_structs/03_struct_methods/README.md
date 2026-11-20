# Lesson 03: Struct Methods

## 1. What is a Method?

Method คือ Function ที่กำหนดให้ทำงานร่วมกับ Struct หรือชนิดข้อมูลหนึ่ง ๆ

ใน Rust เราใช้ `impl` เพื่อกำหนด Methods ให้กับ Struct

ตัวอย่างเช่น Struct `Rectangle` สามารถมี Method สำหรับคำนวณพื้นที่ของตัวเองได้

## 2. The impl Block

รูปแบบพื้นฐาน:

```rust
struct StructName {
    field_name: DataType,
}

impl StructName {
    fn method_name(&self) {
        // method body
    }
}
```

อธิบาย:

- `impl` ใช้กำหนด Methods หรือ Associated Functions ให้กับชนิดข้อมูล
- `StructName` คือชนิดข้อมูลที่ต้องการเพิ่มความสามารถ
- `method_name` คือชื่อ Method
- `&self` ใช้อ้างอิง Instance ที่เรียก Method

## 3. Method with &self

`&self` เป็นการยืม Instance แบบ Immutable Reference

เหมาะสำหรับ Method ที่ต้องการอ่านข้อมูล แต่ไม่ต้องแก้ไขค่าใน Struct

ตัวอย่าง:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

ภายใน Method สามารถเข้าถึง Fields ผ่าน `self.field_name` ได้

## 4. Calling a Method

เรียก Method ผ่าน Instance ด้วยเครื่องหมายจุด

```rust
let rectangle = Rectangle {
    width: 10,
    height: 5,
};

println!("Area: {}", rectangle.area());
```

ผลลัพธ์:

```text
Area: 50
```

## 5. Method with &mut self

`&mut self` ใช้ยืม Instance แบบ Mutable Reference ทำให้ Method สามารถแก้ไขข้อมูลภายใน Struct ได้

ตัวอย่าง:

```rust
struct Counter {
    value: u32,
}

impl Counter {
    fn increase(&mut self) {
        self.value += 1;
    }
}
```

การเรียกใช้ต้องใช้ Mutable Instance:

```rust
let mut counter = Counter {
    value: 0,
};

counter.increase();
```

## 6. Method with self

`self` หมายถึง Method รับ Ownership ของ Instance

เมื่อเรียก Method ด้วย `self` ตัว Instance จะถูกย้ายเข้าไปใน Method และโดยทั่วไปจะไม่สามารถนำตัวแปรเดิมมาใช้งานต่อได้หลังจากถูกย้าย Ownership แล้ว

รูปแบบนี้เหมาะกับกรณีที่ Method ต้องการรับ Ownership ของ Instance ไปจัดการ

## 7. Associated Functions

Function ที่อยู่ภายใน `impl` แต่ไม่มี `self` เรียกว่า Associated Function

มักใช้สร้าง Constructor สำหรับสร้าง Instance

ตัวอย่าง:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
```

เรียกใช้ด้วยชื่อ Struct และเครื่องหมาย `::`

```rust
let rectangle = Rectangle::new(10, 5);
```

`Self` หมายถึงชนิดข้อมูลที่กำลังอยู่ภายใน `impl`

## 8. Summary

| Receiver       | Meaning                        | Typical Use                                           |
| -------------- | ------------------------------ | ----------------------------------------------------- |
| `&self`        | ยืม Instance แบบอ่านอย่างเดียว | อ่านข้อมูลหรือคำนวณ                                   |
| `&mut self`    | ยืม Instance แบบแก้ไขได้       | เปลี่ยนค่า Fields                                     |
| `self`         | รับ Ownership ของ Instance     | รับ Instance ไปเป็นเจ้าของ                            |
| ไม่มี Receiver | Associated Function            | สร้าง Instance หรือทำงานที่ไม่ต้องใช้ Instance โดยตรง |

## 9. Knowledge Check

1. `impl` ใช้ทำอะไร?
2. Method แตกต่างจาก Function ทั่วไปอย่างไร?
3. `&self` ใช้ในกรณีใด?
4. `&mut self` แตกต่างจาก `&self` อย่างไร?
5. `self` มีผลต่อ Ownership อย่างไร?
6. Associated Function เรียกใช้อย่างไร?
7. `Self` หมายถึงอะไร?

## 10. Completion Checklist

- [ ] เข้าใจความหมายของ Method
- [ ] สร้าง `impl` Block ได้
- [ ] สร้าง Method ที่ใช้ `&self` ได้
- [ ] สร้าง Method ที่ใช้ `&mut self` ได้
- [ ] เข้าใจการรับ Ownership ด้วย `self`
- [ ] สร้าง Associated Function ได้
- [ ] ใช้ `Self` ใน Constructor ได้
- [ ] เรียก Methods ผ่าน Instance ได้
