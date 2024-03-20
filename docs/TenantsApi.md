# \TenantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant**](TenantsApi.md#create_tenant) | **POST** /v2/facts/{proj_id}/{env_id}/tenants | Create Tenant
[**delete_tenant**](TenantsApi.md#delete_tenant) | **DELETE** /v2/facts/{proj_id}/{env_id}/tenants/{tenant_id} | Delete Tenant
[**delete_tenant_user**](TenantsApi.md#delete_tenant_user) | **DELETE** /v2/facts/{proj_id}/{env_id}/tenants/{tenant_id}/users/{user_id} | Delete Tenant User
[**get_tenant**](TenantsApi.md#get_tenant) | **GET** /v2/facts/{proj_id}/{env_id}/tenants/{tenant_id} | Get Tenant
[**list_tenant_users**](TenantsApi.md#list_tenant_users) | **GET** /v2/facts/{proj_id}/{env_id}/tenants/{tenant_id}/users | List Tenant Users
[**list_tenants**](TenantsApi.md#list_tenants) | **GET** /v2/facts/{proj_id}/{env_id}/tenants | List Tenants
[**update_tenant**](TenantsApi.md#update_tenant) | **PATCH** /v2/facts/{proj_id}/{env_id}/tenants/{tenant_id} | Update Tenant



## create_tenant

> models::TenantRead create_tenant(proj_id, env_id, tenant_create)
Create Tenant

Creates a new tenant inside the Permit.io system.  If the tenant is already created: will return 200 instead of 201, and will return the existing tenant object in the response body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**tenant_create** | [**TenantCreate**](TenantCreate.md) |  | [required] |

### Return type

[**models::TenantRead**](TenantRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant

> delete_tenant(tenant_id, proj_id, env_id)
Delete Tenant

Deletes the tenant and all its related data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Either the unique id of the tenant, or the URL-friendly key of the tenant (i.e: the \"slug\"). | [required] |
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_user

> delete_tenant_user(tenant_id, user_id, proj_id, env_id)
Delete Tenant User

Deletes a user under a tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Either the unique id of the tenant, or the URL-friendly key of the tenant (i.e: the \"slug\"). | [required] |
**user_id** | **String** | Either the unique id of the user, or the URL-friendly key of the user (i.e: the \"slug\"). | [required] |
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant

> models::TenantRead get_tenant(proj_id, env_id, tenant_id)
Get Tenant

Gets a tenant, if such tenant exists. Otherwise returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**tenant_id** | **String** | Either the unique id of the tenant, or the URL-friendly key of the tenant (i.e: the \"slug\"). | [required] |

### Return type

[**models::TenantRead**](TenantRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tenant_users

> models::PaginatedResultUserRead list_tenant_users(tenant_id, proj_id, env_id, search, role, include_resource_instance_roles, page, per_page)
List Tenant Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Either the unique id of the tenant, or the URL-friendly key of the tenant (i.e: the \"slug\"). | [required] |
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


## list_tenants

> models::ResponseListTenantsV2FactsProjIdEnvIdTenantsGet list_tenants(proj_id, env_id, search, include_total_count, page, per_page)
List Tenants

Lists all the tenants defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**search** | Option<**String**> | Text search for the tenant name or key |  |
**include_total_count** | Option<**bool**> | Include total count in response |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::ResponseListTenantsV2FactsProjIdEnvIdTenantsGet**](Response_List_Tenants_V2_Facts__Proj_Id___Env_Id__Tenants_Get.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant

> models::TenantRead update_tenant(tenant_id, proj_id, env_id, tenant_update)
Update Tenant

Partially updates the tenant definition. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Either the unique id of the tenant, or the URL-friendly key of the tenant (i.e: the \"slug\"). | [required] |
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**tenant_update** | [**TenantUpdate**](TenantUpdate.md) |  | [required] |

### Return type

[**models::TenantRead**](TenantRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

