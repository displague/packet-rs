# \PlansApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_plans**](PlansApi.md#find_plans) | **get** /plans | Retrieve all plans
[**find_plans_by_organization**](PlansApi.md#find_plans_by_organization) | **get** /organizations/{id}/plans | Retrieve all plans visible by the organization
[**find_plans_by_project**](PlansApi.md#find_plans_by_project) | **get** /projects/{id}/plans | Retrieve all plans visible by the project



## find_plans

> crate::models::PlanList find_plans(include)
Retrieve all plans

Provides a listing of available plans to provision your device on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::PlanList**](PlanList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_plans_by_organization

> crate::models::PlanList find_plans_by_organization(id, include, page, per_page)
Retrieve all plans visible by the organization

Returns a listing of available plans for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::PlanList**](PlanList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_plans_by_project

> crate::models::PlanList find_plans_by_project(id, include, page, per_page)
Retrieve all plans visible by the project

Returns a listing of available plans for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::PlanList**](PlanList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

