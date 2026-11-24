# Lesson 03: Performance and Optimization

## 1. Performance Optimization คืออะไร?

Performance Optimization คือการปรับปรุงโปรแกรมให้ใช้ทรัพยากรอย่างมีประสิทธิภาพมากขึ้น

ตัวอย่างเป้าหมาย:

- ลดเวลาในการประมวลผล
- ลด Memory Usage
- ลด CPU Usage
- ลดจำนวน Allocations
- เพิ่ม Throughput
- ลด Latency

ไม่ควร Optimize โดยอาศัยความรู้สึกเพียงอย่างเดียว ควรวัดผลก่อนและหลังปรับปรุง

## 2. Algorithm Complexity

Algorithm Complexity ใช้อธิบายการเติบโตของต้นทุนเมื่อ Input เพิ่มขึ้น

ตัวอย่าง Big O:

| Complexity | ตัวอย่างแนวคิด          |
| ---------- | ----------------------- |
| O(1)       | เข้าถึง Array ตาม Index |
| O(log n)   | Binary Search           |
| O(n)       | วน Loop หนึ่งรอบ        |
| O(n log n) | Efficient Sorting       |
| O(n²)      | Nested Loop บางรูปแบบ   |

ควรเลือก Algorithm ให้เหมาะกับขนาดและลักษณะข้อมูล

## 3. Memory Allocation

Rust ใช้ Ownership และ Borrowing เพื่อจัดการ Memory

สิ่งที่ควรพิจารณา:

- Stack และ Heap
- Allocation
- Deallocation
- Move และ Copy
- Clone
- Collection Growth
- Memory Layout

ตัวอย่าง:

```rust
let name = String::from("Alice");
```

`String` เก็บข้อมูลข้อความที่สามารถขยายได้ และมีการจัดการ Heap Allocation

## 4. Clone และ Borrowing

การ Clone อาจทำให้เกิดการคัดลอกข้อมูล

ตัวอย่าง:

```rust
let original = String::from("Rust");

let copied = original.clone();
```

หากไม่จำเป็นต้องเป็นเจ้าของข้อมูลใหม่ อาจใช้ Reference:

```rust
fn print_name(name: &str) {
    println!("{}", name);
}
```

อย่างไรก็ตาม Clone ไม่ได้ผิดเสมอไป ควรเลือกตาม Ownership และความต้องการของโปรแกรม

## 5. Capacity และ Collection

Collection เช่น `Vec<T>` สามารถเพิ่ม Capacity ได้

ตัวอย่าง:

```rust
let mut values = Vec::with_capacity(100);
```

หากทราบขนาดข้อมูลล่วงหน้า การกำหนด Capacity อาจช่วยลดจำนวน Allocations ระหว่างเพิ่มข้อมูล

ควรใช้เมื่อมีเหตุผลรองรับ ไม่จำเป็นต้องกำหนด Capacity สูงเกินจริง

## 6. Benchmarking

Benchmarking คือการวัดประสิทธิภาพของ Code ด้วยการทดลองที่ควบคุมเงื่อนไข

เครื่องมือที่นิยมใช้:

- Criterion
- `std::time::Instant`
- Profiling Tools
- System Monitoring Tools

## 7. ใช้ Instant วัดเวลา

ตัวอย่าง:

```rust
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut sum = 0_u64;

    for number in 0..1_000_000 {
        sum += number;
    }

    let elapsed = start.elapsed();

    println!("Sum: {}", sum);
    println!("Elapsed: {:?}", elapsed);
}
```

ข้อควรจำ:

- ผลการวัดอาจแตกต่างตามเครื่อง
- Background Processes อาจรบกวนผล
- การวัดครั้งเดียวไม่น่าเชื่อถือเพียงพอ
- ต้องใช้ Input และเงื่อนไขเดียวกันเมื่อต้องการเปรียบเทียบ

## 8. Criterion Benchmark

Criterion เป็น Library สำหรับทำ Benchmark

ติดตั้ง:

```powershell
cargo add criterion --dev
```

ตัวอย่างแนวคิด:

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn calculate_sum() {
    let mut sum = 0_u64;

    for number in 0..1_000_000 {
        sum += number;
    }

    std::hint::black_box(sum);
}

fn benchmark_sum(c: &mut Criterion) {
    c.bench_function("calculate_sum", |b| {
        b.iter(calculate_sum);
    });
}

