#[cfg(test)]
mod integration_tests {
    use crate::analyzer::*;
    use crate::types::*;
    use crate::scanner::*;
    use std::time::Duration;

    /// Integration test: End-to-end token analysis workflow
    #[tokio::test]
    async fn test_complete_analysis_workflow() {
        let metrics = create_realistic_metrics();

        // Test all strategies on same token
        let conservative = ConservativeStrategy::new();
        let ultra_early = UltraEarlySniper::new();
        let momentum = MomentumScalper::new();
        let graduation = GraduationAnticipator::new();

        let r1 = conservative.analyze(&metrics).unwrap();
        let r2 = ultra_early.analyze(&metrics).unwrap();
        let r3 = momentum.analyze(&metrics).unwrap();
        let r4 = graduation.analyze(&metrics).unwrap();

        println!("\n=== Integration Test: Complete Analysis ===");
        println!("Token: {}", metrics.symbol);
        println!("Bonding Curve: {:.1}%", metrics.bonding_curve_progress);
        println!("\nStrategy Results:");
        println!("  Conservative:  {:.1}% - {:?}", r1.confidence * 100.0, r1.signal_type);
        println!("  Ultra-Early:   {:.1}% - {:?}", r2.confidence * 100.0, r2.signal_type);
        println!("  Momentum:      {:.1}% - {:?}", r3.confidence * 100.0, r3.signal_type);
        println!("  Graduation:    {:.1}% - {:?}", r4.confidence * 100.0, r4.signal_type);

        // At least one should give a signal
        assert!(r1.confidence > 0.0 && r2.confidence > 0.0 && r3.confidence > 0.0 && r4.confidence > 0.0);
    }

    /// Integration test: Position lifecycle simulation
    #[test]
    fn test_position_lifecycle() {
        println!("\n=== Integration Test: Position Lifecycle ===");

        let metrics = create_realistic_metrics();
        let strategy = ConservativeStrategy::new();

        // 1. Analyze token
        let signal = strategy.analyze(&metrics).unwrap();
        println!("1. Analysis: {:.1}% confidence - {:?}", signal.confidence * 100.0, signal.signal_type);

        if signal.signal_type == SignalType::StrongBuy {
            // 2. Get exit parameters
            let exit_params = strategy.get_exit_params();
            println!("2. Entry approved");
            println!("   Take Profit: {}x", exit_params.take_profit_multiplier);
            println!("   Stop Loss: {:.0}%", exit_params.stop_loss_percentage * 100.0);
            println!("   Timeout: {}s", exit_params.position_timeout_seconds);

            // 3. Simulate position
            let entry_price = 1000000.0;
            let target_price = entry_price * exit_params.take_profit_multiplier;
            let stop_price = entry_price * (1.0 - exit_params.stop_loss_percentage);

            println!("3. Position opened");
            println!("   Entry: ${:.2}", entry_price);
            println!("   Target: ${:.2}", target_price);
            println!("   Stop: ${:.2}", stop_price);

            // 4. Simulate price movement (profit scenario)
            let exit_price = target_price;
            let pnl_pct = ((exit_price - entry_price) / entry_price) * 100.0;

            println!("4. Position closed");
            println!("   Exit: ${:.2}", exit_price);
            println!("   PnL: +{:.1}%", pnl_pct);

            assert!(pnl_pct > 0.0);
        }
    }

    /// Integration test: Multi-strategy comparison
    #[test]
    fn test_strategy_comparison() {
        println!("\n=== Integration Test: Strategy Comparison ===");

        let test_scenarios = vec![
            ("Very Early Token", create_ultra_early_metrics()),
            ("Mid Curve Token", create_mid_curve_metrics()),
            ("Near Graduation", create_graduation_metrics()),
            ("Explosive Momentum", create_momentum_metrics()),
        ];

        let strategies: Vec<(&str, Box<dyn TradingStrategy>)> = vec![
            ("Conservative", Box::new(ConservativeStrategy::new())),
            ("Ultra-Early", Box::new(UltraEarlySniper::new())),
            ("Momentum", Box::new(MomentumScalper::new())),
            ("Graduation", Box::new(GraduationAnticipator::new())),
        ];

        for (scenario_name, metrics) in test_scenarios {
            println!("\nScenario: {}", scenario_name);
            println!("  Curve: {:.1}%, Age: {}m, Price Change: {:.1}%",
                metrics.bonding_curve_progress,
                metrics.age_minutes,
                metrics.price_change_1h
            );

            let mut best_strategy = "";
            let mut best_confidence = 0.0;

            for (strategy_name, strategy) in &strategies {
                let result = strategy.analyze(&metrics).unwrap();
                println!("  {} -> {:.1}%", strategy_name, result.confidence * 100.0);

                if result.confidence > best_confidence {
                    best_confidence = result.confidence;
                    best_strategy = strategy_name;
                }
            }

            println!("  ✓ Best: {} ({:.1}%)", best_strategy, best_confidence * 100.0);
        }
    }

    /// Integration test: Risk management validation
    #[test]
    fn test_risk_management() {
        println!("\n=== Integration Test: Risk Management ===");

        let strategies: Vec<Box<dyn TradingStrategy>> = vec![
            Box::new(ConservativeStrategy::new()),
            Box::new(UltraEarlySniper::new()),
            Box::new(MomentumScalper::new()),
            Box::new(GraduationAnticipator::new()),
        ];

        for strategy in strategies {
            let params = strategy.get_exit_params();
            let name = strategy.name();

            println!("\n{}:", name);
            println!("  Take Profit: {}x ({:.0}% gain)",
                params.take_profit_multiplier,
                (params.take_profit_multiplier - 1.0) * 100.0
            );
            println!("  Stop Loss: {:.0}%", params.stop_loss_percentage * 100.0);
            println!("  Risk/Reward: {:.2}:1",
                (params.take_profit_multiplier - 1.0) / params.stop_loss_percentage
            );
            println!("  Max Hold: {}min", params.position_timeout_seconds / 60);

            // Verify risk management is sane
            assert!(params.take_profit_multiplier > 1.0);
            assert!(params.stop_loss_percentage > 0.0 && params.stop_loss_percentage < 1.0);
            assert!(params.position_timeout_seconds > 0);

            // Risk/reward should be at least 1:1
            let risk_reward = (params.take_profit_multiplier - 1.0) / params.stop_loss_percentage;
            assert!(risk_reward >= 1.0, "R:R ratio too low for {}", name);

            println!("  ✓ Risk management validated");
        }
    }

    /// Integration test: Edge case handling
    #[test]
    fn test_edge_case_handling() {
        println!("\n=== Integration Test: Edge Case Handling ===");

        let edge_cases = vec![
            ("All Zeros", create_zero_metrics()),
            ("Extreme High", create_extreme_high_metrics()),
            ("Extreme Low", create_extreme_low_metrics()),
            ("Mixed Signals", create_mixed_signal_metrics()),
        ];

        let strategy = ConservativeStrategy::new();

        for (case_name, metrics) in edge_cases {
            println!("\nCase: {}", case_name);

            match strategy.analyze(&metrics) {
                Ok(result) => {
                    println!("  ✓ Handled gracefully: {:.1}% confidence", result.confidence * 100.0);
                    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
                }
                Err(e) => {
                    println!("  ✗ Error: {:?}", e);
                    panic!("Should handle edge case gracefully");
                }
            }
        }
    }

    /// Integration test: Performance under load
    #[test]
    fn test_analysis_performance() {
        println!("\n=== Integration Test: Analysis Performance ===");

        let strategy = ConservativeStrategy::new();
        let metrics = create_realistic_metrics();

        let iterations = 1000;
        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let _ = strategy.analyze(&metrics).unwrap();
        }

        let duration = start.elapsed();
        let avg_time = duration.as_micros() / iterations;

        println!("Analyzed {} tokens in {:?}", iterations, duration);
        println!("Average time per token: {}μs", avg_time);
        println!("Throughput: {} tokens/sec", 1_000_000 / avg_time);

        // Should be able to analyze >1000 tokens per second
        assert!(avg_time < 1000, "Analysis too slow: {}μs", avg_time);
        println!("✓ Performance acceptable");
    }

    // Helper functions to create test metrics

    fn create_realistic_metrics() -> TokenMetrics {
        TokenMetrics {
            address: "RealisticToken123".to_string(),
            symbol: "REAL".to_string(),
            name: "Realistic Token".to_string(),
            volume_5m: 15.0,
            volume_1h: 60.0,
            volume_24h: 250.0,
            volume_acceleration: 2.5,
            unique_buyers_5m: 60,
            unique_sellers_5m: 25,
            buyer_seller_ratio: 2.4,
            liquidity_sol: 12.0,
            market_cap: 150000.0,
            holder_count: 120,
            top_10_holder_percent: 22.0,
            price_change_5m: 18.0,
            price_change_1h: 45.0,
            price_change_24h: 130.0,
            buy_pressure_ratio: 2.2,
            bonding_curve_progress: 55.0,
            is_graduated: false,
            age_minutes: 40,
            created_timestamp: 1000000,
        }
    }

    fn create_ultra_early_metrics() -> TokenMetrics {
        let mut m = create_realistic_metrics();
        m.age_minutes = 3;
        m.bonding_curve_progress = 6.0;
        m.buy_pressure_ratio = 9.0;
        m.volume_acceleration = 6.0;
        m
    }

    fn create_mid_curve_metrics() -> TokenMetrics {
        let mut m = create_realistic_metrics();
        m.bonding_curve_progress = 50.0;
        m.age_minutes = 60;
        m
    }

    fn create_graduation_metrics() -> TokenMetrics {
        let mut m = create_realistic_metrics();
        m.bonding_curve_progress = 78.0;
        m.liquidity_sol = 25.0;
        m.holder_count = 250;
        m.volume_24h = 150.0;
        m
    }

    fn create_momentum_metrics() -> TokenMetrics {
        let mut m = create_realistic_metrics();
        m.price_change_1h = 140.0;
        m.price_change_5m = 20.0;
        m.volume_5m = 50.0;
        m.buy_pressure_ratio = 3.5;
        m
    }

    fn create_zero_metrics() -> TokenMetrics {
        TokenMetrics {
            address: "ZeroToken".to_string(),
            symbol: "ZERO".to_string(),
            name: "Zero Token".to_string(),
            volume_5m: 0.0,
            volume_1h: 0.0,
            volume_24h: 0.0,
            volume_acceleration: 0.0,
            unique_buyers_5m: 0,
            unique_sellers_5m: 0,
            buyer_seller_ratio: 0.0,
            liquidity_sol: 0.0,
            market_cap: 0.0,
            holder_count: 0,
            top_10_holder_percent: 0.0,
            price_change_5m: 0.0,
            price_change_1h: 0.0,
            price_change_24h: 0.0,
            buy_pressure_ratio: 0.0,
            bonding_curve_progress: 0.0,
            is_graduated: false,
            age_minutes: 0,
            created_timestamp: 0,
        }
    }

    fn create_extreme_high_metrics() -> TokenMetrics {
        let mut m = create_realistic_metrics();
        m.volume_5m = 1000000.0;
        m.price_change_1h = 99999.0;
        m.buy_pressure_ratio = 999.0;
        m
    }

    fn create_extreme_low_metrics() -> TokenMetrics {
        let mut m = create_realistic_metrics();
        m.volume_5m = 0.01;
        m.liquidity_sol = 0.1;
        m.holder_count = 1;
        m
    }

    fn create_mixed_signal_metrics() -> TokenMetrics {
        let mut m = create_realistic_metrics();
        m.volume_5m = 50.0;  // Good
        m.liquidity_sol = 1.0;  // Bad
        m.holder_count = 200;  // Good
        m.top_10_holder_percent = 80.0;  // Bad
        m
    }
}
