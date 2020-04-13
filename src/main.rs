
mod lts;



fn main() {
    let st1 = lts::LTSState {state_id: 1, out: 3.0};
    let st2 = lts::LTSState {state_id: "Hello", out: 'c'};
    let st3 = st1.origin(st2);
    println!("st1.id = {}, st1.out = {}", st3.state_id, st3.out);
    println!("Hello, world!");
}
