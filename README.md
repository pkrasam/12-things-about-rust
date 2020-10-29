# 12-things-about-rust
a simple program to quickly go over 12 things in rust

## Topics
1. println
2. mut
3. shadow
4. constants
5. types
6. strings
7. tuples
8. a
9. b
10. c
11. d
12. e


## Code & Output
**println**\
c:
```rust
fn main() {
    // println
    println!("println");   
    println!("Hello, 🦀");
    let mut x = 7;
    let y = 1;
    println!("x is {}, y is {}", x, y);
    println!("x is {valx}, y is {fred}", valx=x, fred=y);
    println!("debug {:?}", (3,4));
    println!("y is {1}, x is {0}", x , y);
}
```
o:
```
println
Hello, 🦀
x is 7, y is 1
x is 7, y is 1
debug (3, 4)
y is 1, x is 7
```

**mut**\
c:
```rust
fn main() {
    // mut
    println!("mut");
    x = 9;
    println!("x is {}", x);
}
```
o:
```
x is 9
```

**shadow**\
c:
```rust
fn main() {
    // shadow
    println!("shadow");
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y is {}", y);
    let abc = "pkr";
    let abc = abc.len();
    println!("abc is {}", abc);
}
```
o:
```
y is 12
abc is 3
```

**constants**\
c:
```rust
fn main() {
    // constants
    println!("constants");
    const FHD_WIDTH: u32 = 1920;
    const BAD_PI: f32 = 22.0/7.0;
    println!("FHD_WIDTH is {}, BAD_PI is {}", FHD_WIDTH, BAD_PI);
}
```
o:
```
FHD_WIDTH is 1920, BAD_PI is 3.142857
```

**types**\
c:
```rust
fn main() {
    // types
    println!("types");
    let x128: u128 = 0xFAFBFCFD_FEF1F2F3_F4F5F6F7_F8F9FAFB;
    let x64: i64 = 123456;
    let xx = 2.0;
    let yy: f32 = 3.0;
    println!("x128, i64, xx, yy is {}, {}, {}, {}", x128, x64, xx, yy);
    let c = 'c';
    let z = 'Z';
    let ferry = '🦀';
    let job_done = false;
    println!("c, z, ferry, job_done is {}, {}, {}, {}", c, z, ferry, job_done);
}
```
o:
```
x128, i64, xx, yy is 333615396748568137220584888834868247291, 123456, 2, 3
c, z, ferry, job_done is c, Z, 🦀, false
```

**strings**\
c:
```rust
fn main() {
    // strings
    println!("strings");
    let spock = "vulcan🖖";
    println!("spock length and is_empty is {}, {}", spock.len(), spock.is_empty());
    println!("bytes of spock is {:?}", spock.as_bytes());
    let mut hello = String::from("hello");
    hello.push('w');
    hello.push_str("orld!");
    println!("hello before is {}", hello);
    hello.insert(5,',');
    println!("hello after is {}", hello);
}
```
o:
```
strings
spock length and is_empty is 10, false
bytes of spock is [118, 117, 108, 99, 97, 110, 240, 159, 150, 150]
hello before is helloworld!
hello after is hello,world!
```

**tuples**\
c:
```rust
fn main() {
    // tuples
    println!("tuples");
    let tup1: (i32, f64, u8, f32) = (500, 6.4, 1, 29.29);
    let tup2 = (1500, 3.4);
    println!("tup1 and tup2 is {:?}, {:?}", tup1, tup2);
    let (a, b, c, d) = tup1;
    println!("a, b, c, d is {}, {}, {}, {}", a, b, c, d);
    let five_hundred = tup1.0;
    let six_point_four = tup1.1;
    let one = tup1.2;
    let twenty_nine_dot_twenty_nine = tup1.3;
    println!("extracting values from tuples {}, {}, {}, {}", five_hundred, six_point_four, one, twenty_nine_dot_twenty_nine);
}
```
o:
```
tuples
tup1 and tup2 is (500, 6.4, 1, 29.29), (1500, 3.4)
a, b, c, d is 500, 6.4, 1, 29.29
extracting values from tuples 500, 6.4, 1, 29.29
```

**n**\
c:
```rust
fn main() {
}
```
o:
```
```

**n**\
c:
```rust
fn main() {
}
```
o:
```
```

**n**\
c:
```rust
fn main() {
}
```
o:
```
```

**n**\
c:
```rust
fn main() {
}
```
o:
```
```

**n**\
c:
```rust
fn main() {
}
```
o:
```
```
