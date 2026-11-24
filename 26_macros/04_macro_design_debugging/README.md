# Lesson 04: Macro Design and Debugging

## 1. Macro Design คืออะไร?

Macro Design คือการออกแบบ Macro ให้ใช้งานง่าย อ่านเข้าใจ และสร้างโค้ดที่ถูกต้อง

Macro ที่ดีควร:

- มีชื่อสื่อความหมาย
- มีรูปแบบ Input ที่ชัดเจน
- รองรับกรณีใช้งานที่จำเป็น
- สร้างโค้ดที่คาดเดาได้
- แสดง Error ที่เข้าใจง่าย
- ไม่สร้างความซับซ้อนเกินความจำเป็น

## 2. เลือกใช้ Function หรือ Macro

ก่อนสร้าง Macro ควรพิจารณาว่า Function สามารถแก้ปัญหาได้หรือไม่

| สถานการณ์                                | เครื่องมือที่เหมาะสม           |
| ---------------------------------------- | ------------------------------ |
| คำนวณค่าทั่วไป                           | Function                       |
| ประมวลผลข้อมูลที่มี Type ชัดเจน          | Function หรือ Generic Function |
| ต้องรับจำนวน Arguments ที่ยืดหยุ่น       | Macro                          |
| ต้องสร้างโค้ดซ้ำตาม Pattern              | Declarative Macro              |
| ต้องตรวจสอบและแปลงโครงสร้าง Rust Code    | Procedural Macro               |
| ต้องสร้าง Trait Implementation อัตโนมัติ | Derive Macro                   |

## 3. Macro Hygiene

Macro Hygiene ช่วยลดปัญหาการชนกันของชื่อ Identifier ที่เกิดจาก Macro

ตัวอย่าง:

```rust
macro_rules! calculate {
    ($value:expr) => {{
        let result = $value;
        result * 2
    }};
}

fn main() {
    let result = 100;

    let value = calculate!(10);

    println!("Original: {}", result);
    println!("Macro: {}", value);
}
```

Macro ใช้ Scope ของตัวแปรภายใน Block ที่สร้างขึ้น โดยไม่ไปเปลี่ยนค่าตัวแปร `result` ของผู้เรียก

ควรใช้ Block เมื่อ Macro ต้องประกาศตัวแปรภายใน

## 4. Debug Macro Expansion

Macro Expansion คือโค้ดที่ Compiler สร้างหลังขยาย Macro

การตรวจสอบ Expansion ช่วยให้เห็นว่า Macro สร้างโค้ดอะไรออกมาจริง

เครื่องมือที่นิยมใช้คือ `cargo-expand`

ติดตั้ง:

```powershell
cargo install cargo-expand
```

ตรวจสอบ Macro Expansion:

```powershell
cargo expand
```

หรือระบุ Crate:

```powershell
cargo expand crate_name
```

หมายเหตุ: `cargo-expand` เป็นเครื่องมือภายนอก ไม่ใช่คำสั่งมาตรฐานที่ติดตั้งมากับ Cargo ทุกชุด

## 5. Debug Declarative Macro

แนวทางตรวจสอบ:

1. ตรวจสอบ Pattern ที่รับ Input
2. ตรวจสอบ Fragment Specifier
3. ตรวจสอบ Repetition
4. ตรวจสอบ Separator
5. ตรวจสอบ Expression ที่ Macro สร้าง
6. ทดลอง Macro ด้วย Input หลายรูปแบบ
7. ตรวจสอบ Expansion เมื่อยังหาสาเหตุไม่พบ

## 6. Debug Procedural Macro

แนวทางตรวจสอบ:

1. ตรวจสอบ TokenStream Input
2. ตรวจสอบการ Parse ด้วย `syn`
3. ตรวจสอบโครงสร้าง Struct หรือ Enum
4. ตรวจสอบโค้ดที่สร้างด้วย `quote`
5. ตรวจสอบ TokenStream Output
6. ทดสอบกับ Struct และ Enum หลายรูปแบบ
7. ตรวจสอบ Error Message

ควรทดสอบกรณีที่ Input ไม่ตรงตามเงื่อนไขด้วย ไม่ใช่ทดสอบเฉพาะกรณีสำเร็จ

## 7. Testing Macros

Macro ควรมี Test อย่างน้อยในกรณี:

- Input ที่ถูกต้อง
- Input หลายรูปแบบ
- Input ที่มีจำนวน Arguments ต่างกัน
- Input ที่ไม่ถูกต้อง
- Generated Code ที่ต้อง Compile
- พฤติกรรมของโค้ดหลัง Macro Expansion

สำหรับ Procedural Macro สามารถใช้ Integration Tests ใน Crate ที่นำ Macro ไปใช้งานได้

## 8. Error Handling ใน Macro

Macro ควรแจ้ง Error ให้ชัดเจนเมื่อ Input ไม่ถูกต้อง

ตัวอย่างแนวทาง:

- แจ้งเมื่อ Struct ไม่มี Field ที่ต้องการ
- แจ้งเมื่อ Type ไม่ตรงตามเงื่อนไข
- แจ้งเมื่อ Attribute มีรูปแบบไม่ถูกต้อง
- แจ้งเมื่อ Macro ได้รับ Input ที่ไม่รองรับ

สำหรับ Procedural Macro สามารถสร้าง Compile Error โดยใช้ `syn::Error` และแปลงเป็น TokenStream ได้

## 9. Best Practices

- ใช้ Function หากไม่จำเป็นต้องใช้ Macro
- เขียน Pattern ให้ชัดเจน
- หลีกเลี่ยง Macro ที่ทำงานซับซ้อนเกินไป
- ตั้งชื่อ Macro ให้สื่อความหมาย
- ระวัง Expression ที่อาจถูกประเมินหลายครั้ง
- ใช้ Block เพื่อจัดการ Scope
- ทดสอบ Generated Code
- ตรวจสอบ Expansion เมื่อเกิดปัญหา
- เขียน Documentation ให้ผู้ใช้งานเข้าใจ

## 10. ข้อควรระวัง

- Macro ที่ซับซ้อนอาจทำให้ Debug ยาก
- Error จาก Generated Code อาจไม่ชี้ไปยังต้นเหตุโดยตรง
- การสร้าง Identifier ต้องคำนึงถึง Hygiene และ Name Collision
- Procedural Macro อาจเพิ่ม Compile Time
- Macro ไม่ควรถูกใช้เพื่อแทนที่ Function ทุกกรณี

## Completion Checklist

- [ ] เข้าใจการออกแบบ Macro
- [ ] เลือก Function หรือ Macro ได้เหมาะสม
- [ ] เข้าใจ Macro Hygiene
- [ ] ใช้ Block เพื่อจัดการ Scope ได้
- [ ] เข้าใจ Macro Expansion
- [ ] ติดตั้ง cargo-expand ได้
- [ ] ตรวจสอบ Macro Expansion ได้
- [ ] ทดสอบ Macro ได้
- [ ] ออกแบบ Error Message ได้
- [ ] เข้าใจ Best Practices ของ Macro
