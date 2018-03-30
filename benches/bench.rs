#![feature(test)]

extern crate fral;
extern crate im;
extern crate rand;
extern crate test;

use fral::Fral;
use im::{ConsList, List};
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use test::Bencher;

macro_rules! matrix_cons {
    ($name:ident, $new:expr, $cons:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = rand::IsaacRng::new_unseeded();
            b.iter(|| {
                let mut v = $new;
                for n in (0..2048).map(|_| rng.gen::<u8>() as usize) {
                    v = v.$cons(n)
                }
            })
        }
    };
}

macro_rules! matrix_uncons {
    ($name:ident, $new:expr, $cons:ident, $uncons:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = rand::IsaacRng::new_unseeded();
            let mut v = $new;
            for n in (0..2048).map(|_| rng.gen::<u8>() as usize) {
                v = v.$cons(n)
            }
            let reset = v.clone();
            b.iter(|| {
                let mut _total = 0;
                if let Some((car, cdr)) = v.uncons() {
                    _total += *car;
                    v = cdr;
                } else {
                    v = reset.clone();
                }
            })
        }
    };
}

macro_rules! matrix_get {
    ($name:ident, $new:expr, $cons:ident, $get:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = rand::IsaacRng::new_unseeded();
            let between = Range::new(0, 2048);
            let mut v = $new;
            for n in (0..2048).map(|_| rng.gen::<u8>() as usize) {
                v = v.$cons(n)
            }
            b.iter(|| {
                let mut _total = 0;
                for n in (0..1000).map(|_| between.ind_sample(&mut rng)) {
                    _total += ($get)(&v, n)
                }
            })
        }
    };
}

macro_rules! matrix {
    (
        $consname:ident,
        $unconsname:ident,
        $getname:ident,
        $new:expr,
        $cons:ident,
        $uncons:ident,
        $get:expr
    ) => {
        matrix_cons!($consname, $new, $cons);
        matrix_uncons!($unconsname, $new, $cons, $uncons);
        matrix_get!($getname, $new, $cons, $get);
    };
}

matrix!(
    im_list_cons,
    im_list_uncons,
    im_list_get,
    List::new(),
    cons,
    uncons,
    |x: &List<_>, n| *x.iter().nth(n).unwrap()
);
matrix!(
    im_conslist_cons,
    im_conslist_uncons,
    im_conslist_get,
    ConsList::new(),
    cons,
    uncons,
    |x: &ConsList<_>, n| *x.iter().nth(n).unwrap()
);
matrix!(
    fral_cons,
    fral_uncons,
    fral_get,
    Fral::new(),
    cons,
    uncons,
    |x: &Fral<_>, n| *x.get(n).unwrap()
);
