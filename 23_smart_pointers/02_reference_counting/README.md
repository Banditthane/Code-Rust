# Lesson 02: Reference Counting

## วัตถุประสงค์

เรียนรู้การแชร์ Ownership ของข้อมูลผ่าน Reference Counting โดยใช้ `Rc<T>` และ `Arc<T>` พร้อมเข้าใจความแตกต่างในการใช้งาน

## 1. Shared Ownership คืออะไร

ตามกฎ Ownership ปกติ ข้อมูลหนึ่งชุดมีเจ้าของหลักเพียงหนึ่งราย

แต่บางสถานการณ์ต้องการให้หลายส่วนของโปรแกรมเข้าถึงข้อมูลชุดเดียวกัน โดยไม่ต้องสร้างสำเนาข้อมูลทั้งหมด

แนวคิดนี้เรียกว่า Shared Ownership

Rust มี Smart Pointer สำหรับจัดการเรื่องนี้ เช่น:

- `Rc<T>`
- `Arc<T>`

## 2. Rc<T>

`Rc<T>` ย่อมาจาก Reference Counted

ใช้สำหรับแชร์ Ownership ของข้อมูลภายใน Thread เดียว

เมื่อมีการสร้าง `Rc::clone()` จะเพิ่มจำนวนเจ้าของข้อมูล โดยไม่ได้คัดลอกข้อมูลภายในทั้งหมด

ตัวอย่าง:

```rust
use std::rc::Rc;

fn main() {
    let value = Rc::new(String::from("Rust"));

    let value_a = Rc::clone(&value);
    let value_b = Rc::clone(&value);

    println!("{}", value);
    println!("{}", value_a);
    println!("{}", value_b);
}
```

ผลลัพธ์:

```text
Rust
Rust
Rust
```

ทั้งสามตัวแปรเป็นเจ้าของร่วมกันของข้อมูลชุดเดียว

## 3. Reference Count

`Rc<T>` สามารถตรวจสอบจำนวน Strong References ได้ด้วย `Rc::strong_count()`

ตัวอย่าง:

```rust
use std::rc::Rc;

fn main() {
    let value = Rc::new(100);

    let value_a = Rc::clone(&value);

    println!("{}", Rc::strong_count(&value));
}
```

ผลลัพธ์:

```text
2
```

เหตุผล:

- `value` เป็นเจ้าของหนึ่งรายการ
- `value_a` เป็นเจ้าของอีกหนึ่งรายการ

เมื่อเจ้าของทั้งหมดถูกทำลาย จำนวน Reference จะลดลง และเมื่อไม่มี Strong Reference เหลือ ข้อมูลจะถูกทำลาย

## 4. Arc<T>

`Arc<T>` ย่อมาจาก Atomically Reference Counted

มีหน้าที่คล้าย `Rc<T>` แต่รองรับการแชร์ Ownership ระหว่าง Threads

ตัวอย่าง:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let value = Arc::new(100);

    let shared_value = Arc::clone(&value);

    let handle = thread::spawn(move || {
        println!("{}", shared_value);
    });

    handle.join().unwrap();
}
```

ผลลัพธ์:

```text
100
```

ตัวอย่างนี้แสดงการแชร์ Ownership ของค่าเดียวกันระหว่าง Thread หลักกับ Thread ที่สร้างขึ้น

## 5. ความแตกต่างระหว่าง Rc<T> และ Arc<T>

| หัวข้อ           | Rc<T>             | Arc<T>                            |
| ---------------- | ----------------- | --------------------------------- |
| ชื่อเต็ม         | Reference Counted | Atomically Reference Counted      |
| แชร์ Ownership   | ได้               | ได้                               |
| ใช้ข้าม Threads  | ไม่ได้            | ได้                               |
| การนับ Reference | ไม่เป็น Atomic    | เป็น Atomic                       |
| ค่าใช้จ่าย       | โดยทั่วไปต่ำกว่า  | มีค่าใช้จ่ายจาก Atomic Operations |

## 6. ข้อควรระวัง

### Rc<T>

- ใช้ภายใน Thread เดียว
- ไม่สามารถส่งข้าม Thread ได้อย่างปลอดภัย
- ไม่ได้ให้สิทธิ์แก้ไขข้อมูลผ่าน `Rc` โดยตรง

### Arc<T>

- รองรับการแชร์ Ownership ระหว่าง Threads
- ไม่ได้ทำให้ข้อมูลภายในเปลี่ยนแปลงได้อย่างปลอดภัยโดยอัตโนมัติ
- หากต้องการแก้ไขข้อมูลร่วมกัน มักต้องใช้ร่วมกับ `Mutex<T>` หรือเครื่องมือที่เหมาะสม

### Reference Cycle

การใช้ `Rc<T>` อ้างอิงกันเป็นวงจรอาจทำให้ Reference Count ไม่ลดลงจนถึงศูนย์

Rust มี `Weak<T>` สำหรับสร้าง Weak Reference ซึ่งไม่เพิ่ม Strong Reference Count และช่วยจัดการโครงสร้างที่มีความสัมพันธ์เป็นวงจรได้

## 7. สรุป

- Shared Ownership ช่วยให้หลายส่วนของโปรแกรมเป็นเจ้าของข้อมูลร่วมกัน
- `Rc<T>` ใช้สำหรับ Thread เดียว
- `Arc<T>` ใช้แชร์ Ownership ระหว่าง Threads
- `Rc::clone()` และ `Arc::clone()` เพิ่มจำนวน Reference โดยไม่ Clone ข้อมูลภายในทั้งหมด
- การแชร์ Ownership ไม่เท่ากับการอนุญาตให้แก้ไขข้อมูลร่วมกัน

## Completion Checklist

- [ ] อธิบาย Shared Ownership ได้
- [ ] สร้าง `Rc<T>` ได้
- [ ] ใช้ `Rc::clone()` ได้
- [ ] ตรวจสอบ Reference Count ได้
- [ ] สร้าง `Arc<T>` ได้
- [ ] อธิบายความแตกต่างระหว่าง Rc และ Arc ได้
- [ ] เข้าใจแนวคิดของ `Weak<T>`
