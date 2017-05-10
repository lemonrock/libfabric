// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Provider(*mut fi_info);

impl Drop for Provider
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { fi_freeinfo(self.0) }
	}
}

impl Clone for Provider
{
	#[inline(always)]
	fn clone(&self) -> Provider
	{
		Provider::fromHandle(Self::duplicate(self.0))
	}
}

impl Provider
{
	#[inline(always)]
	fn fromHandle(handle: *mut fi_info) -> Provider
	{
		debug_assert!(!handle.is_null(), "handle is null");
		
		Provider(handle)
	}
	
	#[inline(always)]
	fn duplicate(handle: *mut fi_info) -> *mut fi_info
	{
		let duplicate = unsafe { fi_dupinfo(handle) };
		if unlikely(duplicate.is_null())
		{
			panic!("Could not duplicate, probably due to lack of memory");
		}
		duplicate
	}
	
	#[inline(always)]
	pub fn allocateForHints() -> Provider
	{
		let handle = unsafe { rust_fi_allocinfo() };
		if unlikely(handle.is_null())
		{
			panic!("Could not allocate, probably due to lack of memory");
		}
		Provider(handle)
	}
	
	pub fn createFabric(&self) -> Fabric
	{
		let mut handle: *mut fid_fabric = unsafe { uninitialized() };
		let userSpecifiedContextReturnedWithAsynchronousEvents = null_mut();
		panic_on_error!("fi_fabric", unsafe { fi_fabric((*self.0).fabric_attr, &mut handle, userSpecifiedContextReturnedWithAsynchronousEvents) });
		
		Fabric::fromHandle(handle)
	}
	
	// Look at fi_getinfo.3.md
	fn supportsCapability(&self, capabilityBit: c_ulonglong) -> bool
	{
		unsafe { *self.0 }.caps & capabilityBit == capabilityBit
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
		unsafe { *self.0 }.mode & modeBit == modeBit
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
		unsafe { transmute((*self.0).addr_format) }
	}
}
