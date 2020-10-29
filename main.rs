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
    
    // abc
    println!("n");
}
