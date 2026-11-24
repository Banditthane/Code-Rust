# 22 - closures_iterators

# Chapter 22: Closures & Iterators

## ภาพรวม

บทนี้เรียนรู้การทำงานกับ Closures และ Iterators ในภาษา Rust ซึ่งช่วยให้สามารถเขียนโค้ดประมวลผลข้อมูลได้กระชับ ยืดหยุ่น และอ่านง่ายขึ้น

Closures คือฟังก์ชันลักษณะพิเศษที่สามารถรับค่าและเข้าถึงตัวแปรจากขอบเขตภายนอกได้ ส่วน Iterators คือเครื่องมือสำหรับอ่านและประมวลผลสมาชิกของชุดข้อมูลทีละรายการ

## วัตถุประสงค์

เมื่อเรียนจบบทนี้ ผู้เรียนจะสามารถ:

- อธิบายความหมายและการใช้งาน Closure ได้
- สร้าง Closure ที่รับพารามิเตอร์และคืนค่าได้
- เข้าใจความแตกต่างระหว่าง `Fn`, `FnMut` และ `FnOnce`
- อธิบายแนวคิด Iterator และการทำงานของ `Iterator` trait ได้
- ใช้ `iter()`, `iter_mut()` และ `into_iter()` ได้
- ใช้ Iterator adapters เช่น `map()` และ `filter()` ได้
- ใช้ Iterator consumers เช่น `collect()`, `sum()` และ `count()` ได้
- ประยุกต์ใช้ Closure และ Iterator กับข้อมูลใน `Vec<T>` ได้

## บทเรียน

| ลำดับ | หัวข้อ                          | เนื้อหาหลัก                                                      |
| ----- | ------------------------------- | ---------------------------------------------------------------- |
| 01    | Closure Basics                  | การสร้าง Closure, Parameters, Return Values, Environment Capture |
| 02    | Closure Traits                  | Fn, FnMut, FnOnce และ Move Closures                              |
| 03    | Iterator Basics                 | Iterator Trait, iter, iter_mut, into_iter                        |
| 04    | Iterator Adapters and Consumers | map, filter, fold, collect, sum, count                           |

## แบบฝึกหัด

| ลำดับ | โปรเจกต์           | ระดับ       |
| ----- | ------------------ | ----------- |
| 01    | closure_basics     | Easy        |
| 02    | closure_traits     | Easy-Medium |
| 03    | iterator_basics    | Medium      |
| 04    | iterator_challenge | Challenge   |

## โครงสร้างโฟลเดอร์

```text
22_closures_iterators/
├── README.md
├── 01_closure_basics/
│   └── README.md
├── 02_closure_traits/
│   └── README.md
├── 03_iterator_basics/
│   └── README.md
├── 04_iterator_adapters/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── closure_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── closure_traits/
    ├── exercise_03/
    │   ├── README.md
    │   └── iterator_basics/
    └── exercise_04/
        ├── README.md
        └── iterator_challenge/
```

## คำสั่งที่ใช้บ่อย

```powershell
cargo new closure_basics --vcs none
cargo run
cargo check
cargo fmt
cargo clippy
```

## ความรู้ที่ควรมีมาก่อน

- Variables และ Data Types
- Functions
- Ownership และ Borrowing
- References
- Structs และ Enums
- Vectors และ Collections
- Traits และ Generics

## เกณฑ์การเรียนจบบท

- [ ] เข้าใจ Closure และสามารถสร้าง Closure ได้
- [ ] เข้าใจ Fn, FnMut และ FnOnce
- [ ] เข้าใจการทำงานของ Iterator
- [ ] ใช้ iter, iter_mut และ into_iter ได้ถูกต้อง
- [ ] ใช้ Iterator adapters และ consumers ได้
- [ ] ทำแบบฝึกหัดทั้ง 4 ชุดได้
- [ ] สามารถอธิบายการทำงานของโค้ดที่เขียนเองได้

---

**หมายเหตุ:** แบบฝึกหัดในบทนี้เน้นให้ผู้เรียนลงมือเขียนโค้ดด้วยตนเอง จึงไม่มีเฉลยโค้ดในโจทย์
