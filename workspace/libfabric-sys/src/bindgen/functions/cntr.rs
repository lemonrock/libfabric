// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_cntr_add(cntr: *mut fid_cntr, value: u64) -> c_int;
	pub fn rust_fi_cntr_adderr(cntr: *mut fid_cntr, value: u64) -> c_int;
	pub fn rust_fi_cntr_open(domain: *mut fid_domain, attr: *mut fi_cntr_attr, cntr: *mut *mut fid_cntr, context: *mut c_void) -> c_int;
	pub fn rust_fi_cntr_read(cntr: *mut fid_cntr) -> u64;
	pub fn rust_fi_cntr_readerr(cntr: *mut fid_cntr) -> u64;
	pub fn rust_fi_cntr_set(cntr: *mut fid_cntr, value: u64) -> c_int;
	pub fn rust_fi_cntr_seterr(cntr: *mut fid_cntr, value: u64) -> c_int;
	pub fn rust_fi_cntr_wait(cntr: *mut fid_cntr, threshold: u64, timeout: c_int) -> c_int;
}
