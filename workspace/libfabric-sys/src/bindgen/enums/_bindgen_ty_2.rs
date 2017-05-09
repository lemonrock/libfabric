// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_2
{
	FI_FORMAT_UNSPEC = 0,
	FI_SOCKADDR = 1,
	FI_SOCKADDR_IN = 2,
	FI_SOCKADDR_IN6 = 3,
	FI_SOCKADDR_IB = 4,
	FI_ADDR_PSMX = 5,
	FI_ADDR_GNI = 6,
	FI_ADDR_BGQ = 7,
	FI_ADDR_MLX = 8,
	FI_ADDR_STR = 9,
}
