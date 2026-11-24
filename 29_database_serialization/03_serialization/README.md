# Lesson 03: Serialization

## 1. Serialization คืออะไร?

Serialization คือการแปลงข้อมูลจากโครงสร้างภายในของโปรแกรมให้เป็นรูปแบบที่สามารถจัดเก็บหรือส่งต่อได้

ตัวอย่าง:

```rust
struct User {
    id: u32,
    name: String,
}
```

สามารถแปลง User เป็น JSON:

```json
{
  "id": 1,
  "name": "Alice"
}
```

## 2. Deserialization

Deserialization คือกระบวนการแปลงข้อมูลจากรูปแบบภายนอกกลับมาเป็นโครงสร้างข้อมูลของโปรแกรม

ตัวอย่าง:

```json
{
  "id": 1,
  "name": "Alice"
}
```

แปลงกลับเป็น Rust Struct:

```rust
struct User {
    id: u32,
    name: String,
}
```

## 3. Serde คืออะไร?

Serde เป็น Framework สำหรับ Serialization และ Deserialization ใน Rust

Crates ที่เกี่ยวข้อง:

| Crate      | หน้าที่                 |
| ---------- | ----------------------- |
| serde      | Serialization Framework |
| serde_json | JSON Support            |
| toml       | TOML Support            |

## 4. ติดตั้ง Serde

```powershell
cargo add serde --features derive
cargo add serde_json
```

หากต้องการ TOML:

```powershell
cargo add toml
```

## 5. Serialize Struct เป็น JSON

ใช้ `Serialize` Derive:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    active: bool,
}
```

สร้างข้อมูล:

```rust
let user = User {
    id: 1,
    name: String::from("Alice"),
    active: true,
};
```

แปลงเป็น JSON:

```rust
let json = serde_json::to_string(&user)?;

println!("{}", json);
```

ตัวอย่างผลลัพธ์:

```json
{ "id": 1, "name": "Alice", "active": true }
```

## 6. Pretty JSON

ใช้ `to_string_pretty()` เพื่อจัดรูปแบบ JSON ให้อ่านง่าย

```rust
let json = serde_json::to_string_pretty(&user)?;

println!("{}", json);
```

ผลลัพธ์:

```json
{
  "id": 1,
  "name": "Alice",
  "active": true
}
```

## 7. Deserialize JSON เป็น Struct

ใช้ `Deserialize` Derive:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
    active: bool,
}
```

แปลง JSON:

```rust
let json = r#"{
    "id": 1,
    "name": "Alice",
    "active": true
}"#;

let user: User = serde_json::from_str(json)?;
```

## 8. Serialize และ Deserialize พร้อมกัน

สามารถ Derive ทั้งสอง Trait:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}
```

เหมาะกับข้อมูลที่ต้องรับจาก API และส่งกลับไปยัง Client

## 9. JSON Value

หากยังไม่ทราบโครงสร้างข้อมูลแน่นอน สามารถใช้ `serde_json::Value`

ตัวอย่าง:

```rust
let json = r#"{
    "name": "Alice",
    "age": 25
}"#;

let value: serde_json::Value = serde_json::from_str(json)?;

println!("{}", value["name"]);
println!("{}", value["age"]);
```

`Value` รองรับ JSON Types เช่น:

- Null
- Boolean
- Number
- String
- Array
- Object

## 10. Serde Attributes

Serde มี Attributes สำหรับปรับแต่งการแปลงข้อมูล

### rename

เปลี่ยนชื่อ Field ใน JSON:

```rust
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_name")]
    name: String,
}
```

### default

กำหนดค่าเริ่มต้นเมื่อ Field ไม่มีอยู่ใน Input:

```rust
#[derive(Deserialize)]
struct Settings {
    #[serde(default)]
    active: bool,
}
```

### skip_serializing

ไม่ส่ง Field ออกไปตอน Serialize:

```rust
#[derive(Serialize)]
struct User {
    id: u32,

