import * as BABYLON from 'babylonjs'

export default function line2D(name, options, scene) {

  //Arrays for vertex positions and indices
  var positions = [];
  var indices = [];
  var normals = [];

  var width = options.width || 1;
  var path = options.path;
  var color = options.color || BABYLON.Color3.White()

  var interiorIndex;

  //Arrays to hold wall corner data
  var innerBaseCorners = [];
  var outerBaseCorners = [];

  var outerData = [];
  var innerData = [];
  var angle = 0;

  var line = BABYLON.Vector3.Zero();
  var nextLine = BABYLON.Vector3.Zero();
  path[1].subtractToRef(path[0], line);
  let lineNormal = new BABYLON.Vector3(line.y, -1 * line.x, 0).normalize();
  line.normalize();
  innerData[0] = path[0].subtract(lineNormal.scale(width));
  outerData[0] = path[0].add(lineNormal.scale(width));



  var nbPoints = path.length;
  for(var p = 0; p < nbPoints - 2; p++) {
    path[p + 2].subtractToRef(path[p + 1], nextLine);
    angle = Math.PI - Math.acos(BABYLON.Vector3.Dot(line, nextLine)/(line.length() * nextLine.length()));
    let direction = BABYLON.Vector3.Cross(line, nextLine).normalize().z;
    lineNormal = new BABYLON.Vector3(line.y, -1 * line.x, 0).normalize();
    line.normalize();
    innerData[p + 1] = path[p + 1].subtract(lineNormal.scale(width)).subtract(line.scale(direction * width/Math.tan(angle/2)));
    outerData[p + 1] = path[p + 1].add(lineNormal.scale(width)).add(line.scale(direction * width/Math.tan(angle/2)));
    line = nextLine.clone();
  }
  if(nbPoints > 2) {
    path[nbPoints - 1].subtractToRef(path[nbPoints - 2], line);
    lineNormal = new BABYLON.Vector3(line.y, -1 * line.x, 0).normalize();
    line.normalize();
    innerData[nbPoints - 1] = path[nbPoints - 1].subtract(lineNormal.scale(width));
    outerData[nbPoints - 1] = path[nbPoints - 1].add(lineNormal.scale(width));
  }
  else{
    innerData[1] = path[1].subtract(lineNormal.scale(width));
    outerData[1] = path[1].add(lineNormal.scale(width));
  }

  for(var p = 0; p < nbPoints; p++) {
    positions.push(innerData[p].x, innerData[p].y, innerData[p].z);
  }

  for(var p = 0; p < nbPoints; p++) {
    positions.push(outerData[p].x, outerData[p].y, outerData[p].z);
  }

  for(var i = 0; i < nbPoints - 1; i++) {
    indices.push(i, i + 1, nbPoints + i + 1);
    indices.push(i, nbPoints + i + 1, nbPoints + i)
  }

  var normals = [];
  var uvs =[];
  BABYLON.VertexData.ComputeNormals(positions, indices, normals);
  BABYLON.VertexData._ComputeSides(BABYLON.Mesh.DOUBLESIDE, positions, indices, normals, uvs);

  //Create a custom mesh
  var customMesh = new BABYLON.Mesh("custom", scene);

  //Create a vertexData object
  var vertexData = new BABYLON.VertexData();

  //Assign positions and indices to vertexData
  vertexData.positions = positions;
  vertexData.indices = indices;
  vertexData.normals = normals;

  //Apply vertexData to custom mesh
  vertexData.applyToMesh(customMesh);

  let mat = new BABYLON.StandardMaterial("", scene);
  mat.specularColor = new BABYLON.Color3(0,0,0)
  mat.emissiveColor = new BABYLON.Color3(0,0,0)
  mat.diffuseColor = color
  customMesh.material = mat


  return customMesh;

}
