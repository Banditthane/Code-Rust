# Lesson 01: Networking Fundamentals

## 1. Networking คืออะไร?

Networking คือการสื่อสารและแลกเปลี่ยนข้อมูลระหว่างอุปกรณ์หรือโปรแกรมผ่านเครือข่าย

ตัวอย่าง:

- Browser ติดต่อ Web Server
- Mobile App ติดต่อ Backend API
- โปรแกรม Chat ส่งข้อความหากัน
- โปรแกรมส่งไฟล์ผ่าน Network

องค์ประกอบพื้นฐาน:

| คำศัพท์    | ความหมาย                                 |
| ---------- | ---------------------------------------- |
| IP Address | หมายเลขระบุ Network Interface            |
| Port       | หมายเลขระบุปลายทางของโปรแกรมหรือ Service |
| Protocol   | กฎสำหรับการสื่อสาร                       |
| Client     | ฝั่งที่เริ่มส่ง Request                  |
| Server     | ฝั่งที่รอรับและประมวลผล Request          |
| Socket     | จุดเชื่อมต่อสำหรับการสื่อสารผ่าน Network |

## 2. IP Address

IP Address ใช้ระบุ Network Interface ภายในระบบเครือข่าย

ตัวอย่าง IPv4:

```text
127.0.0.1
192.168.1.10
8.8.8.8
```

`127.0.0.1` เป็น Loopback Address ซึ่งใช้สื่อสารกับเครื่องตัวเอง

IPv6 เป็นรูปแบบ Address อีกประเภทหนึ่ง เช่น:

```text
::1
2001:db8::1
```

## 3. Port

Port ใช้แยก Service หรือโปรแกรมที่ทำงานอยู่บนเครื่องเดียวกัน

ตัวอย่าง:

| Port | การใช้งานที่พบบ่อย            |
| ---- | ----------------------------- |
| 22   | SSH                           |
| 53   | DNS                           |
| 80   | HTTP                          |
| 443  | HTTPS                         |
| 3000 | Development Server ที่นิยมใช้ |
| 5432 | PostgreSQL ค่าเริ่มต้น        |

Port ไม่ได้ระบุโปรแกรมโดยตรง แต่ช่วยให้ระบบปฏิบัติการส่งข้อมูลไปยัง Socket ที่ถูกต้อง

## 4. TCP คืออะไร?

TCP (Transmission Control Protocol) เป็น Transport Protocol แบบ Connection-oriented

คุณสมบัติ:

- ต้องสร้าง Connection ก่อนส่งข้อมูล
- มีการจัดลำดับข้อมูล
- มีการตรวจจับข้อมูลที่สูญหายและส่งซ้ำ
- ให้ Application อ่านข้อมูลเป็น Byte Stream

ตัวอย่างการใช้งาน:

- Web Traffic
- File Transfer
- Database Connections
- SSH

## 5. UDP คืออะไร?

UDP (User Datagram Protocol) เป็น Transport Protocol แบบ Connectionless

คุณสมบัติ:

- ไม่ต้องสร้าง Connection แบบ TCP
- ส่งข้อมูลเป็น Datagram
- ไม่รับประกันการส่งถึง
- ไม่รับประกันลำดับการรับข้อมูล
- มี Overhead ต่ำกว่า TCP ในหลายกรณี

ตัวอย่างการใช้งาน:

- DNS
- Voice/Video Streaming บางประเภท
- Online Games บางประเภท

## 6. เปรียบเทียบ TCP และ UDP

| รายการ             | TCP                 | UDP                      |
| ------------------ | ------------------- | ------------------------ |
| รูปแบบ             | Connection-oriented | Connectionless           |
| การรับประกันส่งถึง | มีการจัดการส่งซ้ำ   | ไม่มีการรับประกัน        |
| การจัดลำดับ        | มี                  | ไม่มีการรับประกัน        |
| Data Format        | Byte Stream         | Datagram                 |
| Use Case           | Web, File Transfer  | DNS, Streaming บางประเภท |

## 7. Client และ Server

Client เป็นฝ่ายเริ่มการสื่อสาร ส่วน Server เป็นฝ่ายรอรับการเชื่อมต่อ

ตัวอย่าง:

```text
Client
   |
   | Connect
   v
Server
   |
   | Process Request
   v
Response
   |
   v
Client
```

ตัวอย่างใน Rust:

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Cannot bind address");

    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Client connected");
            }
            Err(error) => {
                eprintln!("Connection error: {}", error);
            }
        }
    }
}
```

ตัวอย่างนี้เปิด TCP Listener และรับการเชื่อมต่อ แต่ยังไม่ได้อ่านหรือส่งข้อมูล

## 8. TCP Client

Rust มี `TcpStream` สำหรับเชื่อมต่อ TCP Server

ตัวอย่าง:

```rust
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(_stream) => {
            println!("Connected to server");
        }
        Err(error) => {
            println!("Connection failed: {}", error);
        }
    }
}
```

ต้องเปิด Server ให้ทำงานก่อนจึงจะเชื่อมต่อได้

## 9. ส่งและรับข้อมูล

สามารถใช้ `Read` และ `Write` Trait เพื่ออ่านและเขียนข้อมูลผ่าน TCP Stream

ตัวอย่างแนวคิด:

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("Cannot connect");

    stream.write_all(b"Hello Server")
        .expect("Cannot write");

    let mut buffer = [0; 128];

    let size = stream.read(&mut buffer)
        .expect("Cannot read");

    println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
}
```

หมายเหตุ:

- `read()` อาจอ่านข้อมูลได้น้อยกว่าขนาด Buffer
- `read()` คืนค่า `0` เมื่อถึง EOF
- TCP เป็น Byte Stream จึงไม่ได้รักษาขอบเขตข้อความให้โดยอัตโนมัติ
- โปรแกรมจริงต้องออกแบบ Message Framing เช่น กำหนดขนาดข้อมูลหรือใช้ Delimiter

## 10. Networking Errors

Network Operation สามารถล้มเหลวได้ เช่น:

- Connection Refused
- Connection Timeout
- Address Already in Use
- Connection Reset
- Unexpected EOF

ควรจัดการด้วย `Result` แทนการสมมติว่า Network ทำงานสำเร็จเสมอ

## 11. สรุป

| API                    | หน้าที่              |
| ---------------------- | -------------------- |
| `TcpListener::bind()`  | เปิด TCP Server      |
| `TcpStream::connect()` | เชื่อมต่อ TCP Server |
| `Read::read()`         | อ่านข้อมูล           |
| `Write::write_all()`   | เขียนข้อมูลทั้งหมด   |
| `set_read_timeout()`   | กำหนด Read Timeout   |
| `set_write_timeout()`  | กำหนด Write Timeout  |

## Completion Checklist

- [ ] เข้าใจ Networking
- [ ] เข้าใจ IP Address
- [ ] เข้าใจ Port
- [ ] เข้าใจ Protocol
- [ ] เข้าใจ TCP
- [ ] เข้าใจ UDP
- [ ] เปรียบเทียบ TCP และ UDP ได้
- [ ] เข้าใจ Client และ Server
- [ ] ใช้ `TcpListener` ได้
- [ ] ใช้ `TcpStream` ได้
- [ ] ส่งและรับข้อมูลผ่าน TCP ได้
- [ ] เข้าใจ Network Errors
