import * as d3 from 'd3'

import * as BABYLON from 'babylonjs'

export function drawText(dim, color, pos) {
  const {width, height} = dim
  if (!pos) {
    pos = {x: width / 2, y: height / 2, z: -10}
  }

  let textPlane = BABYLON.MeshBuilder.CreatePlane('textPlane', dim, window.scene)

  textPlane.material = new BABYLON.StandardMaterial('textPlane', window.scene)
  textPlane.position = new BABYLON.Vector3(0, 0, -3);

  let textText = new BABYLON.DynamicTexture('dyn Text', {
    width: dim.width * 20,
    height: dim.height * 20
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

  return ((message, x, y, size) => {
    textText.drawText(message, x * 20, (height - y) * 20, size + "px sans-serif", "white", null, true)
  })


}
