#version 300 es

precision highp float;

uniform sampler2D s_diffuseTexture;

in vec3 v_normal;
in vec2 v_uv;

out vec4 outColor;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 diffuse = texture(s_diffuseTexture, v_uv).xyz;
    float atm_intensity = pow(1.05 - dot(normal, vec3(0.0, 0.0, 1.0)), 3.0);
    vec3 atmosphere = vec3(1.0, 1.0, 1.0) * atm_intensity;
    outColor = vec4(diffuse + atmosphere, 1.0);
}
