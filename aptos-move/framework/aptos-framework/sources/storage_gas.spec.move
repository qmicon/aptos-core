spec aptos_framework::storage_gas {
    spec GasCurve {
        invariant min_gas <= max_gas;
        //invariant len(points) >= 2;
    }

    spec module {
        use aptos_std::chain_status;
        // After genesis, `StateStorageUsage` and `GasParameter` exist.
        invariant [suspendable] chain_status::is_operating() ==> exists<StorageGasConfig>(@aptos_framework);
        invariant [suspendable] chain_status::is_operating() ==> exists<StorageGas>(@aptos_framework);
        invariant [suspendable] chain_status::is_operating() ==> storage_gas_config_is_valid();
    }

    spec validate_curve {
        pragma opaque, verify=false;
        aborts_if curve.max_gas < curve.min_gas;
        let points = curve.points;
        aborts_if exists i in 0..len(curve.points)-1: (
            points[i].x >= points[i+1].x || points[i].y > points[i+1].y
        );
    }

    spec fun storage_gas_config_is_valid(): bool {
        let storage_gas_config = global<StorageGasConfig>(@aptos_framework);
        spec_usage_config_is_validated(storage_gas_config.item_config) &&
            spec_usage_config_is_validated(storage_gas_config.byte_config)
    }

    spec fun spec_curve_is_validated(curve: GasCurve): bool {
        forall i in 0..len(curve.points)-1: (
            curve.points[i].x < curve.points[i+1].x &&
                curve.points[i].y <= curve.points[i+1].y
        )
    }

    spec fun spec_usage_config_is_validated(config: UsageGasConfig): bool {
        spec_curve_is_validated(config.read_curve) &&
            spec_curve_is_validated(config.create_curve) &&
            spec_curve_is_validated(config.write_curve)
    }

    spec UsageGasConfig {
        invariant target_usage > 0;
    }

    spec Point {
        invariant x <= BASIS_POINT_DENOMINATION;
        invariant y <= BASIS_POINT_DENOMINATION;
    }

    spec calculate_create_gas {
        requires spec_usage_config_is_validated(config);
    }
    spec calculate_read_gas {
        requires spec_usage_config_is_validated(config);
    }
    spec calculate_write_gas {
        requires spec_usage_config_is_validated(config);
    }

    spec calculate_gas {
        pragma opaque;
        requires max_usage > 0;
        requires spec_curve_is_validated(curve);


        aborts_if [concrete] false;



        aborts_if [abstract] false;
        ensures [abstract] result == spec_calculate_gas(max_usage, current_usage, curve);
    }

    spec fun spec_calculate_gas(max_usage: u64, current_usage: u64, curve: GasCurve): u64;

    spec on_reconfig {
        use aptos_std::chain_status;
        requires chain_status::is_operating();
        aborts_if false;
    }

}
