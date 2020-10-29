# \FacilitiesApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_facilities**](FacilitiesApi.md#find_facilities) | **get** /facilities | Retrieve all facilities
[**find_facilities_by_organization**](FacilitiesApi.md#find_facilities_by_organization) | **get** /organizations/{id}/facilities | Retrieve all facilities visible by the organization
[**find_facilities_by_project**](FacilitiesApi.md#find_facilities_by_project) | **get** /projects/{id}/facilities | Retrieve all facilities visible by the project



## find_facilities

> crate::models::FacilityList find_facilities()
Retrieve all facilities

Provides a listing of available datacenters where you can provision Packet devices.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FacilityList**](FacilityList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_facilities_by_organization

> crate::models::FacilityList find_facilities_by_organization(id, include)
Retrieve all facilities visible by the organization

Returns a listing of available datacenters for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::FacilityList**](FacilityList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_facilities_by_project

> crate::models::FacilityList find_facilities_by_project(id, include)
Retrieve all facilities visible by the project

Returns a listing of available datacenters for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::FacilityList**](FacilityList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

