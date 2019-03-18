<template>
  <v-container
      id="date-container"
      class="grey lighten-3"
      elevation-2
      pa-1
  >
    <v-layout
        id="date-layout"
        column="true"
        align-center
    >
      <v-spacer></v-spacer>
      <div class="text-xs-center" id="date-display">
        {{dateStr}}
      </div>
      <v-flex id="speed-control-cont">

        <v-btn @click="slower" class="speed-control" icon>
          <v-icon>chevron_left</v-icon>
        </v-btn>
        <v-btn @click="pause" class="speed-control" icon>
          <v-icon>pause</v-icon>
        </v-btn>
        <v-btn @click="faster" class="speed-control" icon>
          <v-icon>chevron_right</v-icon>
        </v-btn>
        <span>{{speed}}</span>

      </v-flex>

    </v-layout>
  </v-container>
</template>

<script>
  import store from "@/store"
  import {mapState} from 'vuex'

  export default {
    name: "Date",
    computed: {
      ...mapState(["date", "speed"]),
      dateStr() {
        // return this.date.m + " " + this.date.d +" " + this.date.y
        return this.date.str
      }
    },
    methods: {
      pause() {
        console.log("hit pause")
        store.commit("setSpeed", 0)
      },
      slower() {
        console.log("hit slower")
        store.commit("setSpeed", -1)
      },
      faster() {
        console.log("hit faster")
        store.commit("setSpeed", 1)
      },
    },
  }
</script>

<style scoped>
  #date-container {
    border: 1px solid #333;
    width: 180px;
    height: 60px;
    position: absolute;
    right: -1px;
    top: -1px;
  }

  #date-layout {
    width: 100%;
    height: 100%;
  }
  /*#date-display {*/
    /*height: 30px;*/
  /*}*/
  .speed-control {
    width: 18px;
    height: 18px;
    /*font-size: 9px;*/
    margin: 0;
  }
  #speed-control-cont {
    height: 25px;
    line-height: 10px;
  }

</style>
