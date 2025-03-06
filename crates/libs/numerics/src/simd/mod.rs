use crate::Vector3;

#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
mod sse;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
use sse as xmimpl;

pub struct XMVector(xmimpl::XMVectorInner);

impl XMVector {
    pub fn load_float3(source: &Vector3) -> Self {
        Self(xmimpl::load_float3(source))
    }

    pub fn store_float3(&self, dest: &mut Vector3) {
        xmimpl::store_float3(dest, self.0);
    }

    pub fn vec3_len(&self) -> Self {
        Self(xmimpl::vec3_len(self.0))
    }

    pub fn vec_divide(&self, other: &Self) -> Self {
        Self(xmimpl::vec_divide(self.0, other.0))
    }
}