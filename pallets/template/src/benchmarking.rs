#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn do_something() {
        let caller: T::AccountId = whitelisted_caller();
        let value: u32 = 100;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller), value);

        assert_eq!(Something::<T>::get(), Some(value));
    }

    // If your pallet doesn't expose this extrinsic, delete this whole benchmark.
    #[benchmark]
    fn cause_error() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller));

        let _ = Something::<T>::get();
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
