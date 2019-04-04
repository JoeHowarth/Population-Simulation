## Terrain Generator Module

This module will produce the base maps that the rest of the world is built on top of.
This module should be standalone and therefore easily runnable and configurable without the 
rest of the project. This has several advantages:
1. Faster compilation times => faster iteration feedback loop 
2. Ability to generate many candidate maps and select the most interesting ones 
   to build a full world with.
3. Loading a world from disk is, generally, much faster than having to recompute it each 
   time from a seed => faster simulation load times.
4. Decoupling module allows easier contribution from volunteers who may not be 
   familiar with Rust (cough, cough, Kostas)
5. Can extend 3rd party plate tectonics libraries in their native language.


### Current Status
This is simultaneously the most complicated and least long-term aspect of the project.
Currently written in Javascript, the current client grew out of the map generator because
that initially allowed the easiest way to visualize the output directly without networking 
infrastructure. 

The current map generator is heavily inspired by Martin O'Leary's Polygonal [Fantasy Map Generator](Amit Patel http://www-cs-students.stanford.edu/~amitp/game-programming/polygon-map-generation).
It computational geometry techniques and data structures ([more detail](MapGen_README.md)) that make for good regional scale maps
with plausible erosion and water flux, but the random hill placement does not scale well beyond 
~500x500 km maps. Additionally, while the use of irregular triangles makes for a non-uniform and thus
organic looking map, it also significantly complicates many implementations. 

### Plans 
With this in mind, I'm currently considering a full rewrite using a 2-stage generation process.
**Stage I** will use use a simpler grid representation at lower resolution and generate the macro
level features of a world map. **Stage II** will 'up-sample' this low resolution map roughly to the detail
of the current polygonal generator. This will add low level detail and variation using some of 
my existing techniques without slowing down the macro plate tectonics etc. 

##### Stage I
This stage should have a resolution between 100,000 km2 to 1,000 km2.
The heightmap generation should include 
1. Tectonic drift
2. Erosion and importantly *fluvial deposition* (which was absent)
3. Hot Spots (hopefully)
4. Old Mountains

A water system becomes immediately necessary for the above and has obvious further utility
for placing rivers, lakes and generating the climate model and biomes. I have some concern with
artifacts arising from a grid based system, but hopefully **Stage II** can remove this.


While I would like to implement this myself as it's an interesting problem, it would likely take
more than a year considering my own experiences with a comparatively simple map generator 
and existing projects such as [TectonicsJS](https://github.com/davidson16807/tectonics.js) 
or [WorldEngine](https://github.com/Mindwerks/worldengine)'s timescale. The most pragmatic 
option would be to use an existing generator's output files and adapt them. Another possibility
doing a rewrite closely following an existing solution. None of the resources I've found so
far are completely satisfactory, but the don't need to be if I can do some post-processing passes
and then up-sample once the base terrain has been generated. 

I'm particularly interested in TectonicJS's 2.0 architecture as it appears to be an extensible,
efficient fluid-dynamics approximation that could be used beyond plate tectonics for atmospheric
modelling, rivers and maybe even ocean currents.

##### Stage II
This stage should have a resolution between 100km2 to 10 km2. For this to be feasible only land 
and ocean near land should be at this resolution and there could even be 2 'lowering' steps. 
The basic idea is once plate tectonics, macro erosion/deposition and climate models have run / 
reached equilibrium, the terrain can be refined and made more organic by a interpolating and 
letting erosion/deposition run on a smaller scale. Then climate models can do several more iterations
starting from a plausible interpolation of the macro model as well. 

This stage could either continue using square grids, or potentially hexagonal grids or even return
to the non-uniform Delaunay triangulation strategy. 


### Usage

The output of **stage II** should be saved to disk and loadable by the 
[World Building](WorldBuilding.md) module.

This whole module should be a separate runnable CLI process to facilitate fast compile iteration
(If this is JS based this is trivially true). This also means that many maps can be generated and 
only the promising ones then undergo World Building, this is a bit of a cop out so that not every
map has to be good, just some.

