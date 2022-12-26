use p1::p1a::p1a;
use p1::p1b::p1b;
use p2::p2b::p2b;
use p3::p3a::p3a;
use p3::p3b::p3b;
use p4::p4a::p4a;
use p4::p4b::p4b;
use p5::p5b::p5b;
use p6::p6a::p6a;
use p7::p7b::p7b;
use p8::p8a::p8a;
use p8::p8b::p8b;
// use p9::p9a::p9a;
use p9::p9b::p9b;

// pub mod debug;
pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;

fn main() {
    p1a().expect("hi");
    p1b().expect("hi");
    p2b().expect("hi");
    p3a().expect("hi");
    p3b().expect("hi");
    p4a().expect("hi");
    p4b().expect("hi");
    p5b().expect("hi");
    p6a().expect("hi");
    p7b().expect("hi");
    p8a().expect("msg");
    p8b().expect("msg");
    // p9a().expect("msg");
    p9b().expect("msg");
}
