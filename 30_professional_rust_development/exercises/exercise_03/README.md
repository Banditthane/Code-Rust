# Exercise 03: Performance Practice

## ระดับ

Medium

## Objective

ฝึกวัด วิเคราะห์ และปรับปรุง Performance ของ Rust Application โดยใช้ Benchmarking และ Profiling

## Problem Statement

สร้างโปรแกรมประมวลผลข้อมูลตัวเลข (Number Processing Application)

โปรแกรมต้องรองรับ:

1. คำนวณผลรวมตัวเลข
2. ค้นหาค่าสูงสุดและต่ำสุด
3. คำนวณค่าเฉลี่ย
4. เรียงลำดับข้อมูล
5. กรองข้อมูลตามเงื่อนไข
6. เปรียบเทียบวิธีประมวลผลที่แตกต่างกัน

เป้าหมายคือวัด Performance ก่อนและหลังการปรับปรุง

## Requirements

### Part 1: Data Generation

สร้างชุดข้อมูลตัวเลขขนาด:

- 1,000 รายการ
- 100,000 รายการ
- 1,000,000 รายการ

กำหนดวิธีสร้างข้อมูลให้ทำซ้ำได้ เพื่อให้การเปรียบเทียบมีความยุติธรรม

### Part 2: Basic Operations

สร้าง Functions:

- `calculate_sum()`
- `find_min_max()`
- `calculate_average()`
- `filter_values()`
- `sort_values()`

ทุก Function ต้องให้ผลลัพธ์ถูกต้อง

### Part 3: Compare Implementations

เลือกอย่างน้อย 2 Operations เพื่อเปรียบเทียบวิธีเขียนที่แตกต่างกัน

ตัวอย่าง:

- ใช้ Loop เทียบกับ Iterator
- ใช้ `Vec::clone()` เทียบกับการใช้ Reference
- กำหนด Capacity เทียบกับการเพิ่มข้อมูลโดยไม่กำหนด Capacity
- เปรียบเทียบ Algorithm ที่มี Complexity ต่างกัน

ห้ามสรุปว่าวิธีใดเร็วกว่าโดยไม่วัดผล

### Part 4: Benchmarking

ติดตั้ง Criterion:

```powershell
cargo add criterion --dev
```

สร้าง Benchmark File:

```text
benches/performance.rs
```

กำหนด Benchmark Target ใน `Cargo.toml`:

```toml
[[bench]]
name = "performance"
harness = false
```

Benchmark ต้องครอบคลุม:

- Dataset ขนาดเล็ก
- Dataset ขนาดกลาง
- Dataset ขนาดใหญ่
- อย่างน้อย 2 Implementations สำหรับ Operation ที่เลือกเปรียบเทียบ

### Part 5: Profiling

เลือก Profiling Tool ที่เหมาะสมกับ Operating System

ตรวจสอบ:

- CPU Usage
- Function ที่ใช้เวลามาก
- Memory Usage
- Allocation ที่เกิดขึ้น
- จุดที่มีการประมวลผลซ้ำ

บันทึกผลที่ตรวจพบอย่างน้อย 3 รายการ

### Part 6: Optimization

เลือกปรับปรุงอย่างน้อย 2 จุด

ตัวอย่าง:

- ลด Clone ที่ไม่จำเป็น
- ปรับ Algorithm
- กำหนด Vector Capacity
- ลดการคำนวณซ้ำ
- เลือก Data Structure ให้เหมาะสม

หลังปรับปรุงต้อง Benchmark อีกครั้ง

### Part 7: Release Build

เปรียบเทียบผลระหว่าง:

```powershell
cargo run
```

และ:

```powershell
cargo run --release
```

ใช้ Dataset และวิธีวัดเดียวกัน

## Constraints

- ต้องใช้ Rust
- ต้องใช้ Criterion
- ต้องมี Benchmark อย่างน้อย 2 Implementations
- ต้องทดสอบ Dataset อย่างน้อย 3 ขนาด
- ต้องบันทึก Baseline ก่อน Optimization
- ต้องปรับปรุง Code อย่างน้อย 2 จุด
- ต้องวัดผลหลัง Optimization
- ต้องตรวจสอบ Correctness หลังปรับปรุง
- ต้องเปรียบเทียบ Dev และ Release
- ต้องไม่ใช้ผล Benchmark ครั้งเดียวเป็นข้อสรุป
- ต้องไม่ Optimize โดยทำให้ผลลัพธ์ผิดพลาด

## Expected Output

```text
Performance Practice

Dataset: 1,000 items
Dataset: 100,000 items
Dataset: 1,000,000 items

Basic Operations
--------------------------------
Sum: Completed
Min/Max: Completed
Average: Completed
Filter: Completed
Sort: Completed
--------------------------------

Benchmark Results
--------------------------------
Implementation A:  ...
Implementation B:  ...
--------------------------------

Optimization
--------------------------------
Optimization 1: Completed
Optimization 2: Completed
--------------------------------

Correctness Tests: Passed
Performance Analysis: Completed
```

ตัวเลขเวลาให้บันทึกจากผลการทดลองจริง ไม่ใช้ค่าตัวอย่างจากเครื่องอื่น

## Hints

- ใช้ `std::time::Instant` สำหรับการวัดเวลาเบื้องต้น
- ใช้ Criterion สำหรับ Benchmark ที่ทำซ้ำและเปรียบเทียบผล
- ใช้ `std::hint::black_box()` ป้องกัน Compiler ตัดการคำนวณบางส่วนที่ไม่ส่งผลต่อผลลัพธ์ที่สังเกตได้
- แยก Data Generation ออกจากส่วนที่ต้องการวัด
- อย่ารวม File I/O หรือ Console Output ใน Benchmark หากไม่ได้ต้องการวัดส่วนเหล่านั้น
- ใช้ Profiling เพื่อค้นหา Bottleneck
- ตรวจสอบ Correctness ด้วย Tests ก่อนและหลัง Optimization

## Performance Report

จัดทำไฟล์:

```text
PERFORMANCE_REPORT.md
```

ต้องมีหัวข้อ:

1. Hardware และ Operating System
2. Rust Version
3. Dataset ที่ใช้
4. Baseline Results
5. Profiling Findings
6. Optimization ที่ทำ
7. Results หลัง Optimization
8. Dev และ Release Comparison
9. ข้อสรุป

## Completion Checklist

- [ ] สร้าง Dataset ได้
- [ ] คำนวณ Sum ได้
- [ ] หา Min/Max ได้
- [ ] คำนวณ Average ได้
- [ ] Filter ข้อมูลได้
- [ ] Sort ข้อมูลได้
- [ ] สร้าง Benchmark ได้
- [ ] เปรียบเทียบอย่างน้อย 2 Implementations ได้
- [ ] ทดสอบ Dataset 3 ขนาดได้
- [ ] บันทึก Baseline ได้
- [ ] ใช้ Profiling Tool ได้
- [ ] ระบุ Bottlenecks ได้อย่างน้อย 3 รายการ
- [ ] Optimize อย่างน้อย 2 จุด
- [ ] Benchmark หลัง Optimize ได้
- [ ] Correctness Tests ผ่าน
- [ ] เปรียบเทียบ Dev และ Release ได้
- [ ] เขียน Performance Report ได้

## Run

```powershell
cd 30_professional_rust_development/exercises/exercise_03/performance_practice

cargo check
cargo test
cargo bench
cargo run --release
```
