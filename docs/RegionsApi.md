# \RegionsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_regions**](RegionsApi.md#find_regions) | **get** /regions | Retrieve all regions



## find_regions

> crate::models::RegionsList find_regions(include, page, per_page)
Retrieve all regions

Returns all regions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::RegionsList**](RegionsList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

