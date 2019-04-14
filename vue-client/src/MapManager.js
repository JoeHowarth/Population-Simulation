import store from './store/store';
import { updateColorsFun, NUM_TILES } from "./map_gen/render/webgl";
import { heightToColorArr } from "./map_gen/render/render-map";
import { mesh } from "@/map_gen/map_gen";
const DEFAULT_MAP_DATA = {
    section: "Terr",
    component: "Height",
    field: "Height"
};
class MapManager {
    constructor() {
        this.buf = new Float32Array(NUM_TILES);
    }
    setColor(vals, valToColor = heightToColorArr) {
        console.log("in set color, buf, vals: ", this.buf, vals);
        if (this.buf.length != NUM_TILES) {
            this.buf = new Float32Array(NUM_TILES);
        }
        for (let i = 0; i < NUM_TILES; i++) {
            this.buf[i] = (vals[i] !== undefined) ? vals[i] : -0.1;
        }
        updateColorsFun(this.buf, valToColor);
    }
    setColorByGroup(groups, vals, valToColor = heightToColorArr) {
        if (this.buf.length != NUM_TILES) {
            this.buf = new Float32Array(NUM_TILES);
        }
        for (let i = 0; i < groups.length; i++) {
            const g = groups[i];
            const val = vals[i];
            for (const id of g) {
                this.buf[id] = val;
            }
        }
        updateColorsFun(this.buf, valToColor);
    }
    checkActiveMapData(del_sec, del_comp) {
        const { section, component } = store.state.activeMapData;
        if (del_sec === section && del_comp === component) {
            this.setMap({ res: mesh.h, sec: DEFAULT_MAP_DATA.section, comp: DEFAULT_MAP_DATA.component });
        }
    }
    setMap({ res, sec, comp, key, key_ind, valToColor }) {
        console.log('setMap ', sec, comp, key, key_ind, res, valToColor);
        res = res ? res : store.state[sec][comp];
        console.log(res);
        if ((key_ind || key_ind === 0) && !key) {
            key = Object.keys(res)[key_ind];
            console.log(Object.keys(res));
        }
        console.log(key);
        let buf = key ? res[key] : res;
        console.log(buf);
        store.commit('setActiveMapData', { section: sec, component: comp });
        if (valToColor) {
            this.setColor(buf, valToColor);
        }
        else {
            this.setColor(buf);
        }
    }
}
function updateColorsFromMap(map, field) {
    let buf = new Float32Array(NUM_TILES);
    for (let i = 0; i < map.length; i++) {
        buf[i] = field ? map[i][field] : map[i];
    }
    updateColorsFun(buf);
}
let map_manager = new MapManager();
export default map_manager;
//# sourceMappingURL=MapManager.js.map