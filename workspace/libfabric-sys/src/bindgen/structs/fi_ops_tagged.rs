// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_tagged
{
	pub size: usize,
	pub recv: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *mut c_void, len: usize, desc: *mut c_void, src_addr: fi_addr_t, tag: u64, ignore: u64, context: *mut c_void) -> isize>,
	pub recvv: Option<unsafe extern "C" fn(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, src_addr: fi_addr_t, tag: u64, ignore: u64, context: *mut c_void) -> isize>,
	pub recvmsg: Option<unsafe extern "C" fn(ep: *mut fid_ep, msg: *const fi_msg_tagged, flags: u64) -> isize>,
	pub send: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, dest_addr: fi_addr_t, tag: u64, context: *mut c_void) -> isize>,
	pub sendv: Option<unsafe extern "C" fn(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, dest_addr: fi_addr_t, tag: u64, context: *mut c_void) -> isize>,
	pub sendmsg: Option<unsafe extern "C" fn(ep: *mut fid_ep, msg: *const fi_msg_tagged, flags: u64) -> isize>,
	pub inject: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, len: usize, dest_addr: fi_addr_t, tag: u64) -> isize>,
	pub senddata: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, data: u64, dest_addr: fi_addr_t, tag: u64, context: *mut c_void) -> isize>,
	pub injectdata: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, len: usize, data: u64, dest_addr: fi_addr_t, tag: u64) -> isize>,
}

impl Clone for fi_ops_tagged
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_tagged
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
