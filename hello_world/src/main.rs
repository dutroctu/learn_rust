use std::mem;

fn main() {
    let a:u8 = 123;
    println!("a = {}", a);

    let mut b:i8 = 0;
    println!("b = {}", b);
    b= 42;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z:isize  = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, takes up {} bytes", d, mem::size_of_val(&d));    
    let e = 2.5;
    println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));    

    let g = true;
    println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));
    
    let f = 4>0;
    println!("f = {}, takes up {} bytes", f, mem::size_of_val(&f));
    
}
