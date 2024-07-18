# Description 

Default formulation of the problem (3-state busy beaver)
![](./BB3.png "Default implementation 3-state busy beaver")

We modify machine as replacing two ribs "START" and "HALT" for one, as now Turing machine loop forever by construction.

Updated formulation of the problem
![](./BB3U.png "Updated implementation 3-state busy beaver")

In any infinite Turing machine, there is a transition such that, and, moreover, the only one:

1. Transition from start transition point to end point has not yet happened (ORDER MATTER A -> B != B -> A)
2. Start transition point passed EXACTLY TWICE
3. End transition point passed ONLY ONCE.

        if number_of_occurrences[&prev] == 2 
        && number_of_occurrences[&next] == 1
        && transition[&prev].contains(&next) {
            super_position_one = Some(prev);
            super_position_two = Some(next);
            break;
        }

Let's assume that the expected halt point is located at the point at the start of the transition, if the point is not there, the machine will not stop by construction. If the point is there, the Turing machine stops.

In essence, we are considering two properties of two machines that stopped and did not stop, but are equivalent. Therefore, when parsing, we can safely say that the machine will not stop, because if it stopped at this point, then in an equivalent Turing machine the cycle would begin from there