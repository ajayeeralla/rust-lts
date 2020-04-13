use core::cmp::Ordering;
use std::cmp::PartialOrd;

#[derive(Eq, Debug, Default)]
pub struct LTSState<U> where U: std::cmp::Eq {
    pub state_id:u32,
    pub out:U
}

impl<U: std::cmp::Eq> PartialOrd for LTSState<U> {
    fn partial_cmp(&self, other: &LTSState<U>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<U: std::cmp::Eq> Ord for LTSState<U> {
    fn cmp(&self, other: &LTSState<U>) -> Ordering {
        self.state_id.cmp(&other.state_id)
    }
}

impl<U: std::cmp::Eq> PartialEq for LTSState<U> {
    fn eq(&self, other: &LTSState<U>) -> bool {
        self.state_id == other.state_id
    }
}




impl<U: std::default::Default +  std::cmp::Eq> LTSState<U> {
    pub fn new() -> Self {
        Self{
            state_id: 0,
            out: Default::default(),
        }
    }
    pub fn return_fields (self) -> (u32, U) {
    (self.state_id, self.out)
    }
    pub fn sort_field(&self) -> u32 {
        self.state_id
    }
}


#[allow(dead_code)]
#[derive(Eq, Debug, Default)]
pub struct Transition<U: std::cmp::Eq, W> {
    pub transition_from: LTSState<U>,
    pub transition_guard: W,
    pub transition_to: LTSState<U>,
}

pub fn build_transition< U: std::cmp::Eq, W> (transition_from: LTSState<U>, transition_guard: W, transition_to: LTSState<U>) ->
 Transition<U, W> {
    Transition {
        transition_from,
        transition_guard,
        transition_to
    }

}

impl<U: std::cmp::Eq, W: std::cmp::Eq> PartialOrd for Transition<U, W> {
    fn partial_cmp(&self, other: &Transition<U, W>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<U: std::cmp::Eq, W: std::cmp::Eq> Ord for Transition<U, W> {
    fn cmp(&self, other: &Transition<U, W>) -> Ordering {
        self.transition_from.cmp(&other.transition_from)
    }
}

impl<U: std::cmp::Eq, W: std::cmp::Eq> PartialEq for Transition<U, W> {
    fn eq(&self, other: &Transition<U, W>) -> bool {
        self.transition_from == other.transition_from && self.transition_to == other.transition_to
        && self.transition_guard == other.transition_guard
    }
}



pub fn return_largest<U: std::cmp::Eq> (list: &[LTSState<U>]) -> LTSState<U> {
     let mut largest = list[0];
     for item in list.iter(){
         if item.state_id > largest.state_id {
             largest = item;
         }
     }
     largest
}

pub fn check_trans_exist <U: std::cmp::Eq, W> (st: LTSState<U>, b: W, list: &[Transition<U,W>]) -> bool {
    let st_id = st.state_id;
    let item = list[0];

    item.transition_from.state_id == st_id

}
