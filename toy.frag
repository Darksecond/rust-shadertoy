precision highp float;

uniform vec2 iResolution;
uniform float iGlobalTime;
uniform vec2 iMouse;

uniform sampler2D iChannel0;

out vec4 fColor;

void mainImage(out vec4 fragColor, in vec2 fragCoord);
void main() {
  vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
  mainImage(color, gl_FragCoord.xy);
  color.w = 1.0;
  fColor = color;
}
