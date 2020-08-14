# \UsersApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_current_user**](UsersApi.md#find_current_user) | **get** /user | Retrieve the current user
[**find_user_by_id**](UsersApi.md#find_user_by_id) | **get** /users/{id} | Retrieve a user
[**find_user_customdata**](UsersApi.md#find_user_customdata) | **get** /users/{id}/customdata | Retrieve the custom metadata of a user
[**find_user_invitations**](UsersApi.md#find_user_invitations) | **get** /invitations | Retrieve current user invitations
[**find_users**](UsersApi.md#find_users) | **get** /users | Retrieve all users
[**update_current_user**](UsersApi.md#update_current_user) | **put** /user | Update the current user



## find_current_user

> crate::models::User find_current_user(include)
Retrieve the current user

Returns the user object for the currently logged-in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_user_by_id

> crate::models::User find_user_by_id(id, include)
Retrieve a user

Returns a single user if the user has access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | User UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_user_customdata

> find_user_customdata(id)
Retrieve the custom metadata of a user

Provides the custom metadata stored for this user in json format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | User UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_user_invitations

> crate::models::InvitationList find_user_invitations(id, include, page, per_page)
Retrieve current user invitations

Returns all invitations in current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | User UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::InvitationList**](InvitationList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_users

> crate::models::UserList find_users(include, page, per_page)
Retrieve all users

Returns a list of users that the are accessible to the current user (all users in the current user’s projects, essentially).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::UserList**](UserList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_user

> crate::models::User update_current_user(user)
Update the current user

Updates the currently logged-in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | [**UserUpdateInput**](UserUpdateInput.md) | User to update | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

