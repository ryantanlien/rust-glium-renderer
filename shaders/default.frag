#version 140

in vec3 vertex_color;
in vec2 v_tex_coords;

uniform sampler2D tex;

out vec4 color;

void main() {
    color = vec4(vertex_color, 1.0);            
}