extern crate fnv;
extern crate rayon;

use fnv::FnvHashMap;
use rayon::prelude::*;
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Clone)]
struct Scratchpad {
    vals: FnvHashMap<u32, u32>,
}

fn main() {
    let per_thread_state = Scratchpad { vals: Default::default() };
    let per_thread_state = Arc::new(per_thread_state);

    thread_local! {
        static STATE : RefCell<Option<Scratchpad>> = RefCell::new(None);
    }

    {
        let pts = per_thread_state.clone();
        let configuration = rayon::Configuration::new().start_handler(move |_| {
            STATE.with(|current_value| {
                assert!(current_value.borrow().is_none());
                *current_value.borrow_mut() = Some((*pts).clone());
            });
        });
        rayon::initialize(configuration);
    }
    let sum: u32 = vec![0; 100].par_iter().map(|&item| item).sum();
    println!("Sum is {}", sum);
}
