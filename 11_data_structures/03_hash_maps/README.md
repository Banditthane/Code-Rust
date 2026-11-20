# Lesson 03: Hash Maps

## 1. What is a HashMap?

HashMap คือ Collection ที่ใช้จัดเก็บข้อมูลในรูปแบบ Key-Value

ข้อมูลแต่ละรายการประกอบด้วย:

- Key: ใช้ระบุหรือค้นหาข้อมูล
- Value: ข้อมูลที่จัดเก็บอยู่ภายใต้ Key นั้น

ตัวอย่างการใช้งาน:

| Key     | Value    |
| ------- | -------- |
| name    | Alice    |
| age     | 25       |
| country | Thailand |

ใน Rust ใช้ `HashMap<K, V>` จาก Standard Library

## 2. Import HashMap

ก่อนใช้งานต้อง Import จาก `std::collections`

```rust
use std::collections::HashMap;

fn main() {
    let mut user = HashMap::new();

    user.insert("name", "Alice");
    user.insert("country", "Thailand");

    println!("{:?}", user);
}
```

Output อาจแสดงลำดับ Key แตกต่างกันได้ เช่น:

```text
{"name": "Alice", "country": "Thailand"}
```

หมายเหตุ: HashMap ไม่รับประกันลำดับการแสดงผลของสมาชิก

## 3. Creating a HashMap

สร้าง HashMap ว่างด้วย `HashMap::new()`

```rust
let mut scores: HashMap<String, i32> = HashMap::new();
```

ตัวอย่างนี้กำหนดว่า:

- Key เป็น `String`
- Value เป็น `i32`

## 4. Inserting Data

ใช้ `.insert(key, value)` เพื่อเพิ่มข้อมูล

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    println!("{:?}", scores);
}
```

หาก Insert Key ที่มีอยู่แล้ว ค่า Value เดิมจะถูกแทนที่

## 5. Accessing Values

ใช้ `.get()` เพื่อค้นหาข้อมูลจาก Key

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    println!("{:?}", scores.get("Alice"));
    println!("{:?}", scores.get("Charlie"));
}
```

Output:

```text
Some(90)
None
```

`.get()` คืนค่า `Option<&V>` ซึ่งหมายความว่าอาจพบหรือไม่พบข้อมูลก็ได้

## 6. Updating Values

สามารถใช้ `.insert()` เพื่อเปลี่ยน Value ของ Key ที่มีอยู่แล้ว

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Alice", 95);

    println!("{:?}", scores.get("Alice"));
}
```

Output:

```text
Some(95)
```

### Using entry()

หากต้องการเพิ่มค่าเฉพาะเมื่อ Key ยังไม่มี สามารถใช้ `.entry().or_insert()`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.entry("Alice").or_insert(90);
    scores.entry("Alice").or_insert(100);

    println!("{:?}", scores.get("Alice"));
}
```

Output:

```text
Some(90)
```

เพราะ Key `"Alice"` มีอยู่แล้ว จึงไม่เปลี่ยนค่าเป็น 100

## 7. Removing Data

ใช้ `.remove()` เพื่อลบข้อมูลตาม Key

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    scores.remove("Bob");

    println!("{:?}", scores);
}
```

Output อาจเป็น:

```text
{"Alice": 90}
```

## 8. Loop Through a HashMap

ใช้ `for` เพื่ออ่าน Key และ Value

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
```

ลำดับที่แสดงผลไม่จำเป็นต้องตรงกับลำดับที่ Insert

## 9. Useful HashMap Methods

| Method               | Description                  |
| -------------------- | ---------------------------- |
| `new()`              | สร้าง HashMap ว่าง           |
| `insert(key, value)` | เพิ่มหรือแทนที่ข้อมูล        |
| `get(&key)`          | ค้นหาข้อมูล                  |
| `contains_key(&key)` | ตรวจสอบว่ามี Key หรือไม่     |
| `remove(&key)`       | ลบข้อมูลตาม Key              |
| `len()`              | นับจำนวนรายการ               |
| `is_empty()`         | ตรวจสอบว่าไม่มีข้อมูลหรือไม่ |
| `entry(key)`         | เข้าถึง Entry ของ Key        |
| `clear()`            | ลบข้อมูลทั้งหมด              |

## 10. Important Notes

- HashMap จัดเก็บข้อมูลเป็น Key-Value
- Key ต้องไม่ซ้ำกัน
- การ Insert Key ซ้ำจะแทนที่ Value เดิม
- `.get()` คืนค่า `Option`
- ลำดับการวนลูปไม่รับประกัน
- Key และ Value ต้องเป็นชนิดข้อมูลที่เหมาะสมกับข้อกำหนดของ HashMap

## Practice

ทดลองเขียนโปรแกรมเพื่อ:

1. สร้าง HashMap สำหรับเก็บชื่อสินค้าและราคา
2. เพิ่มสินค้าอย่างน้อย 3 รายการ
3. ค้นหาราคาสินค้าหนึ่งรายการด้วย `.get()`
4. เปลี่ยนราคาสินค้าหนึ่งรายการ
5. ลบสินค้าออกหนึ่งรายการ
6. แสดงข้อมูลสินค้าทั้งหมดด้วย Loop

## Knowledge Check

1. HashMap ใช้จัดเก็บข้อมูลในรูปแบบใด?
2. Key แตกต่างจาก Value อย่างไร?
3. `.insert()` ใช้ทำอะไร?
4. `.get()` คืนค่าเป็นประเภทใด?
5. หาก Insert Key เดิมอีกครั้งจะเกิดอะไรขึ้น?
6. HashMap รับประกันลำดับการแสดงผลหรือไม่?

## Completion Checklist

- [ ] Import HashMap ได้
- [ ] สร้าง HashMap ได้
- [ ] เพิ่มข้อมูลด้วย `.insert()`
- [ ] ค้นหาข้อมูลด้วย `.get()`
- [ ] อัปเดตข้อมูลได้
- [ ] ลบข้อมูลด้วย `.remove()`
- [ ] วนลูปอ่าน Key และ Value ได้
