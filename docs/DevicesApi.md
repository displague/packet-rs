# \DevicesApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bgp_session**](DevicesApi.md#create_bgp_session) | **post** /devices/{id}/bgp/sessions | Create a BGP session
[**create_device**](DevicesApi.md#create_device) | **post** /projects/{id}/devices | Create a device
[**create_device_batch**](DevicesApi.md#create_device_batch) | **post** /projects/{id}/devices/batch | Create a devices batch
[**create_ip_assignment**](DevicesApi.md#create_ip_assignment) | **post** /devices/{id}/ips | Create a ip assignment
[**delete_device**](DevicesApi.md#delete_device) | **delete** /devices/{id} | Delete the device
[**find_bgp_sessions**](DevicesApi.md#find_bgp_sessions) | **get** /devices/{id}/bgp/sessions | Retrieve all BGP sessions
[**find_device_by_id**](DevicesApi.md#find_device_by_id) | **get** /devices/{id} | Retrieve a device
[**find_device_customdata**](DevicesApi.md#find_device_customdata) | **get** /devices/{id}/customdata | Retrieve the custom metadata of an instance
[**find_device_events**](DevicesApi.md#find_device_events) | **get** /devices/{id}/events | Retrieve device's events
[**find_device_usages**](DevicesApi.md#find_device_usages) | **get** /devices/{id}/usages | Retrieve all usages for device
[**find_instance_bandwidth**](DevicesApi.md#find_instance_bandwidth) | **get** /devices/{id}/bandwidth | Retrieve an instance bandwidth
[**find_ip_assignment_customdata**](DevicesApi.md#find_ip_assignment_customdata) | **get** /devices/:instance_id/ips/:id/customdata | Retrieve the custom metadata of an IP Assignment
[**find_ip_assignments**](DevicesApi.md#find_ip_assignments) | **get** /devices/{id}/ips | Retrieve all ip assignments
[**find_organization_devices**](DevicesApi.md#find_organization_devices) | **get** /organizations/{id}/devices | Retrieve all devices of an organization
[**find_project_devices**](DevicesApi.md#find_project_devices) | **get** /projects/{id}/devices | Retrieve all devices of a project
[**find_project_usage**](DevicesApi.md#find_project_usage) | **get** /projects/{id}/usages | Retrieve all usages for project
[**find_traffic**](DevicesApi.md#find_traffic) | **get** /devices/{id}/traffic | Retrieve device traffic
[**get_bgp_neighbor_data**](DevicesApi.md#get_bgp_neighbor_data) | **get** /devices/{id}/bgp/neighbors | Retrieve BGP neighbor data for this device
[**perform_action**](DevicesApi.md#perform_action) | **post** /devices/{id}/actions | Perform an action
[**update_device**](DevicesApi.md#update_device) | **put** /devices/{id} | Update the device



## create_bgp_session

> crate::models::BgpSession create_bgp_session(id, default_route)
Create a BGP session

Creates a BGP session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**default_route** | Option<**bool**> | Default route |  |

### Return type

[**crate::models::BgpSession**](BgpSession.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_device

> crate::models::Device create_device(id, device)
Create a device

Creates a new device and provisions it in our datacenter.  Type-specific options (such as operating_system for baremetal devices) should be included in the main data structure alongside hostname and plan.  The features attribute allows you to optionally specify what features your server should have.  For example, if you require a server with a TPM chip, you may specify `{ \"features\": { \"tpm\": \"required\" } }` (or `{ \"features\": [\"tpm\"] }` in shorthand).  The request will fail if there are no available servers matching your criteria. Alternatively, if you do not require a certain feature, but would prefer to be assigned a server with that feature if there are any available, you may specify that feature with a preferred value (see the example request below).  The request will not fail if we have no servers with that feature in our inventory.  The facilities attribute specifies in what datacenter you wish to create the device.  You can either specify a single facility `{ \"facility\": \"f1\" }` , or you can instruct to create the device in the best available datacenter `{ \"facility\": \"any\" }`. Additionally it is possible to set a prioritized location selection.  For example `{ \"facility\": [\"f3\", \"f2\", \"any\"] }` will try to assign to the facility f3, if there are no available f2, and so on. If \"any\" is not specified for \"facility\", the request will fail unless it can assign in the selected locations.  The `ip_addresses attribute will allow you to specify the addresses you want created with your device.  To maintain backwards compatibility, If the attribute is not sent in the request, it will be treated as if `{ \"ip_addresses\": [{ \"address_family\": 4, \"public\": true }, { \"address_family\": 4, \"public\": false }, { \"address_family\": 6, \"public\": true }] }` was sent.  The private IPv4 address is required and always need to be sent in the array. Not all operating systems support no public IPv4 address, so in those cases you will receive an error message.  For example, to only configure your server with a private IPv4 address, you can send `{ \"ip_addresses\": [{ \"address_family\": 4, \"public\": false }] }`.  Note: when specifying a subnet size larger than a /30, you will need to supply the UUID(s) of existing ip_reservations in your project to assign IPs from.  For example, `{ \"ip_addresses\": [..., {\"address_family\": 4, \"public\": true, \"ip_reservations\": [\"uuid1\", \"uuid2\"]}] }`  To access a server without public IPs, you can use our Out-of-Band console access (SOS) or use another server with public IPs as a proxy. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**device** | [**DeviceCreateInput**](DeviceCreateInput.md) | Device to create | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_device_batch

> crate::models::BatchesList create_device_batch(id, batch)
Create a devices batch

Creates new devices in batch and provisions them in our datacenter.  Type-specific options (such as operating_system for baremetal devices) should be included in the main data structure alongside hostname and plan.  The features attribute allows you to optionally specify what features your server should have.  For example, if you require a server with a TPM chip, you may specify `{ \"features\": { \"tpm\": \"required\" } }` (or `{ \"features\": [\"tpm\"] }` in shorthand).  The request will fail if there are no available servers matching your criteria. Alternatively, if you do not require a certain feature, but would prefer to be assigned a server with that feature if there are any available, you may specify that feature with a preferred value (see the example request below).  The request will not fail if we have no servers with that feature in our inventory.  The facilities attribute specifies in what datacenter you wish to create the device.  You can either specify a single facility `{ \"facility\": \"f1\" }` , or you can instruct to create the device in the best available datacenter `{ \"facility\": \"any\" }`. Additionally it is possible to set a prioritized location selection.  For example `{ \"facility\": [\"f3\", \"f2\", \"any\"] }` will try to assign to the facility f3, if there are no available f2, and so on. If \"any\" is not specified for \"facility\", the request will fail unless it can assign in the selected locations.  With `{ \"facility\": \"any\" }` you have the option to diversify to indicate how many facilities you are willing to be spread across. For this purpose use parameter: `facility_diversity_level = N`.  For example:  `{ \"facilities\": [\"sjc1\", \"ewr1\", \"any\"] ,  \"facility_diversity_level\" = 1, \"quantity\" = 10 }` will assign 10 devices into the same facility, trying first in \"sjc1\", and if there aren’t available, it will try in  \"ewr1\", otherwise any other.  The `ip_addresses` attribute will allow you to specify the addresses you want created with your device.  To maintain backwards compatibility, If the attribute is not sent in the request, it will be treated as if `{ \"ip_addresses\": [{ \"address_family\": 4, \"public\": true }, { \"address_family\": 4, \"public\": false }, { \"address_family\": 6, \"public\": true }] }` was sent.  The private IPv4 address is required and always need to be sent in the array. Not all operating systems support no public IPv4 address, so in those cases you will receive an error message.  For example, to only configure your server with a private IPv4 address, you can send `{ \"ip_addresses\": [{ \"address_family\": 4, \"public\": false }] }`.  Note: when specifying a subnet size larger than a /30, you will need to supply the UUID(s) of existing ip_reservations in your project to assign IPs from.  For example, `{ \"ip_addresses\": [..., {\"address_family\": 4, \"public\": true, \"ip_reservations\": [\"uuid1\", \"uuid2\"]}] }`  To access a server without public IPs, you can use our Out-of-Band console access (SOS) or use another server with public IPs as a proxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**batch** | [**InstancesBatchCreateInput**](InstancesBatchCreateInput.md) | Batches to create | [required] |

### Return type

[**crate::models::BatchesList**](BatchesList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ip_assignment

> crate::models::IpAssignment create_ip_assignment(id, ip_assignment)
Create a ip assignment

Creates an ip assignment for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**ip_assignment** | [**IpAssignmentInput**](IpAssignmentInput.md) | IPAssignment to create | [required] |

### Return type

[**crate::models::IpAssignment**](IPAssignment.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_device

> delete_device(id, force_delete)
Delete the device

Deletes a device and deprovisions it in our datacenter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**force_delete** | Option<**bool**> | Force the deletion of the device, by detaching any storage volume still active. |  |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

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


## find_device_by_id

> crate::models::Device find_device_by_id(id, include)
Retrieve a device

Type-specific options (such as facility for baremetal devices) will be included as part of the main data structure.                          State value can be one of: active inactive queued or provisioning

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_device_customdata

> find_device_customdata(id)
Retrieve the custom metadata of an instance

Provides the custom metadata stored for this instance in json format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Instance UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_device_events

> crate::models::EventList find_device_events(id, include, page, per_page)
Retrieve device's events

Returns a list of events pertaining to a specific device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::EventList**](EventList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_device_usages

> crate::models::DeviceUsageList find_device_usages(id, created_after, created_before)
Retrieve all usages for device

Returns all usages for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**created_after** | Option<**String**> | Filter usages created after this date |  |
**created_before** | Option<**String**> | Filter usages created before this date |  |

### Return type

[**crate::models::DeviceUsageList**](DeviceUsageList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_instance_bandwidth

> find_instance_bandwidth(id, from, until)
Retrieve an instance bandwidth

Retrieve an instance bandwidth for a given period of time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**from** | **String** | Timestamp from range | [required] |
**until** | **String** | Timestamp to range | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ip_assignment_customdata

> find_ip_assignment_customdata(project_id, id)
Retrieve the custom metadata of an IP Assignment

Provides the custom metadata stored for this IP Assignment in json format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**String**](.md) | Project UUID | [required] |
**id** | [**String**](.md) | Ip Assignment UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ip_assignments

> crate::models::IpAssignmentList find_ip_assignments(id, include)
Retrieve all ip assignments

Returns all ip assignments for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::IpAssignmentList**](IPAssignmentList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organization_devices

> crate::models::DeviceList find_organization_devices(id, include, page, per_page)
Retrieve all devices of an organization

Provides a collection of devices for a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::DeviceList**](DeviceList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_devices

> crate::models::DeviceList find_project_devices(id, include, page, per_page)
Retrieve all devices of a project

Provides a collection of devices for a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::DeviceList**](DeviceList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_usage

> crate::models::ProjectUsageList find_project_usage(id, created_after, created_before)
Retrieve all usages for project

Returns all usages for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**created_after** | Option<**String**> | Filter usages created after this date |  |
**created_before** | Option<**String**> | Filter usages created before this date |  |

### Return type

[**crate::models::ProjectUsageList**](ProjectUsageList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_traffic

> find_traffic(id, direction, timeframe, interval, bucket)
Retrieve device traffic

Returns traffic for a specific device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**direction** | **String** | Traffic direction | [required] |
**timeframe** | [**Timeframe**](Timeframe.md) | Traffic timeframe | [required] |
**interval** | Option<**String**> | Traffic interval |  |
**bucket** | Option<**String**> | Traffic bucket |  |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

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


## perform_action

> perform_action(id, _type)
Perform an action

Performs an action for the given device.  Possible actions include: power_on, power_off, reboot, reinstall, and rescue (reboot the device into rescue OS.)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**_type** | **String** | Action to perform | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_device

> crate::models::Device update_device(id, device)
Update the device

Updates the device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Device UUID | [required] |
**device** | [**DeviceUpdateInput**](DeviceUpdateInput.md) | Facility to update | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

