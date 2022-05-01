precision mediump float;

attribute vec3 a_geom_vertex;
attribute vec2 a_tex_vertex;

uniform mat4 u_camera;

varying vec2 v_uv;

#ifdef PICKER
    attribute vec4 a_color_vertex;
    varying vec4 v_color;
#endif

void main() {
    gl_Position = u_camera * vec4(a_geom_vertex,1);

    v_uv = a_tex_vertex;
    #ifdef PICKER
        v_color = a_color_vertex;
    #endif
}
