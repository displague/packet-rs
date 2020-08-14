# \SSHKeysApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_ssh_key**](SSHKeysApi.md#create_project_ssh_key) | **post** /projects/{id}/ssh-keys | Create a ssh key for the given project
[**create_ssh_key**](SSHKeysApi.md#create_ssh_key) | **post** /ssh-keys | Create a ssh key for the current user
[**delete_ssh_key**](SSHKeysApi.md#delete_ssh_key) | **delete** /ssh-keys/{id} | Delete the ssh key
[**find_device_ssh_keys**](SSHKeysApi.md#find_device_ssh_keys) | **get** /devices/{id}/ssh-keys | Retrieve a device's ssh keys
[**find_project_ssh_keys**](SSHKeysApi.md#find_project_ssh_keys) | **get** /projects/{id}/ssh-keys | Retrieve a project's ssh keys
[**find_ssh_key_by_id**](SSHKeysApi.md#find_ssh_key_by_id) | **get** /ssh-keys/{id} | Retrieve a ssh key
[**find_ssh_keys**](SSHKeysApi.md#find_ssh_keys) | **get** /ssh-keys | Retrieve all ssh keys
[**update_ssh_key**](SSHKeysApi.md#update_ssh_key) | **put** /ssh-keys/{id} | Update the ssh key



## create_project_ssh_key

> crate::models::SshKey create_project_ssh_key(id, ssh_key)
Create a ssh key for the given project

Creates a ssh key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**ssh_key** | [**SshKeyInput**](SshKeyInput.md) | ssh key to create | [required] |

### Return type

[**crate::models::SshKey**](SSHKey.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ssh_key

> crate::models::SshKey create_ssh_key(ssh_key)
Create a ssh key for the current user

Creates a ssh key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key** | [**SshKeyInput**](SshKeyInput.md) | ssh key to create | [required] |

### Return type

[**crate::models::SshKey**](SSHKey.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ssh_key

> delete_ssh_key(id)
Delete the ssh key

Deletes the ssh key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | ssh key UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_device_ssh_keys

> crate::models::SshKeyList find_device_ssh_keys(id, search_string, include)
Retrieve a device's ssh keys

Returns a collection of the device's ssh keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**search_string** | Option<**String**> | Search by key, label, or fingerprint |  |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::SshKeyList**](SSHKeyList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_ssh_keys

> crate::models::SshKeyList find_project_ssh_keys(id, search_string, include)
Retrieve a project's ssh keys

Returns a collection of the project's ssh keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**search_string** | Option<**String**> | Search by key, label, or fingerprint |  |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::SshKeyList**](SSHKeyList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ssh_key_by_id

> crate::models::SshKey find_ssh_key_by_id(id, include)
Retrieve a ssh key

Returns a single ssh key if the user has access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | SSH Key UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::SshKey**](SSHKey.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ssh_keys

> crate::models::SshKeyList find_ssh_keys(search_string, include)
Retrieve all ssh keys

Returns a collection of the user’s ssh keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_string** | Option<**String**> | Search by key, label, or fingerprint |  |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::SshKeyList**](SSHKeyList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ssh_key

> crate::models::SshKey update_ssh_key(id, ssh_key)
Update the ssh key

Updates the ssh key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | SSH Key UUID | [required] |
**ssh_key** | [**SshKeyInput**](SshKeyInput.md) | ssh key to update | [required] |

### Return type

[**crate::models::SshKey**](SSHKey.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

