# \MembersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_members**](MembersApi.md#create_organization_members) | **POST** /v2/members | Invite new members (EAP)
[**delete_organization_member**](MembersApi.md#delete_organization_member) | **DELETE** /v2/members/{member_id} | Remove member (EAP)
[**delete_organization_permissions**](MembersApi.md#delete_organization_permissions) | **DELETE** /v2/members | Remove permission (EAP)
[**get_authenticated_member**](MembersApi.md#get_authenticated_member) | **GET** /v2/members/me | Get the authenticated account member
[**get_organization_member**](MembersApi.md#get_organization_member) | **GET** /v2/members/{member_id} | Get Organization Member
[**list_organization_members**](MembersApi.md#list_organization_members) | **GET** /v2/members | List Organization Members
[**update_organization_member**](MembersApi.md#update_organization_member) | **PATCH** /v2/members/{member_id} | Edit members (EAP)



## create_organization_members

> models::OrgMemberReadWithGrants create_organization_members(org_member_create, inviter_name, inviter_email)
Invite new members (EAP)

Create an organization member if needed, and grant it permissions.  The member can be specified either by ID (for an existing member), or by email (for either an existing member or a new one).  For a new member, an invite will be sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_member_create** | [**OrgMemberCreate**](OrgMemberCreate.md) |  | [required] |
**inviter_name** | Option<**String**> |  |  |
**inviter_email** | Option<**String**> |  |  |

### Return type

[**models::OrgMemberReadWithGrants**](OrgMemberReadWithGrants.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_member

> delete_organization_member(member_id)
Remove member (EAP)

Deletes an account member matching the given id or email address. The member will be removed from the active account in permit.io.  If the member is the only member in its account (organization), returns 400 (bad request), due to nobody remains with access to the account, meaning deletion of the entire account (org). To completely remove an account, call DELETE `/orgs/{org}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_id** | **String** | Either the unique id (UUID) of the account member, or the email address of the account member. | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_permissions

> delete_organization_permissions(org_member_remove_permissions)
Remove permission (EAP)

Remove permissions from a member. If the last permissions a member has are removed, the member is also deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_member_remove_permissions** | [**OrgMemberRemovePermissions**](OrgMemberRemovePermissions.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authenticated_member

> models::OrgMemberReadWithGrants get_authenticated_member()
Get the authenticated account member

Gets the authenticated account member's details.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrgMemberReadWithGrants**](OrgMemberReadWithGrants.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_member

> models::OrgMemberReadWithGrants get_organization_member(member_id)
Get Organization Member

Gets a single account member by its id or email address. matching the given member, if no such member exists under the current active account (organization), returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_id** | **String** | Either the unique id (UUID) of the account member, or the email address of the account member. | [required] |

### Return type

[**models::OrgMemberReadWithGrants**](OrgMemberReadWithGrants.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_members

> Vec<models::OrgMemberReadWithGrants> list_organization_members(project_id, env_id, page, per_page)
List Organization Members

Lists all the account members that current active account has access to, optionally filtering by project or environment. The active account/organization is determined by the API Key used or by the authenticated session id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**uuid::Uuid**> |  |  |
**env_id** | Option<**uuid::Uuid**> |  |  |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::OrgMemberReadWithGrants>**](OrgMemberReadWithGrants.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_member

> models::OrgMemberReadWithGrants update_organization_member(member_id, org_member_update)
Edit members (EAP)

Updates an account member's settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_id** | **String** | Either the unique id (UUID) of the account member, or the email address of the account member. | [required] |
**org_member_update** | [**OrgMemberUpdate**](OrgMemberUpdate.md) |  | [required] |

### Return type

[**models::OrgMemberReadWithGrants**](OrgMemberReadWithGrants.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

