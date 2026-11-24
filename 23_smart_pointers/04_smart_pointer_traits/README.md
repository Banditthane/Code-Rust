# Lesson 04: Smart Pointer Traits

## วัตถุประสงค์

เรียนรู้ Traits ที่เกี่ยวข้องกับ Smart Pointers ได้แก่ `Deref` และ `Drop` รวมถึงแนวคิด Deref Coercion และการเลือกใช้ Smart Pointer ให้เหมาะสม

## 1. Deref Trait

`Deref` เป็น Trait ที่กำหนดพฤติกรรมการ Dereference เพื่อให้เข้าถึงข้อมูลที่ Smart Pointer ถืออยู่ได้

ตัวดำเนินการ Dereference คือ:

```rust
*
```

ตัวอย่าง:

```rust
let value = Box::new(100);

println!("{}", *value);
```

ผลลัพธ์:

```text
100
```

`Box<T>` รองรับ `Deref` อยู่แล้ว จึงสามารถใช้ `*` เพื่อเข้าถึงข้อมูลภายในได้

## 2. Deref Coercion

Deref Coercion คือความสามารถของ Rust ในการแปลง Reference ของชนิดหนึ่งไปเป็น Reference ของชนิดที่ Deref ไปถึงโดยอัตโนมัติ เมื่อบริบทของโปรแกรมต้องการ

ตัวอย่าง:

```rust
fn display_text(text: &str) {
    println!("{}", text);
}

fn main() {
    let value = Box::new(String::from("Rust"));

    display_text(&value);
}
```

ผลลัพธ์:

```text
Rust
```

ในตัวอย่างนี้ Rust สามารถแปลง Reference ผ่าน `Box<String>` และ `String` ไปยัง `&str` ได้ตาม Deref Coercion

ประโยชน์:

- ลดการแปลง Reference ด้วยตนเอง
- ทำให้ Smart Pointer ใช้งานสะดวก
- ช่วยให้ชนิดข้อมูลทำงานร่วมกับ API ที่ต้องการ Reference ได้

## 3. การ Implement Deref

สามารถ Implement `Deref` ให้กับชนิดข้อมูลที่สร้างเองได้

แนวคิดหลัก:

- กำหนด `Target`
- Implement เมธอด `deref()`
- คืนค่า Reference ไปยังข้อมูลเป้าหมาย

รูปแบบ Trait:

```rust
use std::ops::Deref;

trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}
```

ตัวอย่างด้านบนเป็นรูปแบบ Trait เพื่อศึกษา ไม่ใช่โค้ดที่ต้องนำไปประกาศ Trait ซ้ำในโปรเจกต์จริง

ข้อควรระวัง:

- การ Implement `Deref` ส่งผลต่อวิธีที่ผู้ใช้เข้าถึงชนิดข้อมูล
- ควรใช้เมื่อพฤติกรรมของชนิดข้อมูลเหมาะกับการ Dereference จริง ๆ
- อย่าใช้ Deref แทนเมธอดทั่วไปโดยไม่มีเหตุผล

## 4. Drop Trait

`Drop` เป็น Trait ที่ใช้กำหนดการทำงานก่อนค่าหนึ่งถูกทำลาย

Rust จะเรียก `drop()` โดยอัตโนมัติเมื่อค่าหมดอายุการใช้งานตาม Scope หรือถูกทำลายตาม Ownership

ตัวอย่าง:

```rust
struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource dropped");
    }
}

fn main() {
    let _resource = Resource;

    println!("Program running");
}
```

ผลลัพธ์:

```text
Program running
Resource dropped
```

## 5. Drop และ Scope

ตัวแปรจะถูก Drop เมื่อหมด Scope ตามกฎ Ownership

ตัวอย่างแนวคิด:

- สร้าง Resource ในฟังก์ชัน
- ใช้งาน Resource ภายในฟังก์ชัน
- เมื่อออกจาก Scope Resource จะถูก Drop

สามารถเรียก `std::mem::drop(value)` เพื่อย้ายค่าเข้าไปและทำลายก่อนเวลาที่ Scope สิ้นสุดได้

ข้อควรจำ:

- `drop()` ของ Trait เป็นเมธอดที่ Compiler เรียกตาม Lifecycle
- `std::mem::drop()` เป็นฟังก์ชันที่ใช้ทำให้ค่าเข้าสู่การทำลายก่อนเวลา
- ไม่สามารถเรียกเมธอด `Drop::drop()` โดยตรงเพื่อทำลายค่าเองได้

## 6. การเลือกใช้ Smart Pointer

| สถานการณ์                                                     | ตัวเลือก               |
| ------------------------------------------------------------- | ---------------------- |
| ต้องการจัดเก็บข้อมูลบน Heap และมีเจ้าของเดียว                 | `Box<T>`               |
| ต้องการแชร์ Ownership ใน Thread เดียว                         | `Rc<T>`                |
| ต้องการแชร์ Ownership ระหว่าง Threads                         | `Arc<T>`               |
| ต้องการเปลี่ยนค่าแบบ Interior Mutability สำหรับชนิดที่เหมาะสม | `Cell<T>`              |
| ต้องการ Runtime Borrow Checking                               | `RefCell<T>`           |
| ต้องการแชร์ข้อมูลที่แก้ไขร่วมกันระหว่าง Threads               | มักใช้ `Arc<Mutex<T>>` |

## 7. แนวทางการตัดสินใจ

ก่อนเลือก Smart Pointer ให้ถามตัวเอง:

1. ต้องการเก็บข้อมูลบน Heap หรือไม่?
2. ต้องการให้มีเจ้าของข้อมูลกี่ราย?
3. ต้องแชร์ข้อมูลข้าม Thread หรือไม่?
4. ต้องเปลี่ยนแปลงข้อมูลผ่าน Shared Ownership หรือไม่?
5. ยอมรับการตรวจสอบ Borrowing ขณะ Runtime ได้หรือไม่?

หาก References หรือ Ownership ปกติเพียงพอ ให้เลือกวิธีที่ง่ายกว่าก่อน

## 8. สรุป

- `Deref` กำหนดพฤติกรรมการ Dereference
- Deref Coercion ช่วยแปลง Reference ในบริบทที่เหมาะสม
- `Drop` กำหนดพฤติกรรมก่อนข้อมูลถูกทำลาย
- Smart Pointer แต่ละชนิดแก้ปัญหาคนละแบบ
- ควรเลือกใช้เท่าที่จำเป็น

## Completion Checklist

- [ ] อธิบาย `Deref` ได้
- [ ] ใช้ Dereference Operator ได้
- [ ] อธิบาย Deref Coercion ได้
- [ ] เข้าใจแนวคิดการ Implement `Deref`
- [ ] อธิบาย `Drop` ได้
- [ ] เข้าใจว่า Drop ทำงานตาม Scope และ Ownership
- [ ] เลือก Smart Pointer ตามสถานการณ์ได้
