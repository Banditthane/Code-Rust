# Chapter 20: Testing

## Overview

Testing คือกระบวนการตรวจสอบว่าโปรแกรมทำงานได้ถูกต้องตามที่กำหนดหรือไม่

Rust มีระบบ Testing ที่ทำงานร่วมกับ Cargo โดยสามารถเขียน Test Functions และสั่งรันทดสอบได้ด้วยคำสั่ง `cargo test`

การทดสอบช่วยตรวจสอบพฤติกรรมของโปรแกรม ลดข้อผิดพลาด และช่วยให้สามารถปรับปรุงโค้ดได้อย่างมั่นใจ

## Learning Objectives

เมื่อเรียนจบบทนี้ ควรสามารถ:

- อธิบายความหมายและประโยชน์ของ Software Testing ได้
- สร้าง Test Functions ได้
- ใช้ `#[test]` ได้
- รัน Tests ด้วย Cargo ได้
- ใช้ `assert!`, `assert_eq!` และ `assert_ne!` ได้
- สร้าง Unit Tests ได้
- ใช้ `#[cfg(test)]` ได้
- ทดสอบ Private Functions ภายใน Module ได้
- สร้าง Integration Tests ได้
- ทดสอบ Public API ของ Library ได้
- ใช้ `#[should_panic]` และ `#[ignore]` ได้
- เขียน Documentation Tests ได้

## Lessons

| Lesson | Topic                             | Description                             |
| ------ | --------------------------------- | --------------------------------------- |
| 01     | Testing Basics                    | พื้นฐาน Testing และ Test Functions      |
| 02     | Unit Tests                        | การเขียน Unit Tests และ Assertions      |
| 03     | Integration Tests                 | การทดสอบการทำงานร่วมกันของ Modules      |
| 04     | Test Attributes and Documentation | Test Attributes และ Documentation Tests |

## Exercises

| Exercise | Project             | Description                                     |
| -------- | ------------------- | ----------------------------------------------- |
| 01       | testing_basics      | สร้าง Test Functions และทดลอง Assertions        |
| 02       | unit_testing        | เขียน Unit Tests สำหรับ Functions และ Modules   |
| 03       | integration_testing | สร้าง Integration Tests สำหรับ Public API       |
| 04       | testing_challenge   | สร้างระบบพร้อม Unit Tests และ Integration Tests |

## Folder Structure

```text
20_testing/
├── README.md
├── 01_testing_basics/
│   └── README.md
├── 02_unit_tests/
│   └── README.md
├── 03_integration_tests/
│   └── README.md
├── 04_test_attributes_and_documentation/
│   └── README.md
└── exercises/
    ├── exercise_01/
    │   ├── README.md
    │   └── testing_basics/
    ├── exercise_02/
    │   ├── README.md
    │   └── unit_testing/
    ├── exercise_03/
    │   ├── README.md
    │   └── integration_testing/
    └── exercise_04/
        ├── README.md
        └── testing_challenge/
```

## Recommended Learning Sequence

1. ทำความเข้าใจ Testing และ Test Functions
2. ทดลองเขียน Test ด้วย `#[test]`
3. เรียนรู้ Assertions
4. ฝึกสร้าง Unit Tests
5. เรียนรู้ Integration Tests
6. ทดลองใช้ Test Attributes
7. เรียนรู้ Documentation Tests
8. ทำ Exercises ตามลำดับ

## Testing Strategy

ควรทดสอบโปรแกรมอย่างน้อย 3 ประเภท:

### 1. Normal Cases

ทดสอบข้อมูลปกติที่โปรแกรมควรทำงานได้ถูกต้อง

### 2. Edge Cases

ทดสอบข้อมูลบริเวณขอบเขต เช่น:

- ค่าว่าง
- ค่า 0
- ค่าต่ำสุดและสูงสุด
- Collection ที่ไม่มีข้อมูล

### 3. Error Cases

ทดสอบกรณีที่เกิดข้อผิดพลาด เช่น:

- ข้อมูลไม่ถูกต้อง
- การหารด้วยศูนย์
- การค้นหาข้อมูลที่ไม่มีอยู่
- การเรียกใช้งานที่ไม่เป็นไปตามเงื่อนไข

## Useful Cargo Commands

```powershell
# Run all tests
cargo test

# Run tests with output
cargo test -- --show-output

# Run tests matching a name
cargo test test_name

# Run ignored tests
cargo test -- --ignored

# Run documentation tests
cargo test --doc

# Run tests without parallel execution
cargo test -- --test-threads=1
```

## Completion Checklist

- [ ] เข้าใจความหมายของ Testing
- [ ] สร้าง Test Functions ได้
- [ ] ใช้ `#[test]` ได้
- [ ] ใช้ `cargo test` ได้
- [ ] ใช้ Assertions ได้
- [ ] สร้าง Unit Tests ได้
- [ ] ใช้ `#[cfg(test)]` ได้
- [ ] ทดสอบ Private Functions ได้
- [ ] สร้าง Integration Tests ได้
- [ ] ทดสอบ Public API ได้
- [ ] ใช้ `#[should_panic]` ได้
- [ ] ใช้ `#[ignore]` ได้
- [ ] เขียน Documentation Tests ได้
- [ ] ทำ Exercises ทั้ง 4 ข้อสำเร็จ

## Important Note

การที่โปรแกรม Compile ผ่าน ไม่ได้หมายความว่าโปรแกรมทำงานถูกต้องทุกกรณี

ควรทดสอบทั้ง Normal Cases, Edge Cases และ Error Cases เพื่อเพิ่มความมั่นใจในพฤติกรรมของโปรแกรม

## Next Chapter

**Chapter 21: Projects**

นำความรู้ Rust ที่เรียนมาทั้งหมดมาประยุกต์สร้างโปรเจกต์จริง
