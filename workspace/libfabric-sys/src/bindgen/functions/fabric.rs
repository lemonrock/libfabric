// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn fi_fabric(attr: *mut fi_fabric_attr, fabric: *mut *mut fid_fabric, context: *mut c_void) -> c_int;
	pub fn rust_fi_domain(fabric: *mut fid_fabric, info: *mut fi_info, domain: *mut *mut fid_domain, context: *mut c_void) -> c_int;
	pub fn rust_fi_passive_ep(fabric: *mut fid_fabric, info: *mut fi_info, pep: *mut *mut fid_pep, context: *mut c_void) -> c_int;
	pub fn rust_fi_trywait(fabric: *mut fid_fabric, fids: *mut *mut fid, count: c_int) -> c_int;
	pub fn rust_fi_wait_open(fabric: *mut fid_fabric, attr: *mut fi_wait_attr, waitset: *mut *mut fid_wait) -> c_int;
}
