// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct fi_ops_atomic
{
	pub size: usize,
	pub write: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize>,
	pub writev: Option<unsafe extern "C" fn(ep: *mut fid_ep, iov: *const fi_ioc, desc: *mut *mut c_void, count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize>,
	pub writemsg: Option<unsafe extern "C" fn(ep: *mut fid_ep, msg: *const fi_msg_atomic, flags: u64) -> isize>,
	pub inject: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op) -> isize>,
	pub readwrite: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, result: *mut c_void, result_desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize>,
	pub readwritev: Option<unsafe extern "C" fn(ep: *mut fid_ep, iov: *const fi_ioc, desc: *mut *mut c_void, count: usize, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize>,
	pub readwritemsg: Option<unsafe extern "C" fn(ep: *mut fid_ep, msg: *const fi_msg_atomic, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, flags: u64) -> isize>,
	pub compwrite: Option<unsafe extern "C" fn(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, compare: *const c_void, compare_desc: *mut c_void, result: *mut c_void, result_desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize>,
	pub compwritev: Option<unsafe extern "C" fn(ep: *mut fid_ep, iov: *const fi_ioc, desc: *mut *mut c_void, count: usize, comparev: *const fi_ioc, compare_desc: *mut *mut c_void, compare_count: usize, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize>,
	pub compwritemsg: Option<unsafe extern "C" fn(ep: *mut fid_ep, msg: *const fi_msg_atomic, comparev: *const fi_ioc, compare_desc: *mut *mut c_void, compare_count: usize, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, flags: u64) -> isize>,
	pub writevalid: Option<unsafe extern "C" fn(ep: *mut fid_ep, datatype: fi_datatype, op: fi_op, count: *mut usize) -> c_int>,
	pub readwritevalid: Option<unsafe extern "C" fn(ep: *mut fid_ep, datatype: fi_datatype, op: fi_op, count: *mut usize) -> c_int>,
	pub compwritevalid: Option<unsafe extern "C" fn(ep: *mut fid_ep, datatype: fi_datatype, op: fi_op, count: *mut usize) -> c_int>,
}

impl Clone for fi_ops_atomic
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_atomic
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
