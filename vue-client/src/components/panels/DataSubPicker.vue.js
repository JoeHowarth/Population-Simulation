-container;
class {
}
"grey lighten-5" >
    />
    < v - btn;
"printSubs" > print;
subs < /v-btn>
    < v - list >
    -list - tile;
v - ;
for ( = "sec in activeSecs"; ; )
    : key = "sec"
        >
            -list - tile - title >
        {};
{
    sec;
}
/v-list-tile-title>
    < v - list >
    -list - tile;
v - ;
for ( = "comp in activeSubs[sec]"; ; )
    : key = "comp"
        >
            -list - tile - content;
"display(sec, comp)"
    >
        {};
{
    comp;
}
/v-list-tile-content>
    < v - list - tile - action >
    -btn;
icon;
"unsub(sec, comp)" >
    -icon > close < /v-icon>
    < /v-btn>
    < /v-list-tile-action>
    < /v-list-tile>
    < /v-list>
    < /v-list-tile>
    < /v-list>
    < v - form;
"submit" >
    -layout;
column >
    -flex;
xs12 >
    -text - field;
v - model;
"section";
label = "Section" /  >
    -text - field;
v - model;
"comp";
label = "Component" /  >
    -btn;
"submit" > Sub < /v-btn>
    < /v-flex>
    < /v-layout>
    < /v-form>
    < /v-container>
    < /template>
    < script >
;
import { mapState, mapGetters } from 'vuex';
import MapManager from '../../MapManager';
import BackButton from '../util/BackButton';
import Socket from '@/websocket';
export default {
    name: "DataSubPicker",
    components: {
        BackButton
    },
    data() {
        return {
            section: '',
            comp: '',
        };
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
            console.log(this.subs);
            console.log(this.comps);
        },
        unsub(sec, comp) {
            console.log('unsub', sec, comp, false);
            Socket.subReq(sec, comp, false);
        },
        submit() {
            Socket.subReq(this.section, this.comp, true);
            this.section = '';
            this.comp = '';
        },
        display(sec, compName) {
            MapManager.setMap({ sec, comp: compName, key_ind: 0 });
        }
    }
}
    < /script>
    < style;
scoped >
    /style>;
//# sourceMappingURL=DataSubPicker.vue.js.map