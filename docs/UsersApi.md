# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_role_to_user**](UsersApi.md#assign_role_to_user) | **POST** /v2/facts/{proj_id}/{env_id}/users/{user_id}/roles | Assign Role To User
[**create_user**](UsersApi.md#create_user) | **POST** /v2/facts/{proj_id}/{env_id}/users | Create User
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /v2/facts/{proj_id}/{env_id}/users/{user_id} | Delete User
[**get_user**](UsersApi.md#get_user) | **GET** /v2/facts/{proj_id}/{env_id}/users/{user_id} | Get User
[**list_users**](UsersApi.md#list_users) | **GET** /v2/facts/{proj_id}/{env_id}/users | List Users
[**replace_user**](UsersApi.md#replace_user) | **PUT** /v2/facts/{proj_id}/{env_id}/users/{user_id} | Replace User
[**unassign_role_from_user**](UsersApi.md#unassign_role_from_user) | **DELETE** /v2/facts/{proj_id}/{env_id}/users/{user_id}/roles | Unassign Role From User
[**update_user**](UsersApi.md#update_user) | **PATCH** /v2/facts/{proj_id}/{env_id}/users/{user_id} | Update User



## assign_role_to_user

> models::RoleAssignmentRead assign_role_to_user(proj_id, env_id, user_id, user_role_create)
Assign Role To User

Assigns a role to the user within the tenant.  The tenant defines the scope of the assignment. In other words, the role is effective only within the tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_id** | **String** | Either the unique id of the user, or the URL-friendly key of the user (i.e: the \"slug\"). | [required] |
**user_role_create** | [**UserRoleCreate**](UserRoleCreate.md) |  | [required] |

### Return type

[**models::RoleAssignmentRead**](RoleAssignmentRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::UserRead create_user(proj_id, env_id, user_create)
Create User

Creates a new user inside the Permit.io system, from that point forward you may run permission checks on that user.  If the user is already created: will return 200 instead of 201, and will return the existing user object in the response body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_create** | [**UserCreate**](UserCreate.md) |  | [required] |

### Return type

[**models::UserRead**](UserRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(proj_id, env_id, user_id)
Delete User

Deletes the user and all its related data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_id** | **String** | Either the unique id of the user, or the URL-friendly key of the user (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::UserRead get_user(proj_id, env_id, user_id)
Get User

Gets a user, if such user exists. Otherwise returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_id** | **String** | Either the unique id of the user, or the URL-friendly key of the user (i.e: the \"slug\"). | [required] |

### Return type

[**models::UserRead**](UserRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> models::PaginatedResultUserRead list_users(proj_id, env_id, search, role, include_resource_instance_roles, page, per_page)
List Users

Lists all the users defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**search** | Option<**String**> | Text search for the email field |  |
**role** | Option<**String**> | Match users with a specific role |  |
**include_resource_instance_roles** | Option<**bool**> | Should add resource instance roles |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::PaginatedResultUserRead**](PaginatedResult_UserRead_.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_user

> models::UserRead replace_user(proj_id, env_id, user_id, user_create)
Replace User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_id** | **String** | Either the unique id of the user, or the URL-friendly key of the user (i.e: the \"slug\"). | [required] |
**user_create** | [**UserCreate**](UserCreate.md) |  | [required] |

### Return type

[**models::UserRead**](UserRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_role_from_user

> unassign_role_from_user(proj_id, env_id, user_id, user_role_remove)
Unassign Role From User

Unassigns the role from the user within the tenant.  The tenant defines the scope of the assignment. In other words, the role is effective only within the tenant.  If the role is not actually assigned, will return 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_id** | **String** | Either the unique id of the user, or the URL-friendly key of the user (i.e: the \"slug\"). | [required] |
**user_role_remove** | [**UserRoleRemove**](UserRoleRemove.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::UserRead update_user(proj_id, env_id, user_id, user_update)
Update User

Partially updates the user definition. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_id** | **String** | Either the unique id of the user, or the URL-friendly key of the user (i.e: the \"slug\"). | [required] |
**user_update** | [**UserUpdate**](UserUpdate.md) |  | [required] |

### Return type

[**models::UserRead**](UserRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

