# \OperatingSystemsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_operating_systems**](OperatingSystemsApi.md#find_operating_systems) | **get** /operating-systems | Retrieve all operating systems
[**find_operating_systems_by_organization**](OperatingSystemsApi.md#find_operating_systems_by_organization) | **get** /organizations/{id}/operating-systems | Retrieve all operating systems visible by the organization



## find_operating_systems

> Vec<crate::models::OperatingSystem> find_operating_systems()
Retrieve all operating systems

Provides a listing of available operating systems to provision your new device with.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::OperatingSystem>**](OperatingSystem.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_operating_systems_by_organization

> Vec<crate::models::OperatingSystem> find_operating_systems_by_organization(id, include)
Retrieve all operating systems visible by the organization

Returns a listing of available operating systems for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**Vec<crate::models::OperatingSystem>**](OperatingSystem.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

