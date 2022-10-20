# Liquidity Forecasting

---

Mosaic has a built-in forecasting mechanism that can predict in advance when a particular liquidity level will be 
reached for a certain layer and intelligently rebalances. To do this, Mosaic employs 
[ML-enhanced forecasting](https://medium.com/composable-finance/liquidity-forecasting-in-mosaic-part-iv-machine-learning-based-methods-17e8f2e5de14) 
with Gaussian Processes. 

This machine learning model can capture liquidity evolution across vaults and outperforms non-ML-based models, like 
ARIMA. The ability to predict liquidity values in vaults improves user experience and the overall system's performance, 
as low liquidity vaults lead to failed transfers. The goal is to build a balanced network of vaults with sufficient 
liquidity where needed at all times. 
