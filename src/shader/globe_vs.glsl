#version 300 es

uniform mat4 u_projectionMatrix;
uniform mat4 u_modelViewMatrix;
uniform mat3 u_normalMatrix;

in vec4 a_position;
in vec3 a_normal;
in vec2 a_uv;

out vec3 v_normal;
out vec2 v_uv;

void main() {
    gl_Position = u_projectionMatrix * u_modelViewMatrix * a_position;
    v_normal = normalize(u_normalMatrix * a_normal);
    v_uv = a_uv;
}
