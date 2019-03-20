const populationStore = {
    state: {
        RegionCohorts: {}
    },
    mutations: {
        subPushPop(state, { section, data, component }) {
            console.assert(section === 'Pop');
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
//# sourceMappingURL=PopulationStore.js.map