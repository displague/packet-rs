# \HardwareReservationsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_hardware_reservation_by_id**](HardwareReservationsApi.md#find_hardware_reservation_by_id) | **get** /hardware-reservations/{id} | Retrieve a hardware reservation
[**find_project_hardware_reservations**](HardwareReservationsApi.md#find_project_hardware_reservations) | **get** /projects/{id}/hardware-reservations | Retrieve all hardware reservations for a given project
[**hardware_reservations_id_move_post**](HardwareReservationsApi.md#hardware_reservations_id_move_post) | **post** /hardware-reservations/{id}/move | Move a hardware reservation



## find_hardware_reservation_by_id

> crate::models::Device find_hardware_reservation_by_id(id, include)
Retrieve a hardware reservation

Returns a single hardware reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | HardwareReservation UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Device**](Device.md)

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


## hardware_reservations_id_move_post

> crate::models::HardwareReservation hardware_reservations_id_move_post(id, project_id)
Move a hardware reservation

Move a hardware reservation to another project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Hardware Reservation UUID | [required] |
**project_id** | **String** | Project UUID | [required] |

### Return type

[**crate::models::HardwareReservation**](HardwareReservation.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

