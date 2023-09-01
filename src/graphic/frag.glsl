#version 140
in vec3 v_normal;
uniform vec3  u_light;
out vec4 color;

void main()
{
    float brightness = dot(normalize(v_normal),normalize(u_light));
    vec3 dark_col = vec3(0.6, 0.0, 0.0);
    vec3 reg_col = vec3(1.0, 0.0, 0.0);
    color = vec4(mix(dark_col,reg_col,brightness),1.0);
}