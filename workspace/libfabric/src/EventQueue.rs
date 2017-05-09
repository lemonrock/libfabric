// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct EventQueue(*mut fid_eq);

impl Drop for EventQueue
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.close()
	}
}

impl FabricInterfaceDescriptor for EventQueue
{
	type F = fid_eq;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		EventQueue(handle)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid
	{
		self.0 as *mut _
	}
}

impl EventQueue
{
}
