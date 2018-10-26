<template>
  <v-app>
    <v-toolbar app>
      <v-toolbar-title class="headline text-uppercase">
        <span>Vuetify</span>
        <span class="font-weight-light">MATERIAL DESIGN</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
    </v-toolbar>
    <v-spacer></v-spacer>

    <Drawer/>


    <v-content>
      <v-container>
        <v-data-table
            :headers="headers"
            :items="positions"
            hide-actions
            class="elevation-1"
        >
          <template slot="items" slot-scope="props">
            <td>{{ props.item.x }}</td>
            <td class="text-xs-right">{{ props.item.x }}</td>
            <td class="text-xs-right">{{ props.item.y }}</td>
          </template>
        </v-data-table>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
  import HelloWorld from './components/HelloWorld'
  import Drawer from './components/Drawer'
  import { mapState } from 'vuex'

  export default {
    name: 'App',
    components: {
      HelloWorld,
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
        // positions: [
        //   {
        //     id: 1,
        //     x: 52,
        //     y: 4,
        //   },
        //   {
        //     id: 2,
        //     x: 12,
        //     y: 42,
        //   },
        //   {
        //     id: 5,
        //     x: 2,
        //     y: 7,
        //   },
        // ]
      }
    },
    methods: {
      logClick() {
        console.log("Clicked")
      }
    },
    computed: mapState(["positions"]),

  }
</script>
