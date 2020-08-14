# \PasswordResetTokensApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_password_reset_token**](PasswordResetTokensApi.md#create_password_reset_token) | **post** /reset-password | Create a password reset token
[**reset_password**](PasswordResetTokensApi.md#reset_password) | **delete** /reset-password | Reset current user password



## create_password_reset_token

> create_password_reset_token(email)
Create a password reset token

Creates a password reset token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email of user to create password reset token | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password

> crate::models::NewPassword reset_password()
Reset current user password

Resets current user password.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NewPassword**](NewPassword.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

