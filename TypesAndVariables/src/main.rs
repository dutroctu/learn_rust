use std::mem;

fn operators()
{
    // arithmetic
    let mut a = 2+3*4;
    println!("{}", a);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed is {}, {}^pi = {}", b, b_cubed,b,b_to_pi);

    // bitwise
}

fn scope_and_shadowing()
{
    let a = 123;

}

fn main() {
    operators();
}
