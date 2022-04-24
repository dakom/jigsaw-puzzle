precision mediump float;

attribute vec3 a_geom_vertex;
attribute vec2 a_tex_vertex;
attribute mat4 a_model;

uniform mat4 u_camera;

varying vec2 v_uv;

void main() {
    gl_Position = u_camera * vec4(a_geom_vertex,1);

    v_uv = a_tex_vertex;
}
