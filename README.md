[![](https://tokei.rs/b1/github/JoeHowarth/Population-Simulation?category=code)](http://github.com/JoeHowarth/Population-Simulation)

-- XAMPPRocky/tokei?category=lines)](https://github.com/XAMPPRocky/tokei).

See
- [Architecture](design_docs/Architecture.md) for technical details
- [Map Generation](design_docs/MapGen_README.md) for Computational Geometry Final Project
- [Population Design Discussion](design_docs/PopulationDesign.md)
- [How to Challenge the Player](design_docs/ChallengingThePlayer.md) for lamenting about traditional AI's limitation


### How to Run

Requires Rust nightly [here](https://www.rust-lang.org/tools/install)

    rustup install nightly
    
    git clone git@github.com:JoeHowarth/Population-Simulation.git
    cd Population-Simulation
    rustup override set nightly
    cargo build
    cargo run
    
In new terminal

    cd vue-client
    npm install
    npm run serve
    
open localhost:8080 in chrome

---
*Below is outdated*

### Dynamic Grand Strategy 

Grand strategy games like Europa Universalis, Crusader Kings, Civilization, Victoria
and even Total War each immerse the player in a simulated world.
Which aspects of reality they focus on while abstracting (or ignoring) the rest,
defines the experience and the game.
Crusader Kings stands out as the only game attempting to model
leaders' intricacy and humanity.
Victoria, through representing and tying populations' plight or prosperity to events
in the world abroad.
Each bring a different perspective, yet all feel somewhat static in how they model
their world.
None besides Victoria even allow for private economy, shifting populations,
founding new cities, changing provincial areas, shifting national ideas etc. etc.

Inspired by the Europa Universalis 4 mod Meiou and Taxes and specifically Demian Sky,
here I will explore creating a truly dynamic historical simulator and the
technical challenges that come with it.

The world should feel *alive*.


Critical Elements
--------

- Population
- Wealth and Trade
- Ideology, Religion and Culture
- Food
- Settlements
- Knowledge and Technology
- Leaders
- Government Structure
- Diplomacy and Politics
- Geography and Climate

Eventually, all these elements should be linked together.
Of course, doing any of these areas justice requires a monumental amount of work,
so it's necessary to start with a smaller scope and focus.

Initial Roadmap
-----------------

Here's roughly how I envision this project progressing

1. Workable proceducal map generator
2. Technical infrastructure MVP
3. Initial settlement placement
4. Population dynamics
5. Trade and economic systems
6. Re-evaluation


Next we'll take a closer look at map generation and then the infrastructure behind the
simulation
