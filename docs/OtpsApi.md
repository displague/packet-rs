# \OtpsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_ensure_otp**](OtpsApi.md#find_ensure_otp) | **post** /user/otp/verify/{otp} | Verify user by providing an OTP
[**find_recovery_codes**](OtpsApi.md#find_recovery_codes) | **get** /user/otp/recovery-codes | Retrieve my recovery codes
[**receive_codes**](OtpsApi.md#receive_codes) | **post** /user/otp/sms/receive | Receive an OTP per sms
[**regenerate_codes**](OtpsApi.md#regenerate_codes) | **post** /user/otp/recovery-codes | Generate new recovery codes



## find_ensure_otp

> find_ensure_otp(otp)
Verify user by providing an OTP

It verifies the user once a valid OTP is provided. It gives back a session token, essentially logging in the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**otp** | **String** | OTP | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_recovery_codes

> crate::models::RecoveryCodeList find_recovery_codes()
Retrieve my recovery codes

Returns my recovery codes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RecoveryCodeList**](RecoveryCodeList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## receive_codes

> receive_codes()
Receive an OTP per sms

Sends an OTP to the user's mobile phone.

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


## regenerate_codes

> crate::models::RecoveryCodeList regenerate_codes()
Generate new recovery codes

Generate a new set of recovery codes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RecoveryCodeList**](RecoveryCodeList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

