# \BulkOperationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_tenants**](BulkOperationsApi.md#bulk_create_tenants) | **POST** /v2/facts/{proj_id}/{env_id}/bulk/tenants | Bulk Create Tenants
[**bulk_create_users**](BulkOperationsApi.md#bulk_create_users) | **POST** /v2/facts/{proj_id}/{env_id}/bulk/users | Bulk Create Users
[**bulk_delete_resource_instances**](BulkOperationsApi.md#bulk_delete_resource_instances) | **DELETE** /v2/facts/{proj_id}/{env_id}/bulk/resource_instances | Bulk Delete Resource Instances
[**bulk_delete_tenants**](BulkOperationsApi.md#bulk_delete_tenants) | **DELETE** /v2/facts/{proj_id}/{env_id}/bulk/tenants | Bulk Delete Tenants
[**bulk_delete_users**](BulkOperationsApi.md#bulk_delete_users) | **DELETE** /v2/facts/{proj_id}/{env_id}/bulk/users | Bulk Delete Users
[**bulk_replace_resource_instances**](BulkOperationsApi.md#bulk_replace_resource_instances) | **PUT** /v2/facts/{proj_id}/{env_id}/bulk/resource_instances | Bulk Replace Resource Instances
[**bulk_replace_users**](BulkOperationsApi.md#bulk_replace_users) | **PUT** /v2/facts/{proj_id}/{env_id}/bulk/users | Bulk Replace Users



## bulk_create_tenants

> serde_json::Value bulk_create_tenants(proj_id, env_id, tenant_create_bulk_operation)
Bulk Create Tenants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**tenant_create_bulk_operation** | [**TenantCreateBulkOperation**](TenantCreateBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_create_users

> serde_json::Value bulk_create_users(proj_id, env_id, user_create_bulk_operation)
Bulk Create Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_create_bulk_operation** | [**UserCreateBulkOperation**](UserCreateBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_resource_instances

> serde_json::Value bulk_delete_resource_instances(proj_id, env_id, resource_instance_delete_bulk_operation)
Bulk Delete Resource Instances

Deletes many Resource Instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_instance_delete_bulk_operation** | [**ResourceInstanceDeleteBulkOperation**](ResourceInstanceDeleteBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_tenants

> serde_json::Value bulk_delete_tenants(proj_id, env_id, tenant_delete_bulk_operation)
Bulk Delete Tenants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**tenant_delete_bulk_operation** | [**TenantDeleteBulkOperation**](TenantDeleteBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_users

> serde_json::Value bulk_delete_users(proj_id, env_id, user_delete_bulk_operation)
Bulk Delete Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_delete_bulk_operation** | [**UserDeleteBulkOperation**](UserDeleteBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_replace_resource_instances

> serde_json::Value bulk_replace_resource_instances(proj_id, env_id, resource_instance_create_bulk_operation)
Bulk Replace Resource Instances

Creates or replaces Resource Instances. If a resource instance with `key` and `resource` already exists, it will be replaced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_instance_create_bulk_operation** | [**ResourceInstanceCreateBulkOperation**](ResourceInstanceCreateBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_replace_users

> serde_json::Value bulk_replace_users(proj_id, env_id, user_replace_bulk_operation)
Bulk Replace Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**user_replace_bulk_operation** | [**UserReplaceBulkOperation**](UserReplaceBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

