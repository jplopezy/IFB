// IFB Fuzzer Main - Full LibAFL Integration with Coverage-Guided LLM
// Complete refactor to use StdFuzzer::fuzz_loop

use libafl::prelude::*;
use libafl_bolts::{
    current_nanos,
    rands::StdRand,
    tuples::tuple_list,
};
use std::path::PathBuf;

use ifb_fuzzer_core::harness;

#[cfg(feature = "llm")]
use ifb_fuzzer_core::mutators::neuro_mutator::LLMMutator;

// Coverage map size - AFL style (64KB)
const MAP_SIZE: usize = 65536;

// Global coverage map - will be populated by sanitizer coverage
static mut COVERAGE_MAP: [u8; MAP_SIZE] = [0; MAP_SIZE];

// Target function for executor - this is what gets fuzzed
// In LibAFL 0.15, the harness function takes &ValueInput (immutable reference)
fn target_function(input: &ValueInput<Vec<u8>>) -> ExitKind {
    let bytes = input.sub_bytes(..);
    // Convert SubRangeSlice to &[u8] for harness::fuzz_iteration
    harness::fuzz_iteration(bytes.as_slice());
    ExitKind::Ok
}

fn main() -> Result<(), Error> {
    println!("[IFB] 🚀 Operation Cloud Breaker - Full LibAFL Coverage-Guided Fuzzer");
    println!("[IFB] 🎯 Coverage-guided fuzzing with LLM feedback for libcurl");
    println!("[IFB] 📊 Using StdFuzzer::fuzz_loop for maximum performance");

    // Create coverage map observer
    let mut coverage_map = unsafe {
        std::slice::from_raw_parts_mut(COVERAGE_MAP.as_mut_ptr(), MAP_SIZE)
    };
    let edges_observer = StdMapObserver::new("edges", &mut coverage_map);

    let time_observer = TimeObserver::new("time");

    // Feedback: MaxMapFeedback determines if an input is "interesting" based on coverage
    let mut feedback = MaxMapFeedback::new(&edges_observer);
    let mut objective = MaxMapFeedback::new(&edges_observer);

    // Scheduler: QueueScheduler selects inputs from corpus
    let scheduler = QueueScheduler::new();

    // Corpus: store interesting inputs
    let corpus_dir = PathBuf::from("./corpus");
    std::fs::create_dir_all(&corpus_dir).ok();

    let mut corpus = InMemoryCorpus::<ValueInput<Vec<u8>>>::new();
    
    // Initial seeds - starting points for fuzzing
    let seeds = vec![
        b"http://example.com".to_vec(),
        b"https://test.com/path".to_vec(),
        b"".to_vec(),
        b"http://".to_vec(),
        b"https://".to_vec(),
        b"ftp://ftp.example.com".to_vec(),
        b"://".to_vec(),
        b"https:///path".to_vec(),
    ];

    for seed in seeds {
        corpus.add(ValueInput::new(seed).into())?;
    }

    println!("[IFB] 📚 Initial corpus: {} seeds", corpus.count());

    let solutions = InMemoryCorpus::new();
    let _crashes = OnDiskCorpus::new(PathBuf::from("./crashes"))?;

    // State: manages corpus, rand, feedback state
    // StdState::new signature: (rand, corpus, solutions, &mut feedback, &mut objective)
    let rand = StdRand::with_seed(current_nanos());
    let mut state = StdState::new(
        rand,
        corpus,
        solutions,
        &mut feedback,
        &mut objective,
    )?;

    // Monitor: displays progress
    let monitor = SimpleMonitor::new(|s| println!("{}", s));

    // Event Manager: handles events during fuzzing
    let mut mgr = SimpleEventManager::new(monitor);

    // Executor: runs the target function and collects coverage
    // InProcessExecutor::new signature: (harness, observers, state, mgr, objective)
    // The 5th argument appears to be &mut objective based on the generic <OF>
    let mut executor = InProcessExecutor::new(
        &mut target_function,
        tuple_list!(edges_observer, time_observer),
        &mut state,
        &mut mgr,
        &mut objective,
    )?;

    // Mutators: create variations of inputs
    let havoc_mutator = ScheduledMutator::new(havoc_mutations());

    // Stages: sequence of mutation operations
    #[cfg(feature = "llm")]
    let llm_mutator = LLMMutator::new();

    #[cfg(feature = "llm")]
    let mut stages = tuple_list!(
        MutationalStage::new(havoc_mutator),
        MutationalStage::new(llm_mutator),
    );

    #[cfg(not(feature = "llm"))]
    let mut stages = tuple_list!(
        MutationalStage::new(havoc_mutator),
    );

    // Fuzzer: orchestrates the fuzzing loop
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Start fuzzing loop - this is the main fuzzing engine
    println!("[IFB] 🏃 Starting coverage-guided fuzzing loop...");
    println!("[IFB] 💡 Inputs that increase coverage will be saved to corpus");
    #[cfg(feature = "llm")]
    println!("[IFB] 🧬 LLM mutator active - will evolve successful inputs");
    println!();

    fuzzer.fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)?;

    Ok(())
}
