import store from '../store';
import MapManager from "../MapManager"


const agricultureStore  = {
  state: {
    FoodStock: null,
    BaseFarmData: {fertility: [], arable: []},
    FarmingData: null,
  },
  mutations: {
    subPushAgr(state, {section, data, component}) {
      console.assert(section === "Agr")

      let c = state[component]

      let [{id}, value] = data[0]
      let keys = Object.keys(value)

      // initialize
      for (let key of keys) {
        if (!c[key]) {
          c[key] = []
        }
      }
      // populate
      for (let [{id}, value] of data) {
        for (let key of keys) {
          c[key][id] = value[key]
        }
      }
      // state[component] = data
    },
    delDataAgr(state, {section, component}) {
      state[component] = {}
      MapManager.checkActiveMapData(section, component)
    }
  },
  getters: {

  }
}

export default agricultureStore
