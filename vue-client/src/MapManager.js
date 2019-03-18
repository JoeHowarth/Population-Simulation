import store from 'store'
import {updateColorsFun, updateColorsFunSparse} from "./map_gen/render/webgl"
import {mesh} from './map_gen/map_gen'

const DEFAULT_MAP_DATA = {
  section: "Terr",
  component: "Height",
  field: "Height"
}

function checkActiveMapData(del_sec, del_comp) {
  const {section, component} = store.state.activeMapData
  if (del_sec === section && del_comp === component) {
    setMap({res:mesh.h, sec: DEFAULT_MAP_DATA.section, comp: DEFAULT_MAP_DATA.component})
  }
}

function setMap({res, sec, comp, key, key_ind}) {
  console.log('setMap ', sec, comp, key, key_ind, res)
  res = res? res : store.state[sec][comp]
  console.log(res)
  if ((key_ind || key_ind === 0)&& !key) {
    key = Object.keys(res)[key_ind]
    console.log(Object.keys(res))
  }
  console.log(key)
  let buf = key? res[key] : res
  console.log(buf)
  store.commit('setActiveMapData', {section: sec, component: comp})
  updateColorsFunSparse(buf)
}

export default {checkActiveMapData, setMap}
