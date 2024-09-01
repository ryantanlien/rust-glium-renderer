#version 140

in vec2 position;
in vec3 color;
in vec2 tex_coords;
out vec2 v_tex_coords;
out vec3 vertex_color;

uniform mat4 matrix;

void main() {
    vec2 pos = position;
    vertex_color = color;
    v_tex_coords = tex_coords;
    gl_Position = matrix * vec4(pos, 0.0, 1.0);
}