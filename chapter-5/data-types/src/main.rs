fn main() {
    let a: f32 = 1728.144;
    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);
    println!("{:b}", frankentype);

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };

    println!("{}", b);
    assert_eq!(a, b);
}
