precision mediump float;

uniform sampler2D u_sampler;
uniform vec4 u_color;
varying vec2 v_uv;

void main() {
    vec4 tex_color = texture2D(u_sampler, v_uv);
    if(tex_color.a < 1.0) {
        discard;
    } else {
        gl_FragColor = u_color; 
    }
}
