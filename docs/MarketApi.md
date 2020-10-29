# \MarketApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_spot_market_prices**](MarketApi.md#find_spot_market_prices) | **get** /market/spot/prices | Get current spot market prices
[**find_spot_market_prices_history**](MarketApi.md#find_spot_market_prices_history) | **get** /market/spot/prices/history | Get spot market prices for a given period of time



## find_spot_market_prices

> crate::models::SpotMarketPricesList find_spot_market_prices(facility, plan)
Get current spot market prices

Get Packet current spot market prices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**facility** | Option<**String**> | Facility to check spot market prices |  |
**plan** | Option<**String**> | Plan to check spot market prices |  |

### Return type

[**crate::models::SpotMarketPricesList**](SpotMarketPricesList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_spot_market_prices_history

> crate::models::SpotPricesHistoryReport find_spot_market_prices_history(facility, plan, from, until)
Get spot market prices for a given period of time

Get spot market prices for a given plan and facility in a fixed period of time  *Note: In the `200` response, the property `datapoints` contains arrays of `[float, integer]`.*

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**facility** | **String** | Facility to check spot market prices | [required] |
**plan** | **String** | Plan to check spot market prices | [required] |
**from** | **String** | Timestamp from range | [required] |
**until** | **String** | Timestamp to range | [required] |

### Return type

[**crate::models::SpotPricesHistoryReport**](SpotPricesHistoryReport.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

