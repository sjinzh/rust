define_type! { struct u64x8([u64; 8]); }

/*
#[cfg(target_arch = "x86")]
from_aligned! { unsafe u64x8 |bidirectional| core::arch::x86::__m512i }

#[cfg(target_arch = "x86_64")]
from_aligned! { unsafe u64x8 |bidirectional| core::arch::x86_64::__m512i }
*/
