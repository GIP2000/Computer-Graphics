#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in float face;

out vec4 ourColor;
uniform vec4[6] uColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    ourColor = uColor[int(face)];
	gl_Position = projection * view * model * vec4(aPos, 1.0);
}