# \BatchesApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_device_batch**](BatchesApi.md#create_device_batch) | **post** /projects/{id}/devices/batch | Create a devices batch
[**delete_batch**](BatchesApi.md#delete_batch) | **delete** /batches/{id} | Delete the Batch
[**find_batch_by_id**](BatchesApi.md#find_batch_by_id) | **get** /batches/{id} | Retrieve a Batch
[**find_batches_by_project**](BatchesApi.md#find_batches_by_project) | **get** /projects/{id}/batches | Retrieve all batches by project



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


## delete_batch

> delete_batch(id, remove_associated_instances)
Delete the Batch

Deletes the Batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Batch UUID | [required] |
**remove_associated_instances** | **bool** | Default route | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_batch_by_id

> crate::models::Batch find_batch_by_id(id, include)
Retrieve a Batch

Returns a Batch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Batch UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Batch**](Batch.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

