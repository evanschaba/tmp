use std::mem::size_of_val;

fn bar<T>() {  }

fn main() {
   let u_32 = size_of_val(&bar::<u32>);
   println!("u_32: size_of_val({u_32})");

   let i_32 = size_of_val(&bar::<i32>);
   println!("u_32: size_of_val({i_32})");


}


