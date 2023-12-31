// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_pooled_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `pop-os`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet-pooled-staking
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_pooled_staking.
pub trait WeightInfo {
	fn request_delegate() -> Weight;
	fn execute_pending_operations(b: u32, ) -> Weight;
	fn request_undelegate() -> Weight;
	fn claim_manual_rewards(b: u32, ) -> Weight;
	fn rebalance_hold() -> Weight;
	fn update_candidate_position(b: u32, ) -> Weight;
	fn swap_pool() -> Weight;
	fn distribute_rewards() -> Weight;
}

/// Weights for pallet_pooled_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: PooledStaking Pools (r:11 w:5)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(66), added: 2541, mode: MaxEncodedLen)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PooledStaking PendingOperations (r:1 w:1)
	/// Proof Skipped: PooledStaking PendingOperations (max_values: None, max_size: None, mode: Measured)
	fn request_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1321`
		//  Estimated: `29536`
		// Minimum execution time: 127_339_000 picoseconds.
		Weight::from_parts(133_146_000, 29536)
			.saturating_add(T::DbWeight::get().reads(17_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: PooledStaking PendingOperations (r:100 w:100)
	/// Proof Skipped: PooledStaking PendingOperations (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PooledStaking Pools (r:1000 w:800)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(66), added: 2541, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn execute_pending_operations(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `543 + b * (390 ±0)`
		//  Estimated: `3593 + b * (25141 ±0)`
		// Minimum execution time: 89_544_000 picoseconds.
		Weight::from_parts(91_417_000, 3593)
			// Standard Error: 630_031
			.saturating_add(Weight::from_parts(99_103_944, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((11_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((9_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 25141).saturating_mul(b.into()))
	}
	/// Storage: PooledStaking Pools (r:13 w:9)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PooledStaking PendingOperations (r:1 w:1)
	/// Proof Skipped: PooledStaking PendingOperations (max_values: None, max_size: None, mode: Measured)
	fn request_undelegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `724`
		//  Estimated: `33889`
		// Minimum execution time: 111_997_000 picoseconds.
		Weight::from_parts(124_683_000, 33889)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: PooledStaking Pools (r:300 w:100)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn claim_manual_rewards(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `360 + b * (456 ±0)`
		//  Estimated: `6196 + b * (7882 ±0)`
		// Minimum execution time: 57_580_000 picoseconds.
		Weight::from_parts(60_814_000, 6196)
			// Standard Error: 421_370
			.saturating_add(Weight::from_parts(55_273_020, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7882).saturating_mul(b.into()))
	}
	/// Storage: PooledStaking Pools (r:4 w:1)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(66), added: 2541, mode: MaxEncodedLen)
	fn rebalance_hold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `980`
		//  Estimated: `11870`
		// Minimum execution time: 98_014_000 picoseconds.
		Weight::from_parts(128_615_000, 11870)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: PooledStaking Pools (r:600 w:100)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:100 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 100]`.
	fn update_candidate_position(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399 + b * (356 ±0)`
		//  Estimated: `1881 + b * (15206 ±0)`
		// Minimum execution time: 46_082_000 picoseconds.
		Weight::from_parts(60_293_000, 1881)
			// Standard Error: 131_937
			.saturating_add(Weight::from_parts(35_500_124, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 15206).saturating_mul(b.into()))
	}
	/// Storage: PooledStaking Pools (r:12 w:8)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	fn swap_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `31168`
		// Minimum execution time: 80_829_000 picoseconds.
		Weight::from_parts(97_569_000, 31168)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: PooledStaking Pools (r:9 w:5)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn distribute_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1302`
		//  Estimated: `24567`
		// Minimum execution time: 151_254_000 picoseconds.
		Weight::from_parts(178_410_000, 24567)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: PooledStaking Pools (r:11 w:5)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(66), added: 2541, mode: MaxEncodedLen)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PooledStaking PendingOperations (r:1 w:1)
	/// Proof Skipped: PooledStaking PendingOperations (max_values: None, max_size: None, mode: Measured)
	fn request_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1321`
		//  Estimated: `29536`
		// Minimum execution time: 127_339_000 picoseconds.
		Weight::from_parts(133_146_000, 29536)
			.saturating_add(RocksDbWeight::get().reads(17_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: PooledStaking PendingOperations (r:100 w:100)
	/// Proof Skipped: PooledStaking PendingOperations (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PooledStaking Pools (r:1000 w:800)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(66), added: 2541, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn execute_pending_operations(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `543 + b * (390 ±0)`
		//  Estimated: `3593 + b * (25141 ±0)`
		// Minimum execution time: 89_544_000 picoseconds.
		Weight::from_parts(91_417_000, 3593)
			// Standard Error: 630_031
			.saturating_add(Weight::from_parts(99_103_944, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((11_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((9_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 25141).saturating_mul(b.into()))
	}
	/// Storage: PooledStaking Pools (r:13 w:9)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PooledStaking PendingOperations (r:1 w:1)
	/// Proof Skipped: PooledStaking PendingOperations (max_values: None, max_size: None, mode: Measured)
	fn request_undelegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `724`
		//  Estimated: `33889`
		// Minimum execution time: 111_997_000 picoseconds.
		Weight::from_parts(124_683_000, 33889)
			.saturating_add(RocksDbWeight::get().reads(16_u64))
			.saturating_add(RocksDbWeight::get().writes(11_u64))
	}
	/// Storage: PooledStaking Pools (r:300 w:100)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn claim_manual_rewards(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `360 + b * (456 ±0)`
		//  Estimated: `6196 + b * (7882 ±0)`
		// Minimum execution time: 57_580_000 picoseconds.
		Weight::from_parts(60_814_000, 6196)
			// Standard Error: 421_370
			.saturating_add(Weight::from_parts(55_273_020, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7882).saturating_mul(b.into()))
	}
	/// Storage: PooledStaking Pools (r:4 w:1)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(66), added: 2541, mode: MaxEncodedLen)
	fn rebalance_hold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `980`
		//  Estimated: `11870`
		// Minimum execution time: 98_014_000 picoseconds.
		Weight::from_parts(128_615_000, 11870)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: PooledStaking Pools (r:600 w:100)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:100 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 100]`.
	fn update_candidate_position(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399 + b * (356 ±0)`
		//  Estimated: `1881 + b * (15206 ±0)`
		// Minimum execution time: 46_082_000 picoseconds.
		Weight::from_parts(60_293_000, 1881)
			// Standard Error: 131_937
			.saturating_add(Weight::from_parts(35_500_124, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((7_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 15206).saturating_mul(b.into()))
	}
	/// Storage: PooledStaking Pools (r:12 w:8)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	fn swap_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `31168`
		// Minimum execution time: 80_829_000 picoseconds.
		Weight::from_parts(97_569_000, 31168)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: PooledStaking Pools (r:9 w:5)
	/// Proof Skipped: PooledStaking Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PooledStaking SortedEligibleCandidates (r:1 w:1)
	/// Proof Skipped: PooledStaking SortedEligibleCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn distribute_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1302`
		//  Estimated: `24567`
		// Minimum execution time: 151_254_000 picoseconds.
		Weight::from_parts(178_410_000, 24567)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
}
