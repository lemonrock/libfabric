// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_listen(pep: *mut fid_pep) -> c_int;
	pub fn rust_fi_pep_bind(pep: *mut fid_pep, bfid: *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_reject(pep: *mut fid_pep, handle: fid_t, param: *const c_void, paramlen: usize) -> c_int;
}
