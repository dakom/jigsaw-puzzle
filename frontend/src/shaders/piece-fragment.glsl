precision mediump float;

uniform sampler2D u_sampler;
uniform float u_alpha;

varying vec2 v_uv;
#ifdef PICKER
    varying vec4 v_color;
#endif

void main() {
    vec4 tex_color = texture2D(u_sampler, v_uv);
    if(tex_color.a < 0.5) {
        discard;
    } else {
        #ifdef PICKER
            gl_FragColor = v_color; 
        #else
            gl_FragColor = tex_color; 
            gl_FragColor.a = u_alpha;
        #endif
    }
}
