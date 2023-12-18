# Project not yet named
Evolution of softbody creatures.

Combining ideas from different ALife implementations.

This evolution simulator `will` include the most interesting features out of many other ALife and artificial evolution simulators that were never before mixed:
* Softbody creatures
* Competition over resources
* Breeding and mutations, emergent evolution
* Emergent multicelular bodyplans
* Neural networks
* Creature growth over its lifespan

The project is fairly large, so I decided to make some related/testing projects during the development using modules written for the big project.

# Intermediate Project 1 - GLS Evolver
Edit, evolve and watch **G**rid **L**-**S**ystems grow.

Grid LSystem is very simmilar to a normal LSystem, but instead of existing in a smooth domain, it is defined on a grid.

### Example 1: 
Creating a GLS and growing it.

https://github.com/gre-v-el/Grid-L-Systems/assets/95856287/aa2d87e3-9df6-453d-87a9-b8c495899208

https://github.com/gre-v-el/Grid-L-Systems/assets/95856287/3095dad7-7d22-470b-b8db-07e691c0b68b

### Example 2:
Creating a GLS goal, evolving a GLS that will achieve it and growing it.

https://github.com/gre-v-el/Grid-L-Systems/assets/95856287/b6e29b31-dbde-4f3c-8b09-847d0042da6b

https://github.com/gre-v-el/Grid-L-Systems/assets/95856287/62497664-bdd7-46be-ac37-8ec9a73f073e

https://github.com/gre-v-el/Grid-L-Systems/assets/95856287/62e602ca-ad6b-423b-83b8-08948e089d8a

## Roadmap
* [ ] `Project name`
  * [ ] Softbody physics simulator
  * [ ] Creature genome
    * [x] Creature body description - Grid LSystems
	* [ ] Creature control description - (No idea yet)
  * [ ] Creature control - Neural Networks

## Intermediate Projects
* [x] Grid LSystems editor and evolution simulator
  * Grid LSystems implementation
  * Genetic Algorithm implementation
  * Rich editor and viewer
* [ ] Soft creatures controlled evolution
  * Genetic Algorithm implementation
  * Softbody physics
  * Body control
* [ ] Full uncontrolled evolution simulation
  * Environment simulation
  * Creature interaction
