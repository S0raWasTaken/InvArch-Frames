//! Benchmarks for IPF Pallet
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{Config, Pallet, RawOrigin};
use sp_core::H256;
use sp_runtime::traits::StaticLookup;

const SEED: u32 = 0;

const MOCK_DATA: [u8; 32] = [
    12, 47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218,
    0, 230, 247, 32, 73, 152, 66, 243, 27, 92, 95,
];

benchmarks! {
    // Mint IPF
    mint {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: H256 = H256::from(MOCK_DATA);
    }: _(RawOrigin::Signed(caller), metadata, data)
    verify {
        //
    }
/*
    // Burn IPF
    burn {
        let caller: T::AccountId = whitelisted_caller();
        let ipf_id: u64 = 0;
    }: _(RawOrigin::Signed(caller), ipf_id)
        verify {
            //
        }

    // Send IPF
    send {
        let caller: T::AccountId = whitelisted_caller();
        let ipf_id: u64 = 0;
        let target = T::Lookup::unlookup(1);
    }: _(RawOrigin::Signed(caller), ipf_id, target)
        verify {
            //
        }*/
}

//impl_benchmark_test_suite!(Ipf, crate::mock::new_test_ext(), crate::mock::Test);
