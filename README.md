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

Let's "ASSUME"(because  machine from we built it's machine excpect it here) that the expected halt point is located at the point at the start of the transition, if the point is not there, the machine will not stop by construction.
In essence, we are considering two properties of two machines that stopped and did not stop, but are equivalent.
First halt point uniquenes
Second uniqueness of the closing transition
Since our machines are “identical,” we assume that these properties coincide in one place.
