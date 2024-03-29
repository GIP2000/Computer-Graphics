#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTex;

out vec4 ourColor;
out vec2 TexCords;
uniform vec4 uColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    ourColor = uColor;
    TexCords = aTex;
	gl_Position = projection * view * model * vec4(aPos, 1.0);
}
