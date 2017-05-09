// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_3
{
	FI_PROTO_UNSPEC = 0,
	FI_PROTO_RDMA_CM_IB_RC = 1,
	FI_PROTO_IWARP = 2,
	FI_PROTO_IB_UD = 3,
	FI_PROTO_PSMX = 4,
	FI_PROTO_UDP = 5,
	FI_PROTO_SOCK_TCP = 6,
	FI_PROTO_MXM = 7,
	FI_PROTO_IWARP_RDM = 8,
	FI_PROTO_IB_RDM = 9,
	FI_PROTO_GNI = 10,
	FI_PROTO_RXM = 11,
	FI_PROTO_RXD = 12,
	FI_PROTO_MLX = 13,
	FI_PROTO_NETWORKDIRECT = 14,
}
