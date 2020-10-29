# \InvitationsApi

All URIs are relative to *https://api.equinix.com/metal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invitation**](InvitationsApi.md#accept_invitation) | **put** /invitations/{id} | Accept an invitation
[**create_organization_invitation**](InvitationsApi.md#create_organization_invitation) | **post** /organizations/{id}/invitations | Create an invitation for an organization
[**create_project_invitation**](InvitationsApi.md#create_project_invitation) | **post** /projects/{project_id}/invitations | Create an invitation for a project
[**decline_invitation**](InvitationsApi.md#decline_invitation) | **delete** /invitations/{id} | Decline an invitation
[**find_invitation_by_id**](InvitationsApi.md#find_invitation_by_id) | **get** /invitations/{id} | View an invitation
[**find_invitations**](InvitationsApi.md#find_invitations) | **get** /invitations | Retrieve current user invitations
[**find_organization_invitations**](InvitationsApi.md#find_organization_invitations) | **get** /organizations/{id}/invitations | Retrieve organization invitations
[**find_project_invitations**](InvitationsApi.md#find_project_invitations) | **get** /projects/{project_id}/invitations | Retrieve project invitations



## accept_invitation

> crate::models::Membership accept_invitation(id)
Accept an invitation

Accept an invitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Invitation UUID | [required] |

### Return type

[**crate::models::Membership**](Membership.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
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


## decline_invitation

> decline_invitation(id)
Decline an invitation

Decline an invitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Invitation UUID | [required] |

### Return type

 (empty response body)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_invitation_by_id

> crate::models::Invitation find_invitation_by_id(id, include)
View an invitation

Returns a single invitation. (It include the `invitable` to maintain backward compatibility but will be removed soon)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | Invitation UUID | [required] |
**include** | Option<**String**> | related attributes to include |  |

### Return type

[**crate::models::Invitation**](Invitation.md)

### Authorization

[x_auth_token](../README.md#x_auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_invitations

> crate::models::InvitationList find_invitations(include, page, per_page)
Retrieve current user invitations

Returns all invitations in current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

