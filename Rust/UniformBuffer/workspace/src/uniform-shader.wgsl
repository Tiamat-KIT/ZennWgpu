struct OurStruct {
    color: vec4f,
    scale: vec2f,
    offset: vec2f,
};

@group(0) @binding(0) var<uniform> ourStruct: OurStruct;

@vertex fn vs(
    @builtin(vertex_index) vertexIndex : u32
) -> @builtin(position) vec4f {
    let pos = array(
        vec2f( 0.0,  0.5),  // 上中央
        vec2f(-0.5, -0.5),  // 左下
        vec2f( 0.5, -0.5)   // 右下
    );

    return vec4f(
        pos[vertexIndex] * ourStruct.scale + ourStruct.offset, 0.0, 1.0
    );
}

@fragment fn fs() -> @location(0) vec4f {
    return ourStruct.color;
}