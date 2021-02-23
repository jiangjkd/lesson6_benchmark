//! Benchmark-demo pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks!{
    // 初始化 变量b 范围 1 到 1000 ，返回（） 表示初始化b时 不会进行其他操作
	_ {
		let b in 1 .. 1000 => ();
	}
    // 对可调用函数do_something 进行benchmark测试用例
	do_something {
		let b in ...;// 取b值
		let caller = account("caller", 0, 0);
	}: _ (RawOrigin::Signed(caller), b.into()) // _ 代替do_something
	verify { // 验证结果是否正确
		let value = Something::get();
		assert_eq!(value, b.into());
	}
}
// 单元测试
#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Test};
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_do_something::<Test>());
		});
	}
}
