

 extern crate rust_lts;

use rust_lts::lts::*;


fn main() {
    let st1 = LTSState {state_id: 1, out: 3};
    let st2 = LTSState {state_id: 2, out: 10};
    let mut _v = vec![st2, st1];
    _v.sort();
    assert_eq!(_v, vec![st1, st2]);
    println!("st1.id = {}, st1.out = {}", st1.state_id, st1.out);
    println!("st4.id = {}, st4.out = {}", st2.state_id, st2.out);
}
