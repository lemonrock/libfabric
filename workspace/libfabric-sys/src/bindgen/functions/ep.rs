// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


extern "C"
{
	pub fn rust_fi_accept(ep: *mut fid_ep, param: *const c_void, paramlen: usize) -> c_int;
	pub fn rust_fi_atomic(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_atomicmsg(ep: *mut fid_ep, msg: *const fi_msg_atomic, flags: u64) -> isize;
	pub fn rust_fi_atomicv(ep: *mut fid_ep, iov: *const fi_ioc, desc: *mut *mut c_void, count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_atomicvalid(ep: *mut fid_ep, datatype: fi_datatype, op: fi_op, count: *mut usize) -> c_int;
	pub fn rust_fi_compare_atomic(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, compare: *const c_void, compare_desc: *mut c_void, result: *mut c_void, result_desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_compare_atomicmsg(ep: *mut fid_ep, msg: *const fi_msg_atomic, comparev: *const fi_ioc, compare_desc: *mut *mut c_void, compare_count: usize, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, flags: u64) -> isize;
	pub fn rust_fi_compare_atomicv(ep: *mut fid_ep, iov: *const fi_ioc, desc: *mut *mut c_void, count: usize, comparev: *const fi_ioc, compare_desc: *mut *mut c_void, compare_count: usize, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_compare_atomicvalid(ep: *mut fid_ep, datatype: fi_datatype, op: fi_op, count: *mut usize) -> c_int;
	pub fn rust_fi_connect(ep: *mut fid_ep, addr: *const c_void, param: *const c_void, paramlen: usize) -> c_int;
	pub fn rust_fi_enable(ep: *mut fid_ep) -> c_int;
	pub fn rust_fi_ep_alias(ep: *mut fid_ep, alias_ep: *mut *mut fid_ep, flags: u64) -> c_int;
	pub fn rust_fi_ep_bind(ep: *mut fid_ep, bfid: *mut fid, flags: u64) -> c_int;
	pub fn rust_fi_fetch_atomic(ep: *mut fid_ep, buf: *const c_void, count: usize, desc: *mut c_void, result: *mut c_void, result_desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_fetch_atomicmsg(ep: *mut fid_ep, msg: *const fi_msg_atomic, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, flags: u64) -> isize;
	pub fn rust_fi_fetch_atomicv(ep: *mut fid_ep, iov: *const fi_ioc, desc: *mut *mut c_void, count: usize, resultv: *mut fi_ioc, result_desc: *mut *mut c_void, result_count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op, context: *mut c_void) -> isize;
	pub fn rust_fi_fetch_atomicvalid(ep: *mut fid_ep, datatype: fi_datatype, op: fi_op, count: *mut usize) -> c_int;
	pub fn rust_fi_getpeer(ep: *mut fid_ep, addr: *mut c_void, addrlen: *mut usize) -> c_int;
	pub fn rust_fi_inject(ep: *mut fid_ep, buf: *const c_void, len: usize, dest_addr: fi_addr_t) -> isize;
	pub fn rust_fi_inject_atomic(ep: *mut fid_ep, buf: *const c_void, count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, datatype: fi_datatype, op: fi_op) -> isize;
	pub fn rust_fi_inject_write(ep: *mut fid_ep, buf: *const c_void, len: usize, dest_addr: fi_addr_t, addr: u64, key: u64) -> isize;
	pub fn rust_fi_inject_writedata(ep: *mut fid_ep, buf: *const c_void, len: usize, data: u64, dest_addr: fi_addr_t, addr: u64, key: u64) -> isize;
	pub fn rust_fi_injectdata(ep: *mut fid_ep, buf: *const c_void, len: usize, data: u64, dest_addr: fi_addr_t) -> isize;
	pub fn rust_fi_join(ep: *mut fid_ep, addr: *const c_void, flags: u64, mc: *mut *mut fid_mc, context: *mut c_void) -> c_int;
	pub fn rust_fi_read(ep: *mut fid_ep, buf: *mut c_void, len: usize, desc: *mut c_void, src_addr: fi_addr_t, addr: u64, key: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_readmsg(ep: *mut fid_ep, msg: *const fi_msg_rma, flags: u64) -> isize;
	pub fn rust_fi_readv(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, src_addr: fi_addr_t, addr: u64, key: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_recv(ep: *mut fid_ep, buf: *mut c_void, len: usize, desc: *mut c_void, src_addr: fi_addr_t, context: *mut c_void) -> isize;
	pub fn rust_fi_recvmsg(ep: *mut fid_ep, msg: *const fi_msg, flags: u64) -> isize;
	pub fn rust_fi_recvv(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, src_addr: fi_addr_t, context: *mut c_void) -> isize;
	pub fn rust_fi_rx_context(ep: *mut fid_ep, index: c_int, attr: *mut fi_rx_attr, rx_ep: *mut *mut fid_ep, context: *mut c_void) -> c_int;
	pub fn rust_fi_send(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, dest_addr: fi_addr_t, context: *mut c_void) -> isize;
	pub fn rust_fi_senddata(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, data: u64, dest_addr: fi_addr_t, context: *mut c_void) -> isize;
	pub fn rust_fi_sendmsg(ep: *mut fid_ep, msg: *const fi_msg, flags: u64) -> isize;
	pub fn rust_fi_sendv(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, dest_addr: fi_addr_t, context: *mut c_void) -> isize;
	pub fn rust_fi_shutdown(ep: *mut fid_ep, flags: u64) -> c_int;
	pub fn rust_fi_tinject(ep: *mut fid_ep, buf: *const c_void, len: usize, dest_addr: fi_addr_t, tag: u64) -> isize;
	pub fn rust_fi_tinjectdata(ep: *mut fid_ep, buf: *const c_void, len: usize, data: u64, dest_addr: fi_addr_t, tag: u64) -> isize;
	pub fn rust_fi_trecv(ep: *mut fid_ep, buf: *mut c_void, len: usize, desc: *mut c_void, src_addr: fi_addr_t, tag: u64, ignore: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_trecvmsg(ep: *mut fid_ep, msg: *const fi_msg_tagged, flags: u64) -> isize;
	pub fn rust_fi_trecvv(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, src_addr: fi_addr_t, tag: u64, ignore: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_tsend(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, dest_addr: fi_addr_t, tag: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_tsenddata(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, data: u64, dest_addr: fi_addr_t, tag: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_tsendmsg(ep: *mut fid_ep, msg: *const fi_msg_tagged, flags: u64) -> isize;
	pub fn rust_fi_tsendv(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, dest_addr: fi_addr_t, tag: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_tx_context(ep: *mut fid_ep, index: c_int, attr: *mut fi_tx_attr, tx_ep: *mut *mut fid_ep, context: *mut c_void) -> c_int;
	pub fn rust_fi_write(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, dest_addr: fi_addr_t, addr: u64, key: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_writedata(ep: *mut fid_ep, buf: *const c_void, len: usize, desc: *mut c_void, data: u64, dest_addr: fi_addr_t, addr: u64, key: u64, context: *mut c_void) -> isize;
	pub fn rust_fi_writemsg(ep: *mut fid_ep, msg: *const fi_msg_rma, flags: u64) -> isize;
	pub fn rust_fi_writev(ep: *mut fid_ep, iov: *const iovec, desc: *mut *mut c_void, count: usize, dest_addr: fi_addr_t, addr: u64, key: u64, context: *mut c_void) -> isize;
}
