#[allow(unused_variables)]
#[allow(unused_parens)]
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
    
    // mut
    println!("mut");
    x = 9;
    println!("x is {}", x);
    
    // shadow
    println!("shadow");
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y is {}", y);
    let abc = "pkr";
    let abc = abc.len();
    println!("abc is {}", abc);
    
    // constants
    println!("constants");
    const FHD_WIDTH: u32 = 1920;
    const BAD_PI: f32 = 22.0/7.0;
    println!("FHD_WIDTH is {}, BAD_PI is {}", FHD_WIDTH, BAD_PI);
    
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
    
    // arrays
    println!("arrays");
    let n = [1, 2, 3, 4, 5];
    let p: [u16; 5] = [6, 7, 8, 9, 10];
    let q = [0; 5];
    println!("n, p, q is {:?}, {:?}, {:?}", n, p, q);
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let first = n[0];
    let last = p[4];
    let nov = months[10];
    println!("first, last and nov is {}, {}, {}", first, last, nov);
    
    // expressions and statements
    println!("espressions and statements");
    let r = 3 + 7;
    let s = (4 + 6);
    let t = {5 + 5};
    let u = {
        let mut v = 3;
        v = v * 2;
        v + 1
    };
    println!("r, s, t, u is {}, {}, {}, {}", r, s, t, u);
    
    // functions
    println!("functions");
    let q = five();
    println!("q is {}", q);
    let r = plus_one(five());
    println!("r is {}", r);
    let oe5 = is_odd(5);
    let oe6 = is_odd(6);
    println!("odd 5? 6? {}, {}", oe5, oe6);
    let td: (f32, f64) = tuple_demo((4, 6, 3));
    println!("tuple demo is {:?}", td);
    println!("factorial is {}", factorial(5));

    // loop and while
    println!("loop and while");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("result is {}", result);
    counter = 3;
    while counter != 0 {
        println!("counter is {}", counter);
        counter -= 1;
    }

    // abc
    println!("n");
}

fn five() -> i32 {
    5
}

fn plus_one(s: i32) -> i32 {
    s + 1
}

fn is_odd(x: i32) -> bool {
    if(x & 1) == 0 {
        false
    } else {
        true
    }
}

fn tuple_demo(t: (u8,u16,u32)) -> (f32, f64) {
    let x:f32 = t.0 as f32 + t.1 as f32;
    let y:f64 = t.2 as f64;
    (x, y)
}

fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial (num - 1) * num,
    }
}
