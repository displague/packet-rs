# \VolumesApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_volume**](VolumesApi.md#clone_volume) | **post** /storage/{id}/clone | Clone volume/snapshot
[**create_volume**](VolumesApi.md#create_volume) | **post** /projects/{id}/storage | Create a volume
[**create_volume_attachment**](VolumesApi.md#create_volume_attachment) | **post** /storage/{id}/attachments | Attach your volume
[**create_volume_snapshot_policy**](VolumesApi.md#create_volume_snapshot_policy) | **post** /storage/{id}/snapshot-policies | Create a volume snapshot policy
[**delete_volume**](VolumesApi.md#delete_volume) | **delete** /storage/{id} | Delete the volume
[**delete_volume_attachment**](VolumesApi.md#delete_volume_attachment) | **delete** /storage/attachments/{id} | Detach volume
[**delete_volume_snapshot**](VolumesApi.md#delete_volume_snapshot) | **delete** /storage/{volume_id}/snapshots/{id} | Delete volume snapshot
[**delete_volume_snapshot_policy**](VolumesApi.md#delete_volume_snapshot_policy) | **delete** /storage/snapshot-policies/{id} | Delete the volume snapshot policy
[**find_volume_attachment_by_id**](VolumesApi.md#find_volume_attachment_by_id) | **get** /storage/attachments/{id} | Retrieve an attachment
[**find_volume_attachments**](VolumesApi.md#find_volume_attachments) | **get** /storage/{id}/attachments | Retrieve all volume attachment
[**find_volume_by_id**](VolumesApi.md#find_volume_by_id) | **get** /storage/{id} | Retrieve a volume
[**find_volume_customdata**](VolumesApi.md#find_volume_customdata) | **get** /storage/{id}/customdata | Retrieve the custom metadata of a storage volume
[**find_volume_events**](VolumesApi.md#find_volume_events) | **get** /volumes/{id}/events | Retrieve volume's events
[**find_volume_snapshots**](VolumesApi.md#find_volume_snapshots) | **get** /storage/{id}/snapshots | Retrieve all volume snapshot
[**find_volumes**](VolumesApi.md#find_volumes) | **get** /projects/{id}/storage | Retrieve all volumes
[**restore_volume**](VolumesApi.md#restore_volume) | **post** /storage/{id}/restore | Restore volume
[**update_volume**](VolumesApi.md#update_volume) | **put** /storage/{id} | Update the volume
[**update_volume_snapshot_policy**](VolumesApi.md#update_volume_snapshot_policy) | **put** /storage/snapshot-policies/{id} | Update the volume snapshot policy



## clone_volume

> crate::models::Volume clone_volume(id, snapshot_timestamp)
Clone volume/snapshot

Clone your volume or snapshot into a new volume. To clone the volume, send an empty body. To promote a volume snapshot into a new volume, include the snapshot_timestamp attribute in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**snapshot_timestamp** | Option<**String**> | snapshot timestamp |  |

### Return type

[**crate::models::Volume**](Volume.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_volume

> crate::models::Volume create_volume(id, volume)
Create a volume

Creates a new volume in our datacenter. The valid attribute values for `plan` and `facility` are:           \"facility\": \"ams1\", \"ewr1\", \"nrt1\", \"sjc1\"          \"plan\": \"storage_1\" (Standard), \"storage_2\" (Performance)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**volume** | [**VolumeCreateInput**](VolumeCreateInput.md) | Volume to create | [required] |

### Return type

[**crate::models::Volume**](Volume.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_volume_attachment

> crate::models::VolumeAttachment create_volume_attachment(id, attachment)
Attach your volume

Attach your volume to a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**attachment** | [**VolumeAttachmentInput**](VolumeAttachmentInput.md) | Device to attach | [required] |

### Return type

[**crate::models::VolumeAttachment**](VolumeAttachment.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_volume_snapshot_policy

> crate::models::SnapshotPolicy create_volume_snapshot_policy(id, snapshot_frequency, snapshot_count)
Create a volume snapshot policy

Creates a new snapshot policy of your volume.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**snapshot_frequency** | **String** | Snapshot frequency | [required] |
**snapshot_count** | Option<**i32**> | Snapshot count |  |

### Return type

[**crate::models::SnapshotPolicy**](SnapshotPolicy.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume

> delete_volume(id)
Delete the volume

Deletes the volume.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume_attachment

> delete_volume_attachment(id)
Detach volume

Detach volume.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Attachment UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume_snapshot

> delete_volume_snapshot(volume_id, id)
Delete volume snapshot

Delete volume snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | [**String**](.md) | Volume UUID | [required] |
**id** | [**String**](.md) | Snapshot UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume_snapshot_policy

> delete_volume_snapshot_policy(id)
Delete the volume snapshot policy

Deletes the volume snapshot policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Snapshot Policy UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_volume_attachment_by_id

> crate::models::VolumeAttachment find_volume_attachment_by_id(id, include)
Retrieve an attachment

Returns a single attachment if the user has access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Attachment UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::VolumeAttachment**](VolumeAttachment.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_volume_attachments

> crate::models::VolumeAttachmentList find_volume_attachments(id, include)
Retrieve all volume attachment

Returns a list of the current volume’s attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::VolumeAttachmentList**](VolumeAttachmentList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_volume_by_id

> crate::models::Volume find_volume_by_id(id, include)
Retrieve a volume

Returns a single volume if the user has access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Volume**](Volume.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_volume_customdata

> find_volume_customdata(id)
Retrieve the custom metadata of a storage volume

Provides the custom metadata stored for this storage volume in json format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Storage Volume UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_volume_events

> crate::models::EventList find_volume_events(id, include, page, per_page)
Retrieve volume's events

Returns a list of the current volume’s events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
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


## find_volume_snapshots

> crate::models::VolumeSnapshotList find_volume_snapshots(id, include)
Retrieve all volume snapshot

Returns a list of the current volume’s snapshots. To create Volume Snapshots, please check the Volume Snapshot Policies feature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::VolumeSnapshotList**](VolumeSnapshotList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_volumes

> crate::models::VolumeList find_volumes(id, include, page, per_page)
Retrieve all volumes

Returns a list of the current projects’s volumes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::VolumeList**](VolumeList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_volume

> crate::models::Volume restore_volume(id, restore_point)
Restore volume

Restore the volume to the given snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**restore_point** | **String** | restore point | [required] |

### Return type

[**crate::models::Volume**](Volume.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume

> crate::models::Volume update_volume(id, volume)
Update the volume

Updates the volume.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Volume UUID | [required] |
**volume** | [**VolumeUpdateInput**](VolumeUpdateInput.md) | Volume to update | [required] |

### Return type

[**crate::models::Volume**](Volume.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume_snapshot_policy

> crate::models::SnapshotPolicy update_volume_snapshot_policy(id, snapshot_frequency, snapshot_count)
Update the volume snapshot policy

Updates the volume snapshot policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Snapshot Policy UUID | [required] |
**snapshot_frequency** | **String** | Snapshot frequency | [required] |
**snapshot_count** | Option<**i32**> | Snapshot count |  |

### Return type

[**crate::models::SnapshotPolicy**](SnapshotPolicy.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

