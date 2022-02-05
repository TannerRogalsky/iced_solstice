varying vec4 v_Color;
varying vec4 v_BorderColor;
varying vec2 v_Pos;
varying vec2 v_Scale;
varying float v_BorderRadius;
varying float v_BorderWidth;

#ifdef VERTEX
uniform mat4 u_Transform;
uniform float u_Scale;

attribute vec2 position;
attribute vec2 i_Pos;
attribute vec2 i_Scale;
attribute vec4 i_Color;
attribute vec4 i_BorderColor;
attribute float i_BorderRadius;
attribute float i_BorderWidth;

void main() {
    vec2 q_Pos = position;
    vec2 p_Pos = i_Pos * u_Scale;
    vec2 p_Scale = i_Scale  * u_Scale;

    float i_BorderRadius = min(
        i_BorderRadius,
        min(i_Scale.x, i_Scale.y) / 2.0
    );

    mat4 i_Transform = mat4(
        vec4(p_Scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4(0.0, p_Scale.y + 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(p_Pos - vec2(0.5, 0.5), 0.0, 1.0)
    );

    v_Color = i_Color;
    v_BorderColor = i_BorderColor;
    v_Pos = p_Pos;
    v_Scale = p_Scale;
    v_BorderRadius = i_BorderRadius * u_Scale;
    v_BorderWidth = i_BorderWidth * u_Scale;

    gl_Position = u_Transform * i_Transform * vec4(q_Pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform float u_ScreenHeight;

float distance(in vec2 frag_coord, in vec2 position, in vec2 size, float radius)
{
    // TODO: Try SDF approach: https://www.shadertoy.com/view/wd3XRN
    vec2 inner_size = size - vec2(radius, radius) * 2.0;
    vec2 top_left = position + vec2(radius, radius);
    vec2 bottom_right = top_left + inner_size;

    vec2 top_left_distance = top_left - frag_coord;
    vec2 bottom_right_distance = frag_coord - bottom_right;

    vec2 distance = vec2(
    max(max(top_left_distance.x, bottom_right_distance.x), 0.0),
    max(max(top_left_distance.y, bottom_right_distance.y), 0.0)
    );

    return sqrt(distance.x * distance.x + distance.y * distance.y);
}

void main() {
    vec4 mixed_color;

    vec2 fragCoord = vec2(gl_FragCoord.x, u_ScreenHeight - gl_FragCoord.y);

    // TODO: Remove branching (?)
    if(v_BorderWidth > 0.0) {
        float internal_border = max(v_BorderRadius - v_BorderWidth, 0.0);

        float internal_distance = distance(
            fragCoord,
            v_Pos + vec2(v_BorderWidth),
            v_Scale - vec2(v_BorderWidth * 2.0),
            internal_border
        );

        float border_mix = smoothstep(
            max(internal_border - 0.5, 0.0),
            internal_border + 0.5,
            internal_distance
        );

        mixed_color = mix(v_Color, v_BorderColor, border_mix);
    } else {
        mixed_color = v_Color;
    }

    float d = distance(
        fragCoord,
        v_Pos,
        v_Scale,
        v_BorderRadius
    );

    float radius_alpha = 1.0 - smoothstep(max(v_BorderRadius - 0.5, 0.0), v_BorderRadius + 0.5, d);

    fragColor = vec4(mixed_color.xyz, mixed_color.w * radius_alpha);
}

#endif