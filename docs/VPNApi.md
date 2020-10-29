# \VPNApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_current_user_vpn_config**](VPNApi.md#find_current_user_vpn_config) | **get** /user/vpn | Retrieve the client vpn config for current user
[**turn_off_current_user_vpn**](VPNApi.md#turn_off_current_user_vpn) | **delete** /user/vpn | Turn off vpn for the current user
[**turn_on_current_user_vpn**](VPNApi.md#turn_on_current_user_vpn) | **post** /user/vpn | Turn on vpn for the current user



## find_current_user_vpn_config

> crate::models::VpnConfig find_current_user_vpn_config(code)
Retrieve the client vpn config for current user

Returns the client vpn config for the currently logged-in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Facility code | [required] |

### Return type

[**crate::models::VpnConfig**](VPNConfig.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## turn_off_current_user_vpn

> turn_off_current_user_vpn()
Turn off vpn for the current user

Turns off vpn for the currently logged-in user.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## turn_on_current_user_vpn

> turn_on_current_user_vpn()
Turn on vpn for the current user

Turns on vpn for the currently logged-in user.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

