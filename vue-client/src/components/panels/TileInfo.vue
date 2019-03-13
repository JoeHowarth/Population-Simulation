<template>
  <v-card
      elevation-15
  >
    <back-button></back-button>
    <v-card-title primary-title>
      Tile Information
    </v-card-title>
    <v-layout
        column
    >
      <v-list>
        <v-list-tile>
          {{mesh.h[triClicked]}}
        </v-list-tile>
      </v-list>
    </v-layout>
  </v-card>

</template>

<script>
  import BackButton from '@/components/util/BackButton'
  import {getMesh} from '../../map_gen/map_gen'
  import {mapState} from 'vuex'

  export default {
    name: "TileInfo",
    components: {
      BackButton,
    },
    computed: {
      ...mapState(["triClicked"]),
      mesh() {
        return getMesh()
      }
    }
  }
</script>

<style scoped>

</style>
