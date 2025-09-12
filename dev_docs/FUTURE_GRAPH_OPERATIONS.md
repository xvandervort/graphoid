# Future Graph Operations

In order to be a true graph language, glang needs to be able to build and
manipulate graphs. In order to get there we need to add certain functions:

* Add edges. Not just nodes in a list. A node should be able to be connected
   to multiple nodes.
  * Edges must also have metadata.
* Once we have true graphs, we need to be able to do things with them
  * Breadth first search (possibly guided by node meta data)
  * Depth first search (same about meta data)
* Node gathering. That is, find nodes of certain characteristics and extract
  them, maintaining whatever edges are possible between them (not the ones left
behind)
* Cycle detection?
* Graphs with rules
  * For example, a graph might be constrained to only certain numbers of nodes
  * Or to a certain number of edges each (nice for trees)
  * Or only a certain type of nodes
  * Or only a certain detph, number of nodes, or whatever.
  * This is pretty advanced but necessary for building constrained structures.
  * There might be a user defined rule that converts a data type into another
    for storage in the graph (with the original value being metadata). For
    example this could convert tokens to numbers.

## Advanced operations

* Graph union
* graph intersection
* graph difference

## more advanced

* cartesian product
* strong product
* Direct product
* lexicographic product

## Use cases

We should concentrate on structures and techniques for specific use cases, such
as network analysis, heap building, binary trees, etc. (Add fleshed out use
cases here)

Then as we move forward we can have tutorials. 

## Path search.

How to get from point A to point b within certain parameters (number of hops,
or only counting specific node types, or efficiency of search, assuming edges
also can have meta data)


