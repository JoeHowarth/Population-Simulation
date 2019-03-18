/*eslint no-unused-vars: "off"*/
import Vue from 'vue';
import Vuex from 'vuex';
import { updateColorsFun } from "./map_gen/render/webgl";
import { setMesh } from 'map_gen/map_gen';
import { main } from "./main";
import agricultureStore from './store/AgricultureStore';
import terrainStore from './store/TerrainStore';
import populationStore from './store/PopulationStore';
import { mesh } from './map_gen/map_gen';
import Socket from "@/websocket";
Vue.use(Vuex);
export default new Vuex.Store({
    modules: {
        Agr: agricultureStore,
        Terr: terrainStore,
        Pop: populationStore,
    },
    state: {
        socket: {
            isConnected: false,
            message: '',
            reconnectError: false,
            bufferedMessages: []
        },
        sidePanel: {
            show: false,
            component: null,
        },
        date: { year: 0, month: "January", day: 0, str: "" },
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
            console.log("store", _mesh);
            setMesh(_mesh);
            main(_mesh);
        },
        setMapData(state, h) {
            state.mapColorData = h;
            updateColorsFun(h);
        },
        // ----------------
        subPushMisc(state, { section, data, component }) {
            console.assert(section === "Misc");
            if (component === "Date") {
                setDate(state, data);
            }
            else {
                state[component] = data;
            }
        },
        subPushDate(state, { section, data, component }) {
            console.assert(section === "Date");
            state.date = data;
        },
        toggleSidePanel(state, bool) {
            if (bool === null) {
                state.sidePanel.show = !state.sidePanel.show;
            }
            else {
                state.sidePanel.show = bool;
            }
        },
        setActiveSideComponent(state, comp) {
            state.sidePanel.component = comp;
            state.sidePanel.show = true;
        },
        setSpeed(state, val) {
            if (val === 0) {
                state.speed = 0;
            }
            else if (val + state.speed >= 0) {
                state.speed += val;
            }
            // Vue.prototype.Socket.sendAction("Time", "Speed", {value: val})
        },
        updateActiveSubs(state, { section, component, insert }) {
            let { activeSubs } = state;
            if (insert) {
                state.activeSubs[section].push(component);
            }
            else {
                state.activeSubs[section] = state.activeSubs[section].filter(c => c !== component);
            }
        },
        setActiveMapData(state, md) {
            // auto update colors at same time?
            state.activeMapData = md;
        },
        triClicked(state, tri) {
            state.triClicked = tri;
        },
        // ----------------
        SOCKET_BUFFER_MSG(state, msg) {
            state.socket.bufferedMessages.push(msg);
        }
    },
    actions: {
        sendMessage: function (context, message) {
            console.log(Socket);
            if (Socket.readyState !== 1) {
                context.commit('SOCKET_BUFFER_MSG', message);
                if (Socket.onopen) {
                    Socket.onopen = (e) => context.state.socket.bufferedMessages.forEach(msg => socket.send(msg));
                }
            }
            else {
                Socket.send(message);
            }
        }
    },
    getters: {
        height() {
            return mesh.h;
        },
        mesh() {
            return mesh;
        },
        activeSecs(state) {
            return Object.keys(state.activeSubs)
                .filter(s => state.activeSubs[s].length > 0);
        }
    }
});
//# sourceMappingURL=store.js.map