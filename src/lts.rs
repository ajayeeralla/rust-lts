use core::cmp::Ordering;
use std::cmp::PartialOrd;
//extern crate rand;
//use rand::*;

#[allow(dead_code)]

#[derive(Eq, Debug, Default, Copy, Clone)]
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
#[derive(Eq, Debug, Default, Copy, Clone)]
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

pub fn largest<T: std::cmp::Eq>(list: &[LTSState<T>]) -> &LTSState<T> {
    let mut largest = &list[0];
    for item in list.iter(){
        if item.state_id > largest.state_id {
            largest = &item;
        }
    }
    largest
}

pub fn trans_exists <U: std::cmp::Eq, W: std::cmp::Eq> (st: LTSState<U>, b: W, list: &[Transition<U,W>]) -> bool {
    for item in list.iter(){
        if item.transition_from == st && b == item.transition_guard {
            return true;
        }
    }
   false
}

pub fn find_trans_index <U: std::cmp::Eq, W: std::cmp::Eq> (st: LTSState<U>, b: W, list: &[Transition<U,W>]) -> usize {

        list.iter().position(|x| x.transition_from == st && b == x.transition_guard ).unwrap()

}

pub fn check_trans <U: std::cmp::Eq, W> (st: LTSState<U>, list: &[Transition<U,W>]) -> bool {
    for item in list.iter(){
        if item.transition_from != st {
            return false;
        }
    }
    true
}

pub fn get_from_ids<U: std::cmp::Eq, W> (list: &[Transition<U,W>]) -> Vec<u32> {
    let mut v : Vec<u32> =vec![];
    for item in list.iter(){
        v.push(item.transition_from.state_id);
    }
    v
}

pub fn get_to_ids<U: std::cmp::Eq, W> (list: &[Transition<U,W>]) -> Vec<u32> {
    let mut v : Vec<u32> =vec![];
    for item in list.iter(){
        v.push(item.transition_to.state_id);
    }
    v
}

#[inline]
pub fn to_vec<U> (sl: &[LTSState<U>]) -> Vec<LTSState<U>>
where
U: std::cmp::Eq + Clone,
{
let mut v = Vec::with_capacity(sl.len());
v.extend_from_slice(sl);
v
}

/*pub fn nextStateSymbol <U: std::cmp::Eq + std:: default :: Default, W: Copy +  std::cmp::Eq> (st: LTSState<U>, b: W, list: &[Transition<U,W>]) -> Option <&LTSState<U>> {
    if trans_exists(st, b, list)
    { Some(&list[find_trans_index(st, b, list)].transition_to) }
    else { Some(&LTSState::<U>::new()) }
}*/
