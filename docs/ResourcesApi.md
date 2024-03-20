# \ResourcesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource**](ResourcesApi.md#create_resource) | **POST** /v2/schema/{proj_id}/{env_id}/resources | Create Resource
[**delete_resource**](ResourcesApi.md#delete_resource) | **DELETE** /v2/schema/{proj_id}/{env_id}/resources/{resource_id} | Delete Resource
[**get_resource**](ResourcesApi.md#get_resource) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id} | Get Resource
[**list_resources**](ResourcesApi.md#list_resources) | **GET** /v2/schema/{proj_id}/{env_id}/resources | List Resources
[**replace_resource**](ResourcesApi.md#replace_resource) | **PUT** /v2/schema/{proj_id}/{env_id}/resources/{resource_id} | Replace Resource
[**update_resource**](ResourcesApi.md#update_resource) | **PATCH** /v2/schema/{proj_id}/{env_id}/resources/{resource_id} | Update Resource



## create_resource

> models::ResourceRead create_resource(proj_id, env_id, resource_create)
Create Resource

Creates a new resource (a type of object you may protect with permissions).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_create** | [**ResourceCreate**](ResourceCreate.md) |  | [required] |

### Return type

[**models::ResourceRead**](ResourceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource

> delete_resource(proj_id, env_id, resource_id)
Delete Resource

Deletes the resource and all its related data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource

> models::ResourceRead get_resource(proj_id, env_id, resource_id)
Get Resource

Gets a single resource, if such resource exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceRead**](ResourceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resources

> models::ResponseListResourcesV2SchemaProjIdEnvIdResourcesGet list_resources(proj_id, env_id, include_built_in, include_total_count, page, per_page, search)
List Resources

Lists all the resources defined in your schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**include_built_in** | Option<**bool**> | Whether to include or exclude built-in resources, default is False |  |[default to false]
**include_total_count** | Option<**bool**> | Include total count in response |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]
**search** | Option<**String**> | Text search for the object name or key |  |

### Return type

[**models::ResponseListResourcesV2SchemaProjIdEnvIdResourcesGet**](Response_List_Resources_V2_Schema__Proj_Id___Env_Id__Resources_Get.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_resource

> models::ResourceRead replace_resource(proj_id, env_id, resource_id, resource_replace)
Replace Resource

Completely replaces the resource definition.  - If the resource key is changed, all role and permissions assignments for the the resource will be revoked. - If the resource key is unchanged, but some actions are removed or renamed from the resource definition, role and permissions assignments for these actions will be revoked.  TODO: we need to decide if we are auto-revoking, or if we are rejecting the PUT completely while there are permissions that can be affected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**resource_replace** | [**ResourceReplace**](ResourceReplace.md) |  | [required] |

### Return type

[**models::ResourceRead**](ResourceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_resource

> models::ResourceRead update_resource(proj_id, env_id, resource_id, resource_update)
Update Resource

Partially updates the resource definition. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**resource_update** | [**ResourceUpdate**](ResourceUpdate.md) |  | [required] |

### Return type

[**models::ResourceRead**](ResourceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

