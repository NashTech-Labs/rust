use std::cell::RefCell;

pub fn cacher(state: (i32,String)) -> Vec<i32> {
    let result = unsafe {
        pub static mut initial_cache: RefCell<HashMap<i32,String>> = RefCell::new(HashMap::new());
        initial_cache.borrow_mut().insert(state.1,state.2);
        initial_cache
    };
    result
}