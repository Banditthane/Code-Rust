# Lesson 03: Testing and Production Readiness

## 1. Production Readiness คืออะไร?

Production Readiness คือการเตรียม Application ให้พร้อมสำหรับการใช้งานจริง

ไม่ได้หมายถึงแค่ Build ผ่าน แต่ต้องตรวจสอบหลายด้าน เช่น:

- Correctness
- Reliability
- Security
- Performance
- Observability
- Configuration
- Deployment
- Recovery

## 2. Testing Strategy

ควรวางแผน Testing หลายระดับ

### Unit Tests

ทดสอบ Function หรือ Module ขนาดเล็ก

ตัวอย่าง:

- Validation
- Business Rules
- Data Transformation
- Calculations

### Integration Tests

ทดสอบการทำงานร่วมกันระหว่าง Components

ตัวอย่าง:

- API กับ Database
- Service กับ Repository
- File Storage กับ Application

### End-to-End Tests

ทดสอบการทำงานตั้งแต่ต้นจนจบผ่านเส้นทางการใช้งานจริง

ตัวอย่าง:

1. User ส่ง Request
2. API ตรวจสอบข้อมูล
3. Service ประมวลผล
4. Database บันทึกข้อมูล
5. API ส่ง Response

## 3. Test Data Management

การจัดการ Test Data สำคัญมาก

แนวทาง:

- แยก Test Database
- สร้าง Test Data เฉพาะแต่ละ Test
- ล้างข้อมูลหลัง Test ตามความเหมาะสม
- ไม่ใช้ Production Data โดยไม่มีเหตุผลและมาตรการที่เหมาะสม
- ทำให้ Tests สามารถรันซ้ำได้
- ลดการพึ่งพาลำดับการรัน Tests

## 4. Error Handling

Production Application ต้องรองรับ Error ที่เกิดขึ้นจริง

ตัวอย่าง:

- Invalid Input
- Database Unavailable
- Network Timeout
- File Not Found
- Permission Denied
- Resource Not Found

แนวทาง:

1. แยก Error Types
2. กำหนดวิธีจัดการแต่ละ Error
3. กำหนด Retry เฉพาะกรณีที่เหมาะสม
4. ใช้ Timeout กับ Operations ที่อาจรอนาน
5. บันทึก Error ที่จำเป็น
6. แสดงข้อความให้ผู้ใช้เข้าใจ
7. ไม่เปิดเผย Internal Details

ควรระวังการ Retry Operations ที่ไม่เป็น Idempotent เพราะอาจทำให้เกิดข้อมูลซ้ำหรือผลข้างเคียงซ้ำได้

## 5. Security Review

ตรวจสอบหัวข้อ:

### Input Validation

- ตรวจสอบ Request Body
- ตรวจสอบ Query Parameters
- ตรวจสอบ File Uploads
- ตรวจสอบ Path Parameters

### Authentication

- Password Hashing
- Token Validation
- Session Expiration

### Authorization

- ตรวจสอบสิทธิ์ตาม Resource
- ป้องกันการเข้าถึงข้อมูลของผู้อื่น
- ตรวจสอบ Role และ Permission

### Database Security

- Parameterized Queries
- Least Privilege
- Secure Connection Configuration

### Secrets

- ไม่ Commit Secrets
- ใช้ Environment Variables หรือ Secret Manager
- ไม่บันทึก Secrets ลง Logs

## 6. Performance Review

ตรวจสอบ:

- Response Time
- Throughput
- Database Query Time
- Memory Usage
- CPU Usage
- Connection Pool
- Async Task Behavior

ควรใช้ Benchmark และ Profiling ตามลักษณะของปัญหา

อย่าปรับปรุง Performance โดยไม่มีข้อมูลรองรับ

## 7. Logging and Observability

Observability คือความสามารถในการทำความเข้าใจสถานะภายในของระบบจากข้อมูลที่ระบบแสดงออกมา

องค์ประกอบที่สำคัญ:

### Logs

บันทึกเหตุการณ์ที่เกิดขึ้น

### Metrics

วัดค่าการทำงาน เช่น:

- Request Count
- Error Count
- Request Duration
- CPU Usage
- Memory Usage

### Traces

ติดตามการทำงานข้าม Components หรือ Services

