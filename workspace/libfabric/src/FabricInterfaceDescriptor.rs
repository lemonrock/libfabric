// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait FabricInterfaceDescriptor: Drop
{
	type F;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid;
	
	#[doc(hidden)]
	#[inline(always)]
	fn close(&mut self)
	{
		panic_on_error!("rust_fi_close", unsafe { rust_fi_close(self.fid()) });
	}
	
	#[inline(always)]
	fn class(&self) -> FabricInterfaceDescriptorClass
	{
		let fclass = (unsafe { *self.fid() }).fclass;
		unsafe { transmute(fclass as u32) }
	}
	
	/// May be null
	#[inline(always)]
	fn context(&self) -> *mut c_void
	{
		(unsafe { *self.fid() }).context
	}
	
	/// May be null
	#[inline(always)]
	fn operations(&self) -> *mut fi_ops
	{
		(unsafe { *self.fid() }).ops
	}
	
	/*
	
	pub fn rust_fi_alias(fid: *mut fid, alias_fid: *mut *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_cancel(fid: fid_t, context: *mut c_void) -> isize;
	pub fn rust_fi_control(fid: *mut fid, command: c_int, arg: *mut c_void) -> c_int;
	
	pub fn rust_fi_open_ops(fid: *mut fid, name: *const c_char, flags: u64, ops: *mut *mut c_void, context: *mut c_void) -> c_int;
	
	pub fn rust_fi_getname(fid: fid_t, addr: *mut c_void, addrlen: *mut usize) -> c_int;
	pub fn rust_fi_setname(fid: fid_t, addr: *mut c_void, addrlen: usize) -> c_int;
	
	pub fn rust_fi_getopt(fid: fid_t, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut usize) -> c_int;
	pub fn rust_fi_setopt(fid: fid_t, level: c_int, optname: c_int, optval: *const c_void, optlen: usize) -> c_int;
	*/
}
