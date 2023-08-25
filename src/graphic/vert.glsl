#version 140
in vec3 pos;
out vec3 out_pos;
uniform mat4 matrix;
void main()
{
    out_pos = pos;
    gl_Position =  matrix * vec4(pos,1.0);
}