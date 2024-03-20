# \ResourceRolesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_permissions_to_resource_role**](ResourceRolesApi.md#assign_permissions_to_resource_role) | **POST** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles/{role_id}/permissions | Assign Permissions to Role
[**create_resource_role**](ResourceRolesApi.md#create_resource_role) | **POST** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles | Create Resource Role
[**delete_resource_role**](ResourceRolesApi.md#delete_resource_role) | **DELETE** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles/{role_id} | Delete Resource Role
[**get_resource_role**](ResourceRolesApi.md#get_resource_role) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles/{role_id} | Get Resource Role
[**get_resource_role_ancestors**](ResourceRolesApi.md#get_resource_role_ancestors) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles/{role_id}/ancestors | Get Resource Role Ancestors
[**get_resource_role_descendants**](ResourceRolesApi.md#get_resource_role_descendants) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles/{role_id}/descendants | Get Resource Role Descendants
[**list_resource_roles**](ResourceRolesApi.md#list_resource_roles) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles | List Resource Roles
[**remove_permissions_from_resource_role**](ResourceRolesApi.md#remove_permissions_from_resource_role) | **DELETE** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles/{role_id}/permissions | Remove Permissions from Role
[**update_resource_role**](ResourceRolesApi.md#update_resource_role) | **PATCH** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/roles/{role_id} | Update Resource Role



## assign_permissions_to_resource_role

> models::ResourceRoleRead assign_permissions_to_resource_role(proj_id, env_id, resource_id, role_id, add_role_permissions)
Assign Permissions to Role

Assign permissions to role.  If some of the permissions specified are already assigned, will skip them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |
**add_role_permissions** | [**AddRolePermissions**](AddRolePermissions.md) |  | [required] |

### Return type

[**models::ResourceRoleRead**](ResourceRoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_resource_role

> models::ResourceRoleRead create_resource_role(proj_id, env_id, resource_id, resource_role_create)
Create Resource Role

Creates a new role associated with the resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**resource_role_create** | [**ResourceRoleCreate**](ResourceRoleCreate.md) |  | [required] |

### Return type

[**models::ResourceRoleRead**](ResourceRoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_role

> delete_resource_role(proj_id, env_id, resource_id, role_id)
Delete Resource Role

Deletes the role and all its related data. This includes any permissions granted to said role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_role

> models::ResourceRoleRead get_resource_role(proj_id, env_id, resource_id, role_id)
Get Resource Role

Gets a single role defined on the resource, if such role exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceRoleRead**](ResourceRoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_role_ancestors

> models::ResourceRoleList get_resource_role_ancestors(proj_id, env_id, resource_id, role_id)
Get Resource Role Ancestors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceRoleList**](ResourceRoleList.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_role_descendants

> models::ResourceRoleList get_resource_role_descendants(proj_id, env_id, resource_id, role_id)
Get Resource Role Descendants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceRoleList**](ResourceRoleList.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_roles

> Vec<models::ResourceRoleRead> list_resource_roles(proj_id, env_id, resource_id, page, per_page)
List Resource Roles

Lists all the roles defined on the resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ResourceRoleRead>**](ResourceRoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_permissions_from_resource_role

> models::ResourceRoleRead remove_permissions_from_resource_role(proj_id, env_id, resource_id, role_id, remove_role_permissions)
Remove Permissions from Role

Remove permissions from role.  If some of the permissions specified are already unassigned, will skip them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |
**remove_role_permissions** | [**RemoveRolePermissions**](RemoveRolePermissions.md) |  | [required] |

### Return type

[**models::ResourceRoleRead**](ResourceRoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_resource_role

> models::ResourceRoleRead update_resource_role(proj_id, env_id, resource_id, role_id, resource_role_update)
Update Resource Role

Partially updates the role defined on a resource. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**role_id** | **String** | Either the unique id of the role, or the URL-friendly key of the role (i.e: the \"slug\"). | [required] |
**resource_role_update** | [**ResourceRoleUpdate**](ResourceRoleUpdate.md) |  | [required] |

### Return type

[**models::ResourceRoleRead**](ResourceRoleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

