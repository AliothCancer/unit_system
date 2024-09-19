pub mod lenght;
pub mod temperature;
pub mod unit;

use lenght::*;
use temperature::*;
use unit::*;

fn main() {
    let l1 = (5.0).meters();
    let l2 = (5.0).meters();
    let l3 = l1.plus(l2);

    let t1 = 34.5.celsius();
    let t2 = 46.5.celsius();
    let t3 = t1.plus(t2);

    println!("{:?}",l3 );
    println!("{:?}",t3 );
}