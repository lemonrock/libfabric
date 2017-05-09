// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version
{
	major: u16,
	minor: u16,
}

impl Version
{
	pub const Api: Version = Version
	{
		major: FI_MAJOR_VERSION as u16,
		minor: FI_MINOR_VERSION as u16,
	};
	
	#[inline(always)]
	pub fn current() -> Self
	{
		Self::fromCombinedValue(unsafe { fi_version() })
	}
	
	#[inline(always)]
	pub fn fromCombinedValue(version: u32) -> Self
	{
		Self
		{
			major: (version >> 16) as u16,
			minor: (version & 0xFFFF) as u16,
		}
	}
	
	#[inline(always)]
	pub fn toCombinedValue(&self) -> u32
	{
		(self.major as u32) << 16 & (self.minor as u32)
	}
}
