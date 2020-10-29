# \BGPApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bgp_session**](BGPApi.md#create_bgp_session) | **post** /devices/{id}/bgp/sessions | Create a BGP session
[**delete_bgp_session**](BGPApi.md#delete_bgp_session) | **delete** /bgp/sessions/{id} | Delete the BGP session
[**find_bgp_config_by_project**](BGPApi.md#find_bgp_config_by_project) | **get** /projects/{id}/bgp-config | Retrieve a bgp config
[**find_bgp_session_by_id**](BGPApi.md#find_bgp_session_by_id) | **get** /bgp/sessions/{id} | Retrieve a BGP session
[**find_bgp_sessions**](BGPApi.md#find_bgp_sessions) | **get** /devices/{id}/bgp/sessions | Retrieve all BGP sessions
[**find_project_bgp_sessions**](BGPApi.md#find_project_bgp_sessions) | **get** /projects/{id}/bgp/sessions | Retrieve all BGP sessions for project
[**get_bgp_neighbor_data**](BGPApi.md#get_bgp_neighbor_data) | **get** /devices/{id}/bgp/neighbors | Retrieve BGP neighbor data for this device
[**request_bgp_config**](BGPApi.md#request_bgp_config) | **post** /projects/{id}/bgp-configs | Requesting bgp config
[**update_bgp_session**](BGPApi.md#update_bgp_session) | **put** /bgp/sessions/{id} | Update the BGP session



## create_bgp_session

> crate::models::BgpSession create_bgp_session(id, bgp_session)
Create a BGP session

Creates a BGP session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**bgp_session** | [**BgpSessionInput**](BgpSessionInput.md) | BGP session to create | [required] |

### Return type

[**crate::models::BgpSession**](BgpSession.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bgp_session

> delete_bgp_session(id)
Delete the BGP session

Deletes the BGP session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | BGP session UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_bgp_config_by_project

> crate::models::BgpConfig find_bgp_config_by_project(id, include)
Retrieve a bgp config

Returns a bgp config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::BgpConfig**](BgpConfig.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_bgp_session_by_id

> crate::models::BgpSession find_bgp_session_by_id(id, include)
Retrieve a BGP session

Returns a BGP session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | BGP session UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::BgpSession**](BgpSession.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_bgp_sessions

> crate::models::BgpSessionList find_bgp_sessions(id)
Retrieve all BGP sessions

Provides a listing of available BGP sessions for the device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |

### Return type

[**crate::models::BgpSessionList**](BgpSessionList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_bgp_sessions

> crate::models::BgpSessionList find_project_bgp_sessions(id)
Retrieve all BGP sessions for project

Provides a listing of available BGP sessions for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |

### Return type

[**crate::models::BgpSessionList**](BgpSessionList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bgp_neighbor_data

> crate::models::BgpSessionNeighbors get_bgp_neighbor_data(id)
Retrieve BGP neighbor data for this device

Provides a summary of the BGP neighbor data associated to the BGP sessions for this device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |

### Return type

[**crate::models::BgpSessionNeighbors**](BgpSessionNeighbors.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_bgp_config

> request_bgp_config(id, bgp_config_request)
Requesting bgp config

Requests to enable bgp configuration for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**bgp_config_request** | [**BgpConfigRequestInput**](BgpConfigRequestInput.md) | BGP config Request to create | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bgp_session

> update_bgp_session(id, default_route)
Update the BGP session

Updates the BGP session by either enabling or disabling the default route functionality.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | BGP session UUID | [required] |
**default_route** | **bool** | Default route | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

