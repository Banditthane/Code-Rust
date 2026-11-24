# Lesson 04: Security and Deployment

## 1. Application Security

Application Security คือการออกแบบและพัฒนาโปรแกรมให้ลดความเสี่ยงจากการโจมตีและการใช้งานที่ไม่ถูกต้อง

หัวข้อสำคัญ:

- Input Validation
- Authentication
- Authorization
- Secrets Management
- Dependency Security
- Error Handling
- Logging
- Secure Configuration
- Deployment Security

## 2. Input Validation

Input Validation คือการตรวจสอบข้อมูลก่อนนำไปประมวลผล

ตัวอย่างข้อมูลที่ต้องตรวจสอบ:

- Username
- Email
- Password
- File Path
- Numeric Values
- HTTP Request Body
- Database Parameters

แนวทาง:

1. ตรวจสอบชนิดข้อมูล
2. ตรวจสอบความยาว
3. ตรวจสอบช่วงค่า
4. ตรวจสอบรูปแบบ
5. ตรวจสอบข้อจำกัดทางธุรกิจ
6. ปฏิเสธ Input ที่ไม่ถูกต้อง

อย่าเชื่อถือข้อมูลจาก Client แม้จะเป็นข้อมูลที่ส่งผ่าน API ของระบบเอง

## 3. SQL Injection

SQL Injection เกิดจากการนำ Input ไปประกอบเป็น SQL โดยไม่จัดการอย่างเหมาะสม

ไม่ควรทำ:

```rust
let sql = format!(
    "SELECT * FROM users WHERE name = '{}'",
    user_input
);
```

ควรใช้ Parameterized Queries:

```rust
sqlx::query(
    "SELECT * FROM users WHERE name = ?"
)
.bind(user_input)
.fetch_all(&pool)
.await?;
```

## 4. Secrets Management

Secrets ได้แก่:

- Database Password
- API Keys
- Token Signing Keys
- Private Keys
- Service Credentials

ห้ามเขียน Secrets ลงใน Source Code

ตัวอย่าง Environment Variables:

```text
DATABASE_URL
API_SECRET
APP_ENV
APP_PORT
```

อ่าน Environment Variable:

```rust
let database_url = std::env::var("DATABASE_URL")?;
```

ห้ามพิมพ์ Secret ลง Console หรือ Logs

## 5. Environment Configuration

ควรแยก Configuration ตาม Environment:

- Development
- Testing
- Staging
- Production

ตัวอย่าง:

```text
APP_ENV=production
APP_PORT=3000
```

ข้อควรจำ:

- ตรวจสอบ Configuration เมื่อเริ่ม Application
- กำหนด Default Values อย่างเหมาะสม
- ห้ามใช้ Development Secrets ใน Production
- ห้าม Commit `.env` ที่มี Secret จริง
- ควรมี `.env.example` ที่ใส่เฉพาะชื่อ Variables และค่าตัวอย่างที่ไม่เป็นความลับ

## 6. Dependency Security

Dependencies อาจมีช่องโหว่ด้าน Security

แนวทางตรวจสอบ:

1. ตรวจสอบ Crate ที่นำมาใช้
2. ตรวจสอบ Version
3. ตรวจสอบ Maintenance
4. ตรวจสอบ Security Advisories
5. อัปเดต Dependencies
6. ทดสอบหลังอัปเดต

เครื่องมือที่ใช้ได้:

```powershell
cargo tree
cargo audit
```

ติดตั้ง Cargo Audit:

```powershell
cargo install cargo-audit
```

ตรวจสอบ:

```powershell
cargo audit
```

`cargo audit` ตรวจสอบช่องโหว่ที่มีข้อมูลอยู่ใน RustSec Advisory Database ตามฐานข้อมูลที่เครื่องมือเข้าถึงได้ ไม่ได้ยืนยันว่า Application ปลอดภัยจากทุกประเภทการโจมตี

## 7. Unsafe Code Security

Rust มีระบบ Type และ Ownership ช่วยป้องกันปัญหาหน่วยความจำหลายประเภท

อย่างไรก็ตาม Unsafe Code ยังต้องได้รับการตรวจสอบ

แนวทาง:

- ลดขอบเขต Unsafe Blocks
- ระบุ Safety Contract
- ตรวจสอบ Pointer Validity
- ตรวจสอบ Memory Lifetime
- ตรวจสอบ Aliasing
- เขียน Tests สำหรับ Safe Wrapper
- Review Unsafe Code อย่างละเอียด

## 8. Error Handling

Production Application ต้องจัดการ Error อย่างเหมาะสม

แนวทาง:

- แยก User Errors
- แยก Business Errors
- แยก Internal Errors
- ไม่เปิดเผย Stack Trace ให้ Client โดยไม่จำเป็น
- บันทึกรายละเอียดที่จำเป็นสำหรับการตรวจสอบ
- ใช้ Error Types ที่สื่อความหมาย

ตัวอย่าง:

```rust
enum AppError {
    NotFound,
    InvalidInput,
    Internal,
}
```

การแสดง Error ให้ Client ควรให้ข้อมูลเพียงพอสำหรับแก้ไข Request โดยไม่เปิดเผยข้อมูลภายในระบบ

