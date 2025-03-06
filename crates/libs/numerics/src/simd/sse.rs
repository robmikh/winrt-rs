use core::arch::x86_64::{__m128, _mm_add_ss, _mm_castpd_ps, _mm_castps_pd, _mm_div_ps, _mm_load_sd, _mm_load_ss, _mm_movelh_ps, _mm_mul_ps, _mm_permute_ps, _mm_sqrt_ps, _mm_store_sd, _mm_store_ss};

use crate::Vector3;

pub type XMVectorInner = __m128;

pub fn load_float3(source: &Vector3) -> XMVectorInner {
    unsafe {
        let xy = _mm_castpd_ps(_mm_load_sd(source as *const _ as *const _));
        let z = _mm_load_ss(&source.Z);
        _mm_movelh_ps(xy, z)
    }
}

pub fn store_float3(dest: &mut Vector3, v: XMVectorInner) {
    unsafe {
        _mm_store_sd(dest as *mut _ as *mut _, _mm_castps_pd(v));
        const C1: i32 = _MM_SHUFFLE(2, 2, 2, 2);
        let z = XM_PERMUTE_PS::<C1>(v);
        _mm_store_ss(&mut dest.Z as *mut _ as *mut _, z);
    }
}

pub fn vec3_len(v: XMVectorInner) -> XMVectorInner {
    unsafe {
        let mut length_sq = _mm_mul_ps(v, v);
        const C1: i32 = _MM_SHUFFLE(1, 2, 1, 2);
        let mut temp = XM_PERMUTE_PS::<C1>(length_sq);
        length_sq = _mm_add_ss(length_sq, temp);
        const C2: i32 = _MM_SHUFFLE(1, 1, 1, 1);
        temp = XM_PERMUTE_PS::<C2>(temp);
        length_sq = _mm_add_ss(length_sq, temp);
        const C3: i32 = _MM_SHUFFLE(0, 0, 0, 0);
        length_sq = XM_PERMUTE_PS::<C3>(length_sq);
        length_sq = _mm_sqrt_ps(length_sq);
        length_sq
    }
}

pub fn vec_divide(v1: XMVectorInner, v2: XMVectorInner) -> XMVectorInner {
    unsafe {
        _mm_div_ps(v1, v2)
    }
}

fn XM_PERMUTE_PS<const C: i32>(v: __m128) -> __m128 {
    // TODO: Use _mm_shuffle_ps(v,v,c) on non-Intel
    unsafe {
        _mm_permute_ps::<C>(v)
    }
}

const fn _MM_SHUFFLE(fp3: i32, fp2: i32, fp1: i32, fp0: i32) -> i32 {
    ((fp3) << 6) | ((fp2) << 4) | ((fp1) << 2) | ((fp0))
}