// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


pub const FI_AFFINITY: c_uint = 536870912;
pub const FI_ATOMIC: c_uint = 16;
pub const FI_ATOMICS: c_uint = 16;
pub const FI_COMPLETION: c_uint = 16777216;
pub const FI_DELIVERY_COMPLETE: c_uint = 268435456;
pub const FI_DIRECTED_RECV: c_ulonglong = 576460752303423488;
pub const FI_EVENT: c_uint = 16777216;
pub const FI_FENCE: c_uint = 2097152;
pub const FI_INJECT: c_uint = 33554432;
pub const FI_INJECT_COMPLETE: c_uint = 67108864;
pub const FI_LOCAL_COMM: c_ulonglong = 2251799813685248;
pub const FI_MORE: c_uint = 262144;
pub const FI_MSG: c_uint = 2;
pub const FI_MULTICAST: c_uint = 32;
pub const FI_MULTI_RECV: c_uint = 65536;
pub const FI_NAMED_RX_CTX: c_ulonglong = 288230376151711744;
pub const FI_NUMERICHOST: c_ulonglong = 36028797018963968;
pub const FI_PEEK: c_uint = 524288;
pub const FI_PROV_ATTR_ONLY: c_ulonglong = 18014398509481984;
pub const FI_PROV_SPECIFIC: u32 = 2147483648;
pub const FI_READ: c_uint = 256;
pub const FI_RECV: c_uint = 1024;
pub const FI_REMOTE_COMM: c_ulonglong = 4503599627370496;
pub const FI_REMOTE_CQ_DATA: c_uint = 131072;
pub const FI_REMOTE_READ: c_uint = 4096;
pub const FI_REMOTE_WRITE: c_uint = 8192;
pub const FI_RMA: c_uint = 4;
pub const FI_RMA_EVENT: c_ulonglong = 72057594037927936;
pub const FI_SEND: c_uint = 2048;
pub const FI_SHARED_AV: c_ulonglong = 9007199254740992;
pub const FI_SOURCE: c_ulonglong = 144115188075855872;
pub const FI_SOURCE_ERR: c_ulonglong = 1125899906842624;
pub const FI_TAGGED: c_uint = 8;
pub const FI_TRANSMIT: c_uint = 2048;
pub const FI_TRANSMIT_COMPLETE: c_uint = 134217728;
pub const FI_TRIGGER: c_uint = 1048576;
pub const FI_WRITE: c_uint = 512;
