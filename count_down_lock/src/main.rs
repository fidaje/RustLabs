// fn call_x(mut f: impl FnMut (), x: i32 ) {
//     for _ in 0..x{
//         f();
//     }
// }

// fn main(){

//     let mut a = 0;
//     let count_a = move || {a += 1; print!("{} ", a);};

//     a = 2;

//     call_x(count_a, 2);
//     call_x(count_a, 2);

//     println!("\t | a = {}", a);
//     call_x(count_a, 4);
//     println!("\t | a = {}", a);
// }

use std::cell::Cell;

fn funzione(a: &Box<Cell<i32>>) -> &Box<Cell<i32>>{

    let b = a.get();
    a.replace(b*2);
    &a


}



fn main(){
    let  b = Box::new(Cell::new(2));


    println!("{:?}", funzione(&b));
}