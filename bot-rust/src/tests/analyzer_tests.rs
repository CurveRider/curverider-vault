#[cfg(test)]
mod tests {
    use crate::analyzer::*;
    use crate::types::*;

    fn create_test_metrics() -> TokenMetrics {
        TokenMetrics {
            address: "TestToken123".to_string(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            // Volume metrics
            volume_5m: 10.0,
            volume_1h: 50.0,
            volume_24h: 200.0,
            volume_acceleration: 2.0,
            unique_buyers_5m: 50,
            unique_sellers_5m: 20,
            buyer_seller_ratio: 2.5,
            // Liquidity metrics
            liquidity_sol: 10.0,
            market_cap: 100000.0,
            // Holder metrics
            holder_count: 100,
            top_10_holder_percent: 25.0,
            // Momentum metrics
            price_change_5m: 15.0,
            price_change_1h: 40.0,
            price_change_24h: 120.0,
            // Buy pressure
            buy_pressure_ratio: 2.0,
            // Bonding curve
            bonding_curve_progress: 50.0,
            is_graduated: false,
            // Metadata
            age_minutes: 30,
            created_timestamp: 1000000,
        }
    }

    #[test]
    fn test_conservative_strategy_good_signal() {
        let strategy = ConservativeStrategy::new();
        let mut metrics = create_test_metrics();

        // Set metrics for conservative strategy sweet spot
        metrics.bonding_curve_progress = 50.0;
        metrics.volume_5m = 25.0;
        metrics.liquidity_sol = 10.0;
        metrics.holder_count = 150;
        metrics.top_10_holder_percent = 20.0;
        metrics.price_change_1h = 45.0;
        metrics.buy_pressure_ratio = 2.5;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence > 0.80);
        assert_eq!(result.signal_type, SignalType::StrongBuy);
        println!("Conservative confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_conservative_strategy_rejects_low_liquidity() {
        let strategy = ConservativeStrategy::new();
        let mut metrics = create_test_metrics();

        metrics.liquidity_sol = 2.0; // Too low
        metrics.bonding_curve_progress = 50.0;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence < 0.80);
        assert_ne!(result.signal_type, SignalType::StrongBuy);
        println!("Low liquidity confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_conservative_strategy_rejects_high_concentration() {
        let strategy = ConservativeStrategy::new();
        let mut metrics = create_test_metrics();

        metrics.top_10_holder_percent = 45.0; // Too concentrated
        metrics.bonding_curve_progress = 50.0;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence < 0.80);
        println!("High concentration confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_ultra_early_sniper_detects_new_tokens() {
        let strategy = UltraEarlySniper::new();
        let mut metrics = create_test_metrics();

        // Ultra-early conditions
        metrics.age_minutes = 3;
        metrics.bonding_curve_progress = 5.0;
        metrics.buy_pressure_ratio = 8.0;
        metrics.volume_acceleration = 5.0;
        metrics.price_change_5m = 85.0;
        metrics.unique_buyers_5m = 42;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence > 0.75);
        assert_eq!(result.signal_type, SignalType::StrongBuy);
        println!("Ultra-early confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_ultra_early_sniper_rejects_old_tokens() {
        let strategy = UltraEarlySniper::new();
        let mut metrics = create_test_metrics();

        metrics.age_minutes = 30; // Too old
        metrics.bonding_curve_progress = 5.0;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence < 0.75);
        println!("Old token confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_ultra_early_sniper_rejects_high_bonding_curve() {
        let strategy = UltraEarlySniper::new();
        let mut metrics = create_test_metrics();

        metrics.age_minutes = 3;
        metrics.bonding_curve_progress = 25.0; // Too high

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence < 0.75);
        println!("High curve confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_momentum_scalper_detects_explosive_moves() {
        let strategy = MomentumScalper::new();
        let mut metrics = create_test_metrics();

        // Explosive momentum
        metrics.bonding_curve_progress = 60.0;
        metrics.price_change_1h = 120.0; // Explosive!
        metrics.price_change_5m = 15.0;
        metrics.volume_5m = 45.0;
        metrics.volume_acceleration = 3.0;
        metrics.buy_pressure_ratio = 3.2;
        metrics.liquidity_sol = 18.0;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence > 0.75);
        assert_eq!(result.signal_type, SignalType::StrongBuy);
        println!("Momentum confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_momentum_scalper_requires_high_price_change() {
        let strategy = MomentumScalper::new();
        let mut metrics = create_test_metrics();

        metrics.bonding_curve_progress = 60.0;
        metrics.price_change_1h = 30.0; // Not explosive enough

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence < 0.75);
        println!("Low momentum confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_graduation_anticipator_near_graduation() {
        let strategy = GraduationAnticipator::new();
        let mut metrics = create_test_metrics();

        // Near graduation
        metrics.bonding_curve_progress = 75.0;
        metrics.is_graduated = false;
        metrics.liquidity_sol = 28.0;
        metrics.holder_count = 245;
        metrics.top_10_holder_percent = 19.0;
        metrics.volume_24h = 120.0;
        metrics.price_change_1h = 12.0;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence > 0.75);
        assert_eq!(result.signal_type, SignalType::StrongBuy);
        println!("Graduation confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_graduation_anticipator_rejects_already_graduated() {
        let strategy = GraduationAnticipator::new();
        let mut metrics = create_test_metrics();

        metrics.bonding_curve_progress = 90.0;
        metrics.is_graduated = true; // Already graduated!

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence < 0.75);
        println!("Already graduated confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_graduation_anticipator_rejects_too_early() {
        let strategy = GraduationAnticipator::new();
        let mut metrics = create_test_metrics();

        metrics.bonding_curve_progress = 40.0; // Too early
        metrics.is_graduated = false;

        let result = strategy.analyze(&metrics).unwrap();

        assert!(result.confidence < 0.75);
        println!("Too early for graduation confidence: {:.2}%", result.confidence * 100.0);
    }

    #[test]
    fn test_exit_params_conservative() {
        let strategy = ConservativeStrategy::new();
        let params = strategy.get_exit_params();

        assert_eq!(params.take_profit_multiplier, 2.0);
        assert_eq!(params.stop_loss_percentage, 0.50);
        assert_eq!(params.position_timeout_seconds, 3600);
        assert!(!params.use_trailing_stop);
    }

    #[test]
    fn test_exit_params_ultra_early() {
        let strategy = UltraEarlySniper::new();
        let params = strategy.get_exit_params();

        assert_eq!(params.take_profit_multiplier, 3.0);
        assert_eq!(params.stop_loss_percentage, 0.30);
        assert_eq!(params.position_timeout_seconds, 600);
        assert!(!params.use_trailing_stop);
    }

    #[test]
    fn test_exit_params_momentum() {
        let strategy = MomentumScalper::new();
        let params = strategy.get_exit_params();

        assert_eq!(params.take_profit_multiplier, 1.5);
        assert_eq!(params.stop_loss_percentage, 0.25);
        assert_eq!(params.position_timeout_seconds, 1800);
        assert!(params.use_trailing_stop);
        assert_eq!(params.trailing_activation_pct, 0.20);
        assert_eq!(params.trailing_distance_pct, 0.10);
    }

    #[test]
    fn test_exit_params_graduation() {
        let strategy = GraduationAnticipator::new();
        let params = strategy.get_exit_params();

        assert_eq!(params.take_profit_multiplier, 1.8);
        assert_eq!(params.stop_loss_percentage, 0.35);
        assert_eq!(params.position_timeout_seconds, 7200);
        assert!(!params.use_trailing_stop);
    }

    #[test]
    fn test_signal_type_from_confidence() {
        assert_eq!(SignalType::from_confidence(0.90), SignalType::StrongBuy);
        assert_eq!(SignalType::from_confidence(0.75), SignalType::Buy);
        assert_eq!(SignalType::from_confidence(0.55), SignalType::Hold);
        assert_eq!(SignalType::from_confidence(0.35), SignalType::Sell);
        assert_eq!(SignalType::from_confidence(0.15), SignalType::StrongSell);
    }

    #[test]
    fn test_volume_score_calculation() {
        let strategy = ConservativeStrategy::new();
        let metrics = create_test_metrics();

        let result = strategy.analyze(&metrics).unwrap();

        // Should have volume analysis in breakdown
        assert!(result.breakdown.contains_key("volume_analysis"));
        let volume_score = result.breakdown.get("volume_analysis").unwrap();
        assert!(*volume_score >= 0.0 && *volume_score <= 1.0);
    }

    #[test]
    fn test_multiple_strategies_same_token() {
        let metrics = create_test_metrics();

        let conservative = ConservativeStrategy::new();
        let ultra_early = UltraEarlySniper::new();
        let momentum = MomentumScalper::new();
        let graduation = GraduationAnticipator::new();

        let r1 = conservative.analyze(&metrics).unwrap();
        let r2 = ultra_early.analyze(&metrics).unwrap();
        let r3 = momentum.analyze(&metrics).unwrap();
        let r4 = graduation.analyze(&metrics).unwrap();

        println!("Same token, different strategies:");
        println!("  Conservative: {:.2}%", r1.confidence * 100.0);
        println!("  Ultra-Early: {:.2}%", r2.confidence * 100.0);
        println!("  Momentum: {:.2}%", r3.confidence * 100.0);
        println!("  Graduation: {:.2}%", r4.confidence * 100.0);

        // Each strategy should have different confidence for same token
        // (unless it happens to be perfect for all, which is unlikely)
    }

    #[test]
    fn test_edge_case_zero_values() {
        let strategy = ConservativeStrategy::new();
        let mut metrics = create_test_metrics();

        metrics.volume_5m = 0.0;
        metrics.liquidity_sol = 0.0;
        metrics.holder_count = 0;

        let result = strategy.analyze(&metrics).unwrap();

        // Should handle zeros gracefully without panic
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
        assert_eq!(result.signal_type, SignalType::StrongSell);
    }

    #[test]
    fn test_edge_case_extreme_values() {
        let strategy = ConservativeStrategy::new();
        let mut metrics = create_test_metrics();

        metrics.volume_5m = 1000000.0;
        metrics.price_change_1h = 10000.0;
        metrics.buy_pressure_ratio = 100.0;

        let result = strategy.analyze(&metrics);

        // Should handle extreme values without panic
        assert!(result.is_ok());
    }
}
