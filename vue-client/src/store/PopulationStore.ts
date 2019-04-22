import store, {RootState} from './store'
import MapManager from "../MapManager"
import {Module} from 'vuex'
import {SubPush, SubReq} from "@/websocket"

export interface PopState {
  RegionPop: any
}

const populationStore: Module<PopState, RootState> = {
  state: {
    RegionPop: {}
  },
  mutations: {
    subPushPop(state: PopState, {section, data, component}: SubPush) {
      console.assert(section === 'Pop')
  
      console.log(data)
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
