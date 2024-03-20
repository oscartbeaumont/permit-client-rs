# \RoleAssignmentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_role**](RoleAssignmentsApi.md#assign_role) | **POST** /v2/facts/{proj_id}/{env_id}/role_assignments | Assign Role
[**bulk_assign_role**](RoleAssignmentsApi.md#bulk_assign_role) | **POST** /v2/facts/{proj_id}/{env_id}/role_assignments/bulk | Bulk create role assignments(EAP)
[**bulk_unassign_role**](RoleAssignmentsApi.md#bulk_unassign_role) | **DELETE** /v2/facts/{proj_id}/{env_id}/role_assignments/bulk | Bulk Unassign Role
[**list_role_assignments**](RoleAssignmentsApi.md#list_role_assignments) | **GET** /v2/facts/{proj_id}/{env_id}/role_assignments | List Role Assignments
[**unassign_role**](RoleAssignmentsApi.md#unassign_role) | **DELETE** /v2/facts/{proj_id}/{env_id}/role_assignments | Unassign Role



## assign_role

> models::RoleAssignmentRead assign_role(proj_id, env_id, role_assignment_create)
Assign Role

Assigns a role to a user within a tenant.  The tenant defines the scope of the assignment. In other words, the role is effective only within the tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_assignment_create** | [**RoleAssignmentCreate**](RoleAssignmentCreate.md) |  | [required] |

### Return type

[**models::RoleAssignmentRead**](RoleAssignmentRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_assign_role

> models::BulkRoleAssignmentReport bulk_assign_role(proj_id, env_id, role_assignment_create)
Bulk create role assignments(EAP)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_assignment_create** | [**Vec<models::RoleAssignmentCreate>**](RoleAssignmentCreate.md) |  | [required] |

### Return type

[**models::BulkRoleAssignmentReport**](BulkRoleAssignmentReport.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_unassign_role

> models::BulkRoleUnAssignmentReport bulk_unassign_role(proj_id, env_id, role_assignment_remove)
Bulk Unassign Role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_assignment_remove** | [**Vec<models::RoleAssignmentRemove>**](RoleAssignmentRemove.md) |  | [required] |

### Return type

[**models::BulkRoleUnAssignmentReport**](BulkRoleUnAssignmentReport.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_role_assignments

> models::ResponseListRoleAssignmentsV2FactsProjIdEnvIdRoleAssignmentsGet list_role_assignments(proj_id, env_id, user, role, tenant, resource, resource_instance, detailed, page, per_page)
List Role Assignments

Lists the role assignments defined within an environment.  - If the `user` filter is present, will only return the role assignments of that user. - If the `tenant` filter is present, will only return the role assignments in that tenant. - If the `role` filter is present, will only return role assignments that are granting that role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user** | Option<**String**> | optional user filter, will only return role assignments granted to this user. |  |
**role** | Option<**String**> | optional role filter, will only return role assignments granting this role. |  |
**tenant** | Option<**String**> | optional tenant filter, will only return role assignments granted in that tenant. |  |
**resource** | Option<**String**> | optional resource **type** filter, will only return role assignments granted on that resource type. |  |
**resource_instance** | Option<**String**> | optional resource instance filter, will only return role assignments granted on that resource instance. |  |
**detailed** | Option<**bool**> | Whether to return full details about the user, tenant and role |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::ResponseListRoleAssignmentsV2FactsProjIdEnvIdRoleAssignmentsGet**](Response_List_Role_Assignments_V2_Facts__Proj_Id___Env_Id__Role_Assignments_Get.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_role

> unassign_role(proj_id, env_id, role_assignment_remove)
Unassign Role

Unassigns a user role within a tenant.  The tenant defines the scope of the assignment. In other words, the role is effective only within the tenant.  If the role is not actually assigned, will return 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**role_assignment_remove** | [**RoleAssignmentRemove**](RoleAssignmentRemove.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

