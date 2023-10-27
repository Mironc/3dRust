#version 330 core
out vec4 FragColor;
in vec3 frag_pos;
in vec3 normal_dir;
uniform vec3 light_pos;
void main() {
    vec3 light_color = vec3(1.0);
    vec3 object_color = vec3(0.04, 0.28, 0.09);
    float ambientStrength = 0.2;
    vec3 ambient = ambientStrength * light_color;

    // diffuse 
    vec3 norm = normalize(normal_dir);
    vec3 lightDir = normalize(light_pos - frag_pos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light_color;

    vec3 view_pos = vec3(1., 1.5, -1.);
    float specularStrength = 0.5;
    vec3 viewDir = normalize(view_pos - frag_pos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * light_color;

    vec3 result = (ambient + diffuse + specular) * object_color;
    FragColor = vec4(result, 1.0);
}