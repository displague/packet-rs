# \VLANsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_native_vlan**](VLANsApi.md#assign_native_vlan) | **post** /ports/{id}/native-vlan | Assign a native VLAN
[**create_internet_gateway**](VLANsApi.md#create_internet_gateway) | **post** /virtual-networks/{id}/internet-gateways | Create an internet gateway
[**create_virtual_network**](VLANsApi.md#create_virtual_network) | **post** /projects/{id}/virtual-networks | Create an virtual network
[**delete_native_vlan**](VLANsApi.md#delete_native_vlan) | **delete** /ports/{id}/native-vlan | Remove native VLAN
[**delete_virtual_network**](VLANsApi.md#delete_virtual_network) | **delete** /virtual-networks/{id} | Delete a virtual network
[**find_virtual_networks**](VLANsApi.md#find_virtual_networks) | **get** /projects/{id}/virtual-networks | Retrieve all virtual networks
[**get_virtual_network**](VLANsApi.md#get_virtual_network) | **get** /virtual-networks/{id} | Get a virtual network



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


## create_internet_gateway

> crate::models::InternetGateway create_internet_gateway(id, length)
Create an internet gateway

Creates an internet gateway.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Virtual Network UUID | [required] |
**length** | **String** | IP Reservation length | [required] |

### Return type

[**crate::models::InternetGateway**](InternetGateway.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_virtual_network

> crate::models::VirtualNetwork create_virtual_network(id, virtual_network)
Create an virtual network

Creates an virtual network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**virtual_network** | [**VirtualNetworkCreateInput**](VirtualNetworkCreateInput.md) | Virtual Network to create | [required] |

### Return type

[**crate::models::VirtualNetwork**](VirtualNetwork.md)

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


## delete_virtual_network

> crate::models::VirtualNetwork delete_virtual_network(id)
Delete a virtual network

Deletes a virtual network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Virtual Network UUID | [required] |

### Return type

[**crate::models::VirtualNetwork**](VirtualNetwork.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_virtual_networks

> crate::models::VirtualNetworkList find_virtual_networks(id, include)
Retrieve all virtual networks

Provides a list of virtual networks for a single project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::VirtualNetworkList**](VirtualNetworkList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_virtual_network

> crate::models::VirtualNetwork get_virtual_network(id)
Get a virtual network

Get a virtual network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Virtual Network UUID | [required] |

### Return type

[**crate::models::VirtualNetwork**](VirtualNetwork.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

