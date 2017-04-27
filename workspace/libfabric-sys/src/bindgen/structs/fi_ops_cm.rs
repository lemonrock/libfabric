// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct fi_ops_cm
{
	pub size: usize,
	pub setname: Option<unsafe extern "C" fn(fid: fid_t, addr: *mut c_void, addrlen: usize) -> c_int>,
	pub getname: Option<unsafe extern "C" fn(fid: fid_t, addr: *mut c_void, addrlen: *mut usize) -> c_int>,
	pub getpeer: Option<unsafe extern "C" fn(ep: *mut fid_ep, addr: *mut c_void, addrlen: *mut usize) -> c_int>,
	pub connect: Option<unsafe extern "C" fn(ep: *mut fid_ep, addr: *const c_void, param: *const c_void, paramlen: usize) -> c_int>,
	pub listen: Option<unsafe extern "C" fn(pep: *mut fid_pep) -> c_int>,
	pub accept: Option<unsafe extern "C" fn(ep: *mut fid_ep, param: *const c_void, paramlen: usize) -> c_int>,
	pub reject: Option<unsafe extern "C" fn(pep: *mut fid_pep, handle: fid_t, param: *const c_void, paramlen: usize) -> c_int>,
	pub shutdown: Option<unsafe extern "C" fn(ep: *mut fid_ep, flags: u64) -> c_int>,
	pub join: Option<unsafe extern "C" fn(ep: *mut fid_ep, addr: *const c_void, flags: u64, mc: *mut *mut fid_mc, context: *mut c_void) -> c_int>,
}

impl Clone for fi_ops_cm
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for fi_ops_cm
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
