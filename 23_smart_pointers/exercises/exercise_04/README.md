# Exercise 04: Smart Pointer Challenge

**ระดับ:** Challenge

**โปรเจกต์:** `smart_pointer_challenge`

## วัตถุประสงค์

ประยุกต์ใช้ Smart Pointers หลายชนิดร่วมกัน เพื่อสร้างระบบจัดการข้อมูลที่มี Shared Ownership และ Interior Mutability

## โจทย์

สร้างโปรแกรมจำลองระบบจัดการสมาชิกของชมรม โดยสมาชิกแต่ละคนมีข้อมูลดังนี้:

- Member ID
- Name
- Points
- Active Status

ระบบต้องรองรับการแชร์ข้อมูลสมาชิกระหว่างส่วนต่าง ๆ ของโปรแกรม และสามารถปรับปรุงคะแนนของสมาชิกได้

## Requirements

### Part 1: Member Structure

ออกแบบ Struct สำหรับสมาชิก

ข้อมูลที่ต้องมี:

- รหัสสมาชิก
- ชื่อสมาชิก
- คะแนน
- สถานะการใช้งาน

กำหนดชนิดข้อมูลให้เหมาะสมกับแต่ละ Field

### Part 2: Shared Ownership

ใช้ `Rc<T>` เพื่อแชร์ข้อมูลสมาชิกภายใน Thread เดียว

ข้อกำหนด:

- สร้างสมาชิกอย่างน้อย 3 คน
- ให้ส่วนต่าง ๆ ของโปรแกรมเข้าถึงสมาชิกชุดเดียวกัน
- ตรวจสอบ Reference Count
- แสดงข้อมูลสมาชิก

### Part 3: Interior Mutability

ใช้ `RefCell<T>` เพื่อให้สามารถปรับปรุงข้อมูลสมาชิกผ่าน Shared Ownership

ข้อกำหนด:

- รองรับการเพิ่มคะแนน
- รองรับการลดคะแนน
- รองรับการเปลี่ยนสถานะสมาชิก
- แสดงข้อมูลก่อนและหลังการเปลี่ยนแปลง
- ตรวจสอบกรณี Borrow ไม่สำเร็จ

### Part 4: Combined Smart Pointers

ออกแบบโครงสร้างข้อมูลที่ใช้ `Rc<RefCell<T>>` ตามความเหมาะสม

ข้อกำหนด:

- สมาชิกแต่ละคนต้องสามารถถูกอ้างถึงจากหลายส่วนของโปรแกรม
- ทุกส่วนต้องทำงานกับข้อมูลสมาชิกชุดเดียวกัน
- การเปลี่ยนคะแนนจากส่วนหนึ่งต้องสะท้อนในส่วนอื่น
- ต้องจัดการ Borrowing อย่างระมัดระวัง

### Part 5: Drop Trait

สร้าง Resource หรือ Struct เพิ่มเติมหนึ่งชนิด

ข้อกำหนด:

- Implement `Drop` สำหรับ Struct นั้น
- แสดงข้อความเมื่อ Resource ถูก Drop
- ทดลองให้ Resource หมดอายุการใช้งานตาม Scope
- อธิบายว่าเหตุใด Drop จึงเกิดขึ้นในตำแหน่งนั้น

### Part 6: Deref

ศึกษาและทดลองการใช้ Dereference กับ Smart Pointer ที่รองรับ `Deref`

อธิบายความแตกต่างระหว่าง:

- การเข้าถึงค่าผ่าน Smart Pointer
- การใช้ Dereference Operator
- Deref Coercion

## Constraints

- ต้องใช้ `Rc<T>`
- ต้องใช้ `RefCell<T>`
- ต้องใช้ `Rc<RefCell<T>>` อย่างน้อยหนึ่งโครงสร้าง
- ต้อง Implement `Drop` อย่างน้อยหนึ่งครั้ง
- ต้องมีสมาชิกอย่างน้อย 3 คน
- ต้องจัดการ Borrow Conflict อย่างเหมาะสม
- ห้ามใช้ `unsafe`
- ห้ามใช้ Global Mutable Variables
- ห้ามใช้ `unwrap()` กับการ Borrow ที่อาจล้มเหลวโดยไม่มีการตรวจสอบ
- ไม่ต้องใช้ `Arc<T>` เพราะโจทย์นี้เน้น Shared Ownership ภายใน Thread เดียว

