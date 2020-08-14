# \EmailsApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_email**](EmailsApi.md#create_email) | **post** /emails | Create an email
[**delete_email**](EmailsApi.md#delete_email) | **delete** /emails/{id} | Delete the email
[**find_email_by_id**](EmailsApi.md#find_email_by_id) | **get** /emails/{id} | Retrieve an email
[**update_email**](EmailsApi.md#update_email) | **put** /emails/{id} | Update the email



## create_email

> crate::models::Email create_email(email)
Create an email

Add a new email address to the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | [**CreateEmailInput**](CreateEmailInput.md) | Email to create | [required] |

### Return type

[**crate::models::Email**](Email.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email

> delete_email(id)
Delete the email

Deletes the email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Email UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_email_by_id

> crate::models::Email find_email_by_id(id, include)
Retrieve an email

Provides one of the user’s emails.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Email UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Email**](Email.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email

> crate::models::Email update_email(id, email)
Update the email

Updates the email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Email UUID | [required] |
**email** | [**UpdateEmailInput**](UpdateEmailInput.md) | email to update | [required] |

### Return type

[**crate::models::Email**](Email.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

