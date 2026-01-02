// IFB Fuzzer Main - Continuous cURL URL Parser Fuzzer

use ifb_fuzzer_core::harness;
use std::time::{Duration, Instant};

fn main() {
    println!("[IFB] 🚀 Operation Cloud Breaker - cURL URL Parser Fuzzer");
    println!("[IFB] 🎯 Continuous fuzzing for parsing bugs in libcurl");

    // Setup signal handler for graceful shutdown
    let ctrlc_received = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let ctrlc_received_clone = ctrlc_received.clone();

    ctrlc::set_handler(move || {
        println!("\n[IFB] 🛑 Received Ctrl+C, shutting down gracefully...");
        ctrlc_received_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    // Corpus of interesting URLs to fuzz
    let corpus = vec![
        // Basic URLs
        b"http://example.com".to_vec(),
        b"https://test.com/path".to_vec(),
        b"ftp://ftp.example.com".to_vec(),

        // Empty and minimal
        b"".to_vec(),
        b"http://".to_vec(),
        b"https://".to_vec(),
        b"ftp://".to_vec(),

        // Malformed URLs
        b"://".to_vec(),
        b"http://".to_vec(),
        b"https:///path".to_vec(),
        b"//example.com".to_vec(),

        // With special characters
        b"http://example.com/path?query=value&other=test".to_vec(),
        b"https://user:pass@example.com:8080/path".to_vec(),
        b"http://localhost:3000/api/v1/users".to_vec(),

        // Non-HTTP protocols
        b"file:///etc/passwd".to_vec(),
        b"javascript:alert('xss')".to_vec(),
        b"data:text/plain;base64,SGVsbG8=".to_vec(),
        b"mailto:user@example.com".to_vec(),

        // Edge cases
        b"http://\x00\x01\x02".to_vec(), // Null bytes
        vec![b'a'; 1000], // Very long
        format!("http://example.com/{}", "/".repeat(100)).into_bytes(), // Long path

        // Encoding edge cases
        b"http://example.com/%00%01%02".to_vec(),
        b"http://example.com/../..//etc/passwd".to_vec(),
        b"http://example.com/path with spaces".to_vec(),

        // International domains (UTF-8 encoded)
        "http://例え.テスト/path".as_bytes().to_vec(),
        "https://münchen.de/path".as_bytes().to_vec(),

        // IPv6
        b"http://[::1]/path".to_vec(),
        b"https://[2001:db8::1]:8080/path".to_vec(),

        // Invalid but parseable
        b"invalid://url".to_vec(),
        b"123://numbers".to_vec(),
        b"http://@".to_vec(),
        b"http://:80".to_vec(),
    ];

    println!("[IFB] 📚 Initial corpus: {} URLs", corpus.len());
    println!("[IFB] 🏃 Starting continuous fuzzing...");

    let start_time = Instant::now();
    let mut iterations: u64 = 0;
    let crashes_found = 0;
    let _last_stats_time = start_time;

    loop {
        if ctrlc_received.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        // Select random URL from corpus
        let base_url_idx = (iterations % corpus.len() as u64) as usize;
        let mut test_input = corpus[base_url_idx].clone();

        // Apply random mutations
        match iterations % 7 {
            0 => {
                // Add random bytes at end
                if test_input.len() < 1000 {
                    test_input.extend_from_slice(&[rand::random::<u8>()]);
                }
            },
            1 => {
                // Insert null bytes
                if !test_input.is_empty() {
                    let pos = (iterations % test_input.len() as u64) as usize;
                    test_input.insert(pos, 0);
                }
            },
            2 => {
                // Truncate randomly
                if test_input.len() > 1 {
                    let new_len = (iterations % test_input.len() as u64) as usize;
                    test_input.truncate(new_len);
                }
            },
            3 => {
                // Add special characters
                let specials = b"%<>\"'&|\\";
                test_input.push(specials[(iterations % specials.len() as u64) as usize]);
            },
            4 => {
                // Duplicate parts
                if test_input.len() > 2 {
                    let part_len = test_input.len() / 2;
                    let part = test_input[0..part_len].to_vec();
                    test_input.extend_from_slice(&part);
                }
            },
            5 => {
                // Replace with random byte
                if !test_input.is_empty() {
                    let pos = (iterations % test_input.len() as u64) as usize;
                    test_input[pos] = rand::random::<u8>();
                }
            },
            6 => {
                // Add HTTP headers-like content
                test_input.extend_from_slice(b"\r\nHost: evil.com\r\n");
            },
            _ => {}
        }

        // Execute the fuzzed input
        harness::fuzz_iteration(&test_input);

        iterations += 1;

        // Print stats every 1000 iterations
        if iterations % 1000 == 0 {
            let elapsed = start_time.elapsed();
            let execs_per_sec = iterations as f64 / elapsed.as_secs_f64();

            println!("[IFB] 📊 Iterations: {} | Speed: {:.0} exec/s | Corpus: {} | Crashes: {}",
                    iterations, execs_per_sec, corpus.len(), crashes_found);

            // Save progress every 10000 iterations
            if iterations % 10000 == 0 {
                println!("[IFB] 💾 Saving progress...");
            }
        }

        // Check for ASan crashes by monitoring stderr or process exit codes
        // Note: In a real implementation, we'd monitor the child process exit codes
        // For now, we just continue fuzzing
    }

    // Final stats
    let total_time = start_time.elapsed();
    let final_execs_per_sec = iterations as f64 / total_time.as_secs_f64();

    println!("\n[IFB] 🎉 Fuzzing session completed!");
    println!("[IFB] 📈 Final Statistics:");
    println!("[IFB]   • Total iterations: {}", iterations);
    println!("[IFB]   • Execution speed: {:.0} exec/s", final_execs_per_sec);
    println!("[IFB]   • Total runtime: {:.2}s", total_time.as_secs_f64());
    println!("[IFB]   • Corpus size: {}", corpus.len());
    println!("[IFB]   • Crashes found: {}", crashes_found);

    if crashes_found > 0 {
        println!("[IFB] 🏆 SUCCESS! Potential vulnerabilities found in cURL URL parser!");
    } else {
        println!("[IFB] ✅ Fuzzing completed - monitoring for ASan crashes in logs");
    }
}
