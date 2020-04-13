

pub struct LTSState<T, U> {
    pub state_id:T,
    pub out:U
}

impl<T, U> LTSState<T, U> {
    pub fn origin<V, W> (self, other: LTSState<V, W>) -> LTSState<T, W> {
        LTSState {
            state_id: self.state_id,
            out: other.out }
    }
}

#[allow(dead_code)]

pub struct Transition<T, U> {
    pub transition_from:T,
    pub transition_guard:U,
    pub transition_to:T
}
