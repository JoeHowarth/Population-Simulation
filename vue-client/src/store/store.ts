/*eslint no-unused-vars: "off"*/
import Vue from 'vue'
import Vuex, {StoreOptions} from 'vuex'
import {updateColorsFun} from "../map_gen/render/webgl"
import {setMesh} from '../map_gen/map_gen'
import {main} from "../main"
import agricultureStore from './AgricultureStore'
import terrainStore from './TerrainStore'
import populationStore from './PopulationStore'
import {mesh} from '../map_gen/map_gen'
import Socket from "../websocket"

Vue.use(Vuex)

export interface RootState {
  sidePanel: {
    show: boolean,
    component: string
  },
  date: { year: number, month: string, day: number, str: string },
  speed: number,
  triClicked: boolean,
  mapColorData: MapData,
  activeSubs: {
    Time: string[],
    Agr: string[],
    Terr: string[],
    Pop: string[],
  },
  activeMapData: {
    section: string,
    component: string,
    field: string
  }
}

const store: StoreOptions<RootState> = {
  modules: {
    Agr: agricultureStore,
    Terr: terrainStore,
    Pop: populationStore,
  },
  state: {
    sidePanel: {
      show: false,
      component: null,
    },
    date: {year: 0, month: "January", day: 0, str: ""},
    speed: 1,
    triClicked: false,
    mapColorData: [],
    activeSubs: {
      Time: ["Date"],
      Agr: [],
      Terr: [],
      Pop: [],
    },
    activeMapData: {
      section: "Terr",
      component: "Height",
      field: "Height"
    }
  },
  
  mutations: {
    // --------------
    setHMesh(state, _mesh) {
      console.log("store", _mesh)
      setMesh(_mesh)
      main(_mesh)
    },
    setMapData(state, h) {
      state.mapColorData = h
      updateColorsFun(h)
    },
    
    // ----------------
    
    subPushMisc(state, {section, data, component}) {
      console.assert(section === "Misc")
      
      state[component] = data
    },
    subPushDate(state, {section, data, component}) {
      console.assert(section === "Date")
      
      state.date = data
    },
    toggleSidePanel(state, bool) {
      if (bool === null) {
        state.sidePanel.show = !state.sidePanel.show
      } else {
        state.sidePanel.show = bool
      }
    },
    setActiveSideComponent(state, comp) {
      state.sidePanel.component = comp
      state.sidePanel.show = true
    },
    setSpeed(state, val) {
      if (val === 0) {
        state.speed = 0
      } else if (val + state.speed >= 0) {
        state.speed += val
      }
      // Vue.prototype.Socket.sendAction("Time", "Speed", {value: val})
    },
    updateActiveSubs(state, {section, component, insert}) {
      let {activeSubs} = state
      
      
      if (insert) {
        state.activeSubs[section].push(component)
      } else {
        state.activeSubs[section] = state.activeSubs[section].filter(c => c !== component)
      }
    },
    setActiveMapData(state, md) {
      // auto update colors at same time?
      state.activeMapData = md
    },
    triClicked(state, tri) {
      state.triClicked = tri
    },
    // ----------------
  },
  actions: {},
  getters: {
    height() {
      return mesh.h
    },
    mesh() {
      return mesh
    },
    activeSecs(state) {
      return Object.keys(state.activeSubs)
          .filter(s => state.activeSubs[s].length > 0)
    }
  }
}

export default new Vuex.Store<RootState>(store)
