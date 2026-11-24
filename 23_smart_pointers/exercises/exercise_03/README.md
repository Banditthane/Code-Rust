# Exercise 03: RefCell Practice

**ระดับ:** Medium

**โปรเจกต์:** `refcell_practice`

## วัตถุประสงค์

ฝึกใช้ `Cell<T>` และ `RefCell<T>` เพื่อจัดการข้อมูลแบบ Interior Mutability และเรียนรู้การตรวจสอบ Borrowing ขณะ Runtime

## โจทย์

สร้างโปรแกรมจัดการคะแนนนักเรียน โดยใช้ `RefCell<T>` เก็บคะแนนที่สามารถเปลี่ยนแปลงได้

โปรแกรมต้องสามารถ:

1. สร้างข้อมูลคะแนน
2. อ่านคะแนน
3. แก้ไขคะแนน
4. คำนวณคะแนนรวม
5. คำนวณค่าเฉลี่ย
6. ทดลองตรวจสอบการ Borrow ข้อมูล

## Part 1: Cell<T>

สร้างตัวแปรสำหรับเก็บจำนวนครั้งที่มีการปรับปรุงคะแนน

ข้อกำหนด:

- ใช้ `Cell<T>`
- อ่านค่าด้วย `.get()`
- เปลี่ยนค่าด้วย `.set()`
- แสดงจำนวนครั้งที่มีการปรับปรุง

## Part 2: RefCell<T>

สร้างรายการคะแนนนักเรียนอย่างน้อย 4 คน

ข้อกำหนด:

- ใช้ `RefCell<T>` เก็บข้อมูลคะแนน
- อ่านข้อมูลผ่าน Immutable Borrow
- ปรับปรุงคะแนนผ่าน Mutable Borrow
- แสดงคะแนนก่อนและหลังการปรับปรุง
- คำนวณผลรวมและค่าเฉลี่ย

## Part 3: Borrow Checking

ทดลองสถานการณ์ Borrowing อย่างน้อย 2 กรณี:

1. การยืมข้อมูลแบบ Immutable
2. การยืมข้อมูลแบบ Mutable

ศึกษาว่ากรณีใดสามารถทำงานพร้อมกันได้ และกรณีใดไม่สามารถทำได้

หากทดลองกรณีที่ไม่สามารถ Borrow ได้ ให้ใช้ `try_borrow()` หรือ `try_borrow_mut()` เพื่อจัดการผลลัพธ์ โดยไม่ทำให้โปรแกรม Panic โดยไม่ตั้งใจ

## Constraints

- ต้องใช้ `Cell<T>` อย่างน้อย 1 จุด
- ต้องใช้ `RefCell<T>` อย่างน้อย 1 จุด
- ต้องมีคะแนนนักเรียนอย่างน้อย 4 คน
- คะแนนต้องอยู่ในช่วง 0–100
- ต้องจัดการกรณี Borrow ไม่สำเร็จ
- ห้ามใช้ `unsafe`
- ห้ามใช้ `unwrap()` กับการ Borrow ที่อาจล้มเหลวโดยไม่มีการตรวจสอบ

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์:

```text
=== Student Scores ===

Alice: 80
Bob: 75
Charlie: 90
David: 85

=== Update Scores ===

Alice: 85
Bob: 75
Charlie: 90
David: 85

=== Summary ===

Total Score: 335
Average Score: 83.75

Update Count: 1

=== Borrow Checking ===

Immutable Borrow: Success
Mutable Borrow: Success
```

หมายเหตุ: ค่าคะแนนเป็นตัวอย่าง และผลลัพธ์ของ Borrow Checking ต้องสอดคล้องกับสถานการณ์ที่โปรแกรมทดสอบจริง

## Hints

- ใช้ `std::cell::Cell`
- ใช้ `std::cell::RefCell`
- ศึกษา `.borrow()` และ `.borrow_mut()`
- ศึกษา `.try_borrow()` และ `.try_borrow_mut()`
- ระวังอายุการใช้งานของ `Ref<T>` และ `RefMut<T>`
- ทบทวนการคำนวณค่าเฉลี่ยจากจำนวนข้อมูล

## Completion Checklist

- [ ] ใช้ `Cell<T>` ได้
- [ ] ใช้ `.get()` และ `.set()` ได้
- [ ] ใช้ `RefCell<T>` ได้
- [ ] อ่านคะแนนได้
- [ ] ปรับปรุงคะแนนได้
- [ ] คำนวณผลรวมได้
- [ ] คำนวณค่าเฉลี่ยได้
- [ ] ทดลอง Immutable Borrow ได้
- [ ] ทดลอง Mutable Borrow ได้
- [ ] จัดการ Borrow Conflict ได้โดยไม่ Panic โดยไม่ตั้งใจ
- [ ] โปรแกรมทำงานได้ด้วย `cargo run`
