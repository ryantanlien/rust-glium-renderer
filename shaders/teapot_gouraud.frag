#version 150

uniform sampler2D tex;
uniform vec3 u_light;

in vec3 v_normal;
out vec4 color;

// Idea behind Gouraud Shading is that if the direction of the light is perpendicular to object surface,
// Then this surface should be bright, if direction of light is parallel to surface, then it should be dark.
// We do calculation once per fragment, where pixel brightness = sin(angle(surface, light))
// If light is perpendicular, angle is PI/2 radians and brightness is 1.
// If the light is parallel, angle is 0 and brightness is 0.

// But since we don't have access to the direction of each surface, and instead have access to vertex normals,
// We can use that as a proxy for surface direction by using cos instead. -> cos(angle(vertex normal, light)) -> dot product
// -> If vertex normal and light parallel, max brightness
// -> If vertex normal and light perpendicular, 0 brightness
// Not to worry, vertex normals are already interpolated per fragment
void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.6, 0.0, 0.0);
    vec3 regular_color = vec3(1.0, 0.0, 0.0);

    // We then declare two colors: the color when the surface is entirely dark, and the color when the surface is entirely bright. 
    // In real life, it's not because an object is not exposed directly to a light source that it is black. 
    // Even unexposed surfaces receive some light from indirect sources. 
    // Therefore the dark color is not black but an intermediate level of red.
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}