#version 300 es

precision highp float;
precision highp int;

in vec3 v_normal;

out vec4 outColor;

void main() {
    vec3 normal = normalize(v_normal);
    float intensity = pow(1.6 - dot(normal, vec3(0.0, 0.0, 1.0)), 12.0);
    outColor = vec4(1.0, 1.0, 1.0, 1.0) * intensity;
}
