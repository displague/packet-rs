# \ConnectionsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_interconnection**](ConnectionsApi.md#create_organization_interconnection) | **post** /organizations/{organization_id}/connections | Request a new connection for the organization
[**create_project_interconnection**](ConnectionsApi.md#create_project_interconnection) | **post** /projects/{project_id}/connections | Request a new connection for the project's organization
[**delete_interconnection**](ConnectionsApi.md#delete_interconnection) | **delete** /connections/{id} | Delete connection
[**get_connection_port**](ConnectionsApi.md#get_connection_port) | **get** /connections/{connection_id}/ports/{id} | Get a connection port
[**get_interconnection**](ConnectionsApi.md#get_interconnection) | **get** /connections/{id} | Get connection
[**get_virtual_circuit**](ConnectionsApi.md#get_virtual_circuit) | **get** /virtual-circuits/{id} | Get a virtual circuit
[**list_connection_port_virtual_circuits**](ConnectionsApi.md#list_connection_port_virtual_circuits) | **get** /connections/{connection_id}/ports/{port_id}/virtual-circuits | List a connection port's virtual circuits
[**list_connection_ports**](ConnectionsApi.md#list_connection_ports) | **get** /connections/{connection_id}/ports | List a connection's ports
[**organization_list_interconnections**](ConnectionsApi.md#organization_list_interconnections) | **get** /organizations/{organization_id}/connections | List organization connections
[**project_list_interconnections**](ConnectionsApi.md#project_list_interconnections) | **get** /projects/{project_id}/connections | List project connections
[**update_interconnection**](ConnectionsApi.md#update_interconnection) | **put** /connections/{id} | Update connection
[**update_virtual_circuit**](ConnectionsApi.md#update_virtual_circuit) | **put** /virtual-circuits/{id} | Update a virtual circuit



## create_organization_interconnection

> crate::models::Interconnection create_organization_interconnection(organization_id, connection)
Request a new connection for the organization

Creates a new connection request. A Project ID must be specified in the request body for connections on shared ports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | [**String**](.md) | UUID of the organization | [required] |
**connection** | [**InterconnectionCreateInput**](InterconnectionCreateInput.md) | Connection details | [required] |

### Return type

[**crate::models::Interconnection**](Interconnection.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_interconnection

> crate::models::Interconnection create_project_interconnection(project_id, connection)
Request a new connection for the project's organization

Creates a new connection request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**String**](.md) | UUID of the project | [required] |
**connection** | [**InterconnectionCreateInput**](InterconnectionCreateInput.md) | Connection details | [required] |

### Return type

[**crate::models::Interconnection**](Interconnection.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_interconnection

> crate::models::Interconnection delete_interconnection(id)
Delete connection

Delete a connection, its associated ports and virtual circuits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Connection UUID | [required] |

### Return type

[**crate::models::Interconnection**](Interconnection.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_port

> crate::models::InterconnectionPort get_connection_port(connection_id, id)
Get a connection port

Get the details of an connection port.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | [**String**](.md) | UUID of the connection | [required] |
**id** | [**String**](.md) | Port UUID | [required] |

### Return type

[**crate::models::InterconnectionPort**](InterconnectionPort.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_interconnection

> crate::models::Interconnection get_interconnection(id)
Get connection

Get the details of a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Connection UUID | [required] |

### Return type

[**crate::models::Interconnection**](Interconnection.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_virtual_circuit

> crate::models::VirtualCircuit get_virtual_circuit(id)
Get a virtual circuit

Get the details of a virtual circuit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Virtual Circuit UUID | [required] |

### Return type

[**crate::models::VirtualCircuit**](VirtualCircuit.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_connection_port_virtual_circuits

> crate::models::VirtualCircuitList list_connection_port_virtual_circuits(connection_id, port_id)
List a connection port's virtual circuits

List the virtual circuit record(s) associatiated with a particular connection port.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | [**String**](.md) | UUID of the connection | [required] |
**port_id** | [**String**](.md) | UUID of the connection port | [required] |

### Return type

[**crate::models::VirtualCircuitList**](VirtualCircuitList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_connection_ports

> crate::models::InterconnectionPortList list_connection_ports(connection_id)
List a connection's ports

List the ports associated to an connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | [**String**](.md) | UUID of the connection | [required] |

### Return type

[**crate::models::InterconnectionPortList**](InterconnectionPortList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_list_interconnections

> crate::models::InterconnectionList organization_list_interconnections(organization_id)
List organization connections

List the connections belonging to the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | [**String**](.md) | UUID of the organization | [required] |

### Return type

[**crate::models::InterconnectionList**](InterconnectionList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_list_interconnections

> crate::models::InterconnectionList project_list_interconnections(project_id)
List project connections

List the connections belonging to the project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**String**](.md) | UUID of the project | [required] |

### Return type

[**crate::models::InterconnectionList**](InterconnectionList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_interconnection

> crate::models::Interconnection update_interconnection(id, connection)
Update connection

Update the details of a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Connection UUID | [required] |
**connection** | [**InterconnectionUpdateInput**](InterconnectionUpdateInput.md) | Updated connection details | [required] |

### Return type

[**crate::models::Interconnection**](Interconnection.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_virtual_circuit

> crate::models::VirtualCircuit update_virtual_circuit(id, virtual_circuit)
Update a virtual circuit

Update the details of a virtual circuit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Virtual Circuit UUID | [required] |
**virtual_circuit** | [**VirtualCircuitUpdateInput**](VirtualCircuitUpdateInput.md) | Updated Virtual Circuit details | [required] |

### Return type

[**crate::models::VirtualCircuit**](VirtualCircuit.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

