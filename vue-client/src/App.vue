<template>
  <v-app>
    <v-toolbar app>
      <v-toolbar-title class="headline text-uppercase">
        <span>Vuetify</span>
        <span class="font-weight-light">MATERIAL DESIGN</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
    </v-toolbar>
    <!--<v-spacer></v-spacer>-->

    <Drawer/>

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
      }
    },
    computed: mapState(["positions"]),

  }
</script>
