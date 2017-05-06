#version 410

#include "toy.frag"

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
  //vec2 uv = fragCoord.xy / iResolution.xy;
  vec2 uv = iMouse.xy / iResolution.xy;
  vec4 texel = texture(iChannel1, uv);
  fragColor = vec4(uv/2.+texel.xy/2.,0,1.0);
  //fragColor = vec4(texel.xyz,1.0);
}
