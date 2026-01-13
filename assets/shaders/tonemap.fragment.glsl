#version 460 core 

in vec2 TexCoords;

out vec4 FragColor;

layout(std140, binding = 0) uniform Settings {
    float gamma;
    float exposure;
    float _padding1;
    float _padding2;
};

uniform sampler2D hdrTexture;

void main() {
    vec3 hdrColor = texture(hdrTexture, TexCoords).rgb;
    // Apply gamma + exposure tonemapping
    vec3 mapped = vec3(1.0) - exp(-hdrColor * exposure);
    // gamma correction
    mapped = pow(mapped, vec3(1.0 / gamma));
    FragColor = vec4(mapped, 1.0);
    // debug:
    //FragColor = vec4(hdrColor.rrr, 1.0);
    //FragColor = vec4(hdrColor, 1.0);
}