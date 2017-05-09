// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Provider<'a>
{
	handle: *mut fi_info,
	weShouldFreeHandle: *mut fi_info,
	phantomData: PhantomData<&'a fi_info>
}

impl<'a> Drop for Provider<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.weShouldFreeHandle.is_null()
		{
			unsafe { fi_freeinfo(self.weShouldFreeHandle) }
		}
	}
}

impl<'a> Provider<'a>
{
	/*
	 For example: struct fi_info *hints, *fi; hints = fi_allocinfo(); hints->ep_attr->type = FI_RDM; hints->caps = FI_MSG | FI_TAGGED | FI_RMA; hints->mode = FI_CONTEXT; hints->fabric_attr->prov_name = strdup(“psm2”); err = fi_getinfo(FI_VERSION(1,0), NULL, NULL, 0, hints, &fi);
	 
	
	pub src_addrlen: usize,
	pub dest_addrlen: usize,
	pub src_addr: *mut c_void,
	pub dest_addr: *mut c_void,
	
	pub handle: fid_t,
	pub tx_attr: *mut fi_tx_attr,
	pub rx_attr: *mut fi_rx_attr,
	pub ep_attr: *mut fi_ep_attr,
	pub domain_attr: *mut fi_domain_attr,
	
	*/
	pub fn discoverProviders(ourVersion: Version) -> Option<Provider<'a>>
	{
		let mut firstProvider: *mut fi_info = unsafe { uninitialized() };
		
		panic_on_error!("fi_getinfo", unsafe { fi_getinfo(ourVersion.toCombinedValue(), null_mut(), null_mut(), 0, null_mut(), &mut firstProvider) });
		
		if unlikely(firstProvider.is_null())
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					handle: firstProvider,
					weShouldFreeHandle: firstProvider,
					phantomData: PhantomData,
				}
			)
		}
	}
	
	pub fn next_move(self) -> Option<Provider<'a>>
	{
		let next = unsafe { *self.handle }.next;
		
		if unlikely(next.is_null())
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					handle: next,
					weShouldFreeHandle: self.handle,
					phantomData: PhantomData,
				}
			)
		}
	}
	
	pub fn next<'b>(&'a self) -> Option<Provider<'b>>
	where 'a : 'b
	{
		let next = unsafe { *self.handle }.next;
		
		if unlikely(next.is_null())
		{
			None
		}
		else
		{
			Some
			(
				Self
				{
					handle: next,
					weShouldFreeHandle: null_mut(),
					phantomData: PhantomData,
				}
			)
		}
	}
	
	pub fn createFabric(&self) -> Fabric
	{
		let mut handle: *mut fid_fabric = unsafe { uninitialized() };
		let userSpecifiedContextReturnedWithAsynchronousEvents = null_mut();
		panic_on_error!("fi_fabric", unsafe { fi_fabric((*self.handle).fabric_attr, &mut handle, userSpecifiedContextReturnedWithAsynchronousEvents) });
		
		Fabric::fromHandle(handle)
	}
	
	// Look at fi_getinfo.3.md
	fn supportsCapability(&self, capabilityBit: c_ulonglong) -> bool
	{
		unsafe { *self.handle }.caps & capabilityBit == capabilityBit
	}
	
	pub fn supportsPrimaryCapabilitySendingAndReceivingMessages(&self) -> bool
	{
		self.supportsCapability(FI_MSG as c_ulonglong)
	}
	
	pub fn supportsPrimaryCapabilityRemoteMemoryReadAndWrite(&self) -> bool
	{
		self.supportsCapability(FI_RMA as c_ulonglong)
	}
	
	pub fn supportsPrimaryCapabilitySendingAndReceivingTaggedMessages(&self) -> bool
	{
		self.supportsCapability(FI_TAGGED as c_ulonglong)
	}
	
	pub fn supportsPrimaryCapabilityAtomicOperations(&self) -> bool
	{
		self.supportsCapability(FI_ATOMICS as c_ulonglong)
	}
	
	pub fn supportsPrimaryCapabilitySendingToANamedReceiveContext(&self) -> bool
	{
		self.supportsCapability(FI_NAMED_RX_CTX as c_ulonglong)
	}
	
	/// equivalent of TCP receive offload / RETA
	pub fn supportsPrimaryCapabilityCanUseSourceAddressToDirectMessageToAReceiveBuffer(&self) -> bool
	{
		self.supportsCapability(FI_DIRECTED_RECV as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilityMultiReceive(&self) -> bool
	{
		self.supportsCapability(FI_MULTI_RECV as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilitySource(&self) -> bool
	{
		self.supportsCapability(FI_SOURCE as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilityRemoteMemoryCompletionEvents(&self) -> bool
	{
		self.supportsCapability(FI_RMA_EVENT as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilitySharedAddressVectors(&self) -> bool
	{
		self.supportsCapability(FI_SHARED_AV as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilityTrigger(&self) -> bool
	{
		self.supportsCapability(FI_TRIGGER as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilityAtomicFence(&self) -> bool
	{
		self.supportsCapability(FI_FENCE as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilityLocalCommunication(&self) -> bool
	{
		self.supportsCapability(FI_LOCAL_COMM as c_ulonglong)
	}
	
	pub fn supportsSecondaryCapabilityRemoteCommunication(&self) -> bool
	{
		self.supportsCapability(FI_REMOTE_COMM as c_ulonglong)
	}
	
	// Look at fi_getinfo.3.md
	fn applicationMustSupplySupportFor(&self, modeBit: c_ulonglong) -> bool
	{
		unsafe { *self.handle }.mode & modeBit == modeBit
	}
	
	pub fn applicationMustSupplySupportForContext(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_CONTEXT)
	}
	
	pub fn applicationMustSupplySupportForMessagePrefix(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_MSG_PREFIX)
	}
	
	pub fn applicationMustSupplySupportForAsynchronousIoVector(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_ASYNC_IOV)
	}
	
	pub fn applicationMustSupplySupportForReceiveCompletionQueueData(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_RX_CQ_DATA)
	}
	
	pub fn applicationMustSupplySupportForLocalMemoryRegion(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_LOCAL_MR)
	}
	
	pub fn applicationMustSupplySupportForNotifyFlagsOnly(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_NOTIFY_FLAGS_ONLY)
	}
	
	/// This bit indicates that the domain limits completion queues and counters to only be used with endpoints, transmit contexts, and receive contexts that have the same set of capability flags
	pub fn applicationIsLimitedToASubSetOfTheDomainFeaturesWhenUsingCompletionQueuesAndCompletionCounters(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_RESTRICTED_COMP)
	}
	
	pub fn applicationMustSupplySupportForContext2(&self) -> bool
	{
		self.applicationMustSupplySupportFor(FI_CONTEXT2)
	}
	
	pub fn transportAddressFormat(&self) -> TransportAddressFormat
	{
		unsafe { transmute((*self.handle).addr_format) }
	}
}
