# \PaymentMethodsApi

All URIs are relative to *http://api.packet.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_method**](PaymentMethodsApi.md#create_payment_method) | **post** /organizations/{id}/payment-methods | Create a payment method for the given organization
[**delete_payment_method**](PaymentMethodsApi.md#delete_payment_method) | **delete** /payment-methods/{id} | Delete the payment method
[**find_organization_payment_methods**](PaymentMethodsApi.md#find_organization_payment_methods) | **get** /organizations/{id}/payment-methods | Retrieve all payment methods of an organization
[**find_payment_method_by_id**](PaymentMethodsApi.md#find_payment_method_by_id) | **get** /payment-methods/{id} | Retrieve a payment method
[**update_payment_method**](PaymentMethodsApi.md#update_payment_method) | **put** /payment-methods/{id} | Update the payment method



## create_payment_method

> crate::models::PaymentMethod create_payment_method(id, payment_method)
Create a payment method for the given organization

Creates a payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**payment_method** | [**PaymentMethodCreateInput**](PaymentMethodCreateInput.md) | Payment Method to create | [required] |

### Return type

[**crate::models::PaymentMethod**](PaymentMethod.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_method

> delete_payment_method(id)
Delete the payment method

Deletes the payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Payment Method UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organization_payment_methods

> crate::models::PaymentMethodList find_organization_payment_methods(id, include)
Retrieve all payment methods of an organization

Returns all payment methods of an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::PaymentMethodList**](PaymentMethodList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_payment_method_by_id

> crate::models::PaymentMethod find_payment_method_by_id(id, include)
Retrieve a payment method

Returns a payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Payment Method UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::PaymentMethod**](PaymentMethod.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payment_method

> crate::models::PaymentMethod update_payment_method(id, payment_method)
Update the payment method

Updates the payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Payment Method UUID | [required] |
**payment_method** | [**PaymentMethodUpdateInput**](PaymentMethodUpdateInput.md) | Payment Method to update | [required] |

### Return type

[**crate::models::PaymentMethod**](PaymentMethod.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

