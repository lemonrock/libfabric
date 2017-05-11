// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Fabric(*mut fid_fabric);

impl Drop for Fabric
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.close()
	}
}

impl FabricInterfaceDescriptor for Fabric
{
	type F = fid_fabric;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		Fabric(handle)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid
	{
		self.0 as *mut _
	}
}

/// Represents a logical or physical network with shared addresses
/// May potentially span multiple providers
impl Fabric
{
	/// 'Modify' the provider to change domain characteristics
	pub fn openDomain(&self, provider: &Provider) -> Domain
	{
		// seems to use addr_format and dest_addr in addition to domain attr
		
		let mut handle = unsafe { uninitialized() };

		panic_on_error!("rust_fi_domain", unsafe { rust_fi_domain(self.0, provider.0, &mut handle, null_mut()) });

		Domain::fromHandle(handle)
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
		
		let mut handle = unsafe { uninitialized() };
		
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
