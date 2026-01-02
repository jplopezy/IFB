use libafl::prelude::*;
use ifb_fuzzer_core::mutators::neuro_mutator::LLMMutator;

fn main() {
    println!("🎯 Demo: Evolutionary LLM Fuzzing");
    println!("==================================");

    let mut mutator = LLMMutator::new();

    // Test input that will be evolved
    let mut test_input = BytesInput::new(b"http://example.com".to_vec());

    println!("📝 Input inicial: {}", String::from_utf8_lossy(test_input.bytes()));
    println!("🔄 Generación 0 (sin metadata)\n");

    // Simular evolución a través de generaciones
    for generation in 1..=3 {
        println!("🧬 GENERACIÓN {} - Aplicando NeuroMutator", generation);

        // Crear un state mock
        let mut mock_state = NopState::new();

        // Aplicar mutación
        match mutator.mutate(&mut mock_state, &mut test_input, 0) {
            Ok(MutationResult::Mutated) => {
                println!("✅ Mutación exitosa!");
                println!("📤 Resultado: {}", String::from_utf8_lossy(test_input.bytes()));

                // Mostrar metadata si existe
                if let Some(metadata) = test_input.metadata::<ifb_fuzzer_core::mutators::neuro_mutator::LLMHistoryMetadata>() {
                    println!("📊 Metadata:");
                    println!("   • Prompt usado: {}", metadata.last_prompt);
                    println!("   • Generación: {}", metadata.generation);
                }
                println!();
            }
            Ok(MutationResult::Skipped) => {
                println!("⏭️  Mutación omitida (probabilidad)\n");
            }
            Err(e) => {
                println!("❌ Error en mutación: {}\n", e);
            }
        }

        // Pequeña pausa para simular tiempo de ejecución
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("🎉 Demo completada!");
    println!("💡 El NeuroMutator ahora evoluciona inputs exitosos de manera inteligente.");
}

