struct VertexInput {
    @location(0) position vec3f,
    @location(1) color vec4f,
    @location(2) scale vec2f,
    @location(3) offset vec2f
}

struct VertexOutput {

}

@Vertex
fn vs(
    model: VertexInput
) {
   var out:VertexOutput;
   
}

@fragment
fn fs(

) {

}