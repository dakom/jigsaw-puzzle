precision mediump float;

attribute vec3 a_geom_vertex;
uniform mat4 u_camera;

void main() {
    gl_Position = u_camera * vec4(a_geom_vertex,1);
}
