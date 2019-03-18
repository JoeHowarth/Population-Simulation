import store from './store';
const url = 'ws://127.0.0.1:8090';
// Vue.use(VueNativeSock, url, {
//   format: 'json',
// reconnection : true,
// reconnectionAttemps: 5,
// reconnectionDelay: 1000,
// })
let Socket = new WebSocket(url);
Socket.subReq = subReq;
Socket.addEventListener("open", () => {
    store.commit("socketOnOpen");
    // request date subscription
    // subReq("Time", "Date", true);
});
Socket.sendObj = (obj) => {
    const str = JSON.stringify(obj);
    Socket.send(str);
};
Socket.addEventListener("message", function (msg) {
    // console.log(msg)
    let data = JSON.parse(msg.data);
    if (data.mutation) {
        store.commit(data.mutation, data.inner);
    }
    if (data.SubPush) {
        store.commit("subPush" + data.SubPush.section, data.SubPush);
    }
});
// could do validation of subscription requests, but let's just do it serverside
/*
let validation = new Map([
  ["Agr", new Set(["FarmData", "BaseFarmData", "FoodStore"])],
  ["Terr", new Set(["Region", "Weather", "RiverID", "LandMassID", ])]
])*/
export function subReq(section, component, insert, keys = false) {
    let obj = {
        SubReq: {
            section: section,
            insert,
            component,
        }
    };
    if (keys) {
        obj.keys = keys;
    }
    store.commit("updateActiveSubs", obj.SubReq);
    if (!insert) {
        store.commit("delData" + section, { section, component });
    }
    console.log("Sending SubReq: ", obj);
    Socket.sendObj(obj);
}
export default Socket;
//# sourceMappingURL=websocket.js.map