# Chapter 26: Macros

## เป้าหมายของบทเรียน

เรียนรู้ Macro ในภาษา Rust เข้าใจการสร้างโค้ดด้วย Macro และสามารถสร้าง Macro ของตัวเองได้ทั้งแบบ Declarative และ Procedural

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบายความแตกต่างระหว่าง Function และ Macro ได้
- เข้าใจ Macro Expansion
- สร้าง Declarative Macro ด้วย `macro_rules!` ได้
- ใช้ Pattern Matching ภายใน Macro ได้
- ใช้ Repetition เพื่อรับ Arguments หลายรายการได้
- เข้าใจ Macro Hygiene
- เข้าใจ Procedural Macro
- เข้าใจ Derive Macro, Attribute Macro และ Function-like Macro
- ใช้ TokenStream เบื้องต้นได้
- เข้าใจการตรวจสอบและ Debug Macro Expansion

## บทเรียน

| Lesson | หัวข้อ                     | เนื้อหา                                           |
| ------ | -------------------------- | ------------------------------------------------- |
| 01     | Macro Fundamentals         | Macro Concept, Declarative, Procedural, Expansion |
| 02     | Declarative Macros         | macro_rules!, Patterns, Repetition, Arguments     |
| 03     | Procedural Macros          | Derive, Attribute, Function-like, TokenStream     |
| 04     | Macro Design and Debugging | Hygiene, Expansion, Testing, Design               |

## แบบฝึกหัด

| Exercise | ชื่อ                  | ระดับ       |
| -------- | --------------------- | ----------- |
| 01       | Macro Basics          | Easy        |
| 02       | Macro Rules Practice  | Easy-Medium |
| 03       | Derive Macro Practice | Medium      |
| 04       | Macro Challenge       | Challenge   |

## โครงสร้างโฟลเดอร์

```text
26_macros/
├── README.md
├── 01_macro_fundamentals/
│   └── README.md
├── 02_declarative_macros/
│   └── README.md
├── 03_procedural_macros/
│   └── README.md
├── 04_macro_design_debugging/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── macro_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── macro_rules_practice/
    ├── exercise_03/
    │   ├── README.md
    │   └── derive_macro_practice/
    └── exercise_04/
        ├── README.md
        └── macro_challenge/
```

## แนวคิดสำคัญ

### Macro

เครื่องมือสำหรับสร้างหรือแปลงโค้ดในระหว่างกระบวนการ Compile

### Declarative Macro

Macro ที่กำหนดรูปแบบ Input และรูปแบบโค้ดที่ต้องการสร้าง เช่น `macro_rules!`

### Procedural Macro

Macro ที่รับ TokenStream และสร้าง TokenStream ใหม่ โดยมี 3 ประเภทหลัก:

- Derive Macro
- Attribute Macro
- Function-like Macro

### Macro Expansion

กระบวนการที่ Macro ถูกขยายออกมาเป็นโค้ด Rust ก่อนการ Compile ขั้นต่อไป

## เครื่องมือที่ใช้

- Rust Compiler
- `macro_rules!`
- `proc_macro`
- `proc_macro2`
- `syn`
- `quote`

## คำสั่งที่ใช้บ่อย

```powershell
cargo new macro_basics
cd macro_basics

cargo check
cargo run
cargo test
```

สำหรับ Procedural Macro ต้องสร้าง Library Crate และกำหนด `proc-macro = true` ใน `Cargo.toml`

## แนวทางการเรียน

1. เข้าใจแนวคิด Macro และ Macro Expansion
2. ฝึกสร้าง Declarative Macro ด้วย `macro_rules!`
3. เรียนรู้ Procedural Macro ทั้ง 3 ประเภท
4. ฝึกออกแบบ Macro และตรวจสอบ Expansion
5. ทำแบบฝึกหัดตามลำดับ

## ข้อควรระวัง

- Macro ไม่ใช่ Function และมีวิธีตรวจสอบ Input ต่างจาก Function
- Macro สามารถสร้างโค้ดที่ซับซ้อนได้ จึงควรออกแบบให้เข้าใจง่าย
- ต้องระวังชื่อ Identifier ที่อาจชนกับโค้ดของผู้เรียก
- Procedural Macro ต้องอยู่ใน Crate ที่กำหนดเป็น `proc-macro`
- Derive Macro ต้องสร้างโค้ดที่ถูกต้องตามชนิดข้อมูลที่รับเข้ามา
- Macro ที่สร้างโค้ดจำนวนมากอาจทำให้ Compile Time เพิ่มขึ้น

## Completion Checklist

- [ ] เข้าใจ Macro
- [ ] อธิบายความแตกต่างระหว่าง Function และ Macro ได้
- [ ] เข้าใจ Declarative Macro
- [ ] สร้าง `macro_rules!` ได้
- [ ] ใช้ Pattern Matching ได้
- [ ] ใช้ Repetition ได้
- [ ] เข้าใจ Procedural Macro
- [ ] เข้าใจ Derive, Attribute และ Function-like Macro
- [ ] เข้าใจ TokenStream
- [ ] ตรวจสอบ Macro Expansion ได้
- [ ] ทำแบบฝึกหัดทั้ง 4 ข้อครบถ้วน

## เกณฑ์ผ่านบทเรียน

สามารถสร้าง Declarative Macro และเข้าใจโครงสร้างการสร้าง Procedural Macro รวมถึงสามารถตรวจสอบและแก้ไข Macro เบื้องต้นได้
