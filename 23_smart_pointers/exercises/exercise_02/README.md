# Exercise 02: Rc and Arc Practice

**ระดับ:** Easy-Medium

**โปรเจกต์:** `rc_arc_practice`

## วัตถุประสงค์

ฝึกใช้ `Rc<T>` และ `Arc<T>` เพื่อแชร์ Ownership และตรวจสอบ Reference Count

## โจทย์

สร้างโปรแกรมจำลองระบบแชร์ข้อมูลของแผนกภายในบริษัท โดยแบ่งเป็น 2 ส่วน:

1. การแชร์ข้อมูลภายใน Thread เดียวด้วย `Rc<T>`
2. การแชร์ข้อมูลระหว่าง Threads ด้วย `Arc<T>`

## Part 1: Rc<T>

สร้างข้อมูลแผนกบริษัทหนึ่งรายการ เช่น:

- Department Name
- Department Code
- Employee Count

ข้อกำหนด:

- สร้างข้อมูลด้วย `Rc<T>`
- สร้างตัวแปรที่แชร์ Ownership อย่างน้อย 3 ตัว
- แสดงข้อมูลจากแต่ละตัวแปร
- ตรวจสอบจำนวน Strong References
- อธิบายว่าเหตุใดจึงใช้ `Rc<T>` ในส่วนนี้

## Part 2: Arc<T>

สร้างข้อมูลประกาศของบริษัทหนึ่งรายการ

ข้อกำหนด:

- สร้างข้อมูลด้วย `Arc<T>`
- Clone Arc เพื่อแชร์ Ownership
- ส่งข้อมูลที่แชร์ไปยัง Threads อย่างน้อย 2 Threads
- ให้แต่ละ Thread อ่านและแสดงข้อมูล
- รอให้ Threads ทำงานเสร็จด้วยการ Join
- ตรวจสอบ Reference Count ในตำแหน่งที่เหมาะสม

## Constraints

- ต้องใช้ `Rc<T>` และ `Arc<T>` ทั้งสองชนิด
- ห้ามใช้ `Rc<T>` ส่งข้าม Threads
- ห้ามใช้ `unsafe`
- ส่วน Arc ในแบบฝึกหัดนี้ให้เน้นการอ่านข้อมูลร่วมกัน ไม่จำเป็นต้องแก้ไขข้อมูล
- ไม่จำเป็นต้องใช้ External Crates
- ห้ามใช้ Global Mutable Variables

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์:

```text
=== Rc Shared Ownership ===

Department: Engineering
Code: ENG
Employees: 25

Reference Count: 3

=== Arc Shared Ownership ===

Company Announcement:
Rust Training Starts Monday

Thread 1: Reading announcement
Thread 2: Reading announcement

All Threads Completed
```

หมายเหตุ:

- จำนวน Reference Count ขึ้นอยู่กับตำแหน่งที่ตรวจสอบ
- ลำดับข้อความจาก Threads อาจแตกต่างกันได้

## Hints

- ใช้ `std::rc::Rc`
- ใช้ `std::sync::Arc`
- ศึกษา `Rc::clone()` และ `Arc::clone()`
- ศึกษา `Rc::strong_count()` และ `Arc::strong_count()`
- ทบทวน `std::thread::spawn()` และ `join()`
- ใช้ `move` เมื่อส่ง Ownership เข้า Closure ของ Thread

## Completion Checklist

- [ ] สร้างข้อมูลแผนกได้
- [ ] ใช้ `Rc<T>` ได้
- [ ] สร้าง Shared Owners ได้อย่างน้อย 3 ตัว
- [ ] ตรวจสอบ Reference Count ได้
- [ ] ใช้ `Arc<T>` ได้
- [ ] แชร์ข้อมูลไปยัง Threads ได้
- [ ] Join Threads ได้
- [ ] อธิบายความแตกต่างระหว่าง Rc และ Arc ได้
- [ ] โปรแกรมทำงานได้ด้วย `cargo run`
