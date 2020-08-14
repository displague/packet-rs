# \UserVerificationTokensApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**consume_verification_request**](UserVerificationTokensApi.md#consume_verification_request) | **put** /verify-email | Verify a user using an email verification token
[**create_validation_request**](UserVerificationTokensApi.md#create_validation_request) | **post** /verify-email | Create an email verification request



## consume_verification_request

> consume_verification_request(token)
Verify a user using an email verification token

Consumes an email verification token and verifies the user associated with it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | User verification token | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_validation_request

> create_validation_request(login)
Create an email verification request

Creates an email verification request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login** | **String** | Email for verification request | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

