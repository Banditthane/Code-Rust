# Lesson 04: If Let and Option

## 1. What is if let?

`if let` เป็นวิธีตรวจสอบ Pattern ที่สนใจเพียงรูปแบบเดียว โดยไม่จำเป็นต้องเขียน `match` ให้ครอบคลุมทุกกรณีด้วยตัวเอง

ตัวอย่าง:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;

    if let TrafficLight::Red = light {
        println!("Stop");
    }
}
```

ผลลัพธ์:

```text
Stop
```

หาก Pattern ตรงกัน จะทำงานภายใน Block ของ `if let`

## 2. if let with else

สามารถใช้ `else` เพื่อกำหนดการทำงานเมื่อ Pattern ไม่ตรงกันได้

```rust
if let TrafficLight::Red = light {
    println!("Stop");
} else {
    println!("Not red");
}
```

`if let` เหมาะกับกรณีที่สนใจ Variant เพียงบางตัว ส่วน `match` เหมาะเมื่อจำเป็นต้องจัดการหลายกรณีอย่างชัดเจน

## 3. What is Option<T>?

`Option<T>` เป็น Enum มาตรฐานของ Rust ที่ใช้แทนค่าที่อาจมีหรือไม่มี

ประกอบด้วย 2 Variants:
