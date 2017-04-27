// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ep_attr
{
	pub type_: fi_ep_type,
	pub protocol: u32,
	pub protocol_version: u32,
	pub max_msg_size: usize,
	pub msg_prefix_size: usize,
	pub max_order_raw_size: usize,
	pub max_order_war_size: usize,
	pub max_order_waw_size: usize,
	pub mem_tag_format: u64,
	pub tx_ctx_cnt: usize,
	pub rx_ctx_cnt: usize,
	pub auth_key_size: usize,
	pub auth_key: *mut u8,
}

impl Clone for fi_ep_attr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ep_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
