# Lesson 04: Unsafe Boundaries

## 1. Unsafe Boundary คืออะไร?

Unsafe Boundary คือการกำหนดขอบเขตระหว่าง Unsafe Code กับ Safe Code

แนวคิดสำคัญคือ:

- จำกัด Unsafe Code ให้อยู่ในพื้นที่เล็กที่สุด
- ตรวจสอบ Input ก่อนเข้า Unsafe Code
- รักษา Safety Invariants
- เปิด API ที่ปลอดภัยให้ผู้เรียกใช้งาน
- ไม่ให้ผู้ใช้งานต้องเขียน Unsafe เองโดยไม่จำเป็น

## 2. Undefined Behavior (UB)

Undefined Behavior คือพฤติกรรมที่ภาษา Rust ไม่ได้กำหนดผลลัพธ์ไว้

ตัวอย่างสาเหตุที่อาจทำให้เกิด UB:

- Dereference Null Pointer
- Dereference Dangling Pointer
- อ่าน Memory ที่ยังไม่ได้ Initialize
- เขียน Memory โดยละเมิด Aliasing Rules
- เกิด Data Race
- ใช้ Memory ที่มี Alignment ไม่ถูกต้อง
- สร้าง Invalid Value สำหรับชนิดข้อมูล

UB อาจทำให้โปรแกรมแสดงผลผิดพลาด หยุดทำงาน หรือเกิดพฤติกรรมที่คาดเดาไม่ได้

## 3. Mutable Static Variables

Static Variable มีอายุการใช้งานตลอดโปรแกรม

ตัวอย่าง Immutable Static:

```rust
static APP_NAME: &str = "Rust App";

fn main() {
    println!("{}", APP_NAME);
}
```

Mutable Static:

```rust
static mut COUNTER: u32 = 0;
```

การอ่านและเขียน Mutable Static ต้องใช้ Unsafe Context

อย่างไรก็ตาม การใช้ `static mut` มีความเสี่ยงสูง โดยเฉพาะเมื่อมีหลาย Thread เข้าถึงพร้อมกัน

แนวทางที่ควรพิจารณา:

- ใช้ Atomic Types สำหรับข้อมูลตัวเลขที่เหมาะสม
- ใช้ Mutex หรือ RwLock สำหรับข้อมูลที่ต้องแชร์และแก้ไข
- ใช้ Local State หากไม่จำเป็นต้องแชร์ข้าม Thread

## 4. Atomic Types

Atomic Types ใช้จัดการข้อมูลที่ต้องเข้าถึงจากหลาย Thread โดยมี Operation ที่เป็น Atomic

ตัวอย่าง:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    COUNTER.fetch_add(1, Ordering::SeqCst);

    println!("{}", COUNTER.load(Ordering::SeqCst));
}
```

ผลลัพธ์:

```text
1
```

ตัวอย่างนี้ใช้ Atomic Variable โดยไม่ต้องใช้ `unsafe`

## 5. Foreign Function Interface (FFI)

FFI เป็นกลไกที่ Rust ใช้เชื่อมต่อกับภาษาอื่น เช่น C

ตัวอย่างการประกาศ Function ภายนอก:

```rust
unsafe extern "C" {
    fn abs(value: i32) -> i32;
}

fn main() {
    let result = unsafe {
        abs(-50)
    };

    println!("{}", result);
}
```

ผลลัพธ์:

```text
50
```

`"C"` คือ ABI ที่ใช้ในการเรียก Function

## 6. FFI Safety

การใช้ FFI ต้องตรวจสอบ:

1. ABI ตรงกัน
2. ชนิดข้อมูลตรงกัน
3. Memory Layout ตรงกัน
4. Pointer มีอายุการใช้งานถูกต้อง
5. Ownership ถูกจัดการอย่างเหมาะสม
6. String และ Buffer ใช้รูปแบบที่ทั้งสองฝั่งเข้าใจตรงกัน
7. Error และ Panic ไม่ข้าม ABI Boundary อย่างไม่เหมาะสม

## 7. Safe Abstraction

Safe Abstraction คือการซ่อน Unsafe Implementation ไว้ภายใน API ที่ผู้ใช้งานสามารถเรียกผ่าน Safe Rust ได้

แนวคิด:

```text
Safe API
   |
   v
Input Validation
   |
   v
Unsafe Implementation
   |
   v
Safety Invariants
   |
   v
Safe Result
```

ตัวอย่างแนวคิด:

- ผู้เรียกส่งข้อมูลผ่าน Safe Function
- Function ตรวจสอบเงื่อนไข
- Unsafe Code ทำงานภายใน
- Function คืนผลลัพธ์ผ่าน Safe Type

การซ่อน Unsafe Code เพียงอย่างเดียวไม่ทำให้ API ปลอดภัย ต้องพิสูจน์ได้ว่า Safe API ไม่เปิดโอกาสให้ผู้เรียกละเมิด Safety Invariants

## 8. Limit Unsafe Scope

แนวทางที่ควรปฏิบัติ:

- ใช้ Unsafe Block เท่าที่จำเป็น
- หลีกเลี่ยง Unsafe Function หากไม่จำเป็น
- เขียน Safety Comment
- แยก Unsafe Code ออกจาก Business Logic
- ตรวจสอบ Pointer และ Memory ก่อนใช้งาน
- ใช้ Standard Library หากมี Safe API รองรับ
- ทดสอบกรณีขอบเขตและกรณีผิดพลาด

## 9. Tools สำหรับตรวจสอบ

### Miri

Miri เป็นเครื่องมือสำหรับตรวจสอบพฤติกรรมบางประเภทของ Rust Program โดยเฉพาะปัญหาที่เกี่ยวข้องกับ Undefined Behavior

ติดตั้ง:

```powershell
rustup component add miri
```

รัน Test:

```powershell
cargo miri test
```

Miri ช่วยค้นหาปัญหาได้ แต่ไม่สามารถพิสูจน์ว่า Unsafe Code ปลอดภัยทุกกรณี

### Clippy

```powershell
cargo clippy
```

ใช้ตรวจสอบปัญหาและแนะนำแนวทางปรับปรุงโค้ด

## 10. สรุป

| คำศัพท์          | ความหมาย                                       |
| ---------------- | ---------------------------------------------- |
| Unsafe Boundary  | ขอบเขตระหว่าง Safe และ Unsafe Code             |
| UB               | Undefined Behavior                             |
| `static mut`     | Mutable Static Variable                        |
| Atomic           | Operation ที่ทำงานแบบ Atomic                   |
| FFI              | เชื่อมต่อกับภาษาอื่น                           |
| ABI              | Application Binary Interface                   |
| Safe Abstraction | API ที่ซ่อน Unsafe Implementation อย่างปลอดภัย |
| Miri             | เครื่องมือช่วยตรวจสอบ Undefined Behavior       |

## Completion Checklist

- [ ] เข้าใจ Unsafe Boundary
- [ ] เข้าใจ Undefined Behavior
- [ ] เข้าใจ Mutable Static
- [ ] เข้าใจ Atomic Types
- [ ] เข้าใจ FFI
- [ ] เข้าใจ ABI
- [ ] เข้าใจ FFI Safety
- [ ] ออกแบบ Safe Abstraction ได้
- [ ] จำกัด Unsafe Scope ได้
- [ ] เข้าใจการใช้ Miri
- [ ] เข้าใจข้อจำกัดของเครื่องมือตรวจสอบ
