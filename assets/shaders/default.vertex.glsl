#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec4 aTangent;
layout (location = 3) in vec2 aTexCoords;

out vec2 TexCoords;

layout(std140, binding = 1) uniform Camera {
    mat4 view;
    mat4 projection;
    vec3 camera_position;
};

uniform mat4 model;

void main() {
    vec4 positionWorld = model * vec4(aPos, 1.0);
    TexCoords = aTexCoords;
    gl_Position = projection * view * positionWorld;
}
