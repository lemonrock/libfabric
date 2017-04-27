// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_info
{
	pub next: *mut fi_info,
	pub caps: u64,
	pub mode: u64,
	pub addr_format: u32,
	pub src_addrlen: usize,
	pub dest_addrlen: usize,
	pub src_addr: *mut c_void,
	pub dest_addr: *mut c_void,
	pub handle: fid_t,
	pub tx_attr: *mut fi_tx_attr,
	pub rx_attr: *mut fi_rx_attr,
	pub ep_attr: *mut fi_ep_attr,
	pub domain_attr: *mut fi_domain_attr,
	pub fabric_attr: *mut fi_fabric_attr,
}

impl Clone for fi_info
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
