# Special thanks my friends
Vyacheslav Goma, Hirrolot, Dmitry Ylin

# Description 

NOTE: yep points in superposition in meaning that's any of each can be halt or loop start

The halt problem asserts that we cannot prove that the program stops or run forever. That's why we do it simultaneously. To do this, we need to find a state that we will go through exactly twice. And a new transition that has not yet happened (from a point where we have been exactly twice to a point where we have been only once). At the point that we have passed 2 times there is a halt point, the end point of a new transition is it a start point of the cycle.

So where "halt point"? Halt point every where (depends of watcher), and start of loop after it.

NOTE: I find the halt point and the start point of the loop at the same time, in fact I find two equivalent programs at once, one that halt and the other run infinity.

My example is a function of Busy beaver(3). The start point of the loop is "A". The halt point is "C".

NOTE: So what if we found the minimum halt point/start loop point? My hypothesis is that there is a method to come up with to see them all in order by time. Provided that we know at least one most upperest halt point or run loop point (for usual programm it is just main function's that's we expect halt), then we resolve all points in the superpositions.