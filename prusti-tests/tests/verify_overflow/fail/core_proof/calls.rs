// compile-flags: -Punsafe_core_proof=true -Psmt_quant_instantiations_bound=1000

use prusti_contracts::*;

struct T {
    f: u32,
}

struct G {
    g1: u32,
    g2: T,
}

fn callee(a: u32, b: T) -> G {
    G {
        g1: a,
        g2: b,
    }
}

fn test1(a: u32, b: T) -> G {
    callee(a, b)
}

fn test2(a: u32, b: T) {
    let x = callee(a, b);
    assert!(x.g1 == 0);     //~ ERROR: the asserted expression might not hold
}

fn main() {}
