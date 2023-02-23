#version 330 core
out vec4 FragColor;

in vec4 ourColor;
in vec2 TexCords;

uniform sampler2D ourTexture;
uniform bool has_texture;


void main()
{
    if (has_texture) {
        FragColor = texture(ourTexture, TexCords) * ourColor;
    } else {
        FragColor = ourColor;
    }
}
