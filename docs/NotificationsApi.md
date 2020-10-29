# \NotificationsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_notifications**](NotificationsApi.md#find_notifications) | **get** /notifications | Retrieve all notifications



## find_notifications

> crate::models::NotificationList find_notifications(include)
Retrieve all notifications

Returns a collection of the current user’s notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::NotificationList**](NotificationList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

