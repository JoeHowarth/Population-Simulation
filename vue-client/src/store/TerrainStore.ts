import store, {RootState} from './store'
import MapManager from "../MapManager"
import {Module} from 'vuex'
import {SubPush, SubReq} from "@/websocket"

export interface TerrState {
  Region: any,
  Weather: any,
  RiverID: any,
  LandMassID: any,
  TileTopography: any
}

const populationStore: Module<TerrState, RootState> = {
  state: {
    Region: {},
    Weather: {},
    RiverID: {},
    LandMassID: {},
    TileTopography: {},
  },
  mutations: {
    subPushTerr(state: TerrState, {section, data, component}: SubPush) {
      console.assert(section === 'Terr')
      
      if (component === 'Region') {
        console.log("region data", data)
        
        state.Region = data
        
        let buf = []
        for (let reg of data) {
          let color = Math.random()
          for (let t of reg.tiles) {
            console.log("reg: ", reg)
            console.log("t: ", t)
            buf[t.id] = color
          }
        }
        
        console.log("buf ", buf)
        MapManager.setMap({res: buf})
        
        return
      }
      
      let c = state[component]
      
      // assuming data is array
      const arr = <any[]>data;
      let [id_obj, value] = arr[0]
      if (id_obj['id']) {
        
        let keys = Object.keys(value)
        
        // initialize
        for (let key of keys) {
          if (!c[key]) {
            c[key] = []
          }
        }
        // populate
        for (let [{id}, value] of arr) {
          for (let key of keys) {
            c[key][id] = value[key]
          }
        }
      }
    }
  }
}


export default populationStore
