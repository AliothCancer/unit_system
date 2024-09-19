pub mod unit;

use lenght::*;
use temperature::*;
use unit::*;


fn main() {
    let l1 = 2.0.meters();
    let l2 = 2.0.meters();
    let l3 = 2.0.meters();
    let l4 = l1.plus(3.0).plus(l2);
    
    let t1 = 34.0.celsius().plus(3.0);
    let t2 = 3.0.celsius();
    let t3 = t1.plus(t2);

    dbg!(l1.plus(l3));
    dbg!(l1.plus(2.0));
    dbg!(l4);
}
