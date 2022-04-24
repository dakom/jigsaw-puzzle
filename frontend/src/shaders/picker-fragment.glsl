precision mediump float;

uniform sampler2D u_sampler;

varying vec2 v_uv;
varying vec4 v_color;

void main() {
    vec4 tex_color = texture2D(u_sampler, v_uv);
    if(tex_color.a < 0.5) {
        discard;
    } else {
        gl_FragColor = v_color; 
    }
}
