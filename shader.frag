#version 410

#include "toy.frag"

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
  vec2 uv = iMouse.xy / iResolution.xy;
  fragColor = vec4(uv,0,1.0);
}
