# \ResourceActionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource_action**](ResourceActionsApi.md#create_resource_action) | **POST** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/actions | Create Resource Action
[**delete_resource_action**](ResourceActionsApi.md#delete_resource_action) | **DELETE** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/actions/{action_id} | Delete Resource Action
[**get_resource_action**](ResourceActionsApi.md#get_resource_action) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/actions/{action_id} | Get Resource Action
[**list_resource_actions**](ResourceActionsApi.md#list_resource_actions) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/actions | List Resource Actions
[**update_resource_action**](ResourceActionsApi.md#update_resource_action) | **PATCH** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/actions/{action_id} | Update Resource Action



## create_resource_action

> models::ResourceActionRead create_resource_action(proj_id, env_id, resource_id, resource_action_create)
Create Resource Action

Creates a new action that can affect the resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**resource_action_create** | [**ResourceActionCreate**](ResourceActionCreate.md) |  | [required] |

### Return type

[**models::ResourceActionRead**](ResourceActionRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_action

> delete_resource_action(proj_id, env_id, resource_id, action_id)
Delete Resource Action

Deletes the action and all its related data. This includes any permissions granted to perform the action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**action_id** | **String** | Either the unique id of the action, or the URL-friendly key of the action (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_action

> models::ResourceActionRead get_resource_action(proj_id, env_id, resource_id, action_id)
Get Resource Action

Gets a single action defined on the resource, if such action exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**action_id** | **String** | Either the unique id of the action, or the URL-friendly key of the action (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceActionRead**](ResourceActionRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_actions

> Vec<models::ResourceActionRead> list_resource_actions(proj_id, env_id, resource_id, page, per_page)
List Resource Actions

Lists all the actions defined on the resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ResourceActionRead>**](ResourceActionRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_resource_action

> models::ResourceActionRead update_resource_action(proj_id, env_id, resource_id, action_id, resource_action_update)
Update Resource Action

Partially updates the action defined on a resource. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**action_id** | **String** | Either the unique id of the action, or the URL-friendly key of the action (i.e: the \"slug\"). | [required] |
**resource_action_update** | [**ResourceActionUpdate**](ResourceActionUpdate.md) |  | [required] |

### Return type

[**models::ResourceActionRead**](ResourceActionRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

