// The MIT License (MIT)
//
// Copyright (c) 2015 Kashyap
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::spinlock::Spinlock;
use super::spinlock::dummy_lock;
use super::spinlock::init_lock;
use super::uart::uart_put_str;
use super::mmu::Address;
use super::mmu::pg_roundup;
use super::mmu::PG_SIZE;

struct kmem_t{
lock: Spinlock,
use_lock: u32 , //TODO is u32 the right type?
	    // TODO  struct run *freelist;
}


static mut kmem : kmem_t = kmem_t { lock: dummy_lock, use_lock: 0} ;


pub fn kinit1(vstart: Address, vend: Address) {
	unsafe {
		init_lock(& mut kmem.lock, "kmem");
		kmem.use_lock = 0;
	}
	free_range(vstart, vend);
}

fn free_range(vstart: Address, vend: Address) {
	let mut address = pg_roundup(vstart);
	loop {
		kfree(address);
		address = address + PG_SIZE;
		if address > vend {
			break;
		}
	}
}

fn kfree(a : Address){
/*
	struct run *r;

	if((uintp)v % PGSIZE || v < end || v2p(v) >= PHYSTOP)
		panic("kfree");

	// Fill with junk to catch dangling refs.
	memset(v, 1, PGSIZE);

	if(kmem.use_lock)
		acquire(&kmem.lock);
	r = (struct run*)v;
	r->next = kmem.freelist;
	kmem.freelist = r;
	if(kmem.use_lock)
		release(&kmem.lock);



		*/
}
