<template>
  <v-container
      v-if="sidePanel.show"
      id="side_panel"
      ma-2
      pa-0
  >
    <component
        v-bind:is="sidePanel.component"
    />
  </v-container>
</template>

<script>
  import {mapState} from 'vuex'
  import MapDebugger from './panels/MapDebugger'
  import TileInfo from './panels/TileInfo'
  import DataSubPicker from './panels/DataSubPicker'

  export default {
    name: "SidePanel",
    data() {
      return {}
    },
    components: {
      MapDebugger,
      TileInfo,
      DataSubPicker,
    },
    computed: {
      ...mapState(["sidePanel"]),
    },
  }
</script>

<style scoped>
  #side_panel {
    width: 30%;
    position: absolute;
    left: 0;
    top: 0;
  }
</style>
