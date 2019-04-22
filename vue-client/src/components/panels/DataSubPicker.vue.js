-container;
class {
}
"grey lighten-5";
"submit"
    >
        />
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
    < !-- < v - form;
"submit" > -- >
    -- < v - layout;
column > -- >
    -- < v - flex;
xs12 > -- >
    -- < v - text - field;
v - model;
"section";
label = "Section" > /v-text-field>-->
    < !-- < v - text - field;
v - model;
"comp";
label = "Component" > /v-text-field>-->
    < !-- < v - btn;
"submit" > Sub < /v-btn>-->
    < !-- < /v-flex>-->
    < !-- < /v-layout>-->
    < !-- < /v-form>-->
    < v - select;
v - model;
"section";
items = "sections";
attach;
chips;
label = "Section"
    > /v-select>
    < v - select;
v - model;
"comp";
items = "comps";
attach;
chips;
label = "Component";
"submit";
"noop"
    > /v-select>
    < v - btn;
"submit" > Sub < /v-btn>
    < /v-container>
    < /template>
    < script >
;
import { mapState, mapGetters } from 'vuex';
import MapManager from '../../MapManager';
import BackButton from '../util/BackButton';
import Socket, { SectionList } from '@/websocket';
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
        ...mapGetters(['activeSecs']),
        sections() {
            return SectionList;
        },
        comps() {
            const state = this.$store.state;
            if (state.hasOwnProperty(this.section)) {
                return Object.keys(state[this.section]);
            }
            else {
                return [];
            }
        }
    },
    methods: {
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