// IFB Fuzzer Main - Full LibAFL Integration with Coverage-Guided LLM
// Complete refactor to use StdFuzzer::fuzz_loop

use libafl::prelude::*;
use libafl::state::HasCorpus;
use libafl::{
    mutators::{
        scheduled::HavocScheduledMutator,
        havoc_mutations,
    },
    stages::StdMutationalStage,
};
use libafl_bolts::{
    current_nanos,
    rands::StdRand,
    tuples::tuple_list,
};
use std::path::PathBuf;

use ifb_fuzzer_core::harness;

#[cfg(feature = "llm")]
use ifb_fuzzer_core::mutators::neuro_mutator::LLMMutator;

use ifb_fuzzer_core::mutators::structure_aware_mutator::StructureAwareMutator;

// Coverage map size - AFL style (64KB)
const MAP_SIZE: usize = 65536;

// Global coverage map - will be populated by sanitizer coverage
// Note: With trace-pc-guard, the sanitizer will write to this map
// We need to ensure it's properly initialized and reset between runs
static mut COVERAGE_MAP: [u8; MAP_SIZE] = [0; MAP_SIZE];

// Target function for executor - this is what gets fuzzed
// In LibAFL 0.15, the harness function takes &ValueInput (immutable reference)
fn target_function(input: &ValueInput<Vec<u8>>) -> ExitKind {
    // NO resetear el coverage map aquí - el executor lo hace automáticamente
    // Si lo reseteamos aquí, perdemos el coverage antes de que el observer lo lea
    
    let bytes = input.sub_bytes(..);
    // Convert SubRangeSlice to &[u8] for harness::fuzz_iteration
    harness::fuzz_iteration(bytes.as_slice());
    
    // Note: The coverage map should be populated by sanitizer coverage instrumentation
    // The executor will reset it before the next execution
    ExitKind::Ok
}

