#version 330 core
out vec4 FragColor;

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct PointLight {
    vec3 position;
    vec3 color;

    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    samplerCube depthMap;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCords;
#define LIGHT_MAX 10

uniform vec3 viewPos;
uniform PointLight pointLight[LIGHT_MAX];
uniform int light_num;
uniform Material material;
uniform float far_plane;


#define CHECK(I) if (I < light_num) {result += CalcPointLight(pointLight[I], norm, fragPos, viewDir);} else {return result;}


vec3 gridSamplingDisk[20] = vec3[](
   vec3(1, 1,  1), vec3( 1, -1,  1), vec3(-1, -1,  1), vec3(-1, 1,  1),
   vec3(1, 1, -1), vec3( 1, -1, -1), vec3(-1, -1, -1), vec3(-1, 1, -1),
   vec3(1, 1,  0), vec3( 1, -1,  0), vec3(-1, -1,  0), vec3(-1, 1,  0),
   vec3(1, 0,  1), vec3(-1,  0,  1), vec3( 1,  0, -1), vec3(-1, 0, -1),
   vec3(0, 1,  1), vec3( 0, -1,  1), vec3( 0, -1, -1), vec3( 0, 1, -1)
);

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
float ShadowCalculations(PointLight light, vec3 fragPos);

vec3 PointLoops(vec3 norm, vec3 fragPos, vec3 viewDir) {
    vec3 result = vec3(0.0);
    CHECK(0)
    CHECK(1)
    CHECK(2)
    CHECK(3)
    CHECK(4)
    CHECK(5)
    CHECK(6)
    CHECK(7)
    CHECK(8)
    CHECK(9)
    return result;

}

void main()
{
    // properties
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    vec3 result = PointLoops(norm, FragPos, viewDir);

    FragColor = vec4(result,1.0);

}


// calculates the color when using a point light.
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    // return fragPos;
    vec3 lightDir = normalize(light.position - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // attenuation
    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    // combine results
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCords));
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, TexCords));
    vec3 specular = light.specular * spec * vec3(texture(material.specular, TexCords));
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    ambient *= light.color;
    diffuse *= light.color;
    specular *= light.color;

    float shadow = ShadowCalculations(light, fragPos);
    return (ambient + (1.0 - shadow) * (diffuse + specular)) ;
}

float ShadowCalculations(PointLight light, vec3 fragPos){
    // get vector between fragment position and light position
    vec3 fragToLight = fragPos - light.position;
    // use the fragment to light vector to sample from the depth map
    // float closestDepth = texture(depthMap, fragToLight).r;
    // it is currently in linear range between [0,1], let's re-transform it back to original depth value
    // closestDepth *= far_plane;
    // now get current linear depth as the length between the fragment and light position




    float currentDepth = length(fragToLight);




    // test for shadows
    // float bias = 0.05; // we use a much larger bias since depth is now in [near_plane, far_plane] range
    // float shadow = currentDepth -  bias > closestDepth ? 1.0 : 0.0;
    // PCF
    // float shadow = 0.0;
    // float bias = 0.05;
    // float samples = 4.0;
    // float offset = 0.1;
    // for(float x = -offset; x < offset; x += offset / (samples * 0.5))
    // {
        // for(float y = -offset; y < offset; y += offset / (samples * 0.5))
        // {
            // for(float z = -offset; z < offset; z += offset / (samples * 0.5))
            // {
                // float closestDepth = texture(depthMap, fragToLight + vec3(x, y, z)).r; // use lightdir to lookup cubemap
                // closestDepth *= far_plane;   // Undo mapping [0;1]
                // if(currentDepth - bias > closestDepth)
                    // shadow += 1.0;
            // }
        // }
    // }
    // shadow /= (samples * samples * samples);


    float shadow = 0.0;
    float bias = 0.05;
    int samples = 20;
    float viewDistance = length(viewPos - fragPos);
    float diskRadius = (1.0 + (viewDistance / far_plane)) / 25.0;
    float max_depth = 0.0;
    for(int i = 0; i < samples; ++i)
    {
        // this is the bad line of code
        float closestDepth = texture(light.depthMap, fragToLight + gridSamplingDisk[i] * diskRadius).r;
        closestDepth *= far_plane;   // undo mapping [0;1]
        if(currentDepth - bias > closestDepth) {
            shadow += 1.0;
        }
        max_depth = max_depth > closestDepth ? max_depth: closestDepth;
    }
    shadow /= float(samples);

    // display closestDepth as debug (to visualize depth cubemap)

    return shadow;
}
// calculates the color when using a spot light.
