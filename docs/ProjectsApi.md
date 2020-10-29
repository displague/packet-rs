# \ProjectsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_device**](ProjectsApi.md#create_device) | **post** /projects/{id}/devices | Create a device
[**create_license**](ProjectsApi.md#create_license) | **post** /projects/{id}/licenses | Create a License
[**create_organization_project**](ProjectsApi.md#create_organization_project) | **post** /organizations/{id}/projects | Create a project for the organization
[**create_project**](ProjectsApi.md#create_project) | **post** /projects | Create a project
[**create_project_invitation**](ProjectsApi.md#create_project_invitation) | **post** /projects/{project_id}/invitations | Create an invitation for a project
[**create_project_ssh_key**](ProjectsApi.md#create_project_ssh_key) | **post** /projects/{id}/ssh-keys | Create a ssh key for the given project
[**create_spot_market_request**](ProjectsApi.md#create_spot_market_request) | **post** /projects/{id}/spot-market-requests | Create a spot market request
[**create_transfer_request**](ProjectsApi.md#create_transfer_request) | **post** /projects/{id}/transfers | Create a transfer request
[**create_virtual_network**](ProjectsApi.md#create_virtual_network) | **post** /projects/{id}/virtual-networks | Create an virtual network
[**delete_project**](ProjectsApi.md#delete_project) | **delete** /projects/{id} | Delete the project
[**find_batches_by_project**](ProjectsApi.md#find_batches_by_project) | **get** /projects/{id}/batches | Retrieve all batches by project
[**find_bgp_config_by_project**](ProjectsApi.md#find_bgp_config_by_project) | **get** /projects/{id}/bgp-config | Retrieve a bgp config
[**find_device_ssh_keys**](ProjectsApi.md#find_device_ssh_keys) | **get** /devices/{id}/ssh-keys | Retrieve a device's ssh keys
[**find_facilities_by_project**](ProjectsApi.md#find_facilities_by_project) | **get** /projects/{id}/facilities | Retrieve all facilities visible by the project
[**find_ip_reservation_customdata**](ProjectsApi.md#find_ip_reservation_customdata) | **get** /projects/{project_id}/ips/{id}/customdata | Retrieve the custom metadata of an IP Reservation
[**find_ip_reservations**](ProjectsApi.md#find_ip_reservations) | **get** /projects/{id}/ips | Retrieve all ip reservations
[**find_organization_projects**](ProjectsApi.md#find_organization_projects) | **get** /organizations/{id}/projects | Retrieve all projects of an organization
[**find_plans_by_project**](ProjectsApi.md#find_plans_by_project) | **get** /projects/{id}/plans | Retrieve all plans visible by the project
[**find_project_bgp_sessions**](ProjectsApi.md#find_project_bgp_sessions) | **get** /projects/{id}/bgp/sessions | Retrieve all BGP sessions for project
[**find_project_by_id**](ProjectsApi.md#find_project_by_id) | **get** /projects/{id} | Retrieve a project
[**find_project_customdata**](ProjectsApi.md#find_project_customdata) | **get** /projects/{id}/customdata | Retrieve the custom metadata of a project
[**find_project_devices**](ProjectsApi.md#find_project_devices) | **get** /projects/{id}/devices | Retrieve all devices of a project
[**find_project_events**](ProjectsApi.md#find_project_events) | **get** /projects/{id}/events | Retrieve project's events
[**find_project_hardware_reservations**](ProjectsApi.md#find_project_hardware_reservations) | **get** /projects/{id}/hardware-reservations | Retrieve all hardware reservations for a given project
[**find_project_invitations**](ProjectsApi.md#find_project_invitations) | **get** /projects/{project_id}/invitations | Retrieve project invitations
[**find_project_licenses**](ProjectsApi.md#find_project_licenses) | **get** /projects/{id}/licenses | Retrieve all licenses
[**find_project_memberships**](ProjectsApi.md#find_project_memberships) | **get** /projects/{project_id}/memberships | Retrieve project memberships
[**find_project_ssh_keys**](ProjectsApi.md#find_project_ssh_keys) | **get** /projects/{id}/ssh-keys | Retrieve a project's ssh keys
[**find_projects**](ProjectsApi.md#find_projects) | **get** /projects | Retrieve all projects
[**find_virtual_networks**](ProjectsApi.md#find_virtual_networks) | **get** /projects/{id}/virtual-networks | Retrieve all virtual networks
[**list_spot_market_requests**](ProjectsApi.md#list_spot_market_requests) | **get** /projects/{id}/spot-market-requests | List spot market requests
[**request_bgp_config**](ProjectsApi.md#request_bgp_config) | **post** /projects/{id}/bgp-configs | Requesting bgp config
[**request_ip_reservation**](ProjectsApi.md#request_ip_reservation) | **post** /projects/{id}/ips | Requesting IP reservations
[**update_project**](ProjectsApi.md#update_project) | **put** /projects/{id} | Update the project



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


## create_license

> crate::models::License create_license(id, license)
Create a License

Creates a new license for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**license** | [**LicenseCreateInput**](LicenseCreateInput.md) | License to create | [required] |

### Return type

[**crate::models::License**](License.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_project

> crate::models::Project create_organization_project(id, project)
Create a project for the organization

Creates a new project for the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**project** | [**ProjectCreateInput**](ProjectCreateInput.md) | Project to create | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project

> crate::models::Project create_project(project)
Create a project

Creates a new project for the user default organization. If the user don't have an organization, a new one will be created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | [**ProjectCreateFromRootInput**](ProjectCreateFromRootInput.md) | Project to create | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_invitation

> crate::models::Invitation create_project_invitation(project_id, invitation)
Create an invitation for a project

In order to add a user to a project, they must first be invited.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**String**](.md) | Project UUID | [required] |
**invitation** | [**InvitationInput**](InvitationInput.md) | Invitation to create | [required] |

### Return type

[**crate::models::Invitation**](Invitation.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## create_spot_market_request

> crate::models::SpotMarketRequest create_spot_market_request(id, spot_market_request)
Create a spot market request

Creates a new spot market request.  Type-specific options (such as operating_system for baremetal devices) should be included in the main data structure alongside hostname and plan.  The features attribute allows you to optionally specify what features your server should have. For example, if you require a server with a TPM chip, you may specify `{ \"features\": { \"tpm\": \"required\" } }` (or `{ \"features\": [\"tpm\"] }` in shorthand).  The request will fail if there are no available servers matching your criteria. Alternatively, if you do not require a certain feature, but would prefer to be assigned a server with that feature if there are any available, you may specify that feature with a preferred value (see the example request below).  The request will not fail if we have no servers with that feature in our inventory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**spot_market_request** | [**SpotMarketRequestCreateInput**](SpotMarketRequestCreateInput.md) | Spot Market Request to create | [required] |

### Return type

[**crate::models::SpotMarketRequest**](SpotMarketRequest.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transfer_request

> crate::models::TransferRequest create_transfer_request(id, transfer_request)
Create a transfer request

Organization owners can transfer their projects to other organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | UUID of the project to be transferred | [required] |
**transfer_request** | [**TransferRequestInput**](TransferRequestInput.md) | Transfer Request to create | [required] |

### Return type

[**crate::models::TransferRequest**](TransferRequest.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
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


## delete_project

> delete_project(id)
Delete the project

Deletes the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_batches_by_project

> crate::models::BatchesList find_batches_by_project(id, include)
Retrieve all batches by project

Returns all batches for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::BatchesList**](BatchesList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## find_facilities_by_project

> crate::models::FacilityList find_facilities_by_project(id, include)
Retrieve all facilities visible by the project

Returns a listing of available datacenters for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::FacilityList**](FacilityList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ip_reservation_customdata

> find_ip_reservation_customdata(project_id, id)
Retrieve the custom metadata of an IP Reservation

Provides the custom metadata stored for this IP Reservation in json format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**String**](.md) | Project UUID | [required] |
**id** | [**String**](.md) | Ip Reservation UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ip_reservations

> crate::models::IpReservationList find_ip_reservations(id, include)
Retrieve all ip reservations

Provides a list of IP resevations for a single project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::IpReservationList**](IPReservationList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organization_projects

> crate::models::ProjectList find_organization_projects(id, include, page, per_page)
Retrieve all projects of an organization

Returns a collection of projects that belong to the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::ProjectList**](ProjectList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_plans_by_project

> crate::models::PlanList find_plans_by_project(id, include)
Retrieve all plans visible by the project

Returns a listing of available plans for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::PlanList**](PlanList.md)

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


## find_project_by_id

> crate::models::Project find_project_by_id(id, include)
Retrieve a project

Returns a single project if the user has access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_customdata

> find_project_customdata(id)
Retrieve the custom metadata of a project

Provides the custom metadata stored for this project in json format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

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


## find_project_events

> crate::models::EventList find_project_events(id, include, page, per_page)
Retrieve project's events

Returns a list of events for a single project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
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


## find_project_hardware_reservations

> crate::models::HardwareReservationList find_project_hardware_reservations(id, include, page, per_page)
Retrieve all hardware reservations for a given project

Provides a collection of hardware reservations for a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::HardwareReservationList**](HardwareReservationList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_project_invitations

> crate::models::InvitationList find_project_invitations(project_id, include, page, per_page)
Retrieve project invitations

Returns all invitations in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**String**](.md) | Project UUID | [required] |
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


## find_project_licenses

> crate::models::LicenseList find_project_licenses(id, include, page, per_page)
Retrieve all licenses

Provides a collection of licenses for a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::LicenseList**](LicenseList.md)

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


## find_projects

> crate::models::ProjectList find_projects(include, page, per_page)
Retrieve all projects

Returns a collection of projects that the current user is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::ProjectList**](ProjectList.md)

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


## list_spot_market_requests

> crate::models::SpotMarketRequestList list_spot_market_requests(id)
List spot market requests

View all spot market requests for a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |

### Return type

[**crate::models::SpotMarketRequestList**](SpotMarketRequestList.md)

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


## request_ip_reservation

> crate::models::IpReservation request_ip_reservation(id, ip_reservation_request)
Requesting IP reservations

Request more IP space for a project in order to have additional IP addresses to assign to devices.  If the request is within the max quota, an IP reservation will be created. If the project will exceed its IP quota, a request will be submitted for review, and will return an IP Reservation with a `state` of `pending`. You can automatically have the request fail with HTTP status 422 instead of triggering the review process by providing the `fail_on_approval_required` parameter set to `true` in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**ip_reservation_request** | [**IpReservationRequestInput**](IpReservationRequestInput.md) | IP Reservation Request to create | [required] |

### Return type

[**crate::models::IpReservation**](IPReservation.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> crate::models::Project update_project(id, project)
Update the project

Updates the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**project** | [**ProjectUpdateInput**](ProjectUpdateInput.md) | Project to update | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

