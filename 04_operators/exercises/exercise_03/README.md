# Exercise 03: Logical Conditions

## Objective

ฝึกใช้ Logical Operators เพื่อสร้างเงื่อนไขหลายรูปแบบ

## Requirements

สร้างตัวแปร:

- `age` มีค่าเป็น 20
- `has_ticket` มีค่าเป็น true
- `is_member` มีค่าเป็น false
- `is_banned` มีค่าเป็น false

สร้างเงื่อนไขต่อไปนี้:

### Condition 1: AND

ผู้ใช้งานสามารถเข้าร่วมกิจกรรมได้เมื่อ:

- อายุอย่างน้อย 18 ปี
- มีตั๋วเข้าร่วมกิจกรรม

### Condition 2: OR

ผู้ใช้งานได้รับสิทธิพิเศษเมื่อ:

- เป็นสมาชิก หรือ
- มีตั๋วเข้าร่วมกิจกรรม

### Condition 3: NOT

ตรวจสอบว่าผู้ใช้งานไม่ได้ถูกระงับสิทธิ์

### Condition 4: Combined Conditions

ผู้ใช้งานสามารถเข้าร่วมกิจกรรมได้เมื่อ:

- อายุอย่างน้อย 18 ปี
- มีตั๋วเข้าร่วมกิจกรรม
- ไม่ถูกระงับสิทธิ์

## Expected Output

```text
Can join: true
Has privilege: true
Not banned: true
Final permission: true
```

## Requirements Checklist

- [ ] ใช้ `&&`
- [ ] ใช้ `||`
- [ ] ใช้ `!`
- [ ] ใช้ Comparison Operators ร่วมกับ Logical Operators
- [ ] แสดงผลลัพธ์ Boolean

## Run

```powershell
cargo run
```
