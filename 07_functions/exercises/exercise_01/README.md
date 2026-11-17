# Exercise 01: Basic Function

## วัตถุประสงค์

ฝึกประกาศ Function และเรียกใช้งาน Function ด้วยคำสั่ง `fn`

## โจทย์

เขียนโปรแกรมโดยสร้าง Function สำหรับแสดงข้อความต่าง ๆ แล้วเรียกใช้งาน Function เหล่านั้นจาก `main()`

## เงื่อนไข

1. สร้าง Function ชื่อ `greet` เพื่อแสดงข้อความ `Hello, Rust!`
2. สร้าง Function ชื่อ `show_name` เพื่อแสดงชื่อตนเอง
3. สร้าง Function ชื่อ `show_age` เพื่อแสดงอายุ
4. เรียกใช้งาน Function ทั้ง 3 ตัวจากภายใน `main()`
5. ทดลองเรียกใช้งาน Function `greet` ซ้ำ 2 ครั้ง

## ตัวอย่างผลลัพธ์

```text
Hello, Rust!
Hello, Rust!
Name: Alex
Age: 20
```

หมายเหตุ: เปลี่ยนชื่อและอายุเป็นข้อมูลที่ต้องการได้

## โจทย์เพิ่มเติม

- สร้าง Function ชื่อ `show_country` เพื่อแสดงประเทศ
- สร้าง Function ชื่อ `show_language` เพื่อแสดงภาษาที่กำลังเรียน
- ทดลองเปลี่ยนลำดับการเรียกใช้งาน Function

## เกณฑ์การตรวจสอบ

- [ ] ประกาศ Function ด้วย `fn` ได้
- [ ] กำหนดชื่อ Function ได้เหมาะสม
- [ ] เรียกใช้งาน Function ได้
- [ ] เรียก Function เดิมซ้ำได้
- [ ] สร้าง Function ได้มากกว่าหนึ่งตัว

## วิธีรันโปรแกรม

```powershell
cargo check
cargo run
```
