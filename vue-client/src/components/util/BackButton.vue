<template>
  <v-btn icon @click="toggleShow" class="align-self-start" size="28">
    <v-icon>arrow_back</v-icon>
  </v-btn>
</template>

<script>
  import store from '../../store/store'
  import {Component, Vue} from 'vue-property-decorator'

  @Component
  export default class BackButton extends Vue {
    toggleShow() {
      store.commit("toggleSidePanel")
    }
  }
  // export default {
  //   name: "BackButton",
  //   methods: {
  //     toggleShow() {
  //       store.commit("toggleSidePanel")
  //     },
  //   }
  // }
</script>

<style scoped>

</style>
