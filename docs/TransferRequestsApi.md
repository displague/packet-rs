# \TransferRequestsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_transfer_request**](TransferRequestsApi.md#accept_transfer_request) | **put** /transfers/{id} | Accept a transfer request
[**create_transfer_request**](TransferRequestsApi.md#create_transfer_request) | **post** /projects/{id}/transfers | Create a transfer request
[**decline_transfer_request**](TransferRequestsApi.md#decline_transfer_request) | **delete** /transfers/{id} | Decline a transfer request
[**find_organization_transfers**](TransferRequestsApi.md#find_organization_transfers) | **get** /organizations/{id}/transfers | Retrieve all project transfer requests from or to an organization
[**find_transfer_request_by_id**](TransferRequestsApi.md#find_transfer_request_by_id) | **get** /transfers/{id} | View a transfer request



## accept_transfer_request

> accept_transfer_request(id)
Accept a transfer request

Accept a transfer request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Transfer request UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transfer_request

> crate::models::TransferRequest create_transfer_request(id, transfer_request)
Create a transfer request

Organization owners can transfer their projects to other organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | UUID of the project to be transferred | [required] |
**transfer_request** | [**TransferRequestInput**](TransferRequestInput.md) | Transfer Request to create | [required] |

### Return type

[**crate::models::TransferRequest**](TransferRequest.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decline_transfer_request

> decline_transfer_request(id)
Decline a transfer request

Decline a transfer request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Transfer request UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organization_transfers

> crate::models::TransferRequestList find_organization_transfers(id, include)
Retrieve all project transfer requests from or to an organization

Provides a collection of project transfer requests from or to the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::TransferRequestList**](TransferRequestList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_transfer_request_by_id

> crate::models::TransferRequest find_transfer_request_by_id(id, include)
View a transfer request

Returns a single transfer request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Transfer request UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::TransferRequest**](TransferRequest.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

