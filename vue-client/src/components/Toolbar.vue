<template>
  <v-toolbar app>
    <v-toolbar-title class="headline text-uppercase">
      <span>Map </span>
      <span class="font-weight-light">Generator</span>
    </v-toolbar-title>
    <v-btn
        v-for="view in views"
        :key="view.name"
        color="info"
        @click="navigate(view)"
    >
      {{view.name}}
    </v-btn>
    <v-spacer></v-spacer>
  </v-toolbar>
</template>

<script>
  import store from '../store'
  import {tri_under_mouse} from "../map_gen/map_gen";

  export default {
    name: "Toolbar",
    data() {
      return {
        views: [
          {
            name: 'Map Debugger',
            component: 'MapDebugger'
          },
          {
            name: 'Tile Info',
            component: 'TileInfo',
            onMount() {
              console.log("Tile Info -> onMount")
              const handler = () => {
                store.commit('triClicked', tri_under_mouse())
              }
              window.canvas.addEventListener('click', handler, false)

              this.unmount = () => {
                window.canvas.removeEventListener('click', handler, false)
              }
            }
          },
        ],
        unmount: () => {}
      }
    },
    methods: {
      navigate(view) {
        console.log("[navigation]: ", view.component)
        this.unmount()
        if (view.onMount) {
          view.onMount()
        }
        store.commit('setActiveSideComponent', view.component)

      },
    },
  }
</script>

<style scoped>

</style>
