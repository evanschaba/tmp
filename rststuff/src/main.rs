use rand::Rng;

#[tokio::main]
async fn main() {
    let x = rand::thread_rng().gen_range::<u8, _>(u8::MIN..=u8::MAX);
    let y = rand::thread_rng().gen_range::<u8, _>(u8::MIN..=u8::MAX);

    println!("{x} + {y} = {}", x.wrapping_add(y));
    println!("{x} * {y} = {}", x.wrapping_mul(y));
    println!("{x} / {y} = {}", x.wrapping_div(if y == 0 { 1 } else { y })); // Prevents division by zero
    println!("{x} - {y} = {}", x.wrapping_sub(y));
}
