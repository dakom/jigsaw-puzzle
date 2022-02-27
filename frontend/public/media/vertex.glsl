precision mediump float;

attribute vec2 a_geom_vertex;
attribute vec2 a_tex_vertex;

uniform vec2 u_cell_size;
uniform mat4 u_camera;
uniform mat4 u_model;

varying vec2 v_uv;

void main() {
    mat4 modelViewProjection = u_camera * u_model; 

    gl_Position = modelViewProjection * (vec4(u_cell_size,0,1) * vec4(a_geom_vertex,0,1));

    //float left = u_coord.x / u_full_size.x;
    //float right = (u_coord.x + u_cell_size.x) / u_full_size.x;
    //float top = u_coord.y / u_full_size.y;
    //float bottom = (u_coord.y + u_cell_size.y) / u_full_size.y;

    v_uv = a_tex_vertex;
}
