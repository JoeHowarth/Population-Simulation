### Population Design Document
How people are represented determines the flexibility, level of detail and 
performance of the above societal model. A vast number of attributes apply to people
including: 
* health, age, sex
* occupation, skills
* culture, religion, race
* ideas/mindsets/attitudes, (imperfect) information 
* relationships, membership of groups
* location

For influential people (ie. [actors](Actors.md)), all these factors can be modelled on
an individual basis, however this approach clearly can't work for the nameless masses. 
Therefore an important design decision is how to balance the trade-offs between detail,
implementation flexibility + complexity and runtime performance. 

#### Representation Problem 
As more factors are simulated, the representation of population needs some way of 
determining who has what factors. The most straightforward solution is to have small 
*units* of population that all share characteristics. So in a given city there might be 
a pop of Welsh, Catholic, apprentice, 20-25 year old, shipbuilders of strong health
who live in London. By having homogeneity within each unit there is no information loss
and it becomes trivial to model some effect on that group: when some % change an attribute 
they're transferred to the newly relevant unit. they all share common susceptibility and 
modifiers so there's no statistical issues that arise from non-uniformly distributed effects.
There's major problem with this however: you need a combinatorial number of *units* 
to describe all the different combinations of factor values. This becomes untenable computationally
very quickly. See bottom for estimate

The other extreme is modelling the percentage of each factor across all groups in a given location.
More concretely, London's culture might be 70% English, 10$ Scottish, 5% Welsh, 5% Cornish, 
5% Irish, 3% French etc. and its religions might be 90% Anglican, 5% Catholic, 5% Reformed. This 
would imply that the number of Catholic and French are 3% * 5% = 0.15%, but in reality almost all
French are also Catholic, so it's actually 3% * 90% = 2.7%. This system loses too much detail about
correlated information and is therefore not acceptable.

I bet you saw this coming, but the logical conclusion is to do a mix of these approaches! First 
divide the population by the most impactful factors, then use distributions on those subdivisions
to represent less impactful factors without balooning the total number of *units*, thus keeping 
computation and memory low and detail reasonably high. But what should the *divisional* and 
*distributional* factors be? Here's a first attempt

**Divisional**
- Location - Regional for rural, Regional for town-urban, City for major cities
- Nation / Culture Group
- Occupation Type (craftsmen not nail maker)
- Religion
- Large age cohorts (maybe 10 years?)

**Distributional**
- Age, Sex, Health
- Sub-location - tile for rural, town for town-urban, (district for major cities?)
- Skills
- Race (?)
- Sub-Culture
- Religious sect?
- ideas/mindsets/attitudes
- relationships (how closely people are tied to other populations)

Currently the population system only really models age and location, using "Cohorts" that divide 
each region's population groups of ~5 years and tracks that group over time. This allows for 
easy and accurate population growth and death calculations and also allows other modifiers to 
affect certain age ranges differently. For example, the young and old are more susceptible to 
health shocks. If the populations are additionally being divided by location, culture group etc. it 
might make sense to only divide ages every 10 years (or even larger). 

#### Estimates


**Homogeneous Population Units**  

50 tiles in a country, 5 cultures present, 2 religions, 20 occupations, 
3 skill levels, 12 5 year age ranges, 3 health levels. A very modest number of factors and 
values per factor, yet we need 50 * 5 * 2 * 20 * 3 * 12 * 3 = 1,080,000 *units* of population. 
Yikes!! We can also easily find out the lower bound on memory consumption. First find the number of
bytes needed to represent the information with is 
log(50) + log(5) + ... = log(50 * 5 *...) = log(1,080,000) = 20 bits => 4 byes (> 2 so for alignment 
it becomes 4). However its not quite so efficient since most structs won't be bitpacked neatly, instead
lets give each factor its own byte, so 7 bytes => 8 bytes. Then there are 1,080,000 * 8 ~= 8 Mb for a 
single country. If there are 100 countries (probably many more...) then that's 800 Mb and 100 million 
*unit* structs to process every time the simulation wants to update any of those values. Let's say 
the number of people and their cumulative wealth are also stored, adding 2 byes each so 8 + 2*2 = 12 => 
16 bytes so now 1,600 Mb and 200 million *units*. Let's say the simulation would like to update each
factor once every month and each update takes 100 cpu cycles with no cache misses. Then that's a minimum
of 7 * 100 * 2e8 = 140 billion cycles ~= 45 seconds / month (now this is a very conservative estimate b/c 
cache misses can easily eat 100s or 1000s of cycles each) and that is JUST for those 7 factors. 
Let's say you wanted to add another factor with 10 values, then multiply that time by 10 => 450 seconds.
Clearly this scales terribly

**Mixed** 

50 tiles in a country, 5 cultures present, 2 religions, 20 occupations, 
3 skill levels, 6 10 year age ranges, 3 health levels. 

Group tiles into regions of 5 each => 10 regions, group occupations into 8 groups.
So 10 * 5 * 6 * 8 * 2  = 4,800 *units*

A huge improvement over 1,080,000, 225x fewer!! Everything else equal that would take about .2 seconds
per month, maybe slightly more since each *unit* is more complicated due to distributional factors.


