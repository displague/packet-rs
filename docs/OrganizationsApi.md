# \OrganizationsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization**](OrganizationsApi.md#create_organization) | **post** /organizations | Create an organization
[**create_organization_invitation**](OrganizationsApi.md#create_organization_invitation) | **post** /organizations/{id}/invitations | Create an invitation for an organization
[**create_organization_project**](OrganizationsApi.md#create_organization_project) | **post** /organizations/{id}/projects | Create a project for the organization
[**create_payment_method**](OrganizationsApi.md#create_payment_method) | **post** /organizations/{id}/payment-methods | Create a payment method for the given organization
[**delete_organization**](OrganizationsApi.md#delete_organization) | **delete** /organizations/{id} | Delete the organization
[**find_facilities_by_organization**](OrganizationsApi.md#find_facilities_by_organization) | **get** /organizations/{id}/facilities | Retrieve all facilities visible by the organization
[**find_operating_systems_by_organization**](OrganizationsApi.md#find_operating_systems_by_organization) | **get** /organizations/{id}/operating-systems | Retrieve all operating systems visible by the organization
[**find_organization_by_id**](OrganizationsApi.md#find_organization_by_id) | **get** /organizations/{id} | Retrieve an organization's details
[**find_organization_customdata**](OrganizationsApi.md#find_organization_customdata) | **get** /organizations/{id}/customdata | Retrieve the custom metadata of an organization
[**find_organization_devices**](OrganizationsApi.md#find_organization_devices) | **get** /organizations/{id}/devices | Retrieve all devices of an organization
[**find_organization_events**](OrganizationsApi.md#find_organization_events) | **get** /organizations/{id}/events | Retrieve organization's events
[**find_organization_invitations**](OrganizationsApi.md#find_organization_invitations) | **get** /organizations/{id}/invitations | Retrieve organization invitations
[**find_organization_payment_methods**](OrganizationsApi.md#find_organization_payment_methods) | **get** /organizations/{id}/payment-methods | Retrieve all payment methods of an organization
[**find_organization_projects**](OrganizationsApi.md#find_organization_projects) | **get** /organizations/{id}/projects | Retrieve all projects of an organization
[**find_organization_transfers**](OrganizationsApi.md#find_organization_transfers) | **get** /organizations/{id}/transfers | Retrieve all project transfer requests from or to an organization
[**find_organizations**](OrganizationsApi.md#find_organizations) | **get** /organizations | Retrieve all organizations
[**find_plans_by_organization**](OrganizationsApi.md#find_plans_by_organization) | **get** /organizations/{id}/plans | Retrieve all plans visible by the organization
[**update_organization**](OrganizationsApi.md#update_organization) | **put** /organizations/{id} | Update the organization



## create_organization

> crate::models::Organization create_organization(organization)
Create an organization

Creates an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | [**OrganizationInput**](OrganizationInput.md) | Organization to create | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_invitation

> crate::models::Invitation create_organization_invitation(id, invitation)
Create an invitation for an organization

In order to add a user to an organization, they must first be invited. To invite to several projects the parameter `projects_ids:[a,b,c]` can be used

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**invitation** | [**InvitationInput**](InvitationInput.md) | Invitation to create | [required] |

### Return type

[**crate::models::Invitation**](Invitation.md)

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


## delete_organization

> delete_organization(id)
Delete the organization

Deletes the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_facilities_by_organization

> crate::models::FacilityList find_facilities_by_organization(id, include)
Retrieve all facilities visible by the organization

Returns a listing of available datacenters for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::FacilityList**](FacilityList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_operating_systems_by_organization

> Vec<crate::models::OperatingSystem> find_operating_systems_by_organization(id, include)
Retrieve all operating systems visible by the organization

Returns a listing of available operating systems for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**Vec<crate::models::OperatingSystem>**](OperatingSystem.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organization_by_id

> crate::models::Organization find_organization_by_id(id, include)
Retrieve an organization's details

Returns a single organization's details, if the user is authorized to view it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organization_customdata

> find_organization_customdata(id)
Retrieve the custom metadata of an organization

Provides the custom metadata stored for this organization in json format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

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


## find_organization_events

> crate::models::EventList find_organization_events(id, include, page, per_page)
Retrieve organization's events

Returns a list of events for a single organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
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


## find_organization_invitations

> crate::models::InvitationList find_organization_invitations(id, include, page, per_page)
Retrieve organization invitations

Returns all invitations in an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
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


## find_organization_payment_methods

> crate::models::PaymentMethodList find_organization_payment_methods(id, include, page, per_page)
Retrieve all payment methods of an organization

Returns all payment methods of an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::PaymentMethodList**](PaymentMethodList.md)

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


## find_organization_transfers

> crate::models::TransferRequestList find_organization_transfers(id, include)
Retrieve all project transfer requests from or to an organization

Provides a collection of project transfer requests from or to the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::TransferRequestList**](TransferRequestList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organizations

> crate::models::OrganizationList find_organizations(personal, without_projects, include, page, per_page)
Retrieve all organizations

Returns a list of organizations that are accessible to the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**personal** | Option<**String**> | Include, exclude or show only personal organizations. |  |
**without_projects** | Option<**String**> | Include, exclude or show only organizations that have no projects. |  |
**include** | Option<**String**> | related attributes to include |  |
**page** | Option<**i32**> | page to display, default to 1, max 100_000 |  |
**per_page** | Option<**i32**> | items per page, default to 10, max 1_000 |  |

### Return type

[**crate::models::OrganizationList**](OrganizationList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_plans_by_organization

> crate::models::PlanList find_plans_by_organization(id, include)
Retrieve all plans visible by the organization

Returns a listing of available plans for the given organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::PlanList**](PlanList.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> crate::models::Organization update_organization(id, organization)
Update the organization

Updates the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Organization UUID | [required] |
**organization** | [**OrganizationInput**](OrganizationInput.md) | Organization to update | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

