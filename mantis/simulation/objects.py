from __future__ import annotations
from decimal import Decimal, getcontext
import enum
import random
from typing import NewType, Callable, get_type_hints
from uuid import uuid4
import copy
from dataclasses import dataclass
import functools
import time

getcontext().prec = 18
BuyToken = NewType("BuyToken", Decimal)
SellToken = NewType("SellToken", Decimal)
Price = NewType("Price", Decimal)


def timeit(func: Callable):
    @functools.wraps(func)
    def wrapper(*arg, **kwargs):
        t1 = time.time()
        res = func(*arg, **kwargs)
        t2 = time.time()
        # if t2 - t1 > 1e-3:
        # print(f"Time elapsed in {func.__name__} {t2-t1:.6f}s args: {arg} kwargs: {kwargs}")
        return res

    return wrapper


class OrderType(enum.Enum):
    BUY = "Buy"
    SELL = "Sell"

    def __str__(self) -> str:
        return self.value


class OrderStatus(enum.Enum):
    PENDING = "Pending"
    PARTIALLY_FILLED = "Partial"
    FILLED = "Filled"

    def __str__(self) -> str:
        return self.value


class OrderBookStatus(enum.Enum):
    PENDING = "Pending"
    MATCHED = "Matched"

    def __str__(self) -> str:
        return self.value


class Order:
    amount_in: BuyToken | SellToken  # Depending on order type
    _filled_price: Price = Decimal(0)
    type: OrderType
    amount_out: SellToken | BuyToken = Decimal(0)
    amount_filled: BuyToken | SellToken = Decimal(0)

    def __init__(
        self,
        amount_in: BuyToken | SellToken,
        limit_price: Price,
        order_type: OrderType,
        id=None,
    ):
        self.amount_in = Decimal(f"{amount_in:6f}")
        self.limit_price = Decimal(f"{limit_price:6f}")
        self.status: OrderStatus = OrderStatus.PENDING
        self.type: OrderType = order_type
        self.id = uuid4() if id is None else id

    @property
    def filled_price(self):
        if self.type is OrderType.BUY:
            return Decimal(1) / self._filled_price
        return self._filled_price

    @filled_price.setter
    def filled_price(self, value: Price):
        self._filled_price = Decimal(value)

    @property
    def to_be_filled(self):
        if self.status is OrderStatus.PARTIALLY_FILLED:
            return self.amount_in - self.amount_out / self.filled_price
        return Decimal(0)

    def is_acceptable_price(self, price: Price):
        if self.type is OrderType.SELL:
            return price >= self.limit_price
        return price <= self.limit_price

    def token1_at_price(self, price: Price) -> BuyToken:
        if self.type is OrderType.SELL:
            return self.amount_in * price
        return self.amount_in

    def fill(self, volume: BuyToken | SellToken, price: Price) -> None:
        if volume == 0:
            return
        elif volume < 0:
            raise ValueError(f"Negative volume {volume}")
        if volume > self.amount_in:
            raise ValueError(
                f"[{self.type}] Volume trying to fill: {volume} Amount in the order: {self.amount_in} diff: {self.amount_in - volume}"
            )

        self.filled_price = price
        self.amount_out = volume * self.filled_price
        self.amount_filled = volume
        if volume < self.amount_in:
            self.status = OrderStatus.PARTIALLY_FILLED
        else:
            self.status = OrderStatus.FILLED
        self.check_constraints()

    @timeit
    def check_constraints(self):
        if self.status is OrderStatus.FILLED:
            assert self.amount_out == self.amount_in * self.filled_price
        elif self.status is OrderStatus.PARTIALLY_FILLED:
            assert self.amount_out < self.amount_in * self.filled_price
        elif self.status is not OrderStatus.PENDING:
            assert self.amount_out == self.amount_filled * self.filled_price
        elif self.status is OrderStatus.PENDING:
            assert self.amount_out == 0
        else:
            raise ValueError("No expected OrderStatus")

    @classmethod
    def random(
        cls, mean: float = 1.0, std: float = 0.05, volume_range: tuple[int, int] = (50, 150)
    ) -> Order:
        return cls(
            random.uniform(*volume_range),
            random.gauss(mean, std),
            random.choice(list(OrderType)),
        )

    def __str__(self):
        colors = {
            OrderStatus.FILLED: "\033[92m\033[1m",
            OrderStatus.PARTIALLY_FILLED: "\033[93m\033[1m",
            OrderStatus.PENDING: "\033[94m",
        }

        return (
            f"{' * ' if self.id == 'solver' else ''}"
            f"{colors[self.status]}"
            f"[{self.type}]-{self.status}-"
            f"\tLimit Price: {self.limit_price:.2f}  "
            f"\tIn: {self.amount_in:.2f} "
            f"\t\tFilled: {self.amount_filled:.2f} "
            f"\t\tFilled Price: {self._filled_price:.2f} "
            f"\t\tOut: {self.amount_out:.2f}"
            "\033[0m"
        )


@dataclass
class OrderList:
    value: list[Order]

    def __iter__(self):
        return iter(self.value)

    def __getitem__(self, items):
        return self.value.__getitem__(items)

    def __bool__(self):
        return bool(self.value)

    def apply_filter(self, expr: Callable[[Order], bool]):
        return OrderList([order for order in self.value if expr(order)])

    def buy(self):
        return self.apply_filter(lambda x: x.type == OrderType.BUY)

    def sell(self):
        return self.apply_filter(lambda x: x.type == OrderType.SELL)

    def pending(self):
        return self.apply_filter(lambda x: x.status is OrderStatus.PENDING)

    def filled(self):
        return self.apply_filter(lambda x: x.status is not OrderStatus.PENDING)

    def is_acceptable_price(self, price):
        return self.apply_filter(lambda x: x.is_acceptable_price(price))

    def amonut_in(self):
        return sum(order.amount_in for order in self.value)

    def amount_out(self):
        return sum(order.amount_out for order in self.value)

    def amount_filled(self):
        return sum(order.amount_filled for order in self.value)

    def token1_sum(self, price: Price):
        return sum(order.token1_at_price(price) for order in self.value)

    def id(self, id):
        return self.apply_filter(lambda x: x.id == id)

    def all(self):
        return self.value

    def clone(self):
        return copy.deepcopy(self)

    def __hash__(self) -> int:
        return hash(tuple(order.id for order in self))
    
    @functools.cache
    def compute_optimal_price(self, num_range=50) -> Price:
        """Computes the optimal price that will maximize the transacted volume in batch auction"""
        optimal_price = -1
        max_volume = -1
        min_price = min(self.value, key=lambda x: x.limit_price).limit_price
        max_price = max(self.value, key=lambda x: x.limit_price).limit_price
        for i in range(num_range+1):
            price = min_price + i * (max_price - min_price) / num_range
            volume = self.volume_by_price(price)
            if volume > max_volume:
                optimal_price = price
                max_volume = volume
        return optimal_price

    def volume_by_price(self, price: Price) -> BuyToken:
        matched = self.is_acceptable_price(price)
        return min(matched.buy().token1_sum(price), matched.sell().token1_sum(price))

    def resolve_predominant(
        self, predominant_orders: OrderList, other_orders: OrderList, price: Price
    ):
        filled = Decimal(0)
        for order in other_orders:
            order.fill(order.amount_in, price)
        other_volume = other_orders.amount_out()
        for order in predominant_orders:
            if filled + order.amount_in > other_volume:
                order.fill(other_volume - filled, price)
                break
            order.fill(order.amount_in, price)
            filled += order.amount_in

    def print(self):
        for order in self.buy():
            print(order)
        print("-" * 10)
        for order in self.sell():
            print(order)
        print("-" * 10)


class Solution:
    orders: OrderList
    matched_price: Price = Decimal("0")
    buy_volume: BuyToken = Decimal("0")
    sell_volume: SellToken = Decimal("0")

    def __init__(self, orders: list[Order]) -> None:
        self.orders = OrderList(sorted(orders, key=lambda x: x.limit_price))
        if self.orders:
            self.matched_price = self.orders[0]._filled_price
        self.buy_volume = self.orders.sell().amount_out()
        self.sell_volume = self.orders.buy().amount_out()

    @property
    def sell_orders(self) -> OrderList:
        return self.orders.sell()

    @property
    def buy_orders(self) -> OrderList:
        return self.orders.buy()
    
    @property
    def match_volume(self) -> Decimal:
        return self.buy_volume * self.sell_volume

    def check_constraints(self):
        if not (
            abs(self.buy_volume - self.orders.buy().amount_filled()) < 1e-20
            or abs(self.sell_volume - self.orders.sell().amount_filled()) < 1e-20
        ):
            raise ValueError(
                f"Error buy_volume: {self.buy_volume} Buy amount filled: {self.orders.buy().amount_filled()}"
                f" sell_volume: {self.sell_volume} Buy amount filled: {self.orders.sell().amount_filled()}"
            )

    def clone(self) -> Solution:
        return copy.deepcopy(self)

    @classmethod
    def match_orders(cls, orders : OrderList, price: Price) -> Solution:
        orders =  orders.clone()
        orders.value.sort(key=lambda x: x.limit_price)

        matched = orders.is_acceptable_price(price)
        buy_orders = matched.buy()
        sell_orders = matched.sell()

        buy_volume = buy_orders.token1_sum(price)
        sell_volume = sell_orders.token1_sum(price)

        is_buy_predominant = buy_volume > sell_volume

        if is_buy_predominant:
            orders.resolve_predominant(buy_orders, sell_orders, price)
        else:
            orders.resolve_predominant(sell_orders, buy_orders, price)

        solution = Solution(matched.filled().value)

        solution.check_constraints()
        
        return solution

    @classmethod
    def random(cls, num_orders=100, *args, **kwargs):
        return cls([Order.random(*args, **kwargs) for _ in range(num_orders)])

    def print(self):
        print("#"* 20 + " Start Solution " + "#"*20)
        self.orders.print()

        print(
            f"\033[1mMatched Price {self.matched_price:.4f} \tSell volume {self.sell_volume:.4f}\tBuy volume {self.buy_volume:.4f}\033[0m"
        )
        print("#"* 20 + " End Solution " + "#"*20)
        print()


@dataclass
class Solver:
    orders: OrderList
    target_price: Price
    buy_token: BuyToken
    sell_token: SellToken
    order: Order = Order(0, 0, OrderType.BUY, "fake-order")

    @property
    def limit_price(self) -> Price:
        return self.target_price

    @timeit
    def f_maximaize(self, order: Order):
        if order.type is OrderType.BUY:
            return (
                self.buy_token
                - order.amount_filled
                + (self.sell_token + order.amount_out) * self.target_price
            )
        return (
            (self.buy_token + order.amount_out) / self.target_price
            + self.sell_token
            - order.amount_filled
        )

    @timeit
    def solve(self, num_orders=1000) -> Solution:
        original_price = self.orders.compute_optimal_price()
        is_buy = original_price > self.target_price
        original_token_amount = self.buy_token if is_buy else self.sell_token
        orders = [
            self.order_for(i * original_token_amount / num_orders, is_buy)
            for i in range(0, num_orders + 1)
        ]
        max_value = 0
        max_solution: Solution = None
        for order in orders:
            solution = self.match_ob_with_order(order)
            introduced_orders = solution.orders.id(order.id)
            if introduced_orders:
                f_value = self.f_maximaize(introduced_orders[0])
                if max_value < f_value:
                    max_value = f_value
                    max_solution = solution
                    self.order = introduced_orders[0]
        if max_solution:
            return max_solution
        return Solution.match_orders(self.orders, original_price)

    def match_ob_with_order(self, order: Order) -> Solution:
        orderbook = self.orders.clone()
        orderbook.value.append(order)
        orderbook.value.sort(key=lambda x: x.limit_price)
        return Solution.match_orders(orderbook, orderbook.compute_optimal_price())

    def order_for(self, amount, is_buy: bool) -> Order:
        if is_buy:
            order = Order(amount, self.limit_price, OrderType.BUY, id="solver")
        else:
            order = Order(amount, self.limit_price, OrderType.SELL, id="solver")
        return order


class CFMMSolver(Solver):
    cfmm: CFMM

    def __init__(self, ob: Solution, cfmm: CFMM, buy_token: BuyToken, sell_token: SellToken):
        self.cfmm = cfmm
        self.orders = ob
        self.buy_token = buy_token
        self.sell_token = sell_token
        self._optimal_price = self.orders.compute_optimal_price()

    @property
    def target_price(self) -> Price:
        return 1 / self.cfmm.price

    @property
    def limit_price(self) -> Price:
        return (
            Decimal(self._optimal_price) * Decimal("1.1")
            if self._optimal_price < self.cfmm.price
            else Decimal(self._optimal_price) / Decimal("1.1")
        )

    def profit(self, order: Order):
        obtained = order.amount_out
        if order.type is OrderType.BUY:
            result = self.cfmm.sell(obtained, simulate=True)
        else:
            result = self.cfmm.buy(obtained, simulate=True)
        return result - order.amount_filled


class CFMMProfitSolver(CFMMSolver):
    def f_maximaize(self, order: Order):
        return self.profit(order)


class CFMMVolumeSolver(CFMMSolver):
    def f_maximaize(self, order: Order):
        if self.profit(order) < 0:
            return -1 * Decimal("inf")
        return self.match_ob_with_order(order).match_volume()


class Mechanism:
    orderbooks: list[Solution] = []

    def submit_orderbook(self, orderbook):
        assert orderbook
        self.orderbooks.append(orderbook)


@dataclass
class CFMM:
    R0: BuyToken
    R1: SellToken
    chain_id: int = 1
    fee: Decimal = Decimal("0.03")

    @property
    def gamma(self) -> Decimal:
        return 1 - self.fee

    @gamma.setter
    def gamma(self, value):
        self.fee = 1 - value

    def sell(self, Delta, simulate=True) -> BuyToken:
        amount_out = self.swap(Delta, self.R1, self.R0)
        if not simulate:
            self.R0 -= amount_out
            self.R1 += Delta
        return amount_out

    def buy(self, Delta, simulate=True) -> SellToken:
        amount_out = self.swap(Delta, self.R0, self.R1)
        if not simulate:
            self.R1 -= amount_out
            self.R0 += Delta
        return amount_out

    def swap(self, Delta: BuyToken | SellToken, in_reserve, out_reserve):
        return out_reserve - in_reserve * out_reserve / (in_reserve + self.gamma * Delta)

    @property
    def price(self):
        return self.R0 / self.R1

    @classmethod
    def random(cls, R0=(500, 1500), R1=(500, 1500)):
        return cls(Decimal(random.uniform(*R0)), Decimal(random.uniform(*R1)))