## 9. Logging

Logging ใช้บันทึกเหตุการณ์ระหว่าง Application ทำงาน

ตัวอย่างข้อมูลที่ควรบันทึก:

- Application Started
- Request Received
- Request Completed
- Database Connection Failed
- Authentication Failed
- Internal Error

ไม่ควรบันทึก:

- Password
- Access Token
- Secret Key
- Database Credentials
- ข้อมูลส่วนบุคคลที่ไม่จำเป็น

## 10. Tracing

Tracing ใช้ติดตามการทำงานของ Application โดยเฉพาะระบบที่มีหลาย Layers หรือ Async Tasks

Crates ที่นิยม:

- `tracing`
- `tracing-subscriber`

ติดตั้ง:

```powershell
cargo add tracing
cargo add tracing-subscriber
```

ตัวอย่าง:

```rust
use tracing::{info, warn};

fn main() {
    tracing_subscriber::fmt::init();

    info!("Application started");
    warn!("Example warning");
}
```

Tracing สามารถเพิ่ม Context ให้กับ Logs และใช้ติดตามการทำงานระหว่าง Tasks ได้

## 11. Build Profiles

ก่อน Deployment ควรสร้าง Release Build:

```powershell
cargo build --release
```

ตรวจสอบว่า:

- Build ผ่าน
- Tests ผ่าน
- Configuration ถูกต้อง
- Dependencies พร้อมใช้งาน
- ไม่มี Secrets ใน Binary หรือ Repository โดยไม่ตั้งใจ

## 12. Cross Compilation

Cross Compilation คือการ Compile โปรแกรมบนเครื่องหนึ่งเพื่อให้ทำงานบนอีก Target Platform หนึ่ง

ตัวอย่าง:

- Windows x86_64
- Linux x86_64
- Linux ARM64
- macOS ARM64

ตรวจสอบ Target ที่ติดตั้ง:

```powershell
rustup target list --installed
```

เพิ่ม Target:

```powershell
rustup target add x86_64-unknown-linux-gnu
```

Build:

```powershell
cargo build --release --target x86_64-unknown-linux-gnu
```

ข้อควรระวัง:

- ต้องมี Linker และ Toolchain ที่เหมาะสม
- Dependencies บางตัวอาจมี Native Libraries
- Target ต้องรองรับระบบปฏิบัติการและ Architecture ที่ต้องการ
- Cross Compilation ไม่ได้หมายความว่า Binary จะทำงานได้ทันทีโดยไม่ต้องทดสอบบน Target จริง

## 13. Deployment

Deployment คือการนำ Application ไปติดตั้งและใช้งานใน Environment ที่กำหนด

ขั้นตอนทั่วไป:

1. ตรวจสอบ Code
2. Run Tests
3. ตรวจสอบ Dependencies
4. Build Release
5. เตรียม Configuration
6. เตรียม Database
7. Deploy Application
8. ตรวจสอบ Logs
9. ตรวจสอบ Health
10. วางแผน Rollback

## 14. Deployment Checklist

### Build

- Release Build ผ่าน
- Target Platform ถูกต้อง
- Binary ทำงานได้

### Configuration

- Environment Variables ครบ
- Secrets ถูกจัดเก็บอย่างปลอดภัย
- Production Configuration ถูกต้อง

### Security

- ตรวจสอบ Dependencies
- ตรวจสอบ Input Validation
- ตรวจสอบ Authentication และ Authorization
- ไม่มี Secrets ใน Repository
- ตรวจสอบ Logging ไม่ให้เปิดเผยข้อมูลสำคัญ

### Reliability

- Error Handling ครบ
- Health Check พร้อมใช้งานตามความเหมาะสม
- Logging ทำงาน
- มีแผน Backup
- มีแผน Rollback

## 15. สรุป

- Input Validation ช่วยตรวจสอบข้อมูล
- Parameterized Queries ลดความเสี่ยง SQL Injection
- Environment Variables ช่วยจัดการ Configuration
- Cargo Audit ตรวจสอบ Security Advisories ของ Dependencies
- Logging และ Tracing ช่วยติดตาม Application
- Release Build ใช้เตรียม Binary สำหรับใช้งาน
- Cross Compilation สร้าง Binary สำหรับ Target อื่น
- Deployment ต้องตรวจสอบทั้ง Build, Security และ Reliability

## Completion Checklist

- [ ] เข้าใจ Input Validation
- [ ] เข้าใจ SQL Injection
- [ ] ใช้ Parameterized Queries ได้
- [ ] เข้าใจ Secrets Management
- [ ] ใช้ Environment Variables ได้
- [ ] แยก Development และ Production Config ได้
- [ ] ตรวจสอบ Dependencies ได้
- [ ] ใช้ Cargo Audit ได้
- [ ] เข้าใจ Unsafe Code Security
- [ ] ออกแบบ Error Handling ได้
- [ ] ใช้ Logging ได้
- [ ] ใช้ Tracing ได้
- [ ] สร้าง Release Build ได้
- [ ] เข้าใจ Cross Compilation
- [ ] เตรียม Deployment Checklist ได้
