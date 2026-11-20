# Lesson 03: Match with Enums

## 1. What is match?

`match` เป็น Control Flow ที่ใช้ตรวจสอบค่าหนึ่ง แล้วเลือกทำงานตาม Pattern ที่ตรงกัน

สามารถใช้กับ Enum ได้อย่างเหมาะสม เพราะ Enum มี Variants ที่กำหนดไว้ชัดเจน

รูปแบบ:

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    pattern3 => expression3,
}
```

## 2. Matching Enum Variants

ตัวอย่าง:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Prepare"),
        TrafficLight::Green => println!("Go"),
    }
}
```

ผลลัพธ์:

```text
Stop
```

อธิบาย:

- `match light` ตรวจสอบค่าของตัวแปร `light`
- แต่ละ Pattern ตรวจสอบ Variant
- เมื่อพบ Pattern ที่ตรงกัน จะทำงานในแขนนั้น
- ในตัวอย่างนี้ `light` เป็น `TrafficLight::Red` จึงแสดง `Stop`

## 3. Exhaustive Matching

Rust กำหนดให้ `match` ต้องครอบคลุมทุกกรณีที่เป็นไปได้ของชนิดข้อมูลนั้น

เรียกว่า **Exhaustive Matching**

ตัวอย่าง Enum ที่มี 3 Variants:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

หากใช้ `match` ต้องจัดการครบทั้ง 4 Variants หรือใช้ Pattern ที่ครอบคลุมกรณีที่เหลือ เช่น `_`

ตัวอย่าง:

```rust
match direction {
    Direction::North => println!("North"),
    Direction::South => println!("South"),
    _ => println!("Other direction"),
}
```

`_` คือ Wildcard Pattern ใช้แทนค่าที่เหลือซึ่งยังไม่ได้ระบุ

## 4. Matching Tuple Variants

สามารถดึงข้อมูลภายใน Tuple Variant ออกมาใช้งานได้

ตัวอย่าง:

```rust
enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
}

fn main() {
    let message = Message::Move(10, 20);

    match message {
        Message::Quit => println!("Quit"),
        Message::Move(x, y) => {
            println!("X: {}", x);
            println!("Y: {}", y);
        }
        Message::Write(text) => {
            println!("Text: {}", text);
        }
    }
}
```

Pattern `Message::Move(x, y)` จะดึงข้อมูลทั้งสองตำแหน่งมาเก็บในตัวแปร `x` และ `y`

## 5. Matching Struct Variants

สามารถดึงข้อมูลจาก Struct Variant ด้วยชื่อ Field ได้

ตัวอย่าง:

```rust
enum Message {
    Move { x: i32, y: i32 },
}

fn main() {
    let message = Message::Move {
        x: 10,
        y: 20,
    };

    match message {
        Message::Move { x, y } => {
            println!("X: {}", x);
            println!("Y: {}", y);
        }
    }
}
```

## 6. match as an Expression

`match` สามารถคืนค่าได้ โดยแต่ละแขนต้องให้ผลลัพธ์ที่เข้ากันได้กับชนิดข้อมูลที่ต้องการ

ตัวอย่าง:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Green;

    let message = match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Prepare",
        TrafficLight::Green => "Go",
```
