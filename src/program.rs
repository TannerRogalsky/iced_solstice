use solstice::shader::DynamicShader;

pub fn create(gl: &mut solstice::Context, vertex: &str, fragment: &str) -> DynamicShader {
    let (v, f) = DynamicShader::create_source(vertex, fragment);
    DynamicShader::new(gl, v.as_str(), f.as_str()).unwrap()
}
