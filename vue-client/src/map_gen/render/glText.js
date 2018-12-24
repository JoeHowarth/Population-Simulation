import * as d3 from 'd3'
import metrics from './OpenSans-Regular'
import * as BABYLON from 'babylonjs'

export function drawText(dim, {color, pos, res} = {color: "white"}) {
  const {width, height} = dim
  if (!pos) {
    pos = {x: width / 2, y: height / 2, z: -10}
  }
  console.log(res)
  res = res ? res : 20
  console.log(res)

  let textPlane = BABYLON.MeshBuilder.CreatePlane('textPlane', dim, window.scene)

  textPlane.material = new BABYLON.StandardMaterial('textPlane', window.scene)
  textPlane.position = new BABYLON.Vector3(0, 0, -3);

  let textText = new BABYLON.DynamicTexture('dyn Text', {
    width: dim.width * res,
    height: dim.height * res
  }, window.scene, true)
  let textTextColor = new BABYLON.DynamicTexture('dyn Text', dim, window.scene, true)

  const ctx = textTextColor.getContext()
  ctx.fillStyle = color
  ctx.fillRect(0, 0, dim.width, dim.height)
  textTextColor.update()

  textPlane.material.opacityTexture = textText
  textPlane.material.diffuseTexture = textTextColor;
  textPlane.material.specularColor = new BABYLON.Color3(0, 0, 0);
  textPlane.material.emissiveColor = new BABYLON.Color3(1, 1, 1);
  textPlane.material.backFaceCulling = false;
  textPlane.position = pos
  textText.hasAlpha = true

  const size_scale = res / 40
  return ((message, x, y, size) => {
    textText.drawText(message, x * res, (height - y) * res, (size * size_scale) + "px sans-serif", color, null, true)
  })
}

function createText(size) {
  const str = "Hello World"

  let vertexElements = []
  let textureElements = []

  let dimensions = measureText(str, size);

  var pen = {x: canvas.width / 2 - dimensions.advance / 2, y: canvas.height / 2};
  // let pen = {x: 20, y: 20}
  for (var i = 0; i < str.length; i++) {
    var chr = str[i];
    drawGlyph(chr, pen, size, vertexElements, textureElements);
  }

  // let texture = new BABYLON.RawTexture(new Float32Array(vertexElements), )

  /*
  gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertexElements), gl.STATIC_DRAW);
  vertexBuffer.numItems = vertexElements.length / 2;

  gl.bindBuffer(gl.ARRAY_BUFFER, textureBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(textureElements), gl.STATIC_DRAW);
  textureBuffer.numItems = textureElements.length / 2;
  */
}

function sdf() {
  const sdf_mat = new BABYLON.ShaderMaterial("text", window.scene, {
      vertex: "sdf",
      fragment: "sdf",
    },
    {
      attributes: ["a_pos", "a_texcoord"],
      uniforms: [
        'worldViewProjection',
        'u_texsize',
        'u_color',
        'u_buffer',
        'u_gamma',
        'u_debug',
      ],
    });
}

function measureText(text, size) {
  var dimensions = {
    advance: 0
  }

  var scale = size / metrics.size;
  for (var i = 0; i < text.length; i++) {
    var horiAdvance = metrics.chars[text[i]][4];
    dimensions.advance += horiAdvance * scale;
  }

  return dimensions;
}

function drawGlyph(chr, pen, size, vertexElements, textureElements) {
  var metric = metrics.chars[chr];
  if (!metric) return;

  var scale = size / metrics.size;

  var factor = 1;

  var width = metric[0];
  var height = metric[1];
  var horiBearingX = metric[2];
  var horiBearingY = metric[3];
  var horiAdvance = metric[4];
  var posX = metric[5];
  var posY = metric[6];

  if (width > 0 && height > 0) {
    width += metrics.buffer * 2;
    height += metrics.buffer * 2;

    // Add a quad (= two triangles) per glyph.
    vertexElements.push(
      (factor * (pen.x + ((horiBearingX - metrics.buffer) * scale))), (factor * (pen.y - horiBearingY * scale)),
      (factor * (pen.x + ((horiBearingX - metrics.buffer + width) * scale))), (factor * (pen.y - horiBearingY * scale)),
      (factor * (pen.x + ((horiBearingX - metrics.buffer) * scale))), (factor * (pen.y + (height - horiBearingY) * scale)),

      (factor * (pen.x + ((horiBearingX - metrics.buffer + width) * scale))), (factor * (pen.y - horiBearingY * scale)),
      (factor * (pen.x + ((horiBearingX - metrics.buffer) * scale))), (factor * (pen.y + (height - horiBearingY) * scale)),
      (factor * (pen.x + ((horiBearingX - metrics.buffer + width) * scale))), (factor * (pen.y + (height - horiBearingY) * scale))
    );

    textureElements.push(
      posX, posY,
      posX + width, posY,
      posX, posY + height,

      posX + width, posY,
      posX, posY + height,
      posX + width, posY + height
    );
  }

  // pen.x += Math.ceil(horiAdvance * scale);
  pen.x = pen.x + horiAdvance * scale;
}

// setup babylon shaders
function init_sdf_shaders() {
  const vs = `
attribute vec2 a_pos;
attribute vec2 a_texcoord;

uniform mat4 worldViewProjection;
uniform vec2 u_texsize;

varying vec2 v_texcoord;

void main() {
    gl_Position = worldViewProjection * vec4(a_pos.xy, 0, 1);
    v_texcoord = a_texcoord / u_texsize;
}
`

  const fs = `
precision mediump float;

uniform sampler2D u_texture;
uniform vec4 u_color;
uniform float u_debug;

varying vec2 v_texcoord;

void main() {
    float u_buffer = 0.1;
    float u_gamma = 3;
    float dist = texture2D(u_texture, v_texcoord).r;
    if (u_debug > 0.0) {
        gl_FragColor = vec4(dist, dist, dist, 1);
    } else {
        float alpha = smoothstep(u_buffer - u_gamma, u_buffer + u_gamma, dist);
        gl_FragColor = vec4(u_color.rgb, alpha * u_color.a);
    }
}
`
  const fs_orig = `
precision mediump float;

uniform sampler2D u_texture;
uniform vec4 u_color;
uniform float u_buffer;
uniform float u_gamma;
uniform float u_debug;

varying vec2 v_texcoord;

void main() {
    float dist = texture2D(u_texture, v_texcoord).r;
    if (u_debug > 0.0) {
        gl_FragColor = vec4(dist, dist, dist, 1);
    } else {
        float alpha = smoothstep(u_buffer - u_gamma, u_buffer + u_gamma, dist);
        gl_FragColor = vec4(u_color.rgb, alpha * u_color.a);
    }
}
`
  BABYLON.Effect.ShadersStore["sdfVertexShader"] = vs;
  BABYLON.Effect.ShadersStore["sdfFragmentShader"] = fs;
}

