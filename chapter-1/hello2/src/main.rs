fn greet_world() {
    println!("Hello, world!");
    let netherlands = "Hallo wereld!";
    let japan = "こんにちは世界！";
    let regions = [netherlands, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
fn main() {
    greet_world();
}
