# Lesson 01: Unsafe Fundamentals

## 1. Safe Rust คืออะไร?

Safe Rust คือส่วนของภาษา Rust ที่ Compiler ตรวจสอบกฎความปลอดภัยด้าน Memory และ Ownership ให้

ตัวอย่าง:

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    println!("{}", numbers[0]);
}
```

Compiler ช่วยตรวจสอบเรื่องต่าง ๆ เช่น:

- Ownership
- Borrowing
- Lifetime
- Mutable References
- Type Safety

อย่างไรก็ตาม Safe Rust ไม่ได้หมายความว่าโปรแกรมไม่มี Bug ทุกประเภท เช่น Logic Error ยังสามารถเกิดขึ้นได้

## 2. Unsafe Rust คืออะไร?

Unsafe Rust เป็นส่วนของภาษา Rust ที่อนุญาตให้ใช้งาน Operation บางอย่างซึ่ง Compiler ไม่สามารถตรวจสอบ Safety ได้ทั้งหมด

ตัวอย่างความสามารถ:

1. Dereference Raw Pointers
2. เรียก Unsafe Functions
3. เข้าถึง Mutable Static Variables
4. Implement Unsafe Traits
5. ใช้ Union Fields

การใช้ `unsafe` ไม่ได้ปิดการตรวจสอบทั้งหมดของ Compiler แต่เป็นการยอมรับความรับผิดชอบต่อ Operation ที่ต้องมีการรับประกันเพิ่มเติมจากผู้เขียน

## 3. Unsafe Block

ใช้ `unsafe` Block เพื่อเรียก Operation ที่ต้องการ Unsafe Context

รูปแบบ:

```rust
unsafe {
    // Unsafe operations
}
```

ตัวอย่าง:

```rust
fn main() {
    let value = 100;
    let pointer = &raw const value;

    unsafe {
        println!("{}", *pointer);
    }
}
```

ในตัวอย่างนี้:

- `&raw const value` สร้าง Raw Pointer
- `*pointer` เป็นการ Dereference
- ต้องใช้ `unsafe` Block สำหรับ Dereference Raw Pointer

## 4. Unsafe Operations

Operation ที่ต้องใช้ Unsafe Context ได้แก่:

| Operation               | รายละเอียด                            |
| ----------------------- | ------------------------------------- |
| Dereference Raw Pointer | อ่านหรือเขียนผ่าน Raw Pointer         |
| Unsafe Function Call    | เรียก Function ที่ประกาศเป็น Unsafe   |
| Access Mutable Static   | เข้าถึง Mutable Static Variable       |
| Implement Unsafe Trait  | Implement Trait ที่มี Safety Contract |
| Access Union Field      | เข้าถึง Field ของ Union               |

หมายเหตุ: Unsafe Context ไม่ได้หมายความว่าโค้ดทุกบรรทัดภายใน Block เป็นอันตราย แต่เฉพาะ Operation ที่ต้องการ Safety Guarantee เท่านั้น

## 5. Unsafe Function

Unsafe Function คือ Function ที่กำหนดให้ผู้เรียกต้องรับผิดชอบต่อ Safety Contract

ตัวอย่าง:

```rust
unsafe fn read_value(pointer: *const i32) -> i32 {
    unsafe {
        *pointer
    }
}

fn main() {
    let value = 100;
    let pointer = &raw const value;

    let result = unsafe {
        read_value(pointer)
    };

    println!("{}", result);
}
```

ตัวอย่างนี้แสดงให้เห็นว่า:

- Function ถูกประกาศด้วย `unsafe fn`
- ผู้เรียกต้องใช้ Unsafe Context
- การ Dereference Pointer ภายใน Function ต้องใช้ Unsafe Block เช่นกัน

## 6. Unsafe ไม่ได้หมายถึงไม่มีการตรวจสอบ

แม้จะใช้ `unsafe` แต่ Rust Compiler ยังคงตรวจสอบ:

- Syntax
- Type
- Function Signature
- Ownership ส่วนที่เกี่ยวข้อง
- Borrowing ส่วนที่เกี่ยวข้อง
- การใช้ API ตามข้อกำหนดของภาษา

สิ่งที่ Compiler ไม่สามารถรับประกันให้ทั้งหมด คือ Safety Conditions ของ Unsafe Operations

## 7. Safety Invariants

Safety Invariant คือเงื่อนไขที่ต้องรักษาไว้เพื่อให้ Unsafe Code ทำงานได้อย่างถูกต้อง

ตัวอย่างเงื่อนไขของ Raw Pointer:

- Pointer ต้องไม่เป็น Null เมื่อ Operation ต้องการ Pointer ที่ใช้งานได้
- Pointer ต้องชี้ไปยัง Memory ที่ยังมีชีวิตอยู่
- Memory ต้องมี Alignment ถูกต้อง
- การอ่านและเขียนต้องสอดคล้องกับ Validity ของชนิดข้อมูล
- ต้องไม่มี Data Race
- การเข้าถึง Mutable Memory ต้องไม่ละเมิด Aliasing Rules

## 8. แนวทางการใช้ Unsafe

ก่อนเขียน Unsafe Code ควรตรวจสอบ:

1. มีวิธีใช้ Safe Rust แทนหรือไม่?
2. Operation ใดจำเป็นต้องใช้ Unsafe?
3. ต้องรักษาเงื่อนไขอะไร?
4. Pointer หรือ Memory มีอายุการใช้งานเพียงพอหรือไม่?
5. มีการเข้าถึง Memory พร้อมกันหรือไม่?
6. สามารถเขียน Test ตรวจสอบพฤติกรรมได้หรือไม่?

## 9. สรุป

| คำศัพท์          | ความหมาย                                                |
| ---------------- | ------------------------------------------------------- |
| Safe Rust        | Rust ที่ Compiler ตรวจสอบ Safety ตามกฎของภาษา           |
| Unsafe Rust      | Rust ที่อนุญาต Operation บางอย่างนอกเหนือการตรวจสอบปกติ |
| Unsafe Block     | ขอบเขตสำหรับ Unsafe Operations                          |
| Unsafe Function  | Function ที่ผู้เรียกต้องรักษา Safety Contract           |
| Safety Invariant | เงื่อนไขที่ต้องรักษาเพื่อความปลอดภัย                    |

## Completion Checklist

- [ ] เข้าใจ Safe Rust
- [ ] เข้าใจ Unsafe Rust
- [ ] อธิบายความแตกต่างได้
- [ ] ใช้ `unsafe` Block ได้
- [ ] เข้าใจ Unsafe Operations
- [ ] เข้าใจ Unsafe Function
- [ ] เข้าใจ Safety Invariants
- [ ] ระบุเหตุผลก่อนใช้ Unsafe ได้
