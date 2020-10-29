# \MembershipsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_membership**](MembershipsApi.md#delete_membership) | **delete** /memberships/{id} | Delete the membership
[**find_membership_by_id**](MembershipsApi.md#find_membership_by_id) | **get** /memberships/{id} | Retrieve a membership
[**find_project_memberships**](MembershipsApi.md#find_project_memberships) | **get** /projects/{project_id}/memberships | Retrieve project memberships
[**update_membership**](MembershipsApi.md#update_membership) | **put** /memberships/{id} | Update the membership



## delete_membership

> delete_membership(id)
Delete the membership

Deletes the membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Membership UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_membership_by_id

> crate::models::Membership find_membership_by_id(id, include)
Retrieve a membership

Returns a single membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Membership UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Membership**](Membership.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_memberships

> crate::models::MembershipList find_project_memberships(project_id, include, page, per_page)
Retrieve project memberships

Returns all memberships in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::MembershipList**](MembershipList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_membership

> crate::models::Membership update_membership(id, membership)
Update the membership

Updates the membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Membership UUID | [required] |
**membership** | [**MembershipInput**](MembershipInput.md) | Membership to update | [required] |

### Return type

[**crate::models::Membership**](Membership.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

