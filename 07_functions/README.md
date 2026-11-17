# บทที่ 07: Functions (ฟังก์ชัน)

## ภาพรวม

บทนี้เรียนรู้การสร้างและเรียกใช้งานฟังก์ชัน (Functions) ในภาษา Rust เพื่อแบ่งโปรแกรมออกเป็นส่วนย่อย ๆ ที่สามารถนำกลับมาใช้งานได้ ช่วยให้โค้ดเป็นระเบียบ อ่านง่าย และลดการเขียนคำสั่งซ้ำ

## วัตถุประสงค์

เมื่อเรียนจบบทนี้แล้ว จะสามารถ:

- เข้าใจความหมายและประโยชน์ของ Function
- ประกาศและเรียกใช้งาน Function ได้
- กำหนด Parameters ให้กับ Function ได้
- ส่ง Arguments เข้าไปใน Function ได้
- กำหนดชนิดข้อมูลของ Parameters ได้
- กำหนด Return Type ได้
- ส่งค่ากลับจาก Function ด้วย `return` หรือ Expression ได้
- สร้าง Function ที่นำกลับมาใช้งานซ้ำได้

## เนื้อหาบทเรียน

| ลำดับ | หัวข้อ                   | โฟลเดอร์                    |
| ----- | ------------------------ | --------------------------- |
| 01    | พื้นฐาน Function         | `01_function_basics`        |
| 02    | Parameters และ Arguments | `02_function_parameters`    |
| 03    | Return Values            | `03_function_return_values` |

## แบบฝึกหัด

| ลำดับ | แบบฝึกหัด                  | โฟลเดอร์      |
| ----- | -------------------------- | ------------- |
| 01    | สร้าง Function เบื้องต้น   | `exercise_01` |
| 02    | Function และ Parameters    | `exercise_02` |
| 03    | Function และ Return Values | `exercise_03` |
| 04    | แบบฝึกหัดประยุกต์ Function | `exercise_04` |

## แนวคิดสำคัญ

### 1. Function

Function คือชุดคำสั่งที่ถูกตั้งชื่อไว้ เพื่อให้สามารถเรียกใช้งานได้ตามต้องการ

### 2. Function Declaration

ใช้คำสั่ง `fn` เพื่อประกาศ Function

### 3. Function Call

การเรียกใช้งาน Function ทำได้โดยระบุชื่อ Function ตามด้วยวงเล็บ `()`

### 4. Parameters และ Arguments

- Parameters คือ ตัวแปรที่กำหนดไว้ในส่วนประกาศ Function
- Arguments คือ ค่าที่ส่งเข้าไปเมื่อเรียกใช้งาน Function

### 5. Return Value

Return Value คือค่าที่ Function ส่งกลับไปยังจุดที่เรียกใช้งาน

### 6. Return Type

Return Type คือชนิดข้อมูลของค่าที่ Function จะส่งกลับ โดยระบุด้วยเครื่องหมาย `->`

## ตัวอย่าง Function

```rust
fn greet() {
    println!("Hello, Rust!");
}

fn main() {
    greet();
}
```

ผลลัพธ์:

```text
Hello, Rust!
```

## วิธีรันโปรเจกต์

เข้าไปยังโฟลเดอร์ของแบบฝึกหัดที่ต้องการ แล้วใช้คำสั่ง:

```powershell
cargo check
cargo run
```

## เกณฑ์การเรียนจบบท

- [ ] เข้าใจการประกาศ Function ด้วย `fn`
- [ ] เรียกใช้งาน Function ได้
- [ ] เข้าใจ Parameters และ Arguments
- [ ] กำหนดชนิดข้อมูลของ Parameters ได้
- [ ] กำหนด Return Type ได้
- [ ] ส่งค่ากลับจาก Function ได้
- [ ] ทำแบบฝึกหัดทั้ง 4 ข้อ
- [ ] สามารถสร้าง Function เพื่อแก้ปัญหาด้วยตนเองได้

## แหล่งเรียนรู้

- [W3Schools - Rust Functions](https://www.w3schools.com/rust/rust_functions.php)
- [The Rust Programming Language - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
