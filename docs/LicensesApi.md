# \LicensesApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_license**](LicensesApi.md#create_license) | **post** /projects/{id}/licenses | Create a License
[**delete_license**](LicensesApi.md#delete_license) | **delete** /licenses/{id} | Delete the license
[**find_license_by_id**](LicensesApi.md#find_license_by_id) | **get** /licenses/{id} | Retrieve a license
[**find_licenses**](LicensesApi.md#find_licenses) | **get** /projects/{id}/licenses | Retrieve all licenses
[**update_license**](LicensesApi.md#update_license) | **put** /licenses/{id} | Update the license



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


## delete_license

> delete_license(id)
Delete the license

Deletes a license.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | License UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_license_by_id

> crate::models::License find_license_by_id(id, include)
Retrieve a license

Returns a license

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | License UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::License**](License.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_licenses

> crate::models::LicenseList find_licenses(id, include)
Retrieve all licenses

Provides a collection of licenses for a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Project UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::LicenseList**](LicenseList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_license

> crate::models::License update_license(id, license)
Update the license

Updates the license.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | License UUID | [required] |
**license** | [**LicenseUpdateInput**](LicenseUpdateInput.md) | License to update | [required] |

### Return type

[**crate::models::License**](License.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

