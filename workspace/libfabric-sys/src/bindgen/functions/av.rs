// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_av_bind(av: *mut fid_av, fid: *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_av_insert(av: *mut fid_av, addr: *const c_void, count: usize, fi_addr: *mut fi_addr_t, flags: u64, context: *mut c_void) -> c_int;
	pub fn rust_fi_av_insertsvc(av: *mut fid_av, node: *const c_char, service: *const c_char, fi_addr: *mut fi_addr_t, flags: u64, context: *mut c_void) -> c_int;
	pub fn rust_fi_av_insertsym(av: *mut fid_av, node: *const c_char, nodecnt: usize, service: *const c_char, svccnt: usize, fi_addr: *mut fi_addr_t, flags: u64, context: *mut c_void) -> c_int;
	pub fn rust_fi_av_lookup(av: *mut fid_av, fi_addr: fi_addr_t, addr: *mut c_void, addrlen: *mut usize) -> c_int;
	pub fn rust_fi_av_remove(av: *mut fid_av, fi_addr: *mut fi_addr_t, count: usize, flags: u64) -> c_int;
	pub fn rust_fi_av_straddr(av: *mut fid_av, addr: *const c_void, buf: *mut c_char, len: *mut usize) -> *const c_char;
}
