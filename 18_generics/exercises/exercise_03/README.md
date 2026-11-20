# Exercise 03: Generic Structs and Enums

## Project

`generic_structs`

## Objective

ฝึกสร้าง Generic Structs และ Enums เพื่อจัดเก็บข้อมูลหลายชนิด

## Requirements

สร้างโปรแกรมสำหรับจัดเก็บข้อมูล โดยมีรายละเอียดดังต่อไปนี้

### Task 1: Generic Struct

สร้าง Struct ชื่อ `Data` ที่:

- ใช้ Generic Type Parameter
- มี Field ชื่อ `value`
- สามารถจัดเก็บข้อมูลชนิดต่าง ๆ ได้

ทดลองสร้าง Instance ที่จัดเก็บ:

- Integer
- Floating Point
- String

### Task 2: Multiple Generic Parameters

สร้าง Struct ชื่อ `Pair` ที่:

- ใช้ Generic Type Parameters สองตัว
- มี Field ชื่อ `first`
- มี Field ชื่อ `second`
- สามารถจัดเก็บข้อมูลที่มีชนิดแตกต่างกันได้

ทดลองสร้างข้อมูล เช่น:

- Integer กับ String
- String กับ Floating Point

### Task 3: Generic Enum

สร้าง Enum ชื่อ `Response` ที่มี Variants:

- `Success(T)`
- `Error(String)`
- `Empty`

โดยใช้ Generic Type Parameter สำหรับข้อมูลใน Success

### Task 4: Handle Response

ทดลองสร้าง Response อย่างน้อย 3 กรณี:

1. Success ที่เก็บ Integer
2. Error ที่เก็บข้อความแจ้งข้อผิดพลาด
3. Empty ที่ไม่มีข้อมูล

ใช้ `match` เพื่อจัดการแต่ละ Variant

## Constraints

- ต้องใช้ Generic Struct
- ต้องใช้ Generic Type Parameters
- ต้องสร้าง Struct ที่มี Type Parameters สองตัว
- ต้องสร้าง Generic Enum
- ต้องใช้ `match` จัดการ Enum
- ห้ามสร้าง Struct แยกตามชนิดข้อมูล
- ต้องจัดการทุก Variant อย่างครบถ้วน

## Expected Output

ตัวอย่างผลลัพธ์:

```text
Integer Data: 100
Float Data: 25.5
String Data: Rust

Pair 1: 100 - Rust
Pair 2: Rust - 25.5

Success: 200
Error: Something went wrong
No data available
```

รูปแบบข้อความสามารถแตกต่างได้ แต่ต้องแสดงข้อมูลครบตามโจทย์

## Checklist

- [ ] สร้าง Generic Struct `Data` ได้
- [ ] สร้าง Generic Struct `Pair` ได้
- [ ] ใช้ Generic Parameters หลายตัวได้
- [ ] สร้าง Generic Enum `Response` ได้
- [ ] ใช้ `match` จัดการ Variants ได้
- [ ] ทดลองข้อมูลหลายชนิดได้
- [ ] โปรแกรม Compile และ Run ผ่าน

## Run Project

```powershell
cd 18_generics/exercises/exercise_03/generic_structs
cargo run
```
