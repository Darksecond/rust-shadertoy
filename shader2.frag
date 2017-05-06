#version 410

#include "toy.frag"

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
  vec2 uv = fragCoord.xy / iResolution.xy;
  vec4 texel = texture(iChannel0, uv);
  fragColor = vec4(texel.xyz,1.0);
}
