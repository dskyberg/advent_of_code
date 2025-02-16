#  Day 20
Part 1 is straightforward, but does not scale for part 2.

This clue helped me solve part 2:
> Yeah i remember struggling with how to write it in a way that's understandable.
> The basic idea is I have the normal path from start to finish without cheating stored
> chronologically in the variable path, where each entry is an (x, y) coordinate pair
> and the index of each entry is the time t at which we reach that coordinate without cheating.
> Now the goal is to find 2 pairs of (x, y) coordinates in path that are far away in terms of
> time t2 - t1 but close-by in terms of location abs(x1-x2) + abs(y1-y2), because if they are
> close enough location wise we can jump there and if they are far enough in time we can save
> a lot by cheating in this way.

The solution for part 2 will also work for part 1,but runs an order of mag slower.
