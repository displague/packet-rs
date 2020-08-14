# \IPAddressesApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ip_assignment**](IPAddressesApi.md#create_ip_assignment) | **post** /devices/{id}/ips | Create a ip assignment
[**delete_ip_address**](IPAddressesApi.md#delete_ip_address) | **delete** /ips/{id} | Unassign an ip address
[**find_ip_address_by_id**](IPAddressesApi.md#find_ip_address_by_id) | **get** /ips/{id} | Retrieve an ip address
[**find_ip_address_customdata**](IPAddressesApi.md#find_ip_address_customdata) | **get** /ips/{id}/customdata | Retrieve the custom metadata of an IP Reservation or IP Assignment
[**find_ip_assignments**](IPAddressesApi.md#find_ip_assignments) | **get** /devices/{id}/ips | Retrieve all ip assignments
[**find_ip_availabilities**](IPAddressesApi.md#find_ip_availabilities) | **get** /ips/{id}/available | Retrieve all available subnets of a particular reservation
[**find_ip_reservations**](IPAddressesApi.md#find_ip_reservations) | **get** /projects/{id}/ips | Retrieve all ip reservations
[**request_ip_reservation**](IPAddressesApi.md#request_ip_reservation) | **post** /projects/{id}/ips | Requesting IP reservations



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


## delete_ip_address

> delete_ip_address(id)
Unassign an ip address

Note! This call can be used to un-assign an IP assignment or delete an IP reservation. Un-assign an IP address record. Use the assignment UUID you get after attaching the IP. This will remove the relationship between an IP and the device and will make the IP address available to be assigned to another device. Delete and IP reservation. Use the reservation UUID you get after adding the IP to the project. This will permanently delete the IP block reservation from the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | IP Address UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ip_address_by_id

> crate::models::IpAssignment find_ip_address_by_id(id, include)
Retrieve an ip address

Returns a single ip address if the user has access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | IP Address UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::IpAssignment**](IPAssignment.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_ip_address_customdata

> find_ip_address_customdata(project_id, id)
Retrieve the custom metadata of an IP Reservation or IP Assignment

Provides the custom metadata stored for this IP Reservation or IP Assignment in json format

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


## find_ip_availabilities

> crate::models::IpAvailabilitiesList find_ip_availabilities(id, cidr)
Retrieve all available subnets of a particular reservation

Provides a list of IP resevations for a single project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | IP Reservation UUID | [required] |
**cidr** | **String** | Size of subnets in bits | [required] |

### Return type

[**crate::models::IpAvailabilitiesList**](IPAvailabilitiesList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

