// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct fi_tx_attr
{
	pub caps: u64,
	pub mode: u64,
	pub op_flags: u64,
	pub msg_order: u64,
	pub comp_order: u64,
	pub inject_size: usize,
	pub size: usize,
	pub iov_limit: usize,
	pub rma_iov_limit: usize,
}

impl Clone for fi_tx_attr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}
