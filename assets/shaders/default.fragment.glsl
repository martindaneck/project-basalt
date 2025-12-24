#version 460 core

struct Light {
        vec4 position_range; // xyz = position, w = range
        vec4 color_intensity; // xyz = color, w = intensity
};

out vec4 FragColor;

in vec2 TexCoords;
in mat3 TBN;

layout(std140, binding = 0) uniform Settings {
    float gamma;
    float exposure;
    int rendermode;
    float _padding;
};
layout(std140, binding = 1) uniform Camera {
    mat4 view;
    mat4 projection;
    vec3 camera_position;
    float _padding2; // if problems, change camera_position to a vec4.
};
layout(std140, binding = 2) uniform Lights {
    vec4 count; // count.x is the number of lights, rest is padding
    Light lights[1];
};

layout(binding = 0) uniform sampler2D albedo;
layout(binding = 1) uniform sampler2D normal;
layout(binding = 2) uniform sampler2D orm;

void main() {
    vec4 albedo = texture(albedo, TexCoords);
    vec4 normal_tangent_space = texture(normal, TexCoords);
    vec4 orm = texture(orm, TexCoords);

    vec3 normal = normalize(TBN * (normal_tangent_space.rgb * 2.0 - 1.0));

    vec4 color = albedo;

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
