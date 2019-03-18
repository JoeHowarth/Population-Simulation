<template>
  <v-toolbar dense app class="elevation-2">
    <v-toolbar-title class="headline text-uppercase">
      <span>Map </span>
      <span class="font-weight-light">Generator</span>
    </v-toolbar-title>
    <v-btn
        small
        v-for="view in views"
        :key="view.name"
        color="info"
        @click="navigate(view)"
    >
      {{view.name}}
    </v-btn>
    <v-spacer></v-spacer>
    <Date></Date>
  </v-toolbar>
</template>

<script>
  import store from '../store'
  import {tri_under_mouse} from "../map_gen/map_gen";
  import Date from './Date'

  export default {
    name: "Toolbar",
    components:{
      Date,
    },
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

              this.unMount = () => {
                window.canvas.removeEventListener('click', handler, false)
              }
            }
          },
          {
            name: 'Data Picker',
            component: 'DataSubPicker'
          },
        ],
        unMount: () => {}
      }
    },
    methods: {
      navigate(view) {
        console.log("[navigation]: ", view.component)
        if (this.unMount){
          this.unMount()
        }
        if (view.onMount) {
          view.onMount()
        }
        this.unMount = view.unMount
        store.commit('setActiveSideComponent', view.component)

      },
    },
  }
</script>

<style scoped>

</style>