    #[serde(skip_serializing)]
    internal_note: String,
}
```

### skip_serializing_if

ข้าม Field เมื่อเข้าเงื่อนไข:

```rust
#[derive(Serialize)]
struct User {
    id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
}
```

### rename_all

กำหนดรูปแบบชื่อ Fields:

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserProfile {
    first_name: String,
    last_name: String,
}
```

ตัวอย่าง JSON:

```json
{
  "firstName": "Alice",
  "lastName": "Smith"
}
```

## 11. Option และ Missing Fields

Field ที่เป็น `Option<T>` สามารถรับค่า `null` ได้ และโดยปกติ Serde รองรับกรณี Field ไม่มีอยู่ด้วย

ตัวอย่าง:

```rust
#[derive(Deserialize)]
struct User {
    id: u32,
    nickname: Option<String>,
}
```

หากต้องการกำหนด Default หรือกฎเพิ่มเติม สามารถใช้ Serde Attributes ได้

## 12. TOML

TOML เป็นรูปแบบข้อมูลที่นิยมใช้กับ Configuration Files

ตัวอย่าง:

```toml
app_name = "Rust Application"
port = 3000
debug = true
```

Rust Struct:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    app_name: String,
    port: u16,
    debug: bool,
}
```

Deserialize:

```rust
let config: Config = toml::from_str(toml_text)?;
```

Serialize:

```rust
let text = toml::to_string(&config)?;
```

## 13. Serialization Errors

Serialization และ Deserialization สามารถเกิด Error ได้ เช่น:

- JSON Syntax ไม่ถูกต้อง
- Field ที่จำเป็นหายไป
- ชนิดข้อมูลไม่ตรงกัน
- ตัวเลขอยู่นอกช่วงของชนิดข้อมูล
- TOML มีรูปแบบไม่ถูกต้อง

ควรใช้ `Result` เพื่อจัดการ Error

ไม่ควรสมมติว่า Input จาก Client ถูกต้องเสมอ

## 14. Serialization กับ Database

Serialization และ Database มีหน้าที่ต่างกัน:

| หัวข้อ          | หน้าที่                                  |
| --------------- | ---------------------------------------- |
| Database        | จัดเก็บและค้นหาข้อมูล                    |
| Serialization   | แปลงข้อมูลเป็นรูปแบบสำหรับส่งหรือจัดเก็บ |
| Deserialization | แปลงข้อมูลกลับเป็นโครงสร้างของโปรแกรม    |
| Serde           | Framework สำหรับแปลงข้อมูลใน Rust        |

ตัวอย่างการทำงาน:

```text
Rust Struct
    |
    v
Serialize
    |
    v
JSON
    |
    v
HTTP API
```

หรือ:

```text
Database Row
    |
    v
Rust Model
    |
    v
Serialize
    |
    v
JSON Response
```

## 15. สรุป

- Serialize: Rust Data → JSON/TOML
- Deserialize: JSON/TOML → Rust Data
- Serde: Framework สำหรับการแปลงข้อมูล
- `Serialize`: Trait สำหรับแปลงข้อมูลออก
- `Deserialize`: Trait สำหรับแปลงข้อมูลเข้า
- Serde Attributes: ปรับแต่งรูปแบบข้อมูล

## Completion Checklist

- [ ] เข้าใจ Serialization
- [ ] เข้าใจ Deserialization
- [ ] ติดตั้ง Serde ได้
- [ ] Derive Serialize ได้
- [ ] Derive Deserialize ได้
- [ ] Serialize Struct เป็น JSON ได้
- [ ] Deserialize JSON เป็น Struct ได้
- [ ] ใช้ Pretty JSON ได้
- [ ] ใช้ `serde_json::Value` ได้
- [ ] ใช้ Serde Attributes ได้
- [ ] จัดการ Option Fields ได้
- [ ] อ่าน TOML ได้
- [ ] เขียน TOML ได้
- [ ] จัดการ Serialization Errors ได้
