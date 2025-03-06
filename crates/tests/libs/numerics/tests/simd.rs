use windows_numerics::Vector3;

#[test]
fn vec3_normalize() {
    let v = Vector3::new(20.0, 17.0, 37.8);
    assert_eq!(v.normalize(), v.normalize_simd());
}