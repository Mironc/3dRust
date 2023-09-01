#version 150
in vec3 pos;
in vec3 normal;
out vec3 v_normal;
uniform mat4 matrix;
uniform mat4 perspective;
uniform mat4 view;
void main()
{
    mat4 view_model = view * matrix;
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = perspective * view_model * vec4(pos,1.0);
}