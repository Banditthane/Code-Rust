# Lesson 01: Macro Fundamentals

## 1. Macro คืออะไร?

Macro คือเครื่องมือของ Rust ที่ช่วยสร้างหรือขยายโค้ดในระหว่างกระบวนการ Compile

Macro เหมาะกับกรณีที่ต้องการ:

- ลดโค้ดที่เขียนซ้ำ
- สร้างโค้ดตามรูปแบบที่กำหนด
- รองรับจำนวน Arguments ที่ยืดหยุ่น
- สร้างโครงสร้างโค้ดจากข้อมูลที่รับเข้ามา
- เพิ่มความสามารถให้ Struct หรือ Enum

ตัวอย่าง Macro ที่ใช้งานอยู่แล้ว:

```rust
println!("Hello Rust");
vec![1, 2, 3];
format!("Hello {}", "Rust");
```

สังเกตว่า Macro มักมีเครื่องหมาย `!` ต่อท้ายชื่อ

## 2. Function กับ Macro ต่างกันอย่างไร?

| Function                                | Macro                         |
| --------------------------------------- | ----------------------------- |
| รับค่า Arguments ตาม Parameter ที่กำหนด | สามารถกำหนดรูปแบบ Input ได้   |
| ทำงานเมื่อโปรแกรมรัน                    | ขยายโค้ดในกระบวนการ Compile   |
| มี Return Type                          | สร้างโค้ดที่นำไป Compile ต่อ  |
| ใช้ Function Call                       | ใช้รูปแบบ Macro Invocation    |
| ตรวจสอบชนิดข้อมูลตาม Signature          | ตรวจสอบตามโค้ดที่ Macro สร้าง |

ตัวอย่าง Function:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

ตัวอย่าง Macro:

```rust
macro_rules! say_hello {
    () => {
        println!("Hello Rust");
    };
}

fn main() {
    say_hello!();
}
```

## 3. Macro Invocation

การเรียก Macro ใช้เครื่องหมาย `!`

ตัวอย่าง:

```rust
println!("Hello");
vec![10, 20, 30];
format!("Score: {}", 100);
```

เครื่องหมาย `!` ช่วยแยกการเรียก Macro ออกจาก Function Call

## 4. ประเภทของ Macro

Macro ใน Rust แบ่งเป็น 2 กลุ่มหลัก:

### Declarative Macro

กำหนด Pattern ของ Input และ Template ของโค้ดที่ต้องการสร้าง

เครื่องมือหลักคือ:

```rust
macro_rules!
```

### Procedural Macro

รับ TokenStream แล้วสร้าง TokenStream ใหม่

แบ่งเป็น 3 ประเภท:

1. Derive Macro
2. Attribute Macro
3. Function-like Macro

## 5. Macro Expansion

Macro Expansion คือกระบวนการที่ Compiler ขยาย Macro ให้กลายเป็นโค้ด Rust

ตัวอย่าง:

```rust
macro_rules! create_message {
    () => {
        println!("Macro Expanded");
    };
}

fn main() {
    create_message!();
}
```

แนวคิด:

```text
Macro Invocation
       |
       v
Macro Expansion
       |
       v
Generated Rust Code
       |
       v
Compilation
```

Macro ไม่ได้รอให้โปรแกรมทำงานถึงจุดเรียกก่อนจึงสร้างโค้ด แต่การขยายเกิดขึ้นในกระบวนการ Compile

## 6. Macro Arguments

Macro สามารถรับ Input ได้หลายรูปแบบ ขึ้นอยู่กับ Pattern ที่กำหนด

ตัวอย่าง:

```rust
macro_rules! show_value {
    ($value:expr) => {
        println!("Value: {}", $value);
    };
}

fn main() {
    show_value!(100);
    show_value!(10 + 20);
}
```

`$value:expr` หมายถึงรับ Input ที่เป็น Expression

## 7. Macro Hygiene

Macro Hygiene เป็นแนวคิดที่ช่วยป้องกันปัญหาชื่อ Identifier ชนกันระหว่างโค้ดที่ Macro สร้างกับโค้ดของผู้เรียก

Declarative Macro ใน Rust มีระบบ Hygiene สำหรับ Identifier ที่สร้างขึ้นภายใน Macro

อย่างไรก็ตาม Macro ยังสามารถสร้างชื่อที่ตั้งใจให้ผู้เรียกใช้งานได้ จึงควรออกแบบชื่อและขอบเขตให้ชัดเจน

## 8. ข้อควรระวัง

- Macro อาจทำให้โค้ดอ่านยากหากมีความซับซ้อนมากเกินไป
- Error จาก Macro Expansion อาจชี้ไปยังโค้ดที่ถูกสร้างขึ้น
- ควรใช้ Function หาก Function สามารถแก้ปัญหาได้อย่างตรงไปตรงมา
- Macro ควรมีชื่อและรูปแบบ Input ที่สื่อความหมาย
- ควรตรวจสอบโค้ดที่ Macro สร้างขึ้นเมื่อเกิดปัญหา

## 9. สรุป

| คำศัพท์           | ความหมาย                            |
| ----------------- | ----------------------------------- |
| Macro             | เครื่องมือสร้างหรือขยายโค้ด         |
| Invocation        | การเรียก Macro                      |
| Declarative Macro | Macro ที่กำหนด Pattern และ Template |
| Procedural Macro  | Macro ที่ประมวลผล TokenStream       |
| Expansion         | การขยาย Macro เป็นโค้ด              |
| Hygiene           | กลไกช่วยจัดการชื่อ Identifier       |

## Completion Checklist

- [ ] เข้าใจ Macro
- [ ] อธิบาย Function กับ Macro ได้
- [ ] เข้าใจเครื่องหมาย `!`
- [ ] เข้าใจ Declarative Macro
- [ ] เข้าใจ Procedural Macro
- [ ] เข้าใจ Macro Expansion
- [ ] เข้าใจ Macro Hygiene
- [ ] เลือกใช้ Macro หรือ Function ได้เหมาะสม
