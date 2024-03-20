# \RolesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_permissions_to_role**](RolesApi.md#assign_permissions_to_role) | **POST** /v2/schema/{proj_id}/{env_id}/roles/{role_id}/permissions | Assign Permissions To Role
[**create_role**](RolesApi.md#create_role) | **POST** /v2/schema/{proj_id}/{env_id}/roles | Create Role
[**delete_role**](RolesApi.md#delete_role) | **DELETE** /v2/schema/{proj_id}/{env_id}/roles/{role_id} | Delete Role
[**get_role**](RolesApi.md#get_role) | **GET** /v2/schema/{proj_id}/{env_id}/roles/{role_id} | Get Role
[**get_role_ancestors**](RolesApi.md#get_role_ancestors) | **GET** /v2/schema/{proj_id}/{env_id}/roles/{role_id}/ancestors | Get Role Ancestors
[**get_role_descendants**](RolesApi.md#get_role_descendants) | **GET** /v2/schema/{proj_id}/{env_id}/roles/{role_id}/descendants | Get Role Descendants
[**list_roles**](RolesApi.md#list_roles) | **GET** /v2/schema/{proj_id}/{env_id}/roles | List Roles
[**remove_permissions_from_role**](RolesApi.md#remove_permissions_from_role) | **DELETE** /v2/schema/{proj_id}/{env_id}/roles/{role_id}/permissions | Remove Permissions From Role
[**update_role**](RolesApi.md#update_role) | **PATCH** /v2/schema/{proj_id}/{env_id}/roles/{role_id} | Update Role



## assign_permissions_to_role

> models::RoleRead assign_permissions_to_role(proj_id, env_id, role_id, add_role_permissions)
Assign Permissions To Role

Assign permissions to role.  If some of the permissions specified are already assigned, will skip them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |
**add_role_permissions** | [**AddRolePermissions**](AddRolePermissions.md) |  | [required] |

### Return type

[**models::RoleRead**](RoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> models::RoleRead create_role(proj_id, env_id, role_create)
Create Role

Creates a new tenant role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_create** | [**RoleCreate**](RoleCreate.md) |  | [required] |

### Return type

[**models::RoleRead**](RoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> delete_role(proj_id, env_id, role_id)
Delete Role

Deletes a tenant role and all its related data. This includes any permissions granted to said role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> models::RoleRead get_role(proj_id, env_id, role_id)
Get Role

Gets a single tenant role, if such role exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

[**models::RoleRead**](RoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_ancestors

> models::RoleList get_role_ancestors(proj_id, env_id, role_id)
Get Role Ancestors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

[**models::RoleList**](RoleList.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_descendants

> models::RoleList get_role_descendants(proj_id, env_id, role_id)
Get Role Descendants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

[**models::RoleList**](RoleList.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_roles

> models::ResponseListRolesV2SchemaProjIdEnvIdRolesGet list_roles(proj_id, env_id, include_total_count, page, per_page, search)
List Roles

Lists all tenant roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**include_total_count** | Option<**bool**> | Include total count in response |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]
**search** | Option<**String**> | Text search for the object name or key |  |

### Return type

[**models::ResponseListRolesV2SchemaProjIdEnvIdRolesGet**](Response_List_Roles_V2_Schema__Proj_Id___Env_Id__Roles_Get.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_permissions_from_role

> models::RoleRead remove_permissions_from_role(proj_id, env_id, role_id, remove_role_permissions)
Remove Permissions From Role

Remove permissions from role.  If some of the permissions specified are already unassigned, will skip them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |
**remove_role_permissions** | [**RemoveRolePermissions**](RemoveRolePermissions.md) |  | [required] |

### Return type

[**models::RoleRead**](RoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> models::RoleRead update_role(proj_id, env_id, role_id, role_update)
Update Role

Partially updates a tenant role. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |
**role_update** | [**RoleUpdate**](RoleUpdate.md) |  | [required] |

### Return type

[**models::RoleRead**](RoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

