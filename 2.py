from typing import Dict, List
from datamodel import OrderDepth, TradingState, Order
import collections
from collections import defaultdict
import random
import math
import copy
import numpy as np

empty_dict = {'AMETHYSTS' : 0, 'STARFRUIT' : 0,'ORCHIDS' : 0}


def def_value():
    return copy.deepcopy(empty_dict)

INF = int(1e9)

class Trader:

    position = copy.deepcopy(empty_dict)
    POSITION_LIMIT = {'AMETHYSTS' : 20, 'STARFRUIT' : 20,'ORCHIDS' : 100}
    volume_traded = copy.deepcopy(empty_dict)

    person_position = defaultdict(def_value)
    person_actvalof_position = defaultdict(def_value)

    cpnl = defaultdict(lambda : 0)
    STARFRUIT_cache = []
    ORCHIDS_cache = []
    STARFRUIT_dim = 4
    ORCHIDS_dim = 4
    steps = 0
    

   

    def calc_next_price_STARFRUIT(self):
        # STARFRUIT cache stores price from 1 day ago, current day resp
        # by price, here we mean mid price

        coef = [-0.01869561,  0.0455032 ,  0.16316049,  0.8090892]
        intercept = 4.481696494462085
        nxt_price = intercept
        for i, val in enumerate(self.STARFRUIT_cache):
            nxt_price += val * coef[i]

        return int(round(nxt_price))
    

    def calc_next_price_ORCHIDS(self):
        # STARFRUIT cache stores price from 1 day ago, current day resp
        # by price, here we mean mid price

        coef = [-0.01869561,  0.0455032 ,  0.16316049,  0.8090892]
        intercept = 4.481696494462085
        nxt_price = intercept
        for i, val in enumerate(self.ORCHIDS_cache):
            nxt_price += val * coef[i]

        return int(round(nxt_price))
    
    def values_extract(self, order_dict, buy=0):
        tot_vol = 0
        best_val = -1
        mxvol = -1

        for ask, vol in order_dict.items():
            if(buy==0):
                vol *= -1
            tot_vol += vol
            if tot_vol > mxvol:
                mxvol = vol
                best_val = ask
        
        return tot_vol, best_val
    


    def compute_orders_AMETHYSTS(self, product, order_depth, acc_bid, acc_ask):
        orders: list[Order] = []

        osell = collections.OrderedDict(sorted(order_depth.sell_orders.items()))
        obuy = collections.OrderedDict(sorted(order_depth.buy_orders.items(), reverse=True))

        sell_vol, best_sell_pr = self.values_extract(osell)
        buy_vol, best_buy_pr = self.values_extract(obuy, 1)

        cpos = self.position[product]

        mx_with_buy = -1

        for ask, vol in osell.items():
            if ((ask < acc_bid) or ((self.position[product]<0) and (ask == acc_bid))) and cpos < self.POSITION_LIMIT['AMETHYSTS']:
                mx_with_buy = max(mx_with_buy, ask)
                order_for = min(-vol, self.POSITION_LIMIT['AMETHYSTS'] - cpos)
                cpos += order_for
                assert(order_for >= 0)
                orders.append(Order(product, ask, order_for))

        mprice_actual = (best_sell_pr + best_buy_pr)/2
        mprice_ours = (acc_bid+acc_ask)/2

        undercut_buy = best_buy_pr + 1
        undercut_sell = best_sell_pr - 1

        bid_pr = min(undercut_buy, acc_bid-1) # we will shift this by 1 to beat this price
        sell_pr = max(undercut_sell, acc_ask+1)

        if (cpos < self.POSITION_LIMIT['AMETHYSTS']) and (self.position[product] < 0):
            num = min(40, self.POSITION_LIMIT['AMETHYSTS'] - cpos)
            orders.append(Order(product, min(undercut_buy + 1, acc_bid-1), num))
            cpos += num

        if (cpos < self.POSITION_LIMIT['AMETHYSTS']) and (self.position[product] > 15):
            num = min(40, self.POSITION_LIMIT['AMETHYSTS'] - cpos)
            orders.append(Order(product, min(undercut_buy - 1, acc_bid-1), num))
            cpos += num

        if cpos < self.POSITION_LIMIT['AMETHYSTS']:
            num = min(40, self.POSITION_LIMIT['AMETHYSTS'] - cpos)
            orders.append(Order(product, bid_pr, num))
            cpos += num
        
        cpos = self.position[product]

        for bid, vol in obuy.items():
            if ((bid > acc_ask) or ((self.position[product]>0) and (bid == acc_ask))) and cpos > -self.POSITION_LIMIT['AMETHYSTS']:
                order_for = max(-vol, -self.POSITION_LIMIT['AMETHYSTS']-cpos)
                # order_for is a negative number denoting how much we will sell
                cpos += order_for
                assert(order_for <= 0)
                orders.append(Order(product, bid, order_for))

        if (cpos > -self.POSITION_LIMIT['AMETHYSTS']) and (self.position[product] > 0):
            num = max(-40, -self.POSITION_LIMIT['AMETHYSTS']-cpos)
            orders.append(Order(product, max(undercut_sell-1, acc_ask+1), num))
            cpos += num

        if (cpos > -self.POSITION_LIMIT['AMETHYSTS']) and (self.position[product] < -15):
            num = max(-40, -self.POSITION_LIMIT['AMETHYSTS']-cpos)
            orders.append(Order(product, max(undercut_sell+1, acc_ask+1), num))
            cpos += num

        if cpos > -self.POSITION_LIMIT['AMETHYSTS']:
            num = max(-40, -self.POSITION_LIMIT['AMETHYSTS']-cpos)
            orders.append(Order(product, sell_pr, num))
            cpos += num

        return orders


    def compute_orders_ORCHIDS(self, product, order_depth, acc_bid, acc_ask, LIMIT):
        orders: list[Order] = []

        osell = collections.OrderedDict(sorted(order_depth.sell_orders.items()))
        obuy = collections.OrderedDict(sorted(order_depth.buy_orders.items(), reverse=True))

        sell_vol, best_sell_pr = self.values_extract(osell)
        buy_vol, best_buy_pr = self.values_extract(obuy, 1)

        cpos = self.position[product]

        mx_with_buy = -1

        for ask, vol in osell.items():
            if ((ask < acc_bid) or ((self.position[product]<0) and (ask == acc_bid))) and cpos < self.POSITION_LIMIT['ORCHIDS']:
                mx_with_buy = max(mx_with_buy, ask)
                order_for = min(-vol, self.POSITION_LIMIT['ORCHIDS'] - cpos)
                cpos += order_for
                assert(order_for >= 0)
                orders.append(Order(product, ask, order_for))

        mprice_actual = (best_sell_pr + best_buy_pr)/2
        mprice_ours = (acc_bid+acc_ask)/2

        undercut_buy = best_buy_pr + 1
        undercut_sell = best_sell_pr - 1

        bid_pr = min(undercut_buy, acc_bid-1) # we will shift this by 1 to beat this price
        sell_pr = max(undercut_sell, acc_ask+1)

        if (cpos < self.POSITION_LIMIT['ORCHIDS']) and (self.position[product] < 0):
            num = min(200, self.POSITION_LIMIT['ORCHIDS'] - cpos)
            orders.append(Order(product, min(undercut_buy + 1, acc_bid-1), num))
            cpos += num

        if (cpos < self.POSITION_LIMIT['ORCHIDS']) and (self.position[product] > 80):
            num = min(200, self.POSITION_LIMIT['ORCHIDS'] - cpos)
            orders.append(Order(product, min(undercut_buy - 1, acc_bid-1), num))
            cpos += num

        if cpos < self.POSITION_LIMIT['ORCHIDS']:
            num = min(200, self.POSITION_LIMIT['ORCHIDS'] - cpos)
            orders.append(Order(product, bid_pr, num))
            cpos += num
        
        cpos = self.position[product]

        for bid, vol in obuy.items():
            if ((bid > acc_ask) or ((self.position[product]>0) and (bid == acc_ask))) and cpos > -self.POSITION_LIMIT['ORCHIDS']:
                order_for = max(-vol, -self.POSITION_LIMIT['ORCHIDS']-cpos)
                # order_for is a negative number denoting how much we will sell
                cpos += order_for
                assert(order_for <= 0)
                orders.append(Order(product, bid, order_for))

        if (cpos > -self.POSITION_LIMIT['ORCHIDS']) and (self.position[product] > 0):
            num = max(-200, -self.POSITION_LIMIT['ORCHIDS']-cpos)
            orders.append(Order(product, max(undercut_sell-1, acc_ask+1), num))
            cpos += num

        if (cpos > -self.POSITION_LIMIT['ORCHIDS']) and (self.position[product] < -80):
            num = max(-200, -self.POSITION_LIMIT['ORCHIDS']-cpos)
            orders.append(Order(product, max(undercut_sell+1, acc_ask+1), num))
            cpos += num

        if cpos > -self.POSITION_LIMIT['ORCHIDS']:
            num = max(-200, -self.POSITION_LIMIT['ORCHIDS']-cpos)
            orders.append(Order(product, sell_pr, num))
            cpos += num

        return orders






    def compute_orders_regression(self, product, order_depth, acc_bid, acc_ask, LIMIT):
        orders: list[Order] = []

        osell = collections.OrderedDict(sorted(order_depth.sell_orders.items()))
        obuy = collections.OrderedDict(sorted(order_depth.buy_orders.items(), reverse=True))

        sell_vol, best_sell_pr = self.values_extract(osell)
        buy_vol, best_buy_pr = self.values_extract(obuy, 1)

        cpos = self.position[product]

        for ask, vol in osell.items():
            if ((ask <= acc_bid) or ((self.position[product]<0) and (ask == acc_bid+1))) and cpos < LIMIT:
                order_for = min(-vol, LIMIT - cpos)
                cpos += order_for
                assert(order_for >= 0)
                orders.append(Order(product, ask, order_for))

        undercut_buy = best_buy_pr + 1
        undercut_sell = best_sell_pr - 1

        bid_pr = min(undercut_buy, acc_bid -1) # we will shift this by 1 to beat this price
        sell_pr = max(undercut_sell, acc_ask +1 )

        if cpos < LIMIT:
            num = LIMIT - cpos
            orders.append(Order(product, bid_pr, num))
            cpos += num
        
        cpos = self.position[product]
        

        for bid, vol in obuy.items():
            if ((bid >= acc_ask) or ((self.position[product]>0) and (bid+1 == acc_ask))) and cpos > -LIMIT:
                order_for = max(-vol, -LIMIT-cpos)
                # order_for is a negative number denoting how much we will sell
                cpos += order_for
                assert(order_for <= 0)
                orders.append(Order(product, bid, order_for))

        if cpos > -LIMIT:
            num = -LIMIT-cpos
            orders.append(Order(product, sell_pr, num))
            cpos += num

        return orders
    
    
    
    
        

    def compute_orders(self, product, order_depth, acc_bid, acc_ask):

        if product == "AMETHYSTS":
            return self.compute_orders_AMETHYSTS(product, order_depth, acc_bid, acc_ask)
        if product == "STARFRUIT":
            return self.compute_orders_regression(product, order_depth, acc_bid, acc_ask, self.POSITION_LIMIT[product])
        if product == "ORCHIDS":
            return self.compute_orders_ORCHIDS(product, order_depth, acc_bid, acc_ask,self.POSITION_LIMIT[product])
        
    def run(self, state: TradingState) -> Dict[str, List[Order]]:
        """
        Only method required. It takes all buy and sell orders for all symbols as an input,
        and outputs a list of orders to be sent
        """
        # Initialize the method output dict as an empty dict
        result = {'AMETHYSTS' : [], 'STARFRUIT' : [], 'ORCHIDS' : []}

        # print("traderData: " + state.traderData)
        print("Observations: " + str(state.observations.conversionObservations['ORCHIDS'].humidity))
        # print()


        # Iterate over all the keys (the available products) contained in the order dephts
        for key, val in state.position.items():
            self.position[key] = val
        print()
        for key, val in self.position.items():
            print(f'{key} position: {val}')

        

        timestamp = state.timestamp

        if len(self.STARFRUIT_cache) == self.STARFRUIT_dim:
            self.STARFRUIT_cache.pop(0)
        
        if len(self.ORCHIDS_cache) == self.ORCHIDS_dim:
            self.ORCHIDS_cache.pop(0)
        
        _, bs_STARFRUIT = self.values_extract(collections.OrderedDict(sorted(state.order_depths['STARFRUIT'].sell_orders.items())))
        _, bb_STARFRUIT = self.values_extract(collections.OrderedDict(sorted(state.order_depths['STARFRUIT'].buy_orders.items(), reverse=True)), 1)

        self.STARFRUIT_cache.append((bs_STARFRUIT+bb_STARFRUIT)/2)

        _, bs_ORCHIDS = self.values_extract(collections.OrderedDict(sorted(state.order_depths['ORCHIDS'].sell_orders.items())))
        _, bb_ORCHIDS = self.values_extract(collections.OrderedDict(sorted(state.order_depths['ORCHIDS'].buy_orders.items(), reverse=True)), 1)

        self.ORCHIDS_cache.append((bs_ORCHIDS+bb_ORCHIDS)/2)

        INF = 1e9
    
        STARFRUIT_lb = -INF
        STARFRUIT_ub = INF

        if len(self.STARFRUIT_cache) == self.STARFRUIT_dim:
            STARFRUIT_lb = self.calc_next_price_STARFRUIT()-1
            STARFRUIT_ub = self.calc_next_price_STARFRUIT()+1

        AMETHYSTS_lb = 10000
        AMETHYSTS_ub = 10000

        ORCHIDS_lb = 1050
        ORCHIDS_ub = 1100

        '''if len(self.ORCHIDS_cache) == self.ORCHIDS_dim:
            ORCHIDS_lb = self.calc_next_price_ORCHIDS()-1
            ORCHIDS_ub = self.calc_next_price_ORCHIDS()+1'''


        # CHANGE FROM HERE

        acc_bid = {'AMETHYSTS' : AMETHYSTS_lb, 'STARFRUIT' : STARFRUIT_lb, 'ORCHIDS' : ORCHIDS_lb} # we want to buy at slightly below
        acc_ask = {'AMETHYSTS' : AMETHYSTS_ub, 'STARFRUIT' : STARFRUIT_ub, 'ORCHIDS' : ORCHIDS_ub} # we want to sell at slightly above

        self.steps += 1

        for product in state.market_trades.keys():
            for trade in state.market_trades[product]:
                if trade.buyer == trade.seller:
                    continue
                self.person_position[trade.buyer][product] = 1.5
                self.person_position[trade.seller][product] = -1.5
                self.person_actvalof_position[trade.buyer][product] += trade.quantity
                self.person_actvalof_position[trade.seller][product] += -trade.quantity

        '''orders = self.compute_orders_c_and_pc(state.order_depths)
        result['PINA_COLADAS'] += orders['PINA_COLADAS']
        result['COCONUTS'] += orders['COCONUTS']
        orders = self.compute_orders_dg(state.order_depths, state.observations)
        result['DIVING_GEAR'] += orders['DIVING_GEAR']
        orders = self.compute_orders_br(state.order_depths, state.timestamp)
        result['BERRIES'] += orders['BERRIES']

        orders = self.compute_orders_basket(state.order_depths)
        result['PICNIC_BASKET'] += orders['PICNIC_BASKET']
        result['DIP'] += orders['DIP']
        result['BAGUETTE'] += orders['BAGUETTE']
        result['UKULELE'] += orders['UKULELE']'''

        for product in ['AMETHYSTS','STARFRUIT','ORCHIDS']:
            order_depth: OrderDepth = state.order_depths[product]
            orders = self.compute_orders(product, order_depth, acc_bid[product], acc_ask[product])
            result[product] += orders

        for product in state.own_trades.keys():
            for trade in state.own_trades[product]:
                if trade.timestamp != state.timestamp-100:
                    continue
                # print(f'We are trading {product}, {trade.buyer}, {trade.seller}, {trade.quantity}, {trade.price}')
                self.volume_traded[product] += abs(trade.quantity)
                if trade.buyer == "SUBMISSION":
                    self.cpnl[product] -= trade.quantity * trade.price
                else:
                    self.cpnl[product] += trade.quantity * trade.price

        totpnl = 0

        for product in state.order_depths.keys():
            settled_pnl = 0
            best_sell = min(state.order_depths[product].sell_orders.keys())
            best_buy = max(state.order_depths[product].buy_orders.keys())

            if self.position[product] < 0:
                settled_pnl += self.position[product] * best_buy
            else:
                settled_pnl += self.position[product] * best_sell
            totpnl += settled_pnl + self.cpnl[product]
            print(f"For product {product}, {settled_pnl + self.cpnl[product]}, {(settled_pnl+self.cpnl[product])/(self.volume_traded[product]+1e-20)}")

        

        print(f"Timestamp {timestamp}, Total PNL ended up being {totpnl}")
        # print(f'Will trade {result}')
        print("End transmission")
        traderData = "SAMPLE" 
        
		# Sample conversion request. Check more details below. 
        conversions = 1
        return result, conversions, traderData