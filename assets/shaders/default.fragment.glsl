#version 460 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D albedo;
uniform sampler2D normal;
uniform sampler2D orm;

void main() {
    FragColor = texture(albedo, TexCoords);
}
