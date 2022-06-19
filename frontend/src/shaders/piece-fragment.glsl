precision mediump float;

uniform sampler2D u_sampler;
uniform float u_alpha;

varying vec2 v_uv;
#ifdef PICKER
    varying vec4 v_color;
#endif

#ifdef OUTLINE 
// was supposed to be like https://github.com/keijiro/SpriteOutlineFx/blob/master/Assets/SpriteOutlineFx/Shader/SpriteOutlineFx.shader
// but doesn't actually work
#define DISTANCE 0.1
#define OUTLINE_COLOR vec3(1.0, 1.0, 1.0)

uniform vec2 u_size;

vec4 mixinOutline(vec4 tex_color) {

    vec2 d = (gl_FragCoord.xy / u_size).xy * DISTANCE;

    float a1 = texture2D(u_sampler, v_uv + d * vec2(-1, -1)).a;
    float a2 = texture2D(u_sampler, v_uv + d * vec2( 0, -1)).a;
    float a3 = texture2D(u_sampler, v_uv + d * vec2(+1, -1)).a;

    float a4 = texture2D(u_sampler, v_uv + d * vec2(-1,  0)).a;
    float a6 = texture2D(u_sampler, v_uv + d * vec2(+1,  0)).a;

    float a7 = texture2D(u_sampler, v_uv + d * vec2(-1, +1)).a;
    float a8 = texture2D(u_sampler, v_uv + d * vec2( 0, +1)).a;
    float a9 = texture2D(u_sampler, v_uv + d * vec2(+1, +1)).a;

    float gx = - a1 - a2*2.0 - a3 + a7 + a8*2.0 + a9;
    float gy = - a1 - a4*2.0 - a7 + a3 + a6*2.0 + a9;

    float w = sqrt(gx * gx + gy * gy) / 4.0;

    // Mix the contour color.
    return vec4(mix(tex_color.rgb, OUTLINE_COLOR, w), 1);
}
#endif

void main() {
    vec4 tex_color = texture2D(u_sampler, v_uv);
    if(tex_color.a < 0.5) {
        discard;
    } else {
        #ifdef PICKER
            gl_FragColor = v_color; 
        #else
            #ifdef OUTLINE
                gl_FragColor= mixinOutline(tex_color);
            #else
                gl_FragColor = tex_color;
                gl_FragColor.a = u_alpha;
            #endif
        #endif
    }
}

