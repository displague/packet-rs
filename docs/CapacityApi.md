# \CapacityApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_capacity**](CapacityApi.md#check_capacity) | **post** /capacity | Check capacity
[**find_capacity**](CapacityApi.md#find_capacity) | **get** /capacity | View capacity



## check_capacity

> check_capacity(facility)
Check capacity

Validates if a deploy can be fulfilled.  Response: ``` \"servers\": [     { \"available\": true, ... },     { \"available\": false, ... } ] ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**facility** | [**CapacityInput**](CapacityInput.md) | Facility to create | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_capacity

> crate::models::CapacityList find_capacity()
View capacity

Returns a list of facilities and plans with their current capacity.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CapacityList**](CapacityList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