ควรหลีกเลี่ยงการบันทึกข้อมูลลับและข้อมูลส่วนบุคคลที่ไม่จำเป็น

## 8. Health Checks

Health Checks ช่วยตรวจสอบสถานะของ Application

ตัวอย่าง:

```text
GET /health
GET /ready
```

แนวคิด:

- Liveness: ตรวจสอบว่า Process ยังทำงานอยู่หรือไม่
- Readiness: ตรวจสอบว่า Application พร้อมรับ Traffic หรือไม่

Health Check ควรตอบสนองรวดเร็วและไม่สร้างภาระให้ระบบเกินจำเป็น

## 9. Configuration Management

ควรแยก Configuration ตาม Environment:

- Development
- Testing
- Staging
- Production

ตรวจสอบ:

- Database URL
- Port
- Log Level
- Timeout
- Feature Flags
- External Service URLs

ห้ามใช้ Secret ที่ฝังไว้ใน Source Code

## 10. Graceful Shutdown

Graceful Shutdown คือการปิด Application อย่างเป็นระเบียบ

แนวทาง:

1. หยุดรับงานใหม่
2. แจ้งให้ Tasks ที่กำลังทำงานจัดการต่อ
3. รอ Requests ที่กำลังดำเนินการตามเวลาที่กำหนด
4. ปิด Database Connections
5. Flush Logs ตามความเหมาะสม
6. จบ Process

ควรกำหนด Shutdown Timeout เพื่อไม่ให้ระบบรอไม่สิ้นสุด

## 11. Database Reliability

ตรวจสอบ:

- Connection Pool
- Query Timeouts
- Transactions
- Migrations
- Indexes
- Backup
- Restore

ควรทดสอบการ Restore ด้วย ไม่ใช่ตรวจสอบเฉพาะว่ามี Backup File เท่านั้น

## 12. Deployment Readiness

ก่อน Deploy:

- Tests ผ่าน
- Clippy ผ่าน
- Formatting ผ่าน
- Release Build ผ่าน
- Configuration พร้อม
- Database Migrations พร้อม
- Secrets พร้อม
- Logs ทำงาน
- Health Checks พร้อม
- Rollback Plan พร้อม

## 13. CI/CD

CI/CD ช่วยทำให้การตรวจสอบและส่งมอบ Software เป็นระบบ

ตัวอย่าง Pipeline:

```text
Push Code
    |
    v
Formatting Check
    |
    v
Clippy
    |
    v
Unit Tests
    |
    v
Integration Tests
    |
    v
Security Checks
    |
    v
Release Build
    |
    v
Deployment
```

Deployment ควรมี Approval และ Environment Protection ตามความเสี่ยงของระบบ

## 14. Production Readiness Checklist

### Reliability

- [ ] Error Handling
- [ ] Timeouts
- [ ] Graceful Shutdown
- [ ] Health Checks
- [ ] Backup and Restore

### Security

- [ ] Input Validation
- [ ] Authentication
- [ ] Authorization
- [ ] Dependency Audit
- [ ] Secrets Management

### Performance

- [ ] Benchmark
- [ ] Profiling
- [ ] Database Query Review
- [ ] Resource Limits

### Operations

- [ ] Logging
- [ ] Metrics
- [ ] Tracing
- [ ] Configuration
- [ ] Deployment Plan
- [ ] Rollback Plan

## 15. สรุป

Production Readiness ต้องตรวจสอบทั้งความถูกต้อง ความปลอดภัย ประสิทธิภาพ และการดูแลระบบหลัง Deploy

## Completion Checklist

- [ ] เข้าใจ Testing Strategy
- [ ] เขียน Unit Tests ได้
- [ ] เขียน Integration Tests ได้
- [ ] เข้าใจ End-to-End Tests
- [ ] จัดการ Test Data ได้
- [ ] ออกแบบ Error Handling ได้
- [ ] ทำ Security Review ได้
- [ ] ทำ Performance Review ได้
- [ ] เข้าใจ Logs, Metrics และ Traces
- [ ] ออกแบบ Health Checks ได้
- [ ] จัดการ Configuration ได้
- [ ] เข้าใจ Graceful Shutdown
- [ ] วางแผน Database Backup และ Restore ได้
- [ ] วางแผน CI/CD ได้
- [ ] สร้าง Production Readiness Checklist ได้
