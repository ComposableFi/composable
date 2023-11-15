from decimal import Decimal, getcontext
from objects import Order, OrderList, Solver, Solution, CFMMSolver, CFMMVolumeSolver, CFMMProfitSolver, CFMM

def simulate():

    getcontext().prec = 30

    # CoW part
    orders = OrderList([Order.random(std=0.1, mean=2) for _ in range(100)])

    orders.print()

    solution = Solution.match_orders(orders, Decimal("1"))
    solution.print()

    
    solution_2 = Solution.match_orders(orders, Decimal("2.05"))    
    solution_2.print()
    Solution.match_orders(orders, orders.compute_optimal_price()).print()

    order_book = Solution.random(num_orders=300, volume_range=(100,200), mean=1, std=0.01)

    tokens = int(1e5)
    solver = Solver(orders, Decimal(1.1), tokens, 1000)

    ob: Solution = solver.solve()
    ob.print()

    # solve with CFMM
    cfmm = CFMM(Decimal("1e6"), Decimal("0.95e6"), fee=Decimal("0"))

    volume_solver: Solver = CFMMVolumeSolver(orders, cfmm, tokens, tokens)
    v_ob = volume_solver.solve()
    v_ob.print()


    profit_solver: Solver = CFMMProfitSolver(ob.orders, cfmm, tokens, tokens)
    p_ob = profit_solver.solve()
    ob.print()

    print(f"Volume  of volume solver: {v_ob.match_volume:.2f} and Profit solver: {p_ob.match_volume:.2f}")
    print(f"PROFIT volume_solver: {volume_solver.profit(volume_solver.order):.2f} profit_solver: {profit_solver.profit(profit_solver.order):.2f}")

simulate()