# Todo list
* Move to simpler elements
* use element with id array instead of element struct array

* grid functions (see below)

## Grid functions
Instead of having a bunch of complex elements i think it's more clean
to have more of a data oriented aproach
so mutiple grids with specific data in it

Then apply specific functions while iterating over these grids depending on the id of the element (or other values).

That way things like temprature can still update and cause behaviours
even tho the array of elements is static (since heat transfers and stuff)

It would be intressing to make the grid functions pretaining to elements
check if they should even execute, lets say the grid only contains non flamable elements why bother doing attempting the 'spread_fire' grid function?

or if no acid is in they grid all acid grid functions can be ignored!
Intressing idea's! 

---
## Notes on cleanup and above ideas
Sounds good if all data can be seperated but that's not really how reality works pressure comes from something being compressed so it's not seperate from a element, same as temprature which needs elements to exist.

So since temp, pressure and element are always present in a cell it's not cleaner to seperate them..

The update logic of temprature and pressure can require less resources since they will trend to equilibrium so they require no updates until disturbed.

## Current thoughts on cleanup
(06-01-2022)
Moving the elements to one big grid and seprating the computation of that grid into chunks seems more clean then making chunks that contain elements and compute their next state.

This way we can just chunk out the ''unstable'' parts to threads, with some safe gaurds to prevent collisions.

On the grid functions I do think some updates/interactions can still be ignored but i wonder if those checks will be manageable in the future with a lot of interactions.


