#version 460 core
out vec4 FragColor;

in vec2 TexCoords;
in mat3 TBN;

layout(std140, binding = 0) uniform Settings {
    float gamma;
    float exposure;
    int rendermode;
    float _padding;
};

layout(binding = 0) uniform sampler2D albedo;
layout(binding = 1) uniform sampler2D normal;
layout(binding = 2) uniform sampler2D orm;

void main() {
    vec4 albedo = texture(albedo, TexCoords);
    vec4 normal_tangent_space = texture(normal, TexCoords);
    vec4 orm = texture(orm, TexCoords);

    vec3 normal = normalize(TBN * (normal_tangent_space.rgb * 2.0 - 1.0));

    // gamma correction
    vec4 color = pow(albedo, vec4(1.0/gamma));
    // different debug modes
    if (rendermode == 0) { // default
        FragColor = color;
    } else if (rendermode == 1) { // albedo map
        FragColor = vec4(albedo.rgb, 1.0);
    } else if (rendermode == 2) { // normal map
        FragColor = vec4(normal_tangent_space.rgb, 1.0);
    } else if (rendermode == 3) { // ORM map
        FragColor = vec4(orm.rgb, 1.0);
    } else if (rendermode == 4) { // vertex normal
        FragColor = vec4(TBN[2], 1.0);
    } else if (rendermode == 5) { // vertex tangent
        FragColor = vec4(TBN[0], 1.0);
    } else if (rendermode == 6) { // final normal
        FragColor = vec4(normal.rgb, 1.0);
    }
}
