# Exercise 01: Traffic Light

## Objective

ฝึกประกาศ Enum สร้าง Variants และใช้ `match` ตรวจสอบสถานะสัญญาณไฟจราจร

## Project

`traffic_light`

## Scenario

สร้างโปรแกรมจำลองสัญญาณไฟจราจร โดยกำหนดให้มี 3 สถานะ ได้แก่ Red, Yellow และ Green

## Requirements

ประกาศ Enum ชื่อ `TrafficLight` โดยมี Variants ดังนี้:

| Variant | Meaning  | Display |
| ------- | -------- | ------- |
| Red     | ไฟแดง    | Stop    |
| Yellow  | ไฟเหลือง | Prepare |
| Green   | ไฟเขียว  | Go      |

## Tasks

1. ประกาศ Enum ชื่อ `TrafficLight`
2. กำหนด Variants ทั้ง 3 รายการ
3. สร้างตัวแปร `light` และกำหนดค่าเป็น `TrafficLight::Red`
4. ใช้ `match` ตรวจสอบสถานะของ `light`
5. แสดงข้อความตามสถานะที่ตรวจพบ
6. ทดลองเปลี่ยนค่าเป็น Yellow และ Green

## Expected Output

เมื่อกำหนด `light` เป็น Red:

```

```
