#version 330 core
out vec4 FragColor;

in vec4 ourColor;
in vec2 TexCords;

uniform sampler2D ourTexture;
uniform bool has_texture;


void main()
{
   const float border_width = 0.05;
   float maxX = 1.0 - border_width;
   float minX = border_width;
   float maxY = maxX;
   float minY = minX;

   if (!(TexCords.x < maxX && TexCords.x > minX &&
       TexCords.y < maxY && TexCords.y > minY)) {
       FragColor = vec4(0.0,0.0,0.0,1.0);
   } else if (has_texture) {
        FragColor = texture(ourTexture, TexCords) * ourColor;
    } else {
        FragColor = ourColor;
    }
}
