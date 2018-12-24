/*eslint no-unused-vars: "off"*/
import Vue from 'vue'
import Vuex from 'vuex'
import {updateColorsFun} from "./map_gen/render/webgl";
import {getHeight} from "./map_gen/map_gen";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    socket: {
      isConnected: false,
      message: '',
      reconnectError: false,
      bufferedMessages: []
    },
    mapColorData: [],
    mesh: {},
  },
  mutations:{
    setMapData(state, h) {
      state.mapColorData = h
      updateColorsFun(h)
    },
    SOCKET_ONOPEN (state, event)  {
      Vue.prototype.$socket = event.currentTarget
      state.socket.isConnected = true
    },
    SOCKET_ONCLOSE (state, event)  {
      state.socket.isConnected = false
    },
    SOCKET_ONERROR (state, event)  {
      console.error(state, event)
    },
    // mutations for reconnect methods
    SOCKET_RECONNECT(state, count) {
      console.info(state, count)
    },
    SOCKET_RECONNECT_ERROR(state) {
      state.socket.reconnectError = true;
    },
    SOCKET_BUFFER_MSG(state, msg) {
      state.socket.bufferedMessages.push(msg)
    }
  },
  actions: {
    sendMessage: function(context, message) {
      const socket = Vue.prototype.$socket
      console.log(socket)
      if (socket.readyState !== 1) {
        context.commit('SOCKET_BUFFER_MSG', message)
        if (socket.onopen) {
          socket.onopen = (e) => context.state.socket.bufferedMessages.forEach(msg => socket.send(msg))
        }
      }
      else {
        socket.send(message)
      }
    }
  }
})

/*
import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    entities: [],
    positions: [],
    socket: {
      connected: false,
      lastMessage: "",
      reconnectError: ""
    },
  },
  mutations: {
    // default handler called for all methods
    SOCKET_ONMESSAGE(state, message) {
      state.socket.message = message
      console.log("ONMESSAGE", message)

    },

    SOCKET_addEntity(state, payload) {
      state.entities.push(state.entities.length)
    },

    SOCKET_set_pos(state, payload) {
      console.log(payload)

      // Vue.set(state.position, entity, [x,y])
      console.log("hi")
    },
    SOCKET_SET_POS(state, payload) {
      // Vue.set(state.position, entity, [x,y])
      console.log(payload)
      console.log("HI")
    },

    SOCKET_ONOPEN(state, event) {
      console.log("Connected")
      Vue.prototype.$socket = event.currentTarget
      state.socket.isConnected = true
    },

    SOCKET_ONCLOSE(state, event) {
      state.socket.isConnected = false
    },

    SOCKET_ONERROR(state, event) {
      console.error(state, event)
    },

    // mutations for reconnect methods
    SOCKET_RECONNECT(state, count) {
      console.info(state, count)
    },

    SOCKET_RECONNECT_ERROR(state) {
      state.socket.reconnectError = true;
    },
  },
  actions: {
    sendMessage: function (context, message) {
      // probably do something more interesting here
      Vue.prototype.$socket.send(message)
    }
  }
})
*/
