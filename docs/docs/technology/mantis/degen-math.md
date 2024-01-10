# Why solvers will not rug you?

## Let us have 4 orders

1. 100pica=10atom
2. 100pica=10atom
3. 20atom=100pica
4. 20atom=100pica

Read it 100 pica for minumum for 10 atom and other direction.

### "Fair" solution is

1. 20atom
2. 20atom
3. 100pica
4. 100pica

So read it `order number 1 got 20atom as he wanted`

Let look different cases of "unfair".


### Limits

```
1. 9atom
...
```

Solution giving first order 9 ATOM will fail, as it gives than limit wanted.

So nobody will get less than limited, liked in FIFO Order Book.

### Not maximal volume

1. 10atom
2. 10atom
3. 100pica
4. 100pica

So as you can see solver favored 3 and 4 solutions, but underfilled 1 and 2.

Solution will be rejected. Why?

Volume of `fair` solution is `40*200`. 
Volume of this solution is `20*200`. 

Solution with larger volume accepted.

### Not fair price

1. 10atom
2. 30atom
3. 100pica
4. 100pica

In this case volume is good, but settling price for order 2 was better than order 3.

This solution will be rejected, because all orders will be compare to same single accepted price.
1 and 2 violate one accepted price rule, so solution will be rejected.

**CoWs ensure that users get fair prices against each other**

### Price is too far from oracle

Also we do not use Oracle to define optimal price,
but in case two solution are same in volume and fair price,
solution which is closer to oracle is picked.

### Cross chain execution

After users got fair price against each other in CoW, remaining part of batch can be solved cross chain.

Cross chain execution uses MANTIS commitment protocol to ensure safe execution.

### Why batch CoW first and only equilibrium after?

#### Market mechanics in equilibrium

Like with FIFO Order Book like we all know, when Batch auctions has many solvers and many orders, limit difference narrows down and more solvers compete for being accepted, so larger volume. 

That leads to optimal spot price.

We assume that fair price in batch auction eventually will be optimal, as safes gas costs of cross chain execution and generally more safe against MEV.

So instead of these orders:
```
1. 100pica=10atom
2. 100pica=10atom
3. 20atom=100pica
4. 20atom=100pica
```

we get these orders:
```
1. 100pica=17atom
2. 101pica=18atom
3. 20atom=99pica
4. 21atom=100pica 
```
