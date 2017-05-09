// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Fabric(*mut fid_fabric);

/// Represents a logical or physical network with shared addresses
/// May potentially span multiple providers
impl Fabric
{
	#[inline(always)]
	fn fromHandle(handle: *mut fid_fabric) -> Self
	{
		Fabric(handle)
	}
	
	pub fn openDomain(&self)
	{
		// pub fn rust_fi_domain(fabric: *mut fid_fabric, info: *mut fi_info, domain: *mut *mut fid_domain, context: *mut c_void) -> c_int;
		unimplemented!();
	}
	
	pub fn openPassiveEndpoint(&self)
	{
		// pub fn rust_fi_passive_ep(fabric: *mut fid_fabric, info: *mut fi_info, pep: *mut *mut fid_pep, context: *mut c_void) -> c_int;
		unimplemented!();
	}
	
	pub fn openEventQueue(&self) -> EventQueue
	{
		let mut eventQueueAttributes = fi_eq_attr
		{
			size: 64,
			flags: 0,
			wait_obj: fi_wait_obj::FI_WAIT_UNSPEC,
			signaling_vector: 0,
			wait_set: null_mut(),
		};
		
		let mut handle: *mut fid_eq = unsafe { uninitialized() };
		
		panic_on_error!("rust_fi_eq_open", unsafe { rust_fi_eq_open(self.0, &mut eventQueueAttributes, &mut handle, null_mut()) });
		
		EventQueue::fromHandle(handle)
	}
	
	pub fn openWaitSet(&self)
	{
		// pub fn rust_fi_wait_open(fabric: *mut fid_fabric, attr: *mut fi_wait_attr, waitset: *mut *mut fid_wait) -> c_int;
		unimplemented!();
	}
	
	/*
	
	pub fn rust_fi_trywait(fabric: *mut fid_fabric, fids: *mut *mut fid, count: c_int) -> c_int;
	
	*/
}
