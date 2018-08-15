# Playing around with Risk probabilities in Rust
This repo was made to answer some questions about probabilities in the board game Risk and to learn about the Rust programming language.

The number of defenders is chosen based on the attacker throw to maximise the expected value of `loss attacker - loss defender`.

## 3 attackers, up to 2 defenders
```
Outcome (attacker loss, defender loss) probabilities:
{
    (-1, 0): 0.10185185185185208,
    (0, -1): 0.3518518518518517,
    (-2, 0): 0.23829732510288054,
    (0, -2): 0.11291152263374479,
    (-1, -1): 0.19508744855967092}
Expected loss Attacker: -0.773533950617284, Defender: -0.7727623456790123,
Expected diff: -0.0007716049382717749, Variance diff: 1.053197576112635
Optimal strategy (attacker throw, number of defender dice):
[
([1, 1, 1], 2), ([1, 1, 2], 2), ([1, 1, 3], 2), ([1, 1, 4], 2),
([1, 1, 5], 2), ([1, 1, 6], 2), ([1, 2, 2], 2), ([1, 2, 3], 2),
([1, 2, 4], 2), ([1, 2, 5], 2), ([1, 2, 6], 2), ([1, 3, 3], 2),
([1, 3, 4], 2), ([1, 3, 5], 2), ([1, 3, 6], 2), ([1, 4, 4], 2),
([1, 4, 5], 1), ([1, 4, 6], 1), ([1, 5, 5], 1), ([1, 5, 6], 1),
([1, 6, 6], 1), ([2, 2, 2], 2), ([2, 2, 3], 2), ([2, 2, 4], 2),
([2, 2, 5], 2), ([2, 2, 6], 2), ([2, 3, 3], 2), ([2, 3, 4], 2),
([2, 3, 5], 2), ([2, 3, 6], 2), ([2, 4, 4], 2), ([2, 4, 5], 1),
([2, 4, 6], 1), ([2, 5, 5], 1), ([2, 5, 6], 1), ([2, 6, 6], 1),
([3, 3, 3], 2), ([3, 3, 4], 2), ([3, 3, 5], 2), ([3, 3, 6], 2),
([3, 4, 4], 2), ([3, 4, 5], 1), ([3, 4, 6], 1), ([3, 5, 5], 1),
([3, 5, 6], 1), ([3, 6, 6], 1), ([4, 4, 4], 2), ([4, 4, 5], 1),
([4, 4, 6], 1), ([4, 5, 5], 1), ([4, 5, 6], 1), ([4, 6, 6], 1),
([5, 5, 5], 1), ([5, 5, 6], 1), ([5, 6, 6], 1), ([6, 6, 6], 1)
]
```
We see that in expectation a minimal advantage (0.0008 units per turn) exists for the defender, but the variance is high (1.0532 units^2 per turn).

The optimal strategy for the defender is to use 1 die when the two highest dice of the attacker are (5, 4).

## 2 attackers, up to 2 defenders
```
Outcome (attacker loss, defender loss) probabilities:
{
    (-1, -1): 0.25462962962962954,
    (-1, 0): 0.05092592592592593,
    (-2, 0): 0.4197530864197531,
    (0, -2): 0.10339506172839506,
    (0, -1): 0.17129629629629628
}
Expected loss Attacker: -1.1450617283950617, Defender: -0.632716049382716,
Expected diff: -0.5123456790123457, Variance diff: 1.1125781130925165
Optimal strategy (attacker throw, number of defender dice):
[
([1, 1], 2), ([1, 2], 2), ([1, 3], 2), ([1, 4], 2),
([1, 5], 2), ([1, 6], 2), ([2, 2], 2), ([2, 3], 2),
([2, 4], 2), ([2, 5], 2), ([2, 6], 2), ([3, 3], 2),
([3, 4], 2), ([3, 5], 2), ([3, 6], 2), ([4, 4], 2),
([4, 5], 1), ([4, 6], 1), ([5, 5], 1), ([5, 6], 1),
([6, 6], 1)]
```
We see that now a significant expected advantage for the defender exists of 0.5123 units per turn. The optimal defender strategy is the same as in the above scenario.

## Different strategies
If instead, the defender wants to maximise the expected value of `r * loss attacker - loss defender`, different result emerge. For example, if `r=0.1`, the defender is very defensive and cares more about not losing units versus destroying attacking units.

Then, the optimal strategy is to throw one die when:
- the highest two dice of the attacker are 6 and 2 or
- the highest two dice of the attacker are 2 and 2 or
- the second highest attacker die is 3

Interestingly, this shows that for `r != 1`, it is not the case that "if for attacker throw X, for which 1 defender die is optimal, and throw Y is stronger than X, then 1 is also optimal for attacker throw Y."

For 3 attackers, up to 2 defenders, we get:
```
Expected loss Attacker: -0.5435956790123458, Defender: -0.6554783950617283,
Expected diff: 0.11188271604938249, Variance diff: 0.7555616879096176
```

For 2 attackers, up to 2 defenders, we get:
```
Expected loss Attacker: -0.935185185185185, Defender: -0.537037037037037,
Expected diff: -0.39814814814814803, Variance diff: 0.9851680384087791
```
We see that the expected advantage of the defender disappears or decreases, but that the defender is expected to lose fewer units.
