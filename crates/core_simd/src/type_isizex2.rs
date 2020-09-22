define_type! { struct isizex2([isize; 2]); }

#[cfg(all(target_arch = "x86", target_pointer_width = "64"))]
from_aligned! { unsafe isizex2 |bidirectional| core::arch::x86::__m128i }

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
from_aligned! { unsafe isizex2 |bidirectional| core::arch::x86_64::__m128i }
