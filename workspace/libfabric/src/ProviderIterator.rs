// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct ProviderIterator
{
	head: *mut fi_info,
	next: *mut fi_info,
}

impl Drop for ProviderIterator
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if unlikely(self.head.is_null())
		{
			return;
		}
		unsafe { fi_freeinfo(self.head) }
	}
}

impl Iterator for ProviderIterator
{
	type Item = Provider;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely(self.next.is_null())
		{
			None
		}
		else
		{
			Some(Provider::fromHandle(Provider::duplicate(self.next)))
		}
	}
}

impl ProviderIterator
{
	pub fn discoverProviders(ourVersion: Version) -> ProviderIterator
	{
		let mut head: *mut fi_info = unsafe { uninitialized() };
		
		panic_on_error!("fi_getinfo", unsafe { fi_getinfo(ourVersion.toCombinedValue(), null_mut(), null_mut(), 0, null_mut(), &mut head) });
		
		ProviderIterator
		{
			head: head,
			next: head,
		}
	}
}
