import MapManager from "../MapManager";
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
                state.Region = data;
                let buf = [];
                for (let reg of data) {
                    let color = Math.random();
                    for (let t of reg.tiles) {
                        console.log("reg: ", reg);
                        console.log("t: ", t);
                        buf[t.id] = color;
                    }
                }
                console.log("buf ", buf);
                MapManager.setMap({ res: buf });
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