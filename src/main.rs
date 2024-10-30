mod prime_finder;
use prime_finder::MersenePrimeFinder;
mod errors;
pub use errors::*;

use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let mut finder = MersenePrimeFinder::new(3);
    //finder.set_max_exponent(7);

    for prime in finder {
        println!("Prime: {prime}");
    }
}
