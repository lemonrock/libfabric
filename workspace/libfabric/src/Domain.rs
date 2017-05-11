// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Domain(*mut fid_domain);

impl Drop for Domain
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.close()
	}
}

impl FabricInterfaceDescriptor for Domain
{
	type F = fid_domain;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		Domain(handle)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid
	{
		self.0 as *mut _
	}
}

/// Verbs Provider Support: https://ofiwg.github.io/libfabric/v1.4.1/man/fi_verbs.7.html
///
impl Domain
{
	// fi_ep_type - 5 kinds
	
	// https://github.com/ofiwg/libfabric/wiki/Provider-Feature-Matrix-v1.5.0
	
	/// NOTE: Verbs provider supports MSG aka FI_EP_MSG (FI_MSG, FI_RMA, FI_ATOMIC, Shared Receive context FI_SHARED_CONTEXT) endpoints
	/// NOTE: Verbs provider supports RDM aka FI_EP_RDM (FI_MSG, FI_TAGGED, FI_RMA) endpoints (experimental)
	/// 	* FI_MULTI_RECV is also supported
	/// 	* Peek and Claim are supported
	/// NOTE: Verbs provider does not support DGRAM aka FI_EP_DGRAM endpoints
	/// NOTE: Verbs provider does not support scalable endpoints
	/// NOTE: Verbs provider does not support MSG endpoint features: Counters, FI_SOURCE, FI_TAGGED, FI_PEEK, FI_CLAIM, fi_cancel, fi_ep_alias, shared TX context, cq_readfrom operations
	/// NOTE: Verbs provider RDM endpoints do not support iov yet (iov size 1!), wait objects or thread safety
	/// NOTE: Verbs provider requires applications to support the following modes (mode bits): FI_LOCAL_MR for all applications. FI_RX_CQ_DATA for applications that want to use RMA (Remote Memory Access). Applications must take responsibility of posting receives for any incoming CQ (Completion Queue) data.
	/// NOTE: UDP provider can use RxD utility provider to support RDM endpoints, but only for FI_MSG, FI_TAGGED, FI_RMA - not FI_ATOMIC
	/// NOTE: Mellanox Provider for UCX supports TAGGED RDM only
	/// NOTE: Sockets provider supports MSG aka FI_EP_MSG (FI_MSG)
	/// NOTE: Sockets provider supports RDM aka FI_EP_RDM (FI_MSG, FI_TAGGED, FI_RMA, FI_ATOMIC)
	/// NOTE: Sockets needs a progress thread bound to a virtual processor ID using an environment variable. Yuck.
	/// NOTE: Providers: FI_EP_RDM endpoints must support CM getname and setname
	/// NOTE: Providers:Completion queues must support the FI_CQ_FORMAT_CONTEXT and FI_CQ_FORMAT_MSG.
	/// NOTE: Providers that support FI_REMOTE_CQ_DATA shall support FI_CQ_FORMAT_DATA.
	/// NOTE: Providers that support FI_TAGGED shall support FI_CQ_FORMAT_TAGGED.
	/// NOTE: Verbs provider may soon be providing FI_FENCE
	pub fn openActiveEndpoint(&self, provider: &Provider) -> ActiveEndpoint
	{
		let mut handle = unsafe { uninitialized() };
		
		panic_on_error!("rust_fi_endpoint", unsafe { rust_fi_endpoint(self.0, provider.0, &mut handle, null_mut()) });
		
		ActiveEndpoint::fromHandle(handle)
	}
	
	//pub fn rust_fi_scalable_ep(domain: *mut fid_domain, info: *mut fi_info, sep: *mut *mut fid_ep, context: *mut c_void) -> c_int;
	
	pub fn createAddressVectorMapAndBindToActiveEndpointAndEventQueue(&self, activeEndpoint: &ActiveEndpoint, eventQueue: &EventQueue) -> AddressVectorMap
	{
		let mut addressVectorAttributes = fi_av_attr
		{
			type_: fi_av_type::FI_AV_MAP,
			rx_ctx_bits: 0,
			count: 0,
			ep_per_node: 0,
			name: null(),
			map_addr: null_mut(),
			flags: 0,
		};
		
		let mut handle = unsafe { uninitialized() };
		
		panic_on_error!("rust_fi_av_open", unsafe { rust_fi_av_open(self.0, &mut addressVectorAttributes, &mut handle, null_mut()) });
		
		let addressVectorMap = AddressVectorMap::fromHandle(handle);
		
		panic_on_error!("rust_fi_ep_bind", unsafe { rust_fi_ep_bind(activeEndpoint.0, addressVectorMap.fid(), 0)});
		panic_on_error!("rust_fi_av_bind", unsafe { rust_fi_av_bind(handle, eventQueue.fid(), 0)});
		addressVectorMap
	}
	
