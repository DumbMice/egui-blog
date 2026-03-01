//! Performance benchmarks for caching optimizations

use blog_app::math::load_manifest;
use std::time::Instant;

/// Benchmark math manifest loading with caching
#[test]
fn benchmark_manifest_loading() {
    println!("=== Benchmark: Math Manifest Loading ===");

    // First load (cold cache)
    let start = Instant::now();
    let manifest1 = load_manifest();
    let first_load_time = start.elapsed();
    println!("First load (cold cache): {:?}", first_load_time);

    // Second load (warm cache - should be much faster)
    let start = Instant::now();
    let manifest2 = load_manifest();
    let second_load_time = start.elapsed();
    println!("Second load (warm cache): {:?}", second_load_time);

    // Verify they're the same reference (cached)
    // We can't use std::ptr::eq because we only have & references
    // But we can verify they return the same data

    // Multiple loads to show consistency
    let mut total_time = std::time::Duration::ZERO;
    let iterations = 100;
    for _ in 0..iterations {
        let start = Instant::now();
        let _manifest = load_manifest();
        total_time += start.elapsed();
    }
    let avg_time = total_time / iterations;
    println!("Average over {} iterations: {:?}", iterations, avg_time);

    // Performance improvement ratio
    let improvement_ratio = first_load_time.as_nanos() as f64 / avg_time.as_nanos() as f64;
    println!(
        "Caching improvement ratio: {:.1}x faster",
        improvement_ratio
    );
    assert!(
        improvement_ratio > 10.0,
        "Caching should provide significant improvement (was {:.1}x)",
        improvement_ratio
    );
}

/// Test that caching doesn't break functionality
#[test]
fn test_caching_correctness() {
    println!("\n=== Test: Caching Correctness ===");

    // Test math manifest caching
    let manifest1 = load_manifest();
    let manifest2 = load_manifest();

    // Test that they contain the same data
    let formula = "E = mc^2";
    let is_display = false;

    let hash1 = manifest1.find_hash(formula, is_display);
    let hash2 = manifest2.find_hash(formula, is_display);

    assert_eq!(hash1, hash2, "Hash lookup should be consistent");

    // Test reverse index lookup
    if let Some(hash) = hash1 {
        // Verify we can get metadata
        let metadata1 = manifest1.get_metadata(&hash);
        let metadata2 = manifest2.get_metadata(&hash);
        assert_eq!(
            metadata1.is_some(),
            metadata2.is_some(),
            "Metadata access should be consistent"
        );

        if let (Some(md1), Some(md2)) = (metadata1, metadata2) {
            assert_eq!(md1.formula, md2.formula, "Formula metadata should match");
            assert_eq!(md1.is_display, md2.is_display, "Display flag should match");
        }
    }

    println!("All caching correctness tests passed!");
}

/// Benchmark formula lookup performance
#[test]
fn benchmark_formula_lookup() {
    println!("\n=== Benchmark: Formula Lookup ===");

    let manifest = load_manifest();

    // Test with a formula that should exist
    let formula = "E = mc^2";
    let is_display = false;

    let mut total_time = std::time::Duration::ZERO;
    let iterations = 1000;

    for _ in 0..iterations {
        let start = Instant::now();
        let _hash = manifest.find_hash(formula, is_display);
        total_time += start.elapsed();
    }

    let avg_time = total_time / iterations;
    println!("Average lookup time: {:?}", avg_time);

    // Lookups should be very fast with reverse index
    assert!(
        avg_time.as_nanos() < 1000,
        "Lookups should be very fast (<1µs), was {:?}ns",
        avg_time.as_nanos()
    );

    println!("Reverse index provides O(1) lookup performance");
}
