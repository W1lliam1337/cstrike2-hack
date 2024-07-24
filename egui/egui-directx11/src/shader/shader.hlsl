struct vs_in {
  float2 position : POSITION;
  float2 uv : TEXCOORD;
  float4 color : COLOR;
};

struct vs_out {
  float4 clip : SV_POSITION;
  float2 uv : TEXCOORD;
  float4 color : COLOR;
};

vs_out vs_main(vs_in input) {
  vs_out output;
  output.clip = float4(input.position, 0.0, 1.0);
  output.uv = input.uv;
  output.color = input.color;

  return output;
}

sampler sampler0;
Texture2D texture0;

float4 ps_main(vs_out input) : SV_TARGET {
  return pow(input.color, 1.0 / 2.2) * texture0.Sample(sampler0, input.uv);
}