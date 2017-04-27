// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_op_compare_atomic
{
	pub ep: *mut fid_ep,
	pub msg: fi_msg_atomic,
	pub fetch: fi_msg_fetch,
	pub compare: fi_msg_compare,
	pub flags: u64,
}

impl Clone for fi_op_compare_atomic
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_op_compare_atomic
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
