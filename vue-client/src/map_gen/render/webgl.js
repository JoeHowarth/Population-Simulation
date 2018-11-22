import * as BABYLON from 'babylonjs';
import { initCamera, setPositionScale } from './camera';
import { heightToColor, heightToColorArr } from './render-map'


function createScene(mesh, h) {
  console.log('create scene, mesh h', mesh, h)

  // Create a basic BJS Scene object
  var scene = new BABYLON.Scene(engine);
  // Create a basic light, aiming 0, 1, 0 - meaning, to the sky
  var light = new BABYLON.HemisphericLight('light1', new BABYLON.Vector3(0, 1, 0), scene);
  // Create a built-in "sphere" shape; its constructor takes 6 params: name, segment, diameter, scene, updatable, sideOrientation

  let camera = initCamera(scene, mesh.Dkm[0]/ mesh.Dkm[1]);
  console.log(camera)
  setPositionScale(camera, [mesh.Dkm[0] / 2, mesh.Dkm[1] / 2], mesh.Dkm[0] * 0.5 )
  camera.attachControl(canvas, false);

  /*
   *
   *  make custom 'Babylon.Mesh' from geom map mesh
   *
   */

  const indices = indecesFromMesh(mesh);
  const points = []
  for (let i = 0; i < mesh.points.length - 1; i += 2) {
    points.push(mesh.points[i])
    points.push(mesh.points[i + 1])
    points.push(0)
  }

  let mapMesh = new BABYLON.Mesh('map', scene);
  let mapWire = new BABYLON.Mesh('wire', scene);
  let vertexData = new BABYLON.VertexData();
  vertexData.positions = points
  vertexData.indices = indices
  // vertexData.applyToMesh(mapMesh);
  vertexData.applyToMesh(mapWire);

  const points2 = [],
    colors2 = []
  const { triangles } = mesh;
  for (let i = 0; i < indices.length; i++) {
    points2.push(mesh.points[indices[i] * 2])
    points2.push(mesh.points[indices[i] * 2 + 1])
    points2.push(0)


  }

  let indices2 = []
  for (let i = 0; i < points2.length / 3; i++) {
    indices2.push(i)
  }
  for (let i = 0; i < indices2.length / 3; i++) {
    let color = [Math.random(), Math.random(), Math.random()]
    colors2.push(color[0])
    colors2.push(color[1])
    colors2.push(color[2])
    colors2.push(Math.random())
    colors2.push(color[0])
    colors2.push(color[1])
    colors2.push(color[2])
    colors2.push(1)
    colors2.push(color[0])
    colors2.push(color[1])
    colors2.push(color[2])
    colors2.push(1)
  }
  /*
  console.log('indices', indices)
  console.log('points2', points2)
  console.log('indices2', indices2)
  console.log('colors2', colors2)
  */

  let vertexData2 = new BABYLON.VertexData();
  vertexData2.positions = points2
  vertexData2.indices = indices2
  vertexData2.colors = colors2
  vertexData2.applyToMesh(mapMesh, true);

  const num_colors = colors2.length

  h = new Float32Array(h)


  let colors = new Float32Array(num_colors)
  updateColors(colors, mesh, mapMesh, h)

  // scene.onBeforeRenderObservable.add(() => {
  //   h = h.map(v => v + 0.0001);
  //   updateColors(colors, mesh, mapMesh, h)
  // })


  let red_wire = new BABYLON.StandardMaterial('wire', scene)
  red_wire.wireframe = true
  red_wire.diffuseColor = new BABYLON.Color3(1, 0, 0)
  mapWire.material = red_wire
  mapWire.translate(BABYLON.Axis.Z, 2, BABYLON.Space.WORLD)

  let map_mat = new BABYLON.StandardMaterial("map", scene)
  map_mat.specularColor = new BABYLON.Color3(0,0,0)
  map_mat.emissiveColor = new BABYLON.Color3(1,1,1)
  map_mat.diffuseColor = new BABYLON.Color3(0,0,0)
  mapMesh.material = map_mat

  // Return the created scene
  return scene;
}



var canvas,
  engine,
  scene

export async function init_babylon(mesh, h) {
  console.log('initBab', mesh)
  // Get the canvas DOM element
  canvas = document.getElementById('map_canvas');
  // Load the 3D engine
  engine = new BABYLON.Engine(canvas, true, {
    preserveDrawingBuffer: true,
    stencil: true
  });
  // CreateScene function that creates and return the scene

  // call the createScene function
  scene = createScene(mesh, h);

  window.scene = scene

  // run the render loop
  engine.runRenderLoop(function () {
    scene.render();
  });
  // the canvas/window resize event handler
  window.addEventListener('resize', function () {
    engine.resize();
  });



  return scene
}


function updateColors (colors, mesh, mapMesh, h) {
  console.log('hi')
  // let colors = mapMesh.getVerticesData(BABYLON.VertexBuffer.ColorKind);
  for (let i = 0; i < h.length; i++) {


    const c = heightToColorArr(h[i])

    let j = mesh.triIDs[i] * 12

    colors[j] = c.r / 255
    colors[j + 1] = c.g / 255
    colors[j + 2] = c.b / 255
    colors[j + 3] = 1.0

    j += 4
    colors[j] = c.r / 255
    colors[j + 1] = c.g / 255
    colors[j + 2] = c.b / 255
    colors[j + 3] = 1.0

    j += 4
    colors[j] = c.r / 255
    colors[j + 1] = c.g / 255
    colors[j + 2] = c.b / 255
    colors[j + 3] = 1.0
  }


  mapMesh.updateVerticesData(BABYLON.VertexBuffer.ColorKind, colors)
}


function indecesFromMesh(mesh) {
  console.log(mesh.triangles);
  const { triangles } = mesh;
  let inds = []
  for (let i = 0; i < triangles.length - 2; i += 3) {
    inds.push(triangles[i + 2])
    inds.push(triangles[i + 1])
    inds.push(triangles[i])
  }
  return inds
}
