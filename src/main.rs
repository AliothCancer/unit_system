pub mod unit;

use lenght::*;
use temperature::*;
use unit::*;


fn main() {
    // make new meters or celsius f64
    let l1 = 2.0.meters();
    let c1 = 3.0.celsius();

    // meter and celsius for all numbers;
    let k1 = 3.celsius();
    let k2 = 8.celsius();

    // sum, divide, substract or multply either T or Meters<T>
    let l2 = l1.plus(3.0).plus(l1);
    let l2 = l2.mul(3.0).mul(l1);
    let c2 = c1.div(3.0).div(c1);
    let c2 = c2.minus(3.0).minus(c1);

    dbg!(vec![l1,l2]);
    dbg!(vec![c1,c2]);

}
