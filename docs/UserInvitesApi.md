# \UserInvitesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_user_invite**](UserInvitesApi.md#approve_user_invite) | **POST** /v2/facts/{proj_id}/{env_id}/user_invites/{user_invite_id}/approve | Approve User Invite
[**create_user_invite**](UserInvitesApi.md#create_user_invite) | **POST** /v2/facts/{proj_id}/{env_id}/user_invites | Create User Invite
[**delete_user_invite**](UserInvitesApi.md#delete_user_invite) | **DELETE** /v2/facts/{proj_id}/{env_id}/user_invites/{user_invite_id} | Delete User Invite
[**get_user_invite**](UserInvitesApi.md#get_user_invite) | **GET** /v2/facts/{proj_id}/{env_id}/user_invites/{user_invite_id} | Get User Invite
[**list_user_invites**](UserInvitesApi.md#list_user_invites) | **GET** /v2/facts/{proj_id}/{env_id}/user_invites | List User Invites
[**update_user_invite**](UserInvitesApi.md#update_user_invite) | **PATCH** /v2/facts/{proj_id}/{env_id}/user_invites/{user_invite_id} | Update User Invite



## approve_user_invite

> models::UserRead approve_user_invite(proj_id, env_id, user_invite_id, elements_user_invite_approve)
Approve User Invite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_invite_id** | **String** | Either the unique id of the user_invite, or the URL-friendly key of the user_invite (i.e: the \"slug\"). | [required] |
**elements_user_invite_approve** | [**ElementsUserInviteApprove**](ElementsUserInviteApprove.md) |  | [required] |

### Return type

[**models::UserRead**](UserRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_invite

> models::ElementsUserInviteRead create_user_invite(proj_id, env_id, elements_user_invite_create)
Create User Invite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**elements_user_invite_create** | [**ElementsUserInviteCreate**](ElementsUserInviteCreate.md) |  | [required] |

### Return type

[**models::ElementsUserInviteRead**](ElementsUserInviteRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_invite

> delete_user_invite(proj_id, env_id, user_invite_id)
Delete User Invite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_invite_id** | **String** | Either the unique id of the user_invite, or the URL-friendly key of the user_invite (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_invite

> models::ElementsUserInviteRead get_user_invite(proj_id, env_id, user_invite_id, page, per_page)
Get User Invite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_invite_id** | **String** | Either the unique id of the user_invite, or the URL-friendly key of the user_invite (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::ElementsUserInviteRead**](ElementsUserInviteRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_invites

> models::PaginatedResultElementsUserInviteRead list_user_invites(proj_id, env_id, role, tenant, search, page, per_page)
List User Invites

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role** | Option<**String**> | optional role filter, will only return invited users with this role. |  |
**tenant** | Option<**String**> | optional tenant filter, will only return invited users in that tenant. |  |
**search** | Option<**String**> | optional search, will only return invited users to that email, key or name. |  |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::PaginatedResultElementsUserInviteRead**](PaginatedResult_ElementsUserInviteRead_.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_invite

> models::ElementsUserInviteRead update_user_invite(proj_id, env_id, user_invite_id, elements_user_invite_update)
Update User Invite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_invite_id** | **String** | Either the unique id of the user_invite, or the URL-friendly key of the user_invite (i.e: the \"slug\"). | [required] |
**elements_user_invite_update** | [**ElementsUserInviteUpdate**](ElementsUserInviteUpdate.md) |  | [required] |

### Return type

[**models::ElementsUserInviteRead**](ElementsUserInviteRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

