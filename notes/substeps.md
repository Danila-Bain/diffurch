I want to implement a thing that allows the event to be called not only at each step, but multiple times per step.

This, first of all, would allow easier debug for interpolation (by providing the data form that interpolation). 

Also, it may be that for higher order methods, it is cheaper to go with large steps, and get intermediate points from interpolation, rather than do several smaller steps to get the same density of points. 
Also, dense output iw useful for parametric plots, where deviations due to linear interpolation become wery noticable, despite relatively small steps.



So, how?

In the end, I want to be able call the same (f64, [f64; N]) function, but with different arguments. 

One idea is to modify the to_state_function prosedure. To do that, we need to mark the event somehow, such that it differs from regular functions. This is difficult, becausewe need to make a lot of changes, due to the fact that now the to_state_function implementation differentiates functions by their arguments and return types. The only way is to make a marker which is orthogonal to argument types and return value type. 

On the other hand, Event should be modified to implement such a marker, adding ever more to complexity.


This looks like the most appropriate place to make a change, because it is the place that determines how function evaluated from state.
