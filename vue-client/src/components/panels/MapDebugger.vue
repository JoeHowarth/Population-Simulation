<template>
    <v-layout
        column="true"
        class="grey lighten-5"
        elevation-5
    >
      <back-button></back-button>
      <v-btn color="info"  @click="generateMap">Generate Map File</v-btn>
      <v-btn color="info"  @click="setHeight">Height Map</v-btn>
      <v-btn color="info"  @click="setER">Erosion Rate</v-btn>
      <v-btn color="info"  @click="setFlux">Water Flux</v-btn>
      <v-btn color="info"  @click="setSlope">Slope</v-btn>
      <v-btn color="info"  @click="setCityScore">City Score</v-btn>
      <v-btn color="info"  @click="setCities">Place Cities</v-btn>
      <v-btn color="info"  @click="setSent">View Last Sent</v-btn>
      <v-btn color="info"  @click="setNormalize">Normalize Last Sent</v-btn>
      <v-btn color="info"  @click="setPeaky">Peaky Last Sent</v-btn>
      <v-btn color="info"  @click="quickStats">Log Stats on Sent</v-btn>
      <v-btn color="error" @click="regen">Regen</v-btn>
    </v-layout>
</template>

<script>
  import {updateColorsFun} from "../../map_gen/render/webgl";
  import mapGen, {cities, exportMesh, getER, getHeight, getMesh, showCities} from "../../map_gen/map_gen";
  import {cityScore, getFlux, getSlope, normalize, peaky, quick_stats} from "../../map_gen/heightmap";
  import BackButton from '@/components/util/BackButton'

  export default {
    name: "MapDebugger",
    components: {
      BackButton
    },
    methods: {
      async generateMap() {
        console.log("in generateMap")
        let [mesh, h] = await mapGen()
        mesh.h = h
        this.$store.commit("setHMesh", mesh)
        exportMesh(mesh, true)
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
    }
  }
</script>

<style scoped>

</style>
