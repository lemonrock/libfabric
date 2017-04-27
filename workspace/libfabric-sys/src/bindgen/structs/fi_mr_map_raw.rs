// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_mr_map_raw
{
	pub flags: u64,
	pub base_addr: u64,
	pub raw_key: *mut u8,
	pub key_size: usize,
	pub key: *mut u64,
}

impl Clone for fi_mr_map_raw
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_mr_map_raw
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