fn main() -> Result<(), Error> {
    println!("[IFB] 🚀 Operation Cloud Breaker - Full LibAFL Coverage-Guided Fuzzer");
    println!("[IFB] 🎯 Coverage-guided fuzzing with LLM feedback for libcurl");
    println!("[IFB] 📊 Using StdFuzzer::fuzz_loop for maximum performance");

           // Create coverage map observer
           #[allow(static_mut_refs)]
           let mut coverage_map = unsafe {
               std::slice::from_raw_parts_mut(COVERAGE_MAP.as_mut_ptr(), MAP_SIZE)
           };
    let edges_observer = unsafe {
        StdMapObserver::new("edges", &mut coverage_map)
    };

    let time_observer = TimeObserver::new("time");

    // Feedback: MaxMapFeedback determines if an input is "interesting" based on coverage
    let mut feedback = MaxMapFeedback::new(&edges_observer);
    // Objective: CrashFeedback detects crashes (ExitKind::Crash)
    // This is separate from feedback and doesn't conflict with MaxMapFeedback metadata
    let mut objective = CrashFeedback::new();

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

    for (i, seed) in seeds.iter().enumerate() {
        let testcase = ValueInput::new(seed.clone()).into();
        let id = corpus.add(testcase)?;
        println!("[IFB] 📝 Added seed {} to corpus (ID: {:?}): {:?}", i+1, id, std::str::from_utf8(seed).unwrap_or("<invalid>"));
    }

    let final_count = corpus.count();
    println!("[IFB] 📚 Initial corpus: {} seeds (verificado: {})", seeds.len(), final_count);
    
    if final_count == 0 {
        eprintln!("[IFB] ❌ ERROR: Corpus está vacío después de agregar seeds!");
        return Err(Error::illegal_state("Corpus is empty after adding seeds"));
    }

    let solutions = InMemoryCorpus::new();
    let _crashes = OnDiskCorpus::<ValueInput<Vec<u8>>>::new(PathBuf::from("./crashes"))?;

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

    // Fuzzer: orchestrates the fuzzing loop
    // IMPORTANT: Fuzzer must be created BEFORE executor in LibAFL 0.15
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Executor: runs the target function and collects coverage
    // InProcessExecutor::new signature: (harness, observers, fuzzer, state, mgr)
    // Need to create a binding for target_function so it lives long enough
    let mut harness_fn = target_function;
    let mut executor = InProcessExecutor::new(
        &mut harness_fn,
        tuple_list!(edges_observer, time_observer),
        &mut fuzzer,
        &mut state,
        &mut mgr,
    )?;

    // Mutators: create variations of inputs
    // HavocScheduledMutator is the concrete type (ScheduledMutator is a trait)
    let havoc_mutator = HavocScheduledMutator::new(havoc_mutations());
    
    // Structure-Aware Mutator: parses URLs and mutates components intelligently
    let structure_aware_mutator = StructureAwareMutator::new();

    // Stages: sequence of mutation operations
    // StdMutationalStage is the concrete type (MutationalStage is a trait)
    #[cfg(feature = "llm")]
    let llm_mutator = LLMMutator::new();

    #[cfg(feature = "llm")]
    let mut stages = tuple_list!(
        StdMutationalStage::new(havoc_mutator),
        StdMutationalStage::new(structure_aware_mutator),
        StdMutationalStage::new(llm_mutator),
    );

    #[cfg(not(feature = "llm"))]
    let mut stages = tuple_list!(
        StdMutationalStage::new(havoc_mutator),
        StdMutationalStage::new(structure_aware_mutator),
    );

    // Start fuzzing loop - this is the main fuzzing engine
    println!("[IFB] 🏃 Starting coverage-guided fuzzing loop...");
    println!("[IFB] 💡 Inputs that increase coverage will be saved to corpus");
    #[cfg(feature = "llm")]
    println!("[IFB] 🧬 LLM mutator active - will evolve successful inputs");
    println!();

    // Verify corpus is not empty before starting
    let corpus_count = state.corpus().count();
    println!("[IFB] 🔍 Corpus count before fuzz_loop: {}", corpus_count);
    
    if corpus_count == 0 {
        eprintln!("[IFB] ❌ ERROR: Corpus is empty! Cannot start fuzzing.");
        return Err(Error::illegal_state("Corpus is empty"));
    }

    println!("[IFB] 🚀 Calling fuzz_loop...");
    println!("[IFB] 📊 Executor creado, stages configurados");
    println!("[IFB] 🔄 Iniciando loop de fuzzing...");
    println!("[IFB] ⚡ El fuzzer está corriendo. Presiona Ctrl+C para detener.");
    println!("[IFB] 📈 El SimpleMonitor mostrará estadísticas cada 15 segundos.");
    println!();
    
    // El fuzz_loop debería correr indefinidamente hasta que se encuentre un crash
    // o se presione Ctrl+C. El SimpleMonitor mostrará estadísticas periódicamente.
    // NOTA: Si termina inmediatamente, puede ser porque:
    // 1. El coverage map no se está actualizando (sanitizer coverage no funciona)
    // 2. El feedback no detecta diferencias (todos los inputs parecen iguales)
    // 3. El scheduler agota el corpus (no encuentra más inputs interesantes)
    
    // Agregar un panic hook para capturar cualquier panic silencioso
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("[IFB] 💥 PANIC capturado: {:?}", panic_info);
    }));
    
    println!("[IFB] 🔍 Antes de fuzz_loop - verificando estado...");
    println!("[IFB]    - Corpus: {} elementos", state.corpus().count());
    unsafe {
        let non_zero_before = COVERAGE_MAP.iter().filter(|&&x| x != 0).count();
        println!("[IFB]    - Coverage map antes: {} bytes no-cero", non_zero_before);
    }
    
    println!("[IFB] 🚀 Llamando a fuzz_loop (esto debería correr indefinidamente)...");
    let result = fuzzer.fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr);
    println!("[IFB] ⚠️  fuzz_loop retornó (esto es inesperado si no presionaste Ctrl+C)");
    
    // Diagnóstico después del loop
    let final_corpus_count = state.corpus().count();
    unsafe {
        let non_zero_count = COVERAGE_MAP.iter().filter(|&&x| x != 0).count();
        println!();
        println!("[IFB] 📊 Diagnóstico después de fuzz_loop:");
        println!("[IFB]    - Corpus final: {} elementos", final_corpus_count);
        println!("[IFB]    - Coverage map: {} bytes no-cero", non_zero_count);
    }
    
    match result {
        Ok(_) => {
            println!();
            println!("[IFB] ⚠️  fuzz_loop retornó Ok() - esto es inusual, debería correr indefinidamente");
            if final_corpus_count == 8 {
                println!("[IFB] 💡 El corpus no creció (sigue en 8 seeds)");
                println!("[IFB] 💡 Esto sugiere que el feedback no está detectando nuevos paths");
                println!("[IFB] 💡 Posible causa: el coverage map no se está actualizando");
                println!("[IFB] 💡 Verifica que libcurl esté compilado con -fsanitize-coverage=trace-pc-guard");
            } else {
                println!("[IFB] 💡 El corpus creció a {} elementos", final_corpus_count);
                println!("[IFB] 💡 El loop terminó por alguna razón desconocida");
            }
        }
        Err(e) => {
            eprintln!();
            eprintln!("[IFB] ❌ ERROR in fuzz_loop: {:?}", e);
            eprintln!("[IFB] 📋 Detalles del error: {}", e);
            return Err(e);
        }
    }
    
    println!("[IFB] 🏁 Fuzzer terminó");

    Ok(())
}
