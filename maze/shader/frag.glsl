#version 330 core
out vec4 FragColor;

in vec4 ourColor;
in vec2 TexCords;

uniform sampler2D ourTexture;


void main()
{
    FragColor = texture(ourTexture, TexCords) * ourColor;
}
