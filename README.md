# Description 

The halt problem asserts that we cannot prove that the program stops or run forever. That's why we do it simultaneously. To do this, we need to find a state that we will go through exactly twice. And a new transition that has not yet happened. At the point that we have passed 2 times there is a halt point, the end point of a new transition is it a start point of the cycle.

So where "halt point"? Halt point every where (depends of watcher), and start of loop after it.

NOTE: I find the halt point and the start point of the loop at the same time, in fact I find two equivalent programs at once, one that stops (firstly possible halt) and the other that doesn’t.

My example is a function of Busy beaver(3). The start point of the lopp is "A". The halt point is "C".
