#version 460 core

out vec4 FragColor;
in vec3 localPos;

uniform sampler2D equirectangularMap;

const vec2 invAtan = vec2(0.1591, 0.3183);
vec2 sampleSphericalMap(vec3 v)
{
    vec2 uv = vec2(atan(v.z, v.x), asin(v.y));
    uv *= invAtan;
    uv += 0.5;
    return uv;
}

void main()
{
    vec2 uv = sampleSphericalMap(normalize(localPos));
    vec3 color = texture(equirectangularMap, uv).rgb;
    FragColor = vec4(color, 1.0);
    //FragColor = vec4(normalize(localPos) * 0.5 + 0.5, 1.0); 
    /*
    vec3 d = vec3(debug);
    if (d == 0.0){
        d = vec3(1.0, 0.0, 0.0);
    }
    else if (d == 1.0){
        d = vec3(0.0, 1.0, 0.0);
    }
    else if (d == 2.0){
        d = vec3(0.0, 0.0, 1.0);
    }
    else if (d == 3.0){
        d = vec3(1.0, 1.0, 0.0);
    }
    else if (d == 4.0){
        d = vec3(1.0, 0.0, 1.0);
    }
    else if (d == 5.0){
        d = vec3(0.0, 1.0, 1.0);
    }
    FragColor = vec4(d, 1.0); */
}
