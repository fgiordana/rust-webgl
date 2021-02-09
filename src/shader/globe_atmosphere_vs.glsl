#version 300 es

precision highp float;
precision highp int;

uniform mat4 u_projectionMatrix;
uniform mat4 u_modelViewMatrix;
uniform mat3 u_normalMatrix;

in vec4 a_position;
in vec3 a_normal;

out vec3 v_normal;

void main() {
    gl_Position = u_projectionMatrix * u_modelViewMatrix * a_position;
    v_normal = normalize(u_normalMatrix * a_normal);
}