criterion_group!(benches, benchmark_sum);
criterion_main!(benches);
```

กำหนด Benchmark Target ใน `Cargo.toml`:

```toml
[[bench]]
name = "performance"
harness = false
```

วางไฟล์ Benchmark ที่:

```text
benches/performance.rs
```

รัน:

```powershell
cargo bench
```

## 9. Profiling

Profiling คือการตรวจสอบว่าโปรแกรมใช้ทรัพยากรตรงไหนมากที่สุด

สิ่งที่ตรวจสอบ:

- CPU Hotspots
- Memory Allocation
- Function Call Time
- I/O Bottlenecks
- Thread Activity

Profiling ช่วยค้นหาจุดที่ควรปรับปรุงจริง

เครื่องมืออาจแตกต่างตาม Operating System เช่น:

- Linux: `perf`
- Windows: Windows Performance Recorder และ Windows Performance Analyzer
- Cross-platform: เครื่องมือ Profiling ที่รองรับ Rust

## 10. CPU Optimization

แนวทาง:

1. เลือก Algorithm ที่เหมาะสม
2. ลดงานที่ทำซ้ำโดยไม่จำเป็น
3. ลดการคำนวณที่ไม่จำเป็น
4. หลีกเลี่ยงการสร้าง Temporary Data มากเกินไป
5. ใช้ Parallelism เมื่อเหมาะสม
6. วัดผลหลังการเปลี่ยนแปลง

## 11. Memory Optimization

แนวทาง:

- ใช้ Borrowing เมื่อเหมาะสม
- ลด Clone ที่ไม่จำเป็น
- กำหนด Capacity เมื่อทราบขนาด
- เลือก Collection ให้เหมาะกับงาน
- หลีกเลี่ยงการเก็บข้อมูลซ้ำ
- ตรวจสอบ Allocation Hotspots

ไม่ควรใช้ Unsafe Code เพื่อหวัง Performance โดยไม่มีเหตุผลและการตรวจสอบความปลอดภัยที่เพียงพอ

## 12. Release Build

Cargo มี Build Profiles

Development Build:

```powershell
cargo build
```

Release Build:

```powershell
cargo build --release
```

Release Executable จะอยู่ใน:

```text
target/release/
```

Release Profile ใช้การตั้งค่า Optimization ที่ต่างจาก Development Profile

ตัวอย่างการปรับแต่ง:

```toml
[profile.release]
opt-level = 3
```

ควรเลือกค่าที่เหมาะสมและวัดผลจริง เพราะ Optimization อาจเพิ่มเวลา Build หรือขนาด Binary ได้

## 13. Debug กับ Release

| รายการ            | Dev Profile              | Release Profile              |
| ----------------- | ------------------------ | ---------------------------- |
| เป้าหมาย          | พัฒนาและ Debug           | ประสิทธิภาพ                  |
| Optimization      | ต่ำกว่าโดยทั่วไป         | สูงกว่าโดยทั่วไป             |
| Compile Time      | มักเร็วกว่า              | มักช้ากว่า                   |
| Runtime           | อาจช้ากว่า               | มักเร็วกว่า                  |
| Debug Information | รองรับการ Debug ได้สะดวก | ขึ้นอยู่กับ Profile Settings |

อย่าเปรียบเทียบ Performance โดยใช้ Dev Build กับ Release Build คนละแบบ

## 14. Optimization Workflow

แนวทางที่ควรทำ:

1. สร้าง Baseline
2. กำหนด Performance Goal
3. Benchmark
4. Profile
5. หา Bottleneck
6. เปลี่ยน Code
7. Benchmark อีกครั้ง
8. ตรวจสอบ Correctness
9. เปรียบเทียบผล
10. บันทึกผลการทดลอง

## 15. สรุป

- Algorithm Complexity ช่วยประเมินการเติบโตของต้นทุน
- Ownership และ Borrowing มีผลต่อการจัดการ Memory
- Benchmarking ใช้วัดประสิทธิภาพ
- Profiling ใช้ค้นหา Bottlenecks
- Release Build ใช้ Optimization Profile
- ควรวัดผลก่อนและหลังปรับปรุง

## Completion Checklist

- [ ] เข้าใจ Big O เบื้องต้น
- [ ] เข้าใจ Memory Allocation
- [ ] เข้าใจ Clone และ Borrowing
- [ ] เข้าใจ Collection Capacity
- [ ] ใช้ `Instant` วัดเวลาได้
- [ ] เข้าใจ Criterion
- [ ] สร้าง Benchmark ได้
- [ ] เข้าใจ Profiling
- [ ] วิเคราะห์ CPU Bottlenecks ได้
- [ ] วิเคราะห์ Memory Usage ได้
- [ ] ใช้ Release Build ได้
- [ ] เปรียบเทียบ Dev และ Release ได้
- [ ] มี Baseline ก่อน Optimize
- [ ] ตรวจสอบ Correctness หลัง Optimize
