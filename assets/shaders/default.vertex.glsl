#version 460 core
layout (location = 0) in vec3 aPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    vec3 positionWorld = aPos;
    gl_Position = projection * view * vec4(positionWorld, 1.0);
}
