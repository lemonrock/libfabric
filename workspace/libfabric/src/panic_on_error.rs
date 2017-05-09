// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_export]
macro_rules! panic_on_error
{
	($function: tt, $result: expr) =>
	{
		{
			let result = $result;
			debug_assert!(result <= 0, "Function {} returned a positive value, '{}'", $function, result);
			if $crate::rust_extra::unlikely(result != 0)
			{
				let osOrLibfabricErrorNumber = -result;
				let cStringMessage = unsafe { fi_strerror(osOrLibfabricErrorNumber) };
				
				panic!("Function {} had an unexpected error number '{}' with description '{:?}'", $function, osOrLibfabricErrorNumber, cStringMessage);
			}
		}
	}
}
