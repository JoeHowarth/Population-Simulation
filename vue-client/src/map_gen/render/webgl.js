import * as BABYLON from 'babylonjs';
import {createScene, updateColors, indecesFromMesh} from './scene'

export let NUM_TILES;

export let updateColorsFun;


var canvas,
  engine,
  scene

export async function init_babylon(mesh, h) {
  // Get the canvas DOM element
  canvas = document.getElementById('map_canvas');
  // Load the 3D engine
  engine = new BABYLON.Engine(canvas, true, {
    preserveDrawingBuffer: true,
    stencil: true
  });
  window.engine = engine
  window.canvas = canvas
  // CreateScene function that creates and return the scene

  NUM_TILES = h.length;
  // call the createScene function
  [scene, updateColorsFun] = createScene(mesh, h);

  engine.resize()
  window.scene = scene
  scene.preventDefaultOnPointerDown = false;

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

export function setup_canvas(Wkm, Hkm) {
  const ratio = Wkm / Hkm;
  let Hpx = window.innerHeight - 64; // canvas height
  let Wpx = window.innerWidth;

  canvas = document.getElementById('map_canvas');
  canvas.width = Wpx;
  canvas.height = Hpx;
  canvas.margin = '5px'

  /*
  if (!WEBGL) { // old non-webgl
    let ctx = canvas.getContext('2d');
    ctx.font = '18px serif';
    ctx.strokeRect(0, 0, Wpx, Hpx)
  }
  */
}



