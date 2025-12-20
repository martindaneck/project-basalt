#version 460 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D albedo;
uniform sampler2D normal;
uniform sampler2D orm;

void main() {
    vec4 color = texture(albedo, TexCoords);

    // gamma correction
    color = pow(color, vec4(1.0/2.2));
    FragColor = color;
}
