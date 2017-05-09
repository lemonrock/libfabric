// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// aka 'fid' or 'FID' or 'fid_t'
#[derive(Debug, Clone)]
pub struct FabricInterfaceDescriptor(*mut fid, Arc<FabricInterfaceDescriptorDropWrapper>);

impl FabricInterfaceDescriptor
{
	fn fromHandle(handle: *mut fid) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		FabricInterfaceDescriptor(handle, Arc::new(FabricInterfaceDescriptorDropWrapper(handle)))
	}
	
	#[inline(always)]
	pub fn class(&self) -> FabricInterfaceDescriptorClass
	{
		let fclass = (unsafe { *self.0 }).fclass;
		unsafe { transmute(fclass as u32) }
	}
	
	/// May be null
	#[inline(always)]
	pub fn context(&self) -> *mut c_void
	{
		(unsafe { *self.0 }).context
	}
	
	/// May be null
	#[inline(always)]
	pub fn operations(&self) -> *mut fi_ops
	{
		(unsafe { *self.0 }).ops
	}
	
	/*
	
	fid_t ==> *mut fid
	
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
