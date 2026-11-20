# Lesson 02: Enum Variants

## 1. What are Enum Variants?

Variant คือรูปแบบหรือสมาชิกแต่ละแบบที่ประกาศไว้ภายใน Enum

Rust รองรับ Variant ที่สามารถเก็บข้อมูลได้หลายรูปแบบ ไม่จำเป็นต้องเป็นเพียงชื่อสถานะอย่างเดียว

รูปแบบหลักที่ควรรู้จัก:

1. Unit Variant
2. Tuple Variant
3. Struct Variant

## 2. Unit Variants

Unit Variant คือ Variant ที่ไม่มีข้อมูลเพิ่มเติม

ตัวอย่าง:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

สร้างค่า:

```rust
let direction = Direction::North;
```

เหมาะกับกรณีที่ต้องการเก็บเฉพาะประเภทหรือสถานะ

## 3. Tuple Variants

Tuple Variant สามารถเก็บข้อมูลภายในได้ โดยกำหนดชนิดข้อมูลตามตำแหน่ง

ตัวอย่าง:

```rust
enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
    ChangeColor(u8, u8, u8),
}
```

ความหมาย:

| Variant     | Data                |
| ----------- | ------------------- |
| Quit        | ไม่มีข้อมูล         |
| Move        | ค่า i32 จำนวน 2 ค่า |
| Write       | String จำนวน 1 ค่า  |
| ChangeColor | u8 จำนวน 3 ค่า      |

สร้างค่า:

```rust
let message1 = Message::Quit;
let message2 = Message::Move(10, 20);
let message3 = Message::Write(String::from("Hello"));
let message4 = Message::ChangeColor(255, 0, 0);
```

แต่ละ Variant สามารถเก็บข้อมูลคนละชนิดและคนละจำนวนได้

## 4. Struct Variants

Struct Variant ใช้รูปแบบคล้าย Struct โดยกำหนดชื่อ Fields ให้กับ Variant

ตัวอย่าง:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

สร้างค่า:

```rust
let message = Message::Move {
    x: 10,
    y: 20,
};
```

ข้อดีคือชื่อ Field ช่วยให้เข้าใจว่าข้อมูลแต่ละค่าหมายถึงอะไร

## 5. Accessing Variant Data

การตรวจสอบและเข้าถึงข้อมูลภายใน Variant สามารถทำได้ด้วย `match`

ตัวอย่าง:

```rust
match message {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => {
        println!("Move to {}, {}", x, y);
    }
    Message::Write(text) => {
        println!("Message: {}", text);
    }
}
```

ภายใน Pattern สามารถดึงข้อมูลออกมาใช้ผ่านตัวแปรได้

## 6. Important Notes

- Enum หนึ่งตัวสามารถมีหลาย Variants
- Unit Variant ไม่มีข้อมูลเพิ่มเติม
- Tuple Variant เก็บข้อมูลตามตำแหน่ง
- Struct Variant เก็บข้อมูลด้วยชื่อ Fields
- แต่ละ Variant สามารถมี Data Type แตกต่างกันได้
- ใช้ `match` เพื่อแยกการทำงานและเข้าถึงข้อมูลใน Variant

## 7. Knowledge Check

1. Unit Variant คืออะไร?
2. Tuple Variant แตกต่างจาก Unit Variant อย่างไร?
3. Struct Variant มีจุดเด่นอย่างไร?
4. `Message::Move(10, 20)` เก็บข้อมูลอะไร?
5. สามารถใช้ Data Type ต่างกันในแต่ละ Variant ได้หรือไม่?

## 8. Completion Checklist

- [ ] เข้าใจ Unit Variant
- [ ] ประกาศ Tuple Variant ได้
- [ ] สร้าง Tuple Variant พร้อมข้อมูลได้
- [ ] ประกาศ Struct Variant ได้
- [ ] สร้าง Struct Variant พร้อม Fields ได้
- [ ] ใช้ `match` เข้าถึงข้อมูลใน Variant ได้
