import store from '../store';


const agricultureStore  = {
  state: {
    FoodStock: null,
    BaseFarmData: null,
    FarmingData: null,
  },
  mutations: {
    subPushAgr(state, {section, data, component}) {
      console.assert(section === "Agr")

      state[component] = data
    },
  },
  getters: {

  }
}

export default agricultureStore
