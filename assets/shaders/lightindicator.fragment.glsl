#version 460 core

out vec4 FragColor;

struct Light {
        vec4 position_range; // xyz = position, w = range
        vec4 color_intensity; // xyz = color, w = intensity
};

layout(std140, binding = 2) uniform Lights {
    vec4 count; // count.x is the number of lights, rest is padding
    Light lights[1];
};

void main() {
    // assume we are drawing only the first light as an indicator
    // if we want to show multiple lights, we would need to pass an index as a uniform
    int index = 0;
    Light light = lights[index];
    vec3 light_color = light.color_intensity.xyz;
    float intensity = light.color_intensity.w;
    FragColor = vec4(light_color * intensity, 1.0);  
}