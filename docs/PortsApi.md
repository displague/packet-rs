# \PortsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_native_vlan**](PortsApi.md#assign_native_vlan) | **post** /ports/{id}/native-vlan | Assign a native VLAN
[**assign_port**](PortsApi.md#assign_port) | **post** /ports/{id}/assign | Assign a port to virtual network
[**bond_port**](PortsApi.md#bond_port) | **post** /ports/{id}/bond | Enabling bonding
[**convert_layer2**](PortsApi.md#convert_layer2) | **post** /ports/{id}/convert/layer-2 | Convert to Layer 2
[**convert_layer3**](PortsApi.md#convert_layer3) | **post** /ports/{id}/convert/layer-3 | Convert to Layer 3
[**delete_native_vlan**](PortsApi.md#delete_native_vlan) | **delete** /ports/{id}/native-vlan | Remove native VLAN
[**disbond_port**](PortsApi.md#disbond_port) | **post** /ports/{id}/disbond | Disabling bonding
[**unassign_port**](PortsApi.md#unassign_port) | **post** /ports/{id}/unassign | Unassign a port



## assign_native_vlan

> crate::models::Port assign_native_vlan(id, vnid)
Assign a native VLAN

Assigns a virtual network to this port as a \"native VLAN\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |
**vnid** | **String** | UUID or VNID of the virtual network to assign | [required] |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_port

> crate::models::Port assign_port(id, vnid)
Assign a port to virtual network

Assign a port for a hardware to virtual network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |
**vnid** | [**PortAssignInput**](PortAssignInput.md) | Virtual Network ID | [required] |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bond_port

> crate::models::Port bond_port(id, bulk_enable)
Enabling bonding

Enabling bonding for one or all ports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |
**bulk_enable** | Option<**bool**> | enable both ports |  |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_layer2

> crate::models::Port convert_layer2(id, vnid)
Convert to Layer 2

Converts a bond port to Layer 2. IP assignments of the port will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |
**vnid** | Option<[**PortAssignInput**](PortAssignInput.md)> | Virtual Network ID |  |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_layer3

> crate::models::Port convert_layer3(id, request_ips)
Convert to Layer 3

Converts a bond port to Layer 3. VLANs must first be unassigned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |
**request_ips** | Option<[**PortConvertLayer3Input**](PortConvertLayer3Input.md)> | IPs to request |  |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_native_vlan

> crate::models::Port delete_native_vlan(id)
Remove native VLAN

Removes the native VLAN from this port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disbond_port

> crate::models::Port disbond_port(id, bulk_disable)
Disabling bonding

Disabling bonding for one or all ports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |
**bulk_disable** | Option<**bool**> | disable both ports |  |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_port

> crate::models::Port unassign_port(id, vnid)
Unassign a port

Unassign a port for a hardware.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Port UUID | [required] |
**vnid** | [**PortAssignInput**](PortAssignInput.md) | Virtual Network ID | [required] |

### Return type

[**crate::models::Port**](Port.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

