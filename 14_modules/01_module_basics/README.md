# Lesson 01: Module Basics

## 1. What is a Module?

Module คือส่วนหนึ่งของโปรแกรมที่ใช้จัดกลุ่ม Items ที่เกี่ยวข้องกัน เช่น:

- Functions
- Structs
- Enums
- Constants
- Other Modules

ตัวอย่างเช่น โปรแกรมร้านค้าอาจแบ่งเป็น:

- `products` สำหรับจัดการสินค้า
- `customers` สำหรับจัดการลูกค้า
- `orders` สำหรับจัดการคำสั่งซื้อ

การแบ่ง Module ช่วยให้โปรแกรมมีโครงสร้างที่เข้าใจง่ายขึ้น

## 2. Declaring a Module

ใช้ Keyword `mod` เพื่อประกาศ Module

รูปแบบ:

```rust
mod module_name {
    // items
}
```

ตัวอย่าง:

```rust
mod greetings {
    fn hello() {
        println!("Hello from greetings module!");
    }
}
```

ในตัวอย่างนี้:

- `mod` ใช้ประกาศ Module
- `greetings` คือชื่อ Module
- `hello()` คือ Function ภายใน Module

## 3. Calling an Item in a Module

สามารถอ้างอิง Item ด้วย Path โดยใช้เครื่องหมาย `::`

อย่างไรก็ตาม Items ภายใน Module เป็น Private โดยค่าเริ่มต้น

หากต้องการเรียก Function จากภายนอก Module ต้องกำหนดให้เป็น Public ด้วย `pub`

ตัวอย่าง:

```rust
mod greetings {
    pub fn hello() {
        println!("Hello from greetings module!");
    }
}

fn main() {
    greetings::hello();
}
```

ผลลัพธ์:

```text
Hello from greetings module!
```

## 4. Nested Modules

Module สามารถมี Module ย่อยอยู่ภายในได้

ตัวอย่าง:

```rust
mod company {
    pub mod employees {
        pub fn show_employee() {
            println!("Employee information");
        }
    }
}

fn main() {
    company::employees::show_employee();
}
```

Path ที่ใช้เรียก Function คือ:

```rust
company::employees::show_employee()
```

ลำดับของ Path เริ่มจาก Module ชั้นนอกไปยัง Module ชั้นใน

## 5. Modules and Functions

สามารถจัดกลุ่ม Functions ที่ทำงานเกี่ยวข้องกันไว้ใน Module เดียวกันได้

ตัวอย่าง:

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    println!("{}", calculator::add(10, 5));
    println!("{}", calculator::subtract(10, 5));
}
```

ผลลัพธ์:

```text
15
5
```

## 6. Important Notes

- `mod` ใช้ประกาศ Module
- Module ใช้จัดกลุ่ม Items
- Module สามารถซ้อนกันได้
- ใช้ `::` เพื่ออ้างอิง Items ผ่าน Path
- Items เป็น Private โดยค่าเริ่มต้น
- ใช้ `pub` เมื่อต้องการเปิดให้เข้าถึงจากภายนอก Module

## 7. Knowledge Check

1. Module คืออะไร?
2. `mod` ใช้ทำอะไร?
3. Module สามารถมี Function ได้หรือไม่?
4. `pub` มีหน้าที่อะไร?
5. `::` ใช้ทำอะไร?
6. Nested Module หมายถึงอะไร?

## 8. Completion Checklist

- [ ] เข้าใจความหมายของ Module
- [ ] ประกาศ Module ด้วย `mod` ได้
- [ ] ประกาศ Function ภายใน Module ได้
- [ ] ใช้ `pub` กับ Function ได้
- [ ] เรียก Function ผ่าน Module Path ได้
- [ ] สร้าง Nested Modules ได้
