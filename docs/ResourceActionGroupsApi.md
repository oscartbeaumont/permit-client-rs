# \ResourceActionGroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource_action_group**](ResourceActionGroupsApi.md#create_resource_action_group) | **POST** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/action_groups | Create Resource Action Group
[**delete_resource_action_group**](ResourceActionGroupsApi.md#delete_resource_action_group) | **DELETE** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/action_groups/{action_group_id} | Delete Resource Action Group
[**get_resource_action_group**](ResourceActionGroupsApi.md#get_resource_action_group) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/action_groups/{action_group_id} | Get Resource Action Group
[**list_resource_action_groups**](ResourceActionGroupsApi.md#list_resource_action_groups) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/action_groups | List Resource Action Groups
[**update_resource_action_group**](ResourceActionGroupsApi.md#update_resource_action_group) | **PATCH** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/action_groups/{action_group_id} | Update Resource Action Group



## create_resource_action_group

> models::ResourceActionGroupRead create_resource_action_group(proj_id, env_id, resource_id, resource_action_group_create)
Create Resource Action Group

Creates a new action group that can affect the resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**resource_action_group_create** | [**ResourceActionGroupCreate**](ResourceActionGroupCreate.md) |  | [required] |

### Return type

[**models::ResourceActionGroupRead**](ResourceActionGroupRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_action_group

> delete_resource_action_group(proj_id, env_id, resource_id, action_group_id)
Delete Resource Action Group

Deletes the action and all its related data. This includes any permissions granted to perform the action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**action_group_id** | **String** | Either the unique id of the action group, or the URL-friendly key of the action group (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_action_group

> models::ResourceActionGroupRead get_resource_action_group(proj_id, env_id, resource_id, action_group_id)
Get Resource Action Group

Gets a single action group defined on the resource, if such action exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**action_group_id** | **String** | Either the unique id of the action group, or the URL-friendly key of the action group (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceActionGroupRead**](ResourceActionGroupRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_action_groups

> Vec<models::ResourceActionGroupRead> list_resource_action_groups(proj_id, env_id, resource_id, page, per_page)
List Resource Action Groups

Lists all the action groups defined on the resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ResourceActionGroupRead>**](ResourceActionGroupRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_resource_action_group

> models::ResourceActionGroupRead update_resource_action_group(resource_id, action_group_id, proj_id, env_id, resource_action_group_update)
Update Resource Action Group

Partially updates the action defined on a resource. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**action_group_id** | **String** | Either the unique id of the action group, or the URL-friendly key of the action group (i.e: the \"slug\"). | [required] |
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_action_group_update** | [**ResourceActionGroupUpdate**](ResourceActionGroupUpdate.md) |  | [required] |

### Return type

[**models::ResourceActionGroupRead**](ResourceActionGroupRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

