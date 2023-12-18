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

`Editor`

`Growth`

### Example 2:
Creating a GLS goal, evolving a GLS that will achieve it and growing it.

`Editor`

`Evolver`

`Growth`


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