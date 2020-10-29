# \InternetGatewaysApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_internet_gateway**](InternetGatewaysApi.md#create_internet_gateway) | **post** /virtual-networks/{id}/internet-gateways | Create an internet gateway



## create_internet_gateway

> crate::models::InternetGateway create_internet_gateway(id, length)
Create an internet gateway

Creates an internet gateway.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Virtual Network UUID | [required] |
**length** | **String** | IP Reservation length | [required] |

### Return type

[**crate::models::InternetGateway**](InternetGateway.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