## Expected Output

ตัวอย่างรูปแบบผลลัพธ์:

```text
=== Club Members ===

ID: 001
Name: Alice
Points: 100
Status: Active

ID: 002
Name: Bob
Points: 80
Status: Active

ID: 003
Name: Charlie
Points: 60
Status: Active

=== Update Member Points ===

Alice: 100 -> 120
Bob: 80 -> 90

=== Shared Ownership Check ===

Reference Count: 2

=== Updated Members ===

Alice: 120
Bob: 90
Charlie: 60

=== Resource Lifecycle ===

Resource Created
Program Running
Resource Dropped
```

หมายเหตุ:

- จำนวน Reference Count ขึ้นอยู่กับตำแหน่งที่ตรวจสอบ
- ตัวเลขและข้อความสามารถแตกต่างได้ตามการออกแบบ
- ข้อความ Drop ต้องเกิดตาม Lifecycle จริงของ Resource

## Hints

- แยก Struct และฟังก์ชันตามหน้าที่
- พิจารณาใช้ `Rc<RefCell<Member>>`
- ใช้ `.borrow()` เมื่อต้องการอ่านข้อมูล
- ใช้ `.borrow_mut()` เมื่อต้องการแก้ไขข้อมูล
- ใช้ `.try_borrow()` และ `.try_borrow_mut()` เมื่อต้องการจัดการข้อผิดพลาดจาก Borrowing
- ศึกษา `Rc::strong_count()`
- ศึกษา `Drop` Trait
- ทบทวน Deref และ Deref Coercion
- ระวังไม่ให้ `Ref` หรือ `RefMut` มีอายุการใช้งานยาวเกินความจำเป็น

## Completion Checklist

- [ ] ออกแบบ Member Struct ได้
- [ ] สร้างสมาชิกอย่างน้อย 3 คน
- [ ] ใช้ `Rc<T>` แชร์ Ownership ได้
- [ ] ตรวจสอบ Reference Count ได้
- [ ] ใช้ `RefCell<T>` ได้
- [ ] ใช้ `Rc<RefCell<T>>` ได้
- [ ] เพิ่มคะแนนสมาชิกได้
- [ ] ลดคะแนนสมาชิกได้
- [ ] เปลี่ยนสถานะสมาชิกได้
- [ ] จัดการ Borrow Conflict ได้
- [ ] Implement `Drop` ได้
- [ ] อธิบาย Lifecycle ของ Resource ได้
- [ ] ทดลอง Dereference ได้
- [ ] อธิบาย Deref Coercion ได้
- [ ] โปรแกรมทำงานได้ด้วย `cargo run`
- [ ] ผ่านการตรวจสอบด้วย `cargo check`
- [ ] ผ่านการตรวจสอบด้วย `cargo clippy`

## เกณฑ์ประเมิน

| หัวข้อ                          |   คะแนน |
| ------------------------------- | ------: |
| Member Structure                |      10 |
| Rc Shared Ownership             |      20 |
| RefCell Interior Mutability     |      20 |
| Rc<RefCell<T>> Integration      |      20 |
| Drop Trait                      |      10 |
| Deref Understanding             |      10 |
| Code Quality and Error Handling |      10 |
| **รวม**                         | **100** |

## เงื่อนไขการผ่าน

- คะแนนรวมอย่างน้อย 70 คะแนน
- ต้องผ่านหัวข้อ Rc, RefCell และ Rc<RefCell<T>>
- ต้องไม่มีการใช้ `unsafe`
- ต้องจัดการ Borrow Conflict โดยไม่ทำให้โปรแกรม Panic โดยไม่ตั้งใจ
