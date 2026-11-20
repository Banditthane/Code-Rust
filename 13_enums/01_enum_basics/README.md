# Lesson 01: Enum Basics

## 1. What is an Enum?

Enum คือ Custom Data Type ที่ใช้กำหนดชุดของตัวเลือกหรือรูปแบบที่เป็นไปได้

ตัวอย่างเช่น สัญญาณไฟจราจรมี 3 สถานะ:

- Red
- Yellow
- Green

เราสามารถสร้าง Enum เพื่อแทนสถานะเหล่านี้ได้

## 2. Enum Declaration

รูปแบบการประกาศ Enum:

```rust
enum EnumName {
    VariantOne,
    VariantTwo,
    VariantThree,
}
```

ตัวอย่าง:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

อธิบาย:

- `enum` ใช้ประกาศ Enum
- `TrafficLight` คือชื่อ Enum
- `Red`, `Yellow`, `Green` คือ Variants

Variant คือหนึ่งในตัวเลือกที่ Enum ชนิดนั้นสามารถมีได้

## 3. Creating an Enum Value

เมื่อประกาศ Enum แล้ว สามารถสร้างค่าโดยระบุชื่อ Enum และ Variant ผ่าน `::`

```rust
let light = TrafficLight::Red;
```

อธิบาย:

- `TrafficLight` คือชื่อ Enum
- `::` ใช้ระบุสมาชิกของ Enum
- `Red` คือ Variant ที่เลือกใช้

ตัวอย่างเพิ่มเติม:

```rust
let first = TrafficLight::Red;
let second = TrafficLight::Yellow;
let third = TrafficLight::Green;
```

ตัวแปรทั้งสามมีชนิดข้อมูลเป็น `TrafficLight`

## 4. Using Enum with Functions

สามารถใช้ Enum เป็น Parameter และ Return Type ของ Function ได้

ตัวอย่าง:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn print_light(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Prepare"),
        TrafficLight::Green => println!("Go"),
    }
}
```

Function `print_light()` รับค่า Enum ชนิด `TrafficLight` แล้วเลือกการทำงานตาม Variant

## 5. Enum vs Struct

| Enum                           | Struct                               |
| ------------------------------ | ------------------------------------ |
| ใช้แทนชุดตัวเลือกที่เป็นไปได้  | ใช้รวม Fields ที่เกี่ยวข้องกัน       |
| มี Variants                    | มี Fields                            |
| แต่ละค่าเลือก Variant หนึ่งแบบ | Instance มี Fields ตามที่ประกาศ      |
| เหมาะกับสถานะหรือประเภท        | เหมาะกับข้อมูลที่ประกอบกันเป็นรายการ |

ตัวอย่าง:

- `TrafficLight` เหมาะกับ Enum เพราะไฟจราจรมีสถานะที่เลือกได้
- `Student` เหมาะกับ Struct เพราะนักเรียนหนึ่งคนมีชื่อ อายุ และคะแนนพร้อมกัน

## 6. Important Notes

- Enum เป็น Custom Data Type
- Variant เป็นตัวเลือกภายใน Enum
- ใช้ `EnumName::VariantName` เพื่อระบุ Variant
- Enum สามารถใช้เป็น Parameter และ Return Type ได้
- Enum สามารถนำไปใช้ร่วมกับ `match` ได้

## 7. Knowledge Check

1. Enum คืออะไร?
2. Variant หมายถึงอะไร?
3. `enum` ใช้ทำอะไร?
4. `TrafficLight::Red` หมายถึงอะไร?
5. Enum แตกต่างจาก Struct อย่างไร?

## 8. Completion Checklist

- [ ] เข้าใจความหมายของ Enum
- [ ] ประกาศ Enum ได้
- [ ] กำหนด Variants ได้
- [ ] สร้าง Enum Value ได้
- [ ] ใช้ `EnumName::VariantName` ได้
- [ ] เข้าใจความแตกต่างระหว่าง Enum และ Struct
