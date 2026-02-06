// Regression test for issue #151815: ICE in ArgFolder when MIR contains
// closure types with generic_const_exprs inside.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Matrix<const R: usize, const C: usize>;

fn gen_mac<const K: usize>() -> Matrix<K, 1> {
    [0; K].map(|_| Matrix::<K, 1>)
    //~^ ERROR mismatched types
}

fn main() {
    let _ = gen_mac::<2>();
}

