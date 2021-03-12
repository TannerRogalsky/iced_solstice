varying vec4 v_Color;

#ifdef VERTEX
uniform mat4 u_Transform;

attribute vec2 i_Position;
attribute vec4 i_Color;

void main() {
    gl_Position = u_Transform * vec4(i_Position, 0.0, 1.0);
    v_Color = i_Color;
}
#endif

#ifdef FRAGMENT
void main() {
    fragColor = v_Color;
}
#endif