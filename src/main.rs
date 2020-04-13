
mod lts;



fn main() {
    let st1 = lts::LTSState {state_id: 1, out: 3};
    let st2 = lts::LTSState {state_id: 2, out: 'c'};
    println!("st1.id = {}, st1.out = {}", st1.state_id, st1.out);
    println!("st4.id = {}, st4.out = {}", st2.state_id, st2.out);
}
