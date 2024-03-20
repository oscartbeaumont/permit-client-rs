# \ResourceInstancesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource_instance**](ResourceInstancesApi.md#create_resource_instance) | **POST** /v2/facts/{proj_id}/{env_id}/resource_instances | Create Resource Instance
[**delete_resource_instance**](ResourceInstancesApi.md#delete_resource_instance) | **DELETE** /v2/facts/{proj_id}/{env_id}/resource_instances/{instance_id} | Delete Resource Instance
[**get_resource_instance**](ResourceInstancesApi.md#get_resource_instance) | **GET** /v2/facts/{proj_id}/{env_id}/resource_instances/{instance_id} | Get Resource Instance
[**list_resource_instances**](ResourceInstancesApi.md#list_resource_instances) | **GET** /v2/facts/{proj_id}/{env_id}/resource_instances | List Resource Instances
[**update_resource_instance**](ResourceInstancesApi.md#update_resource_instance) | **PATCH** /v2/facts/{proj_id}/{env_id}/resource_instances/{instance_id} | Update Resource Instance



## create_resource_instance

> models::ResourceInstanceRead create_resource_instance(proj_id, env_id, resource_instance_create)
Create Resource Instance

Creates a new instance inside the Permit.io system.  If the instance is already created: will return 200 instead of 201, and will return the existing instance object in the response body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_instance_create** | [**ResourceInstanceCreate**](ResourceInstanceCreate.md) |  | [required] |

### Return type

[**models::ResourceInstanceRead**](ResourceInstanceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_instance

> delete_resource_instance(proj_id, env_id, instance_id)
Delete Resource Instance

Deletes the instance and all its related data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**instance_id** | **String** | Either the unique id of the resource instance, or the URL-friendly key of the resource instance (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_instance

> models::ResourceInstanceRead get_resource_instance(proj_id, env_id, instance_id)
Get Resource Instance

Gets a instance, if such instance exists. Otherwise returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**instance_id** | **String** | Either the unique id of the resource instance, or the URL-friendly key of the resource instance (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceInstanceRead**](ResourceInstanceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_instances

> Vec<models::ResourceInstanceRead> list_resource_instances(proj_id, env_id, tenant, resource, detailed, page, per_page, search)
List Resource Instances

Lists all the resource instances defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**tenant** | Option<**String**> | The tenant key or id to filter by |  |
**resource** | Option<**String**> | The resource key or id to filter by |  |
**detailed** | Option<**bool**> | If true, will return the relationships of the resource instance. |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]
**search** | Option<**String**> | Text search for the object name or key |  |

### Return type

[**Vec<models::ResourceInstanceRead>**](ResourceInstanceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_resource_instance

> models::ResourceInstanceRead update_resource_instance(proj_id, env_id, instance_id, resource_instance_update)
Update Resource Instance

Partially updates the instance definition. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**instance_id** | **String** | Either the unique id of the resource instance, or the URL-friendly key of the resource instance (i.e: the \"slug\"). | [required] |
**resource_instance_update** | [**ResourceInstanceUpdate**](ResourceInstanceUpdate.md) |  | [required] |

### Return type

[**models::ResourceInstanceRead**](ResourceInstanceRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

