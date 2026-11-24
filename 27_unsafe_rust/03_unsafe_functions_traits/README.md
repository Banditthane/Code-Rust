# Lesson 03: Unsafe Functions and Traits

## 1. Unsafe Function คืออะไร?

Unsafe Function คือ Function ที่ผู้เรียกต้องรับผิดชอบต่อเงื่อนไขความปลอดภัยที่กำหนดไว้

ประกาศด้วย:

```rust
unsafe fn function_name() {
    // Function body
}
```

ตัวอย่าง:

```rust
unsafe fn get_value(pointer: *const i32) -> i32 {
    unsafe {
        *pointer
    }
}

fn main() {
    let value = 50;
    let pointer = &raw const value;

    let result = unsafe {
        get_value(pointer)
    };

    println!("{}", result);
}
```

## 2. Unsafe Function กับ Unsafe Block

สองสิ่งนี้มีหน้าที่ต่างกัน:

| รายการ      | หน้าที่                                                  |
| ----------- | -------------------------------------------------------- |
| `unsafe fn` | ประกาศ Function ที่ผู้เรียกต้องรับผิดชอบ Safety Contract |
| `unsafe {}` | กำหนดขอบเขตสำหรับ Unsafe Operations                      |

ใน Rust Edition 2024 การเรียก Unsafe Operation ภายใน Unsafe Function ยังต้องอยู่ใน Unsafe Block อย่างชัดเจน

จึงควรเขียน:

```rust
unsafe fn read_value(pointer: *const i32) -> i32 {
    unsafe {
        *pointer
    }
}
```

## 3. Safety Contract

Safety Contract คือข้อกำหนดที่ผู้เรียกต้องปฏิบัติตามเมื่อเรียก Unsafe Function

ตัวอย่าง:

```rust
/// Returns the value pointed to by `pointer`.
///
/// # Safety
///
/// `pointer` must be non-null, properly aligned,
/// valid, and point to an initialized `i32`
/// that remains alive for the duration of the call.
unsafe fn read_value(pointer: *const i32) -> i32 {
    unsafe {
        *pointer
    }
}
```

ส่วน `# Safety` ใช้ Documentation อธิบายเงื่อนไขที่ผู้เรียกต้องรับประกัน

## 4. Unsafe Trait คืออะไร?

Unsafe Trait คือ Trait ที่การ Implement ต้องรักษา Safety Contract บางอย่าง

ประกาศด้วย:

```rust
unsafe trait MyTrait {
    fn perform(&self);
}
```

การ Implement ต้องใช้:

```rust
unsafe impl MyTrait for MyType {
    fn perform(&self) {
        // Implementation
    }
}
```

## 5. Unsafe Trait กับ Normal Trait

| Normal Trait             | Unsafe Trait                                      |
| ------------------------ | ------------------------------------------------- |
| ประกาศด้วย `trait`       | ประกาศด้วย `unsafe trait`                         |
| Implement ด้วย `impl`    | Implement ด้วย `unsafe impl`                      |
| ตรวจสอบตามกฎปกติของ Rust | ผู้ Implement ต้องรักษา Safety Contract เพิ่มเติม |

ตัวอย่าง Unsafe Trait ใน Standard Library คือ `Send` และ `Sync`

## 6. Send และ Sync

### Send

`Send` หมายถึง Type สามารถย้าย Ownership ไปยัง Thread อื่นได้อย่างปลอดภัย

### Sync

`Sync` หมายถึง Reference `&T` สามารถส่งไปใช้ร่วมกันระหว่าง Thread ได้อย่างปลอดภัย

ตามกฎของ Rust:

```text
T: Sync
```

หมายความว่า:

```text
&T: Send
```

ตัวอย่าง:

```rust
use std::thread;

fn main() {
    let value = 100;

    let handle = thread::spawn(move || {
        println!("{}", value);
    });

    handle.join().unwrap();
}
```

`i32` เป็น Type ที่รองรับ `Send` และ `Sync`

## 7. Implement Unsafe Trait

ตัวอย่างโครงสร้าง:

```rust
unsafe trait SafeTransfer {
    fn value(&self) -> i32;
}

struct Data {
    value: i32,
}

unsafe impl SafeTransfer for Data {
    fn value(&self) -> i32 {
        self.value
    }
}
```

ตัวอย่างนี้ใช้เพื่อแสดง Syntax เท่านั้น

การสร้าง Unsafe Trait จริงต้องมี Safety Contract ที่ชัดเจน และต้องตรวจสอบว่า Implementation ทุกตัวสามารถรักษา Contract นั้นได้

## 8. Unsafe Function กับ FFI

Unsafe Function สามารถใช้ร่วมกับ Foreign Function Interface (FFI) เพื่อเรียก Function จากภาษาอื่นได้

ตัวอย่าง:

```rust
unsafe extern "C" {
    fn abs(value: i32) -> i32;
}

fn main() {
    let result = unsafe {
        abs(-100)
    };

    println!("{}", result);
}
```

ตัวอย่างนี้เรียก Function `abs` ผ่าน C ABI

หมายเหตุ: รูปแบบ `unsafe extern "C"` ใช้กับ Rust Edition 2024 ส่วน Edition เก่าอาจใช้ Syntax ต่างกัน

## 9. ข้อควรระวัง

- Unsafe Function ต้องระบุ Safety Contract
- ผู้เรียกต้องปฏิบัติตาม Contract
- Unsafe Trait ต้องกำหนดเงื่อนไขที่ Implementer ต้องรักษา
- ห้ามใช้ `unsafe impl` เพียงเพื่อหลีกเลี่ยง Compiler Error
- การ Implement `Send` หรือ `Sync` ผิดอาจทำให้เกิด Data Race หรือ Undefined Behavior
- FFI ต้องตรวจสอบ ABI และ Memory Layout ให้ตรงกัน

## 10. สรุป

| คำศัพท์        | ความหมาย                                       |
| -------------- | ---------------------------------------------- |
| `unsafe fn`    | Function ที่มี Safety Contract                 |
| `unsafe {}`    | Unsafe Context                                 |
| `unsafe trait` | Trait ที่มี Safety Contract สำหรับ Implementer |
| `unsafe impl`  | Implementation ของ Unsafe Trait                |
| `Send`         | ย้าย Ownership ข้าม Thread ได้อย่างปลอดภัย     |
| `Sync`         | แชร์ Reference ข้าม Thread ได้อย่างปลอดภัย     |
| FFI            | เชื่อมต่อกับภาษาอื่น                           |

## Completion Checklist

- [ ] ประกาศ Unsafe Function ได้
- [ ] เรียก Unsafe Function ได้
- [ ] ใช้ Unsafe Block ได้
- [ ] เข้าใจ Safety Contract
- [ ] เขียน Documentation `# Safety` ได้
- [ ] เข้าใจ Unsafe Trait
- [ ] เข้าใจ Unsafe Implementation
- [ ] เข้าใจ Send
- [ ] เข้าใจ Sync
- [ ] เข้าใจ FFI เบื้องต้น
