import * as tslib_1 from "tslib";
import { PointerEventTypes, CameraInputTypes, serialize, Observable } from 'babylonjs';
/**
 * Manage the mouse inputs to control the movement of a free camera.
 * @see http://doc.babylonjs.com/how_to/customizing_camera_inputs
 */
export class OrthoCameraMouseInput {
    /**
     * Manage the mouse inputs to control the movement of a free camera.
     * @see http://doc.babylonjs.com/how_to/customizing_camera_inputs
     * @param touchEnabled Defines if touch is enabled or not
     */
    constructor(
    /**
     * Define if touch is enabled in the mouse input
     */
    touchEnabled = true) {
        this.touchEnabled = touchEnabled;
        /**
         * Defines the buttons associated with the input to handle camera move.
         */
        this.buttons = [0, 1, 2];
        /**
         * Defines the pointer angular sensibility  along the X and Y axis or how fast is the camera rotating.
         */
        this.angularSensibility = 2000.0;
        this.previousPosition = null;
        /**
         * Observable for when a pointer move event occurs containing the move offset
         */
        this.onPointerMovedObservable = new Observable();
        /**
         * @hidden
         * If the camera should be rotated automatically based on pointer movement
         */
        this._allowCameraRotation = true;
    }
    /**
     * Attach the input controls to a specific dom element to get the input from.
     * @param element Defines the element the controls should be listened from
     * @param noPreventDefault Defines whether event caught by the controls should call preventdefault() (https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)
     */
    attachControl(element, noPreventDefault) {
        var engine = this.camera.getEngine();
        if (!this._pointerInput) {
            this._pointerInput = (p) => {
                var evt = p.event;
                if (engine.isInVRExclusivePointerMode) {
                    return;
                }
                if (!this.touchEnabled && evt.pointerType === "touch") {
                    return;
                }
                if (p.type !== PointerEventTypes.POINTERMOVE && this.buttons.indexOf(evt.button) === -1) {
                    return;
                }
                let srcElement = (evt.srcElement || evt.target);
                if (p.type === PointerEventTypes.POINTERDOWN && srcElement) {
                    try {
                        srcElement.setPointerCapture(evt.pointerId);
                    }
                    catch (e) {
                        //Nothing to do with the error. Execution will continue.
                    }
                    this.previousPosition = {
                        x: evt.clientX,
                        y: evt.clientY
                    };
                    if (!noPreventDefault) {
                        evt.preventDefault();
                        element.focus();
                    }
                }
                else if (p.type === PointerEventTypes.POINTERUP && srcElement) {
                    try {
                        srcElement.releasePointerCapture(evt.pointerId);
                    }
                    catch (e) {
                        //Nothing to do with the error.
                    }
                    this.previousPosition = null;
                    if (!noPreventDefault) {
                        evt.preventDefault();
                    }
                }
                else if (p.type === PointerEventTypes.POINTERMOVE) {
                    if (!this.previousPosition || engine.isPointerLock) {
                        return;
                    }
                    var offsetX = evt.clientX - this.previousPosition.x;
                    var offsetY = evt.clientY - this.previousPosition.y;
                    if (this.camera.getScene().useRightHandedSystem) {
                        offsetX *= -1;
                    }
                    if (this.camera.parent && this.camera.parent._getWorldMatrixDeterminant() < 0) {
                        offsetX *= -1;
                    }
                    if (this._allowCameraRotation) {
                        // this.camera.cameraRotation.y += offsetX / this.angularSensibility;
                        // this.camera.cameraRotation.x += offsetY / this.angularSensibility;
                        if (!this._rect || !this._rect.width) {
                            this._rect = window.engine.getRenderingCanvasClientRect();
                        }
                        const W = this.camera.orthoRight * 2;
                        const H = this.camera.orthoTop * 2;
                        const px2viewx = W / this._rect.width;
                        const px2viewy = H / this._rect.height;
                        this.camera.position.x += -offsetX * px2viewx;
                        this.camera.position.y += offsetY * px2viewy;
                    }
                    this.onPointerMovedObservable.notifyObservers({ offsetX: offsetX, offsetY: offsetY });
                    this.previousPosition = {
                        x: evt.clientX,
                        y: evt.clientY
                    };
                    if (!noPreventDefault) {
                        evt.preventDefault();
                    }
                }
            };
        }
        this._onMouseMove = (evt) => {
            if (!engine.isPointerLock) {
                return;
            }
            if (engine.isInVRExclusivePointerMode) {
                return;
            }
            var offsetX = evt.movementX || evt.mozMovementX || evt.webkitMovementX || evt.msMovementX || 0;
            if (this.camera.getScene().useRightHandedSystem) {
                offsetX *= -1;
            }
            if (this.camera.parent && this.camera.parent._getWorldMatrixDeterminant() < 0) {
                offsetX *= -1;
            }
            // this.camera.cameraRotation.y += offsetX / this.angularSensibility;
            // this.camera.position.x += offsetX
            var offsetY = evt.movementY || evt.mozMovementY || evt.webkitMovementY || evt.msMovementY || 0;
            // this.camera.cameraRotation.x += offsetY / this.angularSensibility;
            // this.camera.position.y += offsetY
            this.previousPosition = null;
            if (!noPreventDefault) {
                evt.preventDefault();
            }
        };
        this._observer = this.camera.getScene().onPointerObservable.add(this._pointerInput, PointerEventTypes.POINTERDOWN | PointerEventTypes.POINTERUP | PointerEventTypes.POINTERMOVE);
        element.addEventListener("mousemove", this._onMouseMove, false);
        element.addEventListener("contextmenu", this.onContextMenu.bind(this), false);
    }
    /**
     * Called on JS contextmenu event.
     * Override this method to provide functionality.
     */
    onContextMenu(evt) {
        evt.preventDefault();
    }
    /**
     * Detach the current controls from the specified dom element.
     * @param element Defines the element to stop listening the inputs from
     */
    detachControl(element) {
        if (this._observer && element) {
            this.camera.getScene().onPointerObservable.remove(this._observer);
            if (this._onMouseMove) {
                element.removeEventListener("mousemove", this._onMouseMove);
            }
            if (this.onContextMenu) {
                element.removeEventListener("contextmenu", this.onContextMenu);
            }
            if (this.onPointerMovedObservable) {
                this.onPointerMovedObservable.clear();
            }
            this._observer = null;
            this._onMouseMove = null;
            this.previousPosition = null;
        }
    }
    /**
     * Gets the class name of the current intput.
     * @returns the class name
     */
    getClassName() {
        return "OrthoCameraMouseInput";
    }
    /**
     * Get the friendly name associated with the input class.
     * @returns the input friendly name
     */
    getSimpleName() {
        return "mouse";
    }
}
tslib_1.__decorate([
    serialize()
], OrthoCameraMouseInput.prototype, "buttons", void 0);
tslib_1.__decorate([
    serialize()
], OrthoCameraMouseInput.prototype, "angularSensibility", void 0);
CameraInputTypes["OrthoCameraMouseInput"] = OrthoCameraMouseInput;
export default OrthoCameraMouseInput;
//# sourceMappingURL=mouseCameraInput.js.map