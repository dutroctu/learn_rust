use std::mem;
mod pm;

struct Point
{
    x:f64,
    y:f64
}

enum Color {
    Red,
    Green,
    Blue
}

fn enums()
{
    let c:Color = Color::Red;

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
    }
}

fn structures()
{
    let p = Point { x: 0.0, y: 0.0 };
    println!("point p is at ({}, {})", p.x, p.y);
}

fn option()
{
    let x = 3.0;
    let y = 2.0;
    let result:Option<f64> =
    if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result);

    match result 
    {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y)
    }

    // if let / while let
    if let Some(z) = result { println!("z = {}", z);}
}

fn array()
{
    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;

    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a == [321,2,3,4,5]
    {
        println!(" match");
    }

    let b = [1u16;10];

    for i in 0..b.len()
    {
        println!("{}", b[i]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn vectors()
{
    let mut a = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);    
    a.push(44);
    println!("a = {:?}", a);

    let idx:usize = 0;
    a[idx] = 312;
    println!("a[0] = {:?}", a[idx]);    

    // Option
    match a.get(6)
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x); }

    a.push(77);
    println!("a = {:?}", a);

    let last_element = a.pop(); //Option
    println!("last element is {:?}, a = {:?}", last_element, a);

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}

fn use_slice(slice: &mut[i32]) 
{
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices()
{
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    // use_slice(&mut data)
    println!("{:?}", data);
}

fn strings()
{
    let s:&'static str = "hello there!"; // &str = string slice

    for c in s.chars().rev()
    {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}", first_char);
    }

    // String
    let mut letters = String::new();
    let mut a  = 'a' as u8;
    while a<=('z' as u8)
    {
        letters.push(a as char);
        // letters.push(',' as char);
        letters.push_str(",");
        a+=1;
    }
    println!("{}", letters);

    let u:&str = &letters;

    // concatentation
    // String + str
    // let z = letters + &letters;
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbuy"));
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}

fn tuples()
{
    let x= 3;
    let y=4;
    let sp = sum_and_product(x,y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x,y,sp.0, sp.1);

    // destructing
    let (a,b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last element = {}", (combined.1).1 );

    let ( (c,d), (e,f) ) = combined;

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let meaning = (42,);
    println!("{:?}", meaning);
}

struct PointT<T> {
    x: T,
    y: T
}
struct Line<T> {
    start: PointT<T>,
    end: PointT<T>
}

fn generics()
{
    let a:PointT<f64> = PointT{x:0.0, y:4f64};
    let b = PointT{x:1.2, y:3.4};

    let myline  = Line {start: a, end: b};
}

fn main() {
    generics();
    // pm::pattern_matching();
    // tuples();
    // strings();
    // slices();
    // structures();
    // enums();
    // option();
    // array();
    // vectors();
}
