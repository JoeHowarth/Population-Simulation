import * as BABYLON from 'babylonjs'

const CAMERA_LEVEL = -100


export function initCamera(scene, ratio) {
  const orthoInputProto = {
    getSimpleName() {
      return 'keyboardTranslate';
    },
    _onLostFocus(e) {
      this._keys = [];
    },
    getTypeName() {
      return 'FreeCameraKeyboardTranslateInput';
    },
    checkInputs() {
      if (this._onKeyDown) {
        var camera = this.camera;
        // Keyboard
        for (var index = 0; index < this._keys.length; index++) {
          var keyCode = this._keys[index];
          if (this.keysLeft.indexOf(keyCode) !== -1) {
            camera.position.x -= this.sensibility;
          }
          else if (this.keysRight.indexOf(keyCode) !== -1) {
            camera.position.x += this.sensibility;
          }
          else if (this.keysUp.indexOf(keyCode) !== -1) {
            camera.position.y += this.sensibility;
          }
          else if (this.keysDown.indexOf(keyCode) !== -1) {
            camera.position.y -= this.sensibility;
          }
        }
      }
    },
    detachControl(element) {
      if (this._onKeyDown) {
        window.removeEventListener('keydown', this._onKeyDown);
        window.removeEventListener('keyup', this._onKeyUp);
        BABYLON.Tools.UnregisterTopRootEvents([
          {
            name: 'blur',
            handler: this._onLostFocus
          }
        ]);
        this._keys = [];
        this._onKeyDown = null;
        this._onKeyUp = null;
      }
    },
    attachControl(element, noPreventDefault) {
      var _this = this;
      if (!this._onKeyDown) {
        element.tabIndex = 1;
        this._onKeyDown = function (evt) {
          if (_this.keysLeft.indexOf(evt.keyCode) !== -1 ||
            _this.keysRight.indexOf(evt.keyCode) !== -1 ||
            _this.keysDown.indexOf(evt.keyCode) !== -1 ||
            _this.keysUp.indexOf(evt.keyCode) !== -1) {
            var index = _this._keys.indexOf(evt.keyCode);
            if (index === -1) {
              _this._keys.push(evt.keyCode);
            }
            if (!noPreventDefault) {
              evt.preventDefault();
            }
          }
        };
        this._onKeyUp = function (evt) {
          if (_this.keysLeft.indexOf(evt.keyCode) !== -1 ||
            _this.keysRight.indexOf(evt.keyCode) !== -1 ||
            _this.keysDown.indexOf(evt.keyCode) !== -1 ||
            _this.keysUp.indexOf(evt.keyCode) !== -1) {
            var index = _this._keys.indexOf(evt.keyCode);
            if (index >= 0) {
              _this._keys.splice(index, 1);
            }
            if (!noPreventDefault) {
              evt.preventDefault();
            }
          }
        };
        this._onWheel = function (evt) {
          if (!evt.deltaY) {
            return;
          }
          const s = evt.deltaY * 0.001
          camera.orthoLeft *= 1.0 + s
          camera.orthoRight *= 1.0 + s
          camera.orthoTop *= 1.0 + s
          camera.orthoBottom *= 1.0 + s
          evt.preventDefault();

          _this.sensibility = _this._sensibility * camera.orthoRight
          console.log(_this.sensibility, _this._sensibility, camera.orthoRight)

        }

        window.addEventListener('keydown', this._onKeyDown, false);
        window.addEventListener('keyup', this._onKeyUp, false);
        element.addEventListener('wheel', this._onWheel, false)
        BABYLON.Tools.RegisterTopRootEvents([
          {
            name: 'blur',
            handler: this._onLostFocus
          }
        ]);
      }
    },
  }

  var camera = new BABYLON.FreeCamera('camera1', new BABYLON.Vector3(0, 0, CAMERA_LEVEL), scene);
  camera.mode = BABYLON.Camera.ORTHOGRAPHIC_CAMERA

  camera.ratio = ratio

  camera.orthoBottom = -1
  camera.orthoTop = 1
  camera.orthoLeft = -1
  camera.orthoRight = 1


  // Let's remove default keyboard:
  camera.inputs.removeByType('FreeCameraKeyboardMoveInput');

  // Create our own manager:
  var FreeCameraKeyboardTranslateInput = function () {
    this._keys = [];
    this.keysLeft = [37];
    this.keysRight = [39];
    this.keysUp = [38];
    this.keysDown = [40];
    this._sensibility = 0.020;
    this.sensibility = 0.20;
  }

  FreeCameraKeyboardTranslateInput.prototype = { ...orthoInputProto, ...FreeCameraKeyboardTranslateInput.prototype }

  console.log(FreeCameraKeyboardTranslateInput.prototype)

  // Connect to camera:
  camera.inputs.add(new FreeCameraKeyboardTranslateInput());

  // Target the camera to scene origin
  camera.setTarget(BABYLON.Vector3.Zero());
  // Attach the camera to the canvas
  return camera;

}

export function setPositionScale(camera, loc, scale) {
  const [x, y] = loc
  camera.orthoLeft = - scale
  camera.orthoRight = scale
  camera.orthoTop = scale
  camera.orthoBottom = - scale

  let orth = camera.inputs.attached.keyboardTranslate

  orth.sensibility = orth._sensibility * camera.orthoRight
  console.log("cam,  ", camera.__proto__.sensibility, camera.__proto__._sensibility, camera.orthoRight)
  console.log("proto?? ", camera)
  camera.position = new BABYLON.Vector3(loc[0], loc[1], CAMERA_LEVEL)
}
