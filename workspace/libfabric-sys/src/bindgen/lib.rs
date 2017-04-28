// This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.



extern crate libc;


use ::core::clone::Clone;
use ::core::fmt::Debug;
use ::core::fmt::Formatter;
use ::core::fmt::Result;
use ::core::marker::Copy;
use ::core::marker::PhantomData;
use ::core::mem::transmute;
use ::core::mem::zeroed;
use ::core::option::Option;
use ::core::slice::from_raw_parts;
use ::core::slice::from_raw_parts_mut;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::c_ulonglong;
use ::libc::c_void;


#[macro_use] extern crate rust_c;


use ::libc::iovec;
use ::libc::pthread_cond_t;
use ::libc::pthread_mutex_t;

#[link(name = "libfabric", kind = "static-nobundle")]
extern "C"
{
}

include!("bindgen/constants.rs");
include!("bindgen/types.rs");
include!("bindgen/enums.rs");
include!("bindgen/structs.rs");
include!("bindgen/unions.rs");
include!("bindgen/statics.rs");
include!("bindgen/functions.rs");
include!("bindgen/opaques.rs");

c!
{
	#include "rust_types.h"
	#include <libfabric.h>
	#include <stdlib.h>
}

c!
{
	#[inline(always)]
	fn rust_fi_atomic(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", count: usize as "size_t", desc: *mut c_void as "void *", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_atomic(ep, buf, count, desc, dest_addr, addr, key, datatype, op, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_atomicv(ep: *mut fid_ep as "struct fid_ep *", iov: *const fi_ioc as "const struct fi_ioc *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_atomicv(ep, iov, desc, count, dest_addr, addr, key, datatype, op, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_atomicmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg_atomic as "const struct fi_msg_atomic *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_atomicmsg(ep, msg, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_inject_atomic(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", count: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op") -> isize as "ssize_t"
	{
		return fi_inject_atomic(ep, buf, count, dest_addr, addr, key, datatype, op);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_fetch_atomic(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", count: usize as "size_t", desc: *mut c_void as "void *", result: *mut c_void as "void *", result_desc: *mut c_void as "void *", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_fetch_atomic(ep, buf, count, desc, result, result_desc, dest_addr, addr, key, datatype, op, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_fetch_atomicv(ep: *mut fid_ep as "struct fid_ep *", iov: *const fi_ioc as "const struct fi_ioc *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", resultv: *mut fi_ioc as "struct fi_ioc *", result_desc: *mut *mut c_void as "void * *", result_count: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_fetch_atomicv(ep, iov, desc, count, resultv, result_desc, result_count, dest_addr, addr, key, datatype, op, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_fetch_atomicmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg_atomic as "const struct fi_msg_atomic *", resultv: *mut fi_ioc as "struct fi_ioc *", result_desc: *mut *mut c_void as "void * *", result_count: usize as "size_t", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_fetch_atomicmsg(ep, msg, resultv, result_desc, result_count, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_compare_atomic(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", count: usize as "size_t", desc: *mut c_void as "void *", compare: *const c_void as "const void *", compare_desc: *mut c_void as "void *", result: *mut c_void as "void *", result_desc: *mut c_void as "void *", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_compare_atomic(ep, buf, count, desc, compare, compare_desc, result, result_desc, dest_addr, addr, key, datatype, op, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_compare_atomicv(ep: *mut fid_ep as "struct fid_ep *", iov: *const fi_ioc as "const struct fi_ioc *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", comparev: *const fi_ioc as "const struct fi_ioc *", compare_desc: *mut *mut c_void as "void * *", compare_count: usize as "size_t", resultv: *mut fi_ioc as "struct fi_ioc *", result_desc: *mut *mut c_void as "void * *", result_count: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_compare_atomicv(ep, iov, desc, count, comparev, compare_desc, compare_count, resultv, result_desc, result_count, dest_addr, addr, key, datatype, op, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_compare_atomicmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg_atomic as "const struct fi_msg_atomic *", comparev: *const fi_ioc as "const struct fi_ioc *", compare_desc: *mut *mut c_void as "void * *", compare_count: usize as "size_t", resultv: *mut fi_ioc as "struct fi_ioc *", result_desc: *mut *mut c_void as "void * *", result_count: usize as "size_t", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_compare_atomicmsg(ep, msg, comparev, compare_desc, compare_count, resultv, result_desc, result_count, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_atomicvalid(ep: *mut fid_ep as "struct fid_ep *", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", count: *mut usize as "size_t *") -> c_int as "int"
	{
		return fi_atomicvalid(ep, datatype, op, count);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_fetch_atomicvalid(ep: *mut fid_ep as "struct fid_ep *", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", count: *mut usize as "size_t *") -> c_int as "int"
	{
		return fi_fetch_atomicvalid(ep, datatype, op, count);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_compare_atomicvalid(ep: *mut fid_ep as "struct fid_ep *", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", count: *mut usize as "size_t *") -> c_int as "int"
	{
		return fi_compare_atomicvalid(ep, datatype, op, count);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_query_atomic(domain: *mut fid_domain as "struct fid_domain *", datatype: fi_datatype as "enum fi_datatype", op: fi_op as "enum fi_op", attr: *mut fi_atomic_attr as "struct fi_atomic_attr *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_query_atomic(domain, datatype, op, attr, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_setname(fid: fid_t as "fid_t", addr: *mut c_void as "void *", addrlen: usize as "size_t") -> c_int as "int"
	{
		return fi_setname(fid, addr, addrlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_getname(fid: fid_t as "fid_t", addr: *mut c_void as "void *", addrlen: *mut usize as "size_t *") -> c_int as "int"
	{
		return fi_getname(fid, addr, addrlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_getpeer(ep: *mut fid_ep as "struct fid_ep *", addr: *mut c_void as "void *", addrlen: *mut usize as "size_t *") -> c_int as "int"
	{
		return fi_getpeer(ep, addr, addrlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_listen(pep: *mut fid_pep as "struct fid_pep *") -> c_int as "int"
	{
		return fi_listen(pep);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_connect(ep: *mut fid_ep as "struct fid_ep *", addr: *const c_void as "const void *", param: *const c_void as "const void *", paramlen: usize as "size_t") -> c_int as "int"
	{
		return fi_connect(ep, addr, param, paramlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_accept(ep: *mut fid_ep as "struct fid_ep *", param: *const c_void as "const void *", paramlen: usize as "size_t") -> c_int as "int"
	{
		return fi_accept(ep, param, paramlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_reject(pep: *mut fid_pep as "struct fid_pep *", handle: fid_t as "fid_t", param: *const c_void as "const void *", paramlen: usize as "size_t") -> c_int as "int"
	{
		return fi_reject(pep, handle, param, paramlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_shutdown(ep: *mut fid_ep as "struct fid_ep *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_shutdown(ep, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_join(ep: *mut fid_ep as "struct fid_ep *", addr: *const c_void as "const void *", flags: u64 as "uint64_t", mc: *mut *mut fid_mc as "struct fid_mc * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_join(ep, addr, flags, mc, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mc_addr(mc: *mut fid_mc as "struct fid_mc *") -> fi_addr_t as "fi_addr_t"
	{
		return fi_mc_addr(mc);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_domain(fabric: *mut fid_fabric as "struct fid_fabric *", info: *mut fi_info as "struct fi_info *", domain: *mut *mut fid_domain as "struct fid_domain * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_domain(fabric, info, domain, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_domain_bind(domain: *mut fid_domain as "struct fid_domain *", fid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_domain_bind(domain, fid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_open(domain: *mut fid_domain as "struct fid_domain *", attr: *mut fi_cq_attr as "struct fi_cq_attr *", cq: *mut *mut fid_cq as "struct fid_cq * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_cq_open(domain, attr, cq, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_open(domain: *mut fid_domain as "struct fid_domain *", attr: *mut fi_cntr_attr as "struct fi_cntr_attr *", cntr: *mut *mut fid_cntr as "struct fid_cntr * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_cntr_open(domain, attr, cntr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_wait_open(fabric: *mut fid_fabric as "struct fid_fabric *", attr: *mut fi_wait_attr as "struct fi_wait_attr *", waitset: *mut *mut fid_wait as "struct fid_wait * *") -> c_int as "int"
	{
		return fi_wait_open(fabric, attr, waitset);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_poll_open(domain: *mut fid_domain as "struct fid_domain *", attr: *mut fi_poll_attr as "struct fi_poll_attr *", pollset: *mut *mut fid_poll as "struct fid_poll * *") -> c_int as "int"
	{
		return fi_poll_open(domain, attr, pollset);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_reg(domain: *mut fid_domain as "struct fid_domain *", buf: *const c_void as "const void *", len: usize as "size_t", access: u64 as "uint64_t", offset: u64 as "uint64_t", requested_key: u64 as "uint64_t", flags: u64 as "uint64_t", mr: *mut *mut fid_mr as "struct fid_mr * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_mr_reg(domain, buf, len, access, offset, requested_key, flags, mr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_regv(domain: *mut fid_domain as "struct fid_domain *", iov: *const iovec as "const struct iovec *", count: usize as "size_t", access: u64 as "uint64_t", offset: u64 as "uint64_t", requested_key: u64 as "uint64_t", flags: u64 as "uint64_t", mr: *mut *mut fid_mr as "struct fid_mr * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_mr_regv(domain, iov, count, access, offset, requested_key, flags, mr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_regattr(domain: *mut fid_domain as "struct fid_domain *", attr: *const fi_mr_attr as "const struct fi_mr_attr *", flags: u64 as "uint64_t", mr: *mut *mut fid_mr as "struct fid_mr * *") -> c_int as "int"
	{
		return fi_mr_regattr(domain, attr, flags, mr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_desc(mr: *mut fid_mr as "struct fid_mr *") -> *mut c_void as "void *"
	{
		return fi_mr_desc(mr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_key(mr: *mut fid_mr as "struct fid_mr *") -> u64 as "uint64_t"
	{
		return fi_mr_key(mr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_raw_attr(mr: *mut fid_mr as "struct fid_mr *", base_addr: *mut u64 as "uint64_t *", raw_key: *mut u8 as "uint8_t *", key_size: *mut usize as "size_t *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_mr_raw_attr(mr, base_addr, raw_key, key_size, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_map_raw(domain: *mut fid_domain as "struct fid_domain *", base_addr: u64 as "uint64_t", raw_key: *mut u8 as "uint8_t *", key_size: usize as "size_t", key: *mut u64 as "uint64_t *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_mr_map_raw(domain, base_addr, raw_key, key_size, key, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_unmap_key(domain: *mut fid_domain as "struct fid_domain *", key: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_mr_unmap_key(domain, key);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_bind(mr: *mut fid_mr as "struct fid_mr *", bfid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_mr_bind(mr, bfid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_refresh(mr: *mut fid_mr as "struct fid_mr *", iov: *const iovec as "const struct iovec *", count: usize as "size_t", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_mr_refresh(mr, iov, count, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_mr_enable(mr: *mut fid_mr as "struct fid_mr *") -> c_int as "int"
	{
		return fi_mr_enable(mr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_open(domain: *mut fid_domain as "struct fid_domain *", attr: *mut fi_av_attr as "struct fi_av_attr *", av: *mut *mut fid_av as "struct fid_av * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_av_open(domain, attr, av, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_bind(av: *mut fid_av as "struct fid_av *", fid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_av_bind(av, fid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_insert(av: *mut fid_av as "struct fid_av *", addr: *const c_void as "const void *", count: usize as "size_t", fi_addr: *mut fi_addr_t as "fi_addr_t *", flags: u64 as "uint64_t", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_av_insert(av, addr, count, fi_addr, flags, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_insertsvc(av: *mut fid_av as "struct fid_av *", node: *const c_char as "const char *", service: *const c_char as "const char *", fi_addr: *mut fi_addr_t as "fi_addr_t *", flags: u64 as "uint64_t", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_av_insertsvc(av, node, service, fi_addr, flags, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_insertsym(av: *mut fid_av as "struct fid_av *", node: *const c_char as "const char *", nodecnt: usize as "size_t", service: *const c_char as "const char *", svccnt: usize as "size_t", fi_addr: *mut fi_addr_t as "fi_addr_t *", flags: u64 as "uint64_t", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_av_insertsym(av, node, nodecnt, service, svccnt, fi_addr, flags, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_remove(av: *mut fid_av as "struct fid_av *", fi_addr: *mut fi_addr_t as "fi_addr_t *", count: usize as "size_t", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_av_remove(av, fi_addr, count, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_lookup(av: *mut fid_av as "struct fid_av *", fi_addr: fi_addr_t as "fi_addr_t", addr: *mut c_void as "void *", addrlen: *mut usize as "size_t *") -> c_int as "int"
	{
		return fi_av_lookup(av, fi_addr, addr, addrlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_av_straddr(av: *mut fid_av as "struct fid_av *", addr: *const c_void as "const void *", buf: *mut c_char as "char *", len: *mut usize as "size_t *") -> *const c_char as "const char *"
	{
		return fi_av_straddr(av, addr, buf, len);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_rx_addr(fi_addr: fi_addr_t as "fi_addr_t", rx_index: c_int as "int", rx_ctx_bits: c_int as "int") -> fi_addr_t as "fi_addr_t"
	{
		return fi_rx_addr(fi_addr, rx_index, rx_ctx_bits);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_passive_ep(fabric: *mut fid_fabric as "struct fid_fabric *", info: *mut fi_info as "struct fi_info *", pep: *mut *mut fid_pep as "struct fid_pep * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_passive_ep(fabric, info, pep, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_endpoint(domain: *mut fid_domain as "struct fid_domain *", info: *mut fi_info as "struct fi_info *", ep: *mut *mut fid_ep as "struct fid_ep * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_endpoint(domain, info, ep, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_scalable_ep(domain: *mut fid_domain as "struct fid_domain *", info: *mut fi_info as "struct fi_info *", sep: *mut *mut fid_ep as "struct fid_ep * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_scalable_ep(domain, info, sep, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_ep_bind(ep: *mut fid_ep as "struct fid_ep *", bfid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_ep_bind(ep, bfid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_pep_bind(pep: *mut fid_pep as "struct fid_pep *", bfid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_pep_bind(pep, bfid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_scalable_ep_bind(sep: *mut fid_ep as "struct fid_ep *", bfid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_scalable_ep_bind(sep, bfid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_enable(ep: *mut fid_ep as "struct fid_ep *") -> c_int as "int"
	{
		return fi_enable(ep);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cancel(fid: fid_t as "fid_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_cancel(fid, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_setopt(fid: fid_t as "fid_t", level: c_int as "int", optname: c_int as "int", optval: *const c_void as "const void *", optlen: usize as "size_t") -> c_int as "int"
	{
		return fi_setopt(fid, level, optname, optval, optlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_getopt(fid: fid_t as "fid_t", level: c_int as "int", optname: c_int as "int", optval: *mut c_void as "void *", optlen: *mut usize as "size_t *") -> c_int as "int"
	{
		return fi_getopt(fid, level, optname, optval, optlen);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_ep_alias(ep: *mut fid_ep as "struct fid_ep *", alias_ep: *mut *mut fid_ep as "struct fid_ep * *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_ep_alias(ep, alias_ep, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_tx_context(ep: *mut fid_ep as "struct fid_ep *", index: c_int as "int", attr: *mut fi_tx_attr as "struct fi_tx_attr *", tx_ep: *mut *mut fid_ep as "struct fid_ep * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_tx_context(ep, index, attr, tx_ep, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_rx_context(ep: *mut fid_ep as "struct fid_ep *", index: c_int as "int", attr: *mut fi_rx_attr as "struct fi_rx_attr *", rx_ep: *mut *mut fid_ep as "struct fid_ep * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_rx_context(ep, index, attr, rx_ep, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_stx_context(domain: *mut fid_domain as "struct fid_domain *", attr: *mut fi_tx_attr as "struct fi_tx_attr *", stx: *mut *mut fid_stx as "struct fid_stx * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_stx_context(domain, attr, stx, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_srx_context(domain: *mut fid_domain as "struct fid_domain *", attr: *mut fi_rx_attr as "struct fi_rx_attr *", rx_ep: *mut *mut fid_ep as "struct fid_ep * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_srx_context(domain, attr, rx_ep, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_recv(ep: *mut fid_ep as "struct fid_ep *", buf: *mut c_void as "void *", len: usize as "size_t", desc: *mut c_void as "void *", src_addr: fi_addr_t as "fi_addr_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_recv(ep, buf, len, desc, src_addr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_recvv(ep: *mut fid_ep as "struct fid_ep *", iov: *const iovec as "const struct iovec *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", src_addr: fi_addr_t as "fi_addr_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_recvv(ep, iov, desc, count, src_addr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_recvmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg as "const struct fi_msg *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_recvmsg(ep, msg, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_send(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", desc: *mut c_void as "void *", dest_addr: fi_addr_t as "fi_addr_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_send(ep, buf, len, desc, dest_addr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_sendv(ep: *mut fid_ep as "struct fid_ep *", iov: *const iovec as "const struct iovec *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_sendv(ep, iov, desc, count, dest_addr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_sendmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg as "const struct fi_msg *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_sendmsg(ep, msg, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_inject(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t") -> isize as "ssize_t"
	{
		return fi_inject(ep, buf, len, dest_addr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_senddata(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", desc: *mut c_void as "void *", data: u64 as "uint64_t", dest_addr: fi_addr_t as "fi_addr_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_senddata(ep, buf, len, desc, data, dest_addr, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_injectdata(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", data: u64 as "uint64_t", dest_addr: fi_addr_t as "fi_addr_t") -> isize as "ssize_t"
	{
		return fi_injectdata(ep, buf, len, data, dest_addr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_trywait(fabric: *mut fid_fabric as "struct fid_fabric *", fids: *mut *mut fid as "struct fid * *", count: c_int as "int") -> c_int as "int"
	{
		return fi_trywait(fabric, fids, count);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_wait(waitset: *mut fid_wait as "struct fid_wait *", timeout: c_int as "int") -> c_int as "int"
	{
		return fi_wait(waitset, timeout);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_poll(pollset: *mut fid_poll as "struct fid_poll *", context: *mut *mut c_void as "void * *", count: c_int as "int") -> c_int as "int"
	{
		return fi_poll(pollset, context, count);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_poll_add(pollset: *mut fid_poll as "struct fid_poll *", event_fid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_poll_add(pollset, event_fid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_poll_del(pollset: *mut fid_poll as "struct fid_poll *", event_fid: *mut fid as "struct fid *", flags: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_poll_del(pollset, event_fid, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_eq_open(fabric: *mut fid_fabric as "struct fid_fabric *", attr: *mut fi_eq_attr as "struct fi_eq_attr *", eq: *mut *mut fid_eq as "struct fid_eq * *", context: *mut c_void as "void *") -> c_int as "int"
	{
		return fi_eq_open(fabric, attr, eq, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_eq_read(eq: *mut fid_eq as "struct fid_eq *", event: *mut u32 as "uint32_t *", buf: *mut c_void as "void *", len: usize as "size_t", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_eq_read(eq, event, buf, len, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_eq_readerr(eq: *mut fid_eq as "struct fid_eq *", buf: *mut fi_eq_err_entry as "struct fi_eq_err_entry *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_eq_readerr(eq, buf, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_eq_write(eq: *mut fid_eq as "struct fid_eq *", event: u32 as "uint32_t", buf: *const c_void as "const void *", len: usize as "size_t", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_eq_write(eq, event, buf, len, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_eq_sread(eq: *mut fid_eq as "struct fid_eq *", event: *mut u32 as "uint32_t *", buf: *mut c_void as "void *", len: usize as "size_t", timeout: c_int as "int", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_eq_sread(eq, event, buf, len, timeout, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_eq_strerror(eq: *mut fid_eq as "struct fid_eq *", prov_errno: c_int as "int", err_data: *const c_void as "const void *", buf: *mut c_char as "char *", len: usize as "size_t") -> *const c_char as "const char *"
	{
		return fi_eq_strerror(eq, prov_errno, err_data, buf, len);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_read(cq: *mut fid_cq as "struct fid_cq *", buf: *mut c_void as "void *", count: usize as "size_t") -> isize as "ssize_t"
	{
		return fi_cq_read(cq, buf, count);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_readfrom(cq: *mut fid_cq as "struct fid_cq *", buf: *mut c_void as "void *", count: usize as "size_t", src_addr: *mut fi_addr_t as "fi_addr_t *") -> isize as "ssize_t"
	{
		return fi_cq_readfrom(cq, buf, count, src_addr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_readerr(cq: *mut fid_cq as "struct fid_cq *", buf: *mut fi_cq_err_entry as "struct fi_cq_err_entry *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_cq_readerr(cq, buf, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_sread(cq: *mut fid_cq as "struct fid_cq *", buf: *mut c_void as "void *", count: usize as "size_t", cond: *const c_void as "const void *", timeout: c_int as "int") -> isize as "ssize_t"
	{
		return fi_cq_sread(cq, buf, count, cond, timeout);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_sreadfrom(cq: *mut fid_cq as "struct fid_cq *", buf: *mut c_void as "void *", count: usize as "size_t", src_addr: *mut fi_addr_t as "fi_addr_t *", cond: *const c_void as "const void *", timeout: c_int as "int") -> isize as "ssize_t"
	{
		return fi_cq_sreadfrom(cq, buf, count, src_addr, cond, timeout);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_signal(cq: *mut fid_cq as "struct fid_cq *") -> c_int as "int"
	{
		return fi_cq_signal(cq);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cq_strerror(cq: *mut fid_cq as "struct fid_cq *", prov_errno: c_int as "int", err_data: *const c_void as "const void *", buf: *mut c_char as "char *", len: usize as "size_t") -> *const c_char as "const char *"
	{
		return fi_cq_strerror(cq, prov_errno, err_data, buf, len);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_read(cntr: *mut fid_cntr as "struct fid_cntr *") -> u64 as "uint64_t"
	{
		return fi_cntr_read(cntr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_readerr(cntr: *mut fid_cntr as "struct fid_cntr *") -> u64 as "uint64_t"
	{
		return fi_cntr_readerr(cntr);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_add(cntr: *mut fid_cntr as "struct fid_cntr *", value: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_cntr_add(cntr, value);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_adderr(cntr: *mut fid_cntr as "struct fid_cntr *", value: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_cntr_adderr(cntr, value);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_set(cntr: *mut fid_cntr as "struct fid_cntr *", value: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_cntr_set(cntr, value);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_seterr(cntr: *mut fid_cntr as "struct fid_cntr *", value: u64 as "uint64_t") -> c_int as "int"
	{
		return fi_cntr_seterr(cntr, value);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_cntr_wait(cntr: *mut fid_cntr as "struct fid_cntr *", threshold: u64 as "uint64_t", timeout: c_int as "int") -> c_int as "int"
	{
		return fi_cntr_wait(cntr, threshold, timeout);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_read(ep: *mut fid_ep as "struct fid_ep *", buf: *mut c_void as "void *", len: usize as "size_t", desc: *mut c_void as "void *", src_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_read(ep, buf, len, desc, src_addr, addr, key, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_readv(ep: *mut fid_ep as "struct fid_ep *", iov: *const iovec as "const struct iovec *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", src_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_readv(ep, iov, desc, count, src_addr, addr, key, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_readmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg_rma as "const struct fi_msg_rma *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_readmsg(ep, msg, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_write(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", desc: *mut c_void as "void *", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_write(ep, buf, len, desc, dest_addr, addr, key, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_writev(ep: *mut fid_ep as "struct fid_ep *", iov: *const iovec as "const struct iovec *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_writev(ep, iov, desc, count, dest_addr, addr, key, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_writemsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg_rma as "const struct fi_msg_rma *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_writemsg(ep, msg, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_inject_write(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_inject_write(ep, buf, len, dest_addr, addr, key);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_writedata(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", desc: *mut c_void as "void *", data: u64 as "uint64_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_writedata(ep, buf, len, desc, data, dest_addr, addr, key, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_inject_writedata(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", data: u64 as "uint64_t", dest_addr: fi_addr_t as "fi_addr_t", addr: u64 as "uint64_t", key: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_inject_writedata(ep, buf, len, data, dest_addr, addr, key);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_trecv(ep: *mut fid_ep as "struct fid_ep *", buf: *mut c_void as "void *", len: usize as "size_t", desc: *mut c_void as "void *", src_addr: fi_addr_t as "fi_addr_t", tag: u64 as "uint64_t", ignore: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_trecv(ep, buf, len, desc, src_addr, tag, ignore, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_trecvv(ep: *mut fid_ep as "struct fid_ep *", iov: *const iovec as "const struct iovec *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", src_addr: fi_addr_t as "fi_addr_t", tag: u64 as "uint64_t", ignore: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_trecvv(ep, iov, desc, count, src_addr, tag, ignore, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_trecvmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg_tagged as "const struct fi_msg_tagged *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_trecvmsg(ep, msg, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_tsend(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", desc: *mut c_void as "void *", dest_addr: fi_addr_t as "fi_addr_t", tag: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_tsend(ep, buf, len, desc, dest_addr, tag, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_tsendv(ep: *mut fid_ep as "struct fid_ep *", iov: *const iovec as "const struct iovec *", desc: *mut *mut c_void as "void * *", count: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", tag: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_tsendv(ep, iov, desc, count, dest_addr, tag, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_tsendmsg(ep: *mut fid_ep as "struct fid_ep *", msg: *const fi_msg_tagged as "const struct fi_msg_tagged *", flags: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_tsendmsg(ep, msg, flags);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_tinject(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", dest_addr: fi_addr_t as "fi_addr_t", tag: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_tinject(ep, buf, len, dest_addr, tag);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_tsenddata(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", desc: *mut c_void as "void *", data: u64 as "uint64_t", dest_addr: fi_addr_t as "fi_addr_t", tag: u64 as "uint64_t", context: *mut c_void as "void *") -> isize as "ssize_t"
	{
		return fi_tsenddata(ep, buf, len, desc, data, dest_addr, tag, context);
	}
}

c!
{
	#[inline(always)]
	fn rust_fi_tinjectdata(ep: *mut fid_ep as "struct fid_ep *", buf: *const c_void as "const void *", len: usize as "size_t", data: u64 as "uint64_t", dest_addr: fi_addr_t as "fi_addr_t", tag: u64 as "uint64_t") -> isize as "ssize_t"
	{
		return fi_tinjectdata(ep, buf, len, data, dest_addr, tag);
	}
}
