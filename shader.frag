#version 410
precision highp float;

uniform vec3 iResolution;
uniform float iGlobalTime;
uniform vec2 iMouse;

out vec4 fColor;

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
  //vec2 uv = fragCoord.xy / iResolution.xy;
  vec2 uv = iMouse.xy / iResolution.xy;
  fragColor = vec4(uv,0,1.0);
}

void main() {
  vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
  mainImage(color, gl_FragCoord.xy);
  color.w = 1.0;
  fColor = color;
}
