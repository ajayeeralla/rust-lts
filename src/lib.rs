pub mod lts;


#[cfg(test)]
mod tests {

    use crate::lts::*;
    use std::vec::*;

    const ST1: LTSState<char> = LTSState {state_id: 1, out: 'c'};
    const ST2: LTSState<char> = LTSState {state_id: 2, out: 'd'};
    const ST3: LTSState<char> = LTSState {state_id: 0, out: 'e'};

    //static mut _v:Vec<LTSState<char>> =  vec![ST2, ST3, ST1];

    #[test]
    fn test_return_fields(){
        assert_eq!((1, 'c'), LTSState::return_fields(ST1));
    }

    #[test]
    fn test_new(){
        assert_eq!(0, LTSState::<i32>::new().state_id);
    }

    #[test]
    fn test_sort_field(){
        assert_eq!(1, LTSState::sort_field(&ST1));
    }

    #[test]
    fn test_to_vec(){
        assert_eq!(vec![ST2, ST1, ST3], to_vec(&[ST2, ST1, ST3]));
    }


}
