// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_ep
{
	pub size: usize,
	pub cancel: Option<unsafe extern "C" fn(fid: fid_t, context: *mut c_void) -> isize>,
	pub getopt: Option<unsafe extern "C" fn(fid: fid_t, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut usize) -> c_int>,
	pub setopt: Option<unsafe extern "C" fn(fid: fid_t, level: c_int, optname: c_int, optval: *const c_void, optlen: usize) -> c_int>,
	pub tx_ctx: Option<unsafe extern "C" fn(sep: *mut fid_ep, index: c_int, attr: *mut fi_tx_attr, tx_ep: *mut *mut fid_ep, context: *mut c_void) -> c_int>,
	pub rx_ctx: Option<unsafe extern "C" fn(sep: *mut fid_ep, index: c_int, attr: *mut fi_rx_attr, rx_ep: *mut *mut fid_ep, context: *mut c_void) -> c_int>,
	pub rx_size_left: Option<unsafe extern "C" fn(ep: *mut fid_ep) -> isize>,
	pub tx_size_left: Option<unsafe extern "C" fn(ep: *mut fid_ep) -> isize>,
}

impl Clone for fi_ops_ep
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_ep
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
