# \SpotMarketRequestApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_spot_market_request**](SpotMarketRequestApi.md#create_spot_market_request) | **post** /projects/{id}/spot-market-requests | Create a spot market request
[**delete_spot_market_request**](SpotMarketRequestApi.md#delete_spot_market_request) | **delete** /spot-market-requests/{id} | Delete the spot market request
[**find_spot_market_request_by_id**](SpotMarketRequestApi.md#find_spot_market_request_by_id) | **get** /spot-market-requests/{id} | Retrieve a spot market request
[**list_spot_market_requests**](SpotMarketRequestApi.md#list_spot_market_requests) | **get** /projects/{id}/spot-market-requests | List spot market requests



## create_spot_market_request

> crate::models::SpotMarketRequest create_spot_market_request(id, spot_market_request)
Create a spot market request

Creates a new spot market request.  Type-specific options (such as operating_system for baremetal devices) should be included in the main data structure alongside hostname and plan.  The features attribute allows you to optionally specify what features your server should have. For example, if you require a server with a TPM chip, you may specify `{ \"features\": { \"tpm\": \"required\" } }` (or `{ \"features\": [\"tpm\"] }` in shorthand).  The request will fail if there are no available servers matching your criteria. Alternatively, if you do not require a certain feature, but would prefer to be assigned a server with that feature if there are any available, you may specify that feature with a preferred value (see the example request below).  The request will not fail if we have no servers with that feature in our inventory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**spot_market_request** | [**SpotMarketRequestCreateInput**](SpotMarketRequestCreateInput.md) | Spot Market Request to create | [required] |

### Return type

[**crate::models::SpotMarketRequest**](SpotMarketRequest.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_spot_market_request

> delete_spot_market_request(id, force_termination)
Delete the spot market request

Deletes the spot market request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | SpotMarketRequest UUID | [required] |
**force_termination** | Option<**bool**> | Terminate associated spot instances |  |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_spot_market_request_by_id

> crate::models::SpotMarketRequest find_spot_market_request_by_id(id, include)
Retrieve a spot market request

Returns a single spot market request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | SpotMarketRequest UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::SpotMarketRequest**](SpotMarketRequest.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spot_market_requests

> crate::models::SpotMarketRequestList list_spot_market_requests(id)
List spot market requests

View all spot market requests for a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |

### Return type

[**crate::models::SpotMarketRequestList**](SpotMarketRequestList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

