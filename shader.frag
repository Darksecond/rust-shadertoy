#version 410

#include "toy.frag"

uniform float color_r;
uniform float color_g;
uniform float color_b;

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
  //vec2 uv = iMouse.xy / iResolution.xy;
  //fragColor = vec4(uv,0,1.0);
  fragColor = vec4(color_r, color_g, color_b, 1.0);
}
