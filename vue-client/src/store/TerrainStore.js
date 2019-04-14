import MapManager from "../MapManager";
import { mesh } from "../map_gen/map_gen";
import * as d3 from 'd3';
const populationStore = {
    state: {
        Region: {},
        Weather: {},
        RiverID: {},
        LandMassID: {},
        TileTopography: {},
    },
    mutations: {
        subPushTerr(state, { section, data, component }) {
            console.assert(section === 'Terr');
            if (component === 'Region') {
                console.log("region data", data);
                state.Region = [];
                let buf = [];
                for (let reg of data) {
                    let color = Math.random();
                    state.Region.push([]);
                    for (let t of reg.tiles) {
                        //console.log("reg: ", reg)
                        //console.log("t: ", t)
                        //buf[t.id] = color
                        state.Region[state.Region.length - 1].push(t.id);
                    }
                }
                console.log("state.Region ", state.Region);
                MapManager.setColorByGroup(state.Region, state.Region.map(tiles => Math.random() / 5 + d3.mean(tiles.map(t => mesh.h[t]))));
                return;
            }
            let c = state[component];
            // assuming data is array
            const arr = data;
            let [id_obj, value] = arr[0];
            if (id_obj['id']) {
                let keys = Object.keys(value);
                // initialize
                for (let key of keys) {
                    if (!c[key]) {
                        c[key] = [];
                    }
                }
                // populate
                for (let [{ id }, value] of arr) {
                    for (let key of keys) {
                        c[key][id] = value[key];
                    }
                }
            }
        }
    }
};
export default populationStore;
//# sourceMappingURL=TerrainStore.js.map