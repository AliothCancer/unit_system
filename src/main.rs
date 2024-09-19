pub mod unit;

use lenght::*;
use temperature::*;
use unit::*;


fn main() {
    let l1 = 5.0_f64.meters();
    let l2 = 5.0.meters();
    let l3 = 5.0.meters();
    

    assert_eq!(Meters(10.0), l1.plus(l2).plus(l3));
    assert_eq!(Meters(25.0), l1.plus(2.0));
}