	/// Optional
	pub fn createCompletionQueueAndBindToActiveEndpoint(&self, activeEndpoint: &ActiveEndpoint) -> CompletionQueue
	{
		let mut completionQueueAttributes = fi_cq_attr
		{
			size: 100,
			flags: 0,
			format: fi_cq_format::FI_CQ_FORMAT_TAGGED,
			wait_obj: fi_wait_obj::FI_WAIT_NONE, // Verbs provider supports this or FI_WAIT_FD
			signaling_vector: 0,
			wait_cond: fi_cq_wait_cond::FI_CQ_COND_NONE,
			wait_set: null_mut(), // Verbs provider only supports this
		};
		
		let mut handle = unsafe { uninitialized() };
		
		panic_on_error!("rust_fi_cq_open", unsafe { rust_fi_cq_open(self.0, &mut completionQueueAttributes, &mut handle, null_mut()) });
		
		let completionQueue = CompletionQueue::fromHandle(handle);
		
		// Also FI_SELECTIVE_COMPLETION to force only operations with FI_COMPLETION set being notified on success
		panic_on_error!("rust_fi_ep_bind", unsafe { rust_fi_ep_bind(activeEndpoint.0, completionQueue.fid(), FI_TRANSMIT | FI_RECV)});
		
		completionQueue
	}
	
	/// Optional
	/// NOTE: Counters are not supported for MSG endpoints by the Verbs (iWarp, Infiniband, etc) provider
	pub fn createCompletionCounterAndBindToActiveEndpoint(&self, activeEndpoint: &ActiveEndpoint) -> CompletionCounter
	{
		let mut completionCounterAttributes = fi_cntr_attr
		{
			events: fi_cntr_events::FI_CNTR_EVENTS_COMP,
			wait_obj: fi_wait_obj::FI_WAIT_NONE, // Verbs provider supports this or FI_WAIT_FD
			wait_set: null_mut(),  // Verbs provider only supports this
			flags: 0,
		};
		
		let mut handle = unsafe { uninitialized() };
		
		panic_on_error!("rust_fi_cntr_open", unsafe { rust_fi_cntr_open(self.0, &mut completionCounterAttributes, &mut handle, null_mut()) });
		
		let completionCounter = CompletionQueue::fromHandle(handle);
		
		// Also FI_REMOTE_READ and FI_REMOTE_WRITE, but only if the endpoint was created with the FI_RMA_EVENT flag
		panic_on_error!("rust_fi_ep_bind", unsafe { rust_fi_ep_bind(activeEndpoint.0, completionCounter.fid(), FI_SEND | FI_RECV | FI_READ | FI_WRITE)});
		
		completionCounter
	}
}

#[derive(Debug)]
pub struct AddressVectorMap(*mut fid_ep);

impl Drop for AddressVectorMap
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.close()
	}
}

impl FabricInterfaceDescriptor for AddressVectorMap
{
	type F = fid_ep;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		AddressVectorMap(handle)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid
	{
		self.0 as *mut _
	}
}

#[derive(Debug)]
pub struct ActiveEndpoint(*mut fid_ep);

impl Drop for ActiveEndpoint
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.close()
	}
}

impl FabricInterfaceDescriptor for ActiveEndpoint
{
	type F = fid_ep;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		ActiveEndpoint(handle)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid
	{
		self.0 as *mut _
	}
}

#[derive(Debug)]
pub struct CompletionQueue(*mut fid_ep);

impl Drop for CompletionQueue
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.close()
	}
}

impl FabricInterfaceDescriptor for CompletionQueue
{
	type F = fid_ep;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		CompletionQueue(handle)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid
	{
		self.0 as *mut _
	}
}

#[derive(Debug)]
pub struct CompletionCounter(*mut fid_ep);

impl Drop for CompletionCounter
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.close()
	}
}

impl FabricInterfaceDescriptor for CompletionCounter
{
	type F = fid_ep;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fromHandle(handle: *mut Self::F) -> Self
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		CompletionCounter(handle)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fid(&self) -> *mut fid
	{
		self.0 as *mut _
	}
}
