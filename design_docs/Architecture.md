
# Architecture
High level overview of each module's purpose and function.

### Modules
1. Terrain Generation
2. World Building / State Initialization
3. Server 
   3. Simulation 
   4. Networking
5. Client
   1. UI
   2. Graphical Map

### Terrain Generation
Constructs semi-random continents, islands, heightmap, biomes, raw resources (metal deposits etc.). 
Equivalent of Comp Geom final project, but bigger and better!
* Should use plate tectonics for world level, probably *external library*
* Then 'up-sample' using existing techniques to gain higher detail
* Separate process 
  - Either CLI or GUI
  - Probably long-running
  - Choose from several candidates before proceeding to World Building
  - Export data in common format for re-use (protobuf, json, ...)
  
### World Building / State Initialization 
Take terrain & climate and produce initial state of world before day0
* Example data
  - Agricultural: fertility, arable land, starting yield, area-under-cultivation, 
  - Population: starting cohorts, class mix, ++++
  - Settlements: locations, size, 
  - Production: types of goods, production capability, etc.
  - Trade: trade routes, commodity prices, flows etc.
  - States / political entities 
* Really all non-terrain data that the simulation engine will use 
  must be created 
* Challenges
  - Most parts of societies are inter-connected, which leads to the chicken 
    and egg problem of what causes what
  - Can't have starting state wildly out of equilibrium (ie once simulation starts, it can't 
    change too rapidly at the start)
  
## Server
Written in Rust, massively parallel, enables client and server to be on different machines. 
Later multiplayer support should be easily extended by 'simply' supporting multiple clients.
  
#### Simulation 
The 'dynamics' transforming State A + Actions --> State B. Consumes the initial state produced by 
**World Building**. 
* Needs a semi-realistic state to start from 
* Use specs ECS library to handle parallelism and as 'in-memory database'

#### Networking
* Receives **Subscription Requests** from client for types of data and optionally specific keys/ranges
* For each subscription, find data that A) has been updated and B) is a subbed key, then send to client
* Volume should be relatively low
  - Large, frequently read data sent once to client once (think static mapmodes) 
  - Most data doesn't change every day (tick)
  - Only send needed data
  - Only player visible data will be sent (in game mode, debug is different)
* Ideally process Subs >once per tick, maybe every 10ms of 'downtime'
* Receive **Actions** from player through client, feed into common Actions pool with AIs

## Client
#### UI
Use VueJS + Vuetify + Vuex to display data from server and send **Actions** and **Subscription Requests**

#### Graphical Map
Use BabylonJS to wrap WebGL. Map is a single mesh where each triangle can be set to a specific color
to graphically display 'map-modes' like height, fertility, population etc. Currently a flat, 2D map 
for easier camera, 'clicking' and just generally easier. Could move to 3D later

