<template>
  <v-app>
    <Toolbar></Toolbar>
    <v-content id="main_content">
      <canvas id="map_canvas"></canvas>
      <SidePanel></SidePanel>
    </v-content>
  </v-app>
</template>

<script>
  import {mapState} from 'vuex'
  import SidePanel from './components/SidePanel'
  import Toolbar from './components/Toolbar'

  export default {
    name: 'App',
    components: {
      SidePanel,
      Toolbar,
    },
    data() {
      return {

      }
    },
    computed: mapState(["positions", "mapColorData"]),

  }
</script>

<style scoped>
  #map_canvas {
    position: absolute;
    left: 0px;
    top: 0px;
    border: 1px solid beige;
  }

  #main_content {
    height: calc(100vh - 64px);
    /*margin-top: 64px;*/
  }
</style>
