<template>
  <v-container class="grey lighten-5">
    <BackButton/>
    <v-btn @click="printSubs">print subs</v-btn>
    <v-list>
      <v-list-tile
          v-for="sec in activeSecs"
          :key="sec"
      >
        <v-list-tile-title>
          {{sec}}
        </v-list-tile-title>
        <v-list>
          <v-list-tile
              v-for="comp in activeSubs[sec]"
              :key="comp"
          >
            <v-list-tile-content
                @click="display(sec, comp)"
            >
              {{comp}}
            </v-list-tile-content>
            <v-list-tile-action>
              <v-btn icon @click="unsub(sec, comp)">
                <v-icon>close</v-icon>
              </v-btn>
            </v-list-tile-action>

          </v-list-tile>
        </v-list>

      </v-list-tile>

    </v-list>
    <v-form @submit="submit">
      <v-layout column>
        <v-flex xs12>
          <v-text-field v-model="section" label="Section"/>
          <v-text-field v-model="comp" label="Component"/>
          <v-btn @click="submit">Sub</v-btn>
        </v-flex>
      </v-layout>
    </v-form>
  </v-container>
</template>

<script>
  import {mapState, mapGetters} from 'vuex'
  import store from '../../store'
  import {updateColorsFunSparse} from "../../map_gen/render/webgl"
  import MapManager from '../../MapManager'
  import BackButton from '../util/BackButton'

  export default {
    name: "DataSubPicker",
    components: {
      BackButton
    },
    data() {
      return {
        section: '',
        comp: '',
      }
    },
    computed: {
      ...mapState(["activeSubs"]),
      ...mapGetters(['activeSecs'])
      // subs() {
      //   console.log(this.activeSubs)
      //   let arr = []
      //   for (let sec of this.activeSubs.) {
      //     arr.push(sec)
      //   }
      //   return arr
      // },
      // comps() {
      //   let o = {}
      //
      //   for (let sec of this.activeSubs.secs) {
      //     o[sec] =
      //   }
      //   for (let [sec, comp] of this.activeSubs.entries()) {
      //     o[sec] = Array.from(comp)
      //   }
      //   return o
      // }
    },
    methods: {
      printSubs() {
        console.log(this.subs)
        console.log(this.comps)
      },
      unsub(sec, comp) {
        console.log('unsub', sec, comp, false)
        this.$socket.subReq(sec, comp, false)
      },
      submit() {
        this.$socket.subReq(this.section, this.comp, true)
        this.section = ''
        this.comp = ''
      },
      display(sec, compName) {
        MapManager.setMap({sec, comp: compName, key_ind: 0})
      }
    }
  }
</script>

<style scoped>

</style>
