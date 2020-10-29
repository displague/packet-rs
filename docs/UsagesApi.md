# \UsagesApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_device_usages**](UsagesApi.md#find_device_usages) | **get** /devices/{id}/usages | Retrieve all usages for device
[**find_project_usage**](UsagesApi.md#find_project_usage) | **get** /projects/{id}/usages | Retrieve all usages for project



## find_device_usages

> crate::models::DeviceUsageList find_device_usages(id, created_after, created_before)
Retrieve all usages for device

Returns all usages for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**created_after** | Option<**String**> | Filter usages created after this date |  |
**created_before** | Option<**String**> | Filter usages created before this date |  |

### Return type

[**crate::models::DeviceUsageList**](DeviceUsageList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_usage

> crate::models::ProjectUsageList find_project_usage(id, created_after, created_before)
Retrieve all usages for project

Returns all usages for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**created_after** | Option<**String**> | Filter usages created after this date |  |
**created_before** | Option<**String**> | Filter usages created before this date |  |

### Return type

[**crate::models::ProjectUsageList**](ProjectUsageList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

