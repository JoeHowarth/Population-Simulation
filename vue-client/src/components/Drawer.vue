<template>
  <v-navigation-drawer permanent app v-model="drawer">
    <v-toolbar flat>
      <v-list>
        <v-list-tile>
          <v-list-tile-title class="title">
            Application
          </v-list-tile-title>
        </v-list-tile>
      </v-list>
    </v-toolbar>


    <v-list class="pt-0">
      <v-list-tile
          v-for="c in components"
          :key="c.name"
          @click="logClick"
      >
        <v-list-tile-action>
          <v-icon> {{c.icon}}</v-icon>
        </v-list-tile-action>

        <v-list-tile-content>
          <v-list-tile-title>
            {{c.name}}
          </v-list-tile-title>
        </v-list-tile-content>
      </v-list-tile>
    </v-list>

  </v-navigation-drawer>
</template>

<script>
  export default {
    name: "Drawer",
    data() {
      return {

        components: [
          {name: 'Home', icon: 'dashboard'},
          {name: 'About', icon: 'question_answer'}
        ],
        drawer: true,
      }
    },
    methods: {
      logClick() {
        console.log("Clicked")
      }
    }
  }
</script>

<style scoped>

</style>
