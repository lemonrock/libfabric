// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_domain_attr
{
	pub domain: *mut fid_domain,
	pub name: *mut c_char,
	pub threading: fi_threading,
	pub control_progress: fi_progress,
	pub data_progress: fi_progress,
	pub resource_mgmt: fi_resource_mgmt,
	pub av_type: fi_av_type,
	pub mr_mode: c_int,
	pub mr_key_size: usize,
	pub cq_data_size: usize,
	pub cq_cnt: usize,
	pub ep_cnt: usize,
	pub tx_ctx_cnt: usize,
	pub rx_ctx_cnt: usize,
	pub max_ep_tx_ctx: usize,
	pub max_ep_rx_ctx: usize,
	pub max_ep_stx_ctx: usize,
	pub max_ep_srx_ctx: usize,
	pub cntr_cnt: usize,
	pub mr_iov_limit: usize,
	pub caps: u64,
	pub mode: u64,
	pub auth_key: *mut u8,
	pub auth_key_size: usize,
	pub max_err_data: usize,
}

impl Clone for fi_domain_attr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_domain_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
