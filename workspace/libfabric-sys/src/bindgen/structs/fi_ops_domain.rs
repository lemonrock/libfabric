// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_domain
{
	pub size: usize,
	pub av_open: Option<unsafe extern "C" fn(domain: *mut fid_domain, attr: *mut fi_av_attr, av: *mut *mut fid_av, context: *mut c_void) -> c_int>,
	pub cq_open: Option<unsafe extern "C" fn(domain: *mut fid_domain, attr: *mut fi_cq_attr, cq: *mut *mut fid_cq, context: *mut c_void) -> c_int>,
	pub endpoint: Option<unsafe extern "C" fn(domain: *mut fid_domain, info: *mut fi_info, ep: *mut *mut fid_ep, context: *mut c_void) -> c_int>,
	pub scalable_ep: Option<unsafe extern "C" fn(domain: *mut fid_domain, info: *mut fi_info, sep: *mut *mut fid_ep, context: *mut c_void) -> c_int>,
	pub cntr_open: Option<unsafe extern "C" fn(domain: *mut fid_domain, attr: *mut fi_cntr_attr, cntr: *mut *mut fid_cntr, context: *mut c_void) -> c_int>,
	pub poll_open: Option<unsafe extern "C" fn(domain: *mut fid_domain, attr: *mut fi_poll_attr, pollset: *mut *mut fid_poll) -> c_int>,
	pub stx_ctx: Option<unsafe extern "C" fn(domain: *mut fid_domain, attr: *mut fi_tx_attr, stx: *mut *mut fid_stx, context: *mut c_void) -> c_int>,
	pub srx_ctx: Option<unsafe extern "C" fn(domain: *mut fid_domain, attr: *mut fi_rx_attr, rx_ep: *mut *mut fid_ep, context: *mut c_void) -> c_int>,
	pub query_atomic: Option<unsafe extern "C" fn(domain: *mut fid_domain, datatype: fi_datatype, op: fi_op, attr: *mut fi_atomic_attr, flags: u64) -> c_int>,
}

impl Clone for fi_ops_domain
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_domain
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
