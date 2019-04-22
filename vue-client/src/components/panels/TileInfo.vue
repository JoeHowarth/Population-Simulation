<template>
  <v-card
      elevation-15
  >
    <back-button></back-button>
    <v-card-title primary-title>
      Tile Information
    </v-card-title>
    <v-container>
      <v-layout
          column
      >
        <v-select
            v-model="section"
            :items="sections"
            attach
            chips
            label="Section"
        ></v-select>
        <v-select
            v-model="comp"
            :items="comps"
            attach
            chips
            label="Component"
        ></v-select>
        <v-btn @click="setDeps">Choose</v-btn>
        <v-list>
          <v-list-tile>
            {{mesh.h[triClicked]}}
          </v-list-tile>
          <v-list-tile
              v-for="d in display"
          >
            {{ d }}
          </v-list-tile>
        </v-list>
      </v-layout>
    </v-container>
  </v-card>

</template>

<script>
  import BackButton from '@/components/util/BackButton'
  import {getMesh} from '../../map_gen/map_gen'
  import {mapState} from 'vuex'

  export default {
    name: "TileInfo",
    data() {
      return {
        section: '',
        comp: '',
        deps: []
      }
    },
    components: {
      BackButton,
    },
    computed: {
      ...mapState(["triClicked", "activeSubs", "activeMapData"]),
      sections() {
        return Object.keys(this.activeSubs).filter(sec =>
          sec
          && this.activeSubs[sec].length
          && this.activeSubs[sec].length > 0
        )
      },
      comps() {
        return (this.section !== '') ? this.activeSubs[this.section] : [""]
      },
      mesh() {
        return getMesh()
      },

      data() {
        console.log("in computed data ", this.deps)
        let d = this.deps.map(({section, comp}) => this.$store.state[section][comp])
        console.log("data: ", d)
        return d
      },
      display() {
        console.log("in display")
        return this.data.map(d => {
          console.log(d)
          if (Array.isArray(d)) {
            return d[this.triClicked]
          } else {
            return Object.keys(d).map(k => {
              return Array.isArray(d[k]) ? d[k][this.triClicked] : 'not array'
            });
          }
        });
      },
    },
    methods: {
      setDeps() {
        console.log(this.section, this.comp)
        let newDeps = {section: this.section, comp: this.comp}
        console.log(newDeps)

        this.deps.push(newDeps)
        console.log(this.deps)

        this.section = ''
        this.comp = ''
      }
    }
  }
</script>

<style scoped>

</style>
