<template>
  <v-app>
    <v-toolbar v-if="false" app>
      <v-toolbar-title class="headline text-uppercase">
        <span>Map </span>
        <span class="font-weight-light">Generator</span>
      </v-toolbar-title>
      <v-btn color="info" @click="setHeight">Height Map</v-btn>
      <v-btn color="info" @click="setER">Erosion Rate</v-btn>
      <v-btn color="info" @click="setFlux">Water Flux</v-btn>
      <v-btn color="info" @click="setSlope">Slope</v-btn>
      <v-btn color="info" @click="setCities">Cities</v-btn>
      <v-btn color="error" @click="regen">Regen</v-btn>
      <v-spacer></v-spacer>
    </v-toolbar>

    <!--<v-spacer></v-spacer>-->

    <v-navigation-drawer permanent app v-model="drawer">
      <v-toolbar flat>
        <v-list>
          <v-list-tile>
            <v-list-tile-title class="headline text-uppercase">
              Map
              <span class="font-weight-light">Generator</span>

            </v-list-tile-title>
          </v-list-tile>
        </v-list>
      </v-toolbar>


        <v-container>
          <v-layout column="true" >
            <v-btn color="info" @click="setHeight">Height Map</v-btn>
            <v-btn color="info" @click="setER">Erosion Rate</v-btn>
            <v-btn color="info" @click="setFlux">Water Flux</v-btn>
            <v-btn color="info" @click="setSlope">Slope</v-btn>
            <v-btn color="info" @click="setCityScore">City Score</v-btn>
            <v-btn color="info" @click="setCities">Place Cities</v-btn>
            <v-btn color="info" @click="setSent">View Last Sent</v-btn>
            <v-btn color="info" @click="setNormalize">Normalize Last Sent</v-btn>
            <v-btn color="info" @click="setPeaky">Peaky Last Sent</v-btn>
            <v-btn color="info" @click="quickStats">Log Stats on Sent</v-btn>
            <v-btn color="error" @click="regen">Regen</v-btn>
          </v-layout>
        </v-container>

    </v-navigation-drawer>
    <!--<Drawer/>-->

    <v-content>
        <canvas id="map_canvas"></canvas>
    </v-content>

    <!--<v-content>-->
      <!--<v-container>-->
        <!--<v-data-table-->
            <!--:headers="headers"-->
            <!--:items="positions"-->
            <!--hide-actions-->
            <!--class="elevation-1"-->
        <!--&gt;-->
          <!--<template slot="items" slot-scope="props">-->
            <!--<td>{{ props.item.x }}</td>-->
            <!--<td class="text-xs-right">{{ props.item.x }}</td>-->
            <!--<td class="text-xs-right">{{ props.item.y }}</td>-->
          <!--</template>-->
        <!--</v-data-table>-->
      <!--</v-container>-->
    <!--</v-content>-->
  </v-app>
</template>

<script>
  import Drawer from './components/Drawer'
  import { mapState } from 'vuex'
  import {updateColorsFun} from "./map_gen/render/webgl";
  import {cities, getER, getHeight, getMesh, showCities} from "./map_gen/map_gen";
  import mapGen from './map_gen/map_gen'
  import {cityScore, getFlux, getSlope, normalize, peaky, quick_stats} from "./map_gen/heightmap";

  export default {
    name: 'App',
    components: {
      Drawer,
    },
    data() {
      return {
        components: [
          {name: 'Home', icon: 'dashboard'},
          {name: 'About', icon: 'question_answer'}
        ],
        drawer: true,
        headers: [
          {
            text: 'IDs ',
            align: 'left',
            sortable: false,
            value: 'id'
          },
          {text: 'X', value: 'x'},
          {text: 'Y', value: 'y'},
        ],
      }
    },
    methods: {
      logClick() {
        console.log("Clicked")
      },
      setHeight() {
        updateColorsFun(getHeight())
      },
      setER() {
        updateColorsFun(getER())
      },
      setFlux() {
        updateColorsFun(peaky(peaky(getFlux(getMesh(), getHeight()))))
      },
      setSlope() {
        const vals = normalize(getSlope(getMesh(), getHeight()), 0.5)
        updateColorsFun(vals)
      },
      regen() {
        mapGen()
      },
      setCities() {
        showCities()
      },
      setSent() {
        console.log("HI")
        console.log(this.mapColorData.inner)
        updateColorsFun(this.mapColorData.inner)
      },
      setNormalize() {
        updateColorsFun(normalize(this.mapColorData.inner))
      },
      setPeaky() {
        updateColorsFun(peaky(this.mapColorData.inner))
      },
      quickStats() {
        quick_stats(this.mapColorData.inner)
      },
      setCityScore() {
        const vals = (normalize(cityScore(getMesh(), getHeight(), cities), 0.2))
        updateColorsFun(vals)
      }
    },
    computed: mapState(["positions", "mapColorData"]),

  }
</script>
