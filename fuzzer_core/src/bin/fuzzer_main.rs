// IFB Fuzzer Main (Broker/Workers)

use libafl::mutators::{StdHavocMutator, StdScheduledMutator};
use libafl::prelude::tuple_list;

#[cfg(feature = "llm")]
#[path = "../mutators/neuro_mutator.rs"]
mod neuro_mutator;

fn main() {
    // TODO: Initialize LibAFL broker/worker topology.
    // TODO: Wire harness + mutators + observers.

    #[cfg(feature = "llm")]
    let mutator_list = tuple_list!(
        StdHavocMutator::new(),
        neuro_mutator::LLMMutator::default()
    );

    #[cfg(not(feature = "llm"))]
    let mutator_list = tuple_list!(StdHavocMutator::new());

    let _scheduled = StdScheduledMutator::new(mutator_list);

    println!("[IFB] Fuzzer launcher template. Implement LibAFL workflow here.");
}
