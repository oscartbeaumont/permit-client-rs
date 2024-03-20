# \ConditionSetsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_condition_set**](ConditionSetsApi.md#create_condition_set) | **POST** /v2/schema/{proj_id}/{env_id}/condition_sets | Create Condition Set
[**delete_condition_set**](ConditionSetsApi.md#delete_condition_set) | **DELETE** /v2/schema/{proj_id}/{env_id}/condition_sets/{condition_set_id} | Delete Condition Set
[**get_condition_set**](ConditionSetsApi.md#get_condition_set) | **GET** /v2/schema/{proj_id}/{env_id}/condition_sets/{condition_set_id} | Get Condition Set
[**get_condition_set_ancestors**](ConditionSetsApi.md#get_condition_set_ancestors) | **GET** /v2/schema/{proj_id}/{env_id}/condition_sets/{condition_set_id}/ancestors | Get Condition Set Ancestors
[**get_condition_set_descendants**](ConditionSetsApi.md#get_condition_set_descendants) | **GET** /v2/schema/{proj_id}/{env_id}/condition_sets/{condition_set_id}/descendants | Get Condition Set Descendants
[**list_condition_sets**](ConditionSetsApi.md#list_condition_sets) | **GET** /v2/schema/{proj_id}/{env_id}/condition_sets | List Condition Sets
[**update_condition_set**](ConditionSetsApi.md#update_condition_set) | **PATCH** /v2/schema/{proj_id}/{env_id}/condition_sets/{condition_set_id} | Update Condition Set



## create_condition_set

> models::ConditionSetRead create_condition_set(proj_id, env_id, condition_set_create)
Create Condition Set

Creates a new condition set (can be either a user set or a resource set).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_create** | [**ConditionSetCreate**](ConditionSetCreate.md) |  | [required] |

### Return type

[**models::ConditionSetRead**](ConditionSetRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_condition_set

> delete_condition_set(proj_id, env_id, condition_set_id)
Delete Condition Set

Deletes a condition set and all its related data. This includes any permissions granted to said condition set (i.e: any matching condition set users_with_role).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_id** | **String** | Either the unique id of the condition set, or the URL-friendly key of the condition set (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_condition_set

> models::ConditionSetRead get_condition_set(proj_id, env_id, condition_set_id)
Get Condition Set

Gets a single condition set, if such condition set exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_id** | **String** | Either the unique id of the condition set, or the URL-friendly key of the condition set (i.e: the \"slug\"). | [required] |

### Return type

[**models::ConditionSetRead**](ConditionSetRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_condition_set_ancestors

> Vec<models::ConditionSetRead> get_condition_set_ancestors(proj_id, env_id, condition_set_id, page, per_page)
Get Condition Set Ancestors

Gets all ancestors (parent, parent of parent, and so on)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_id** | **String** | Either the unique id of the condition set, or the URL-friendly key of the condition set (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ConditionSetRead>**](ConditionSetRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_condition_set_descendants

> Vec<models::ConditionSetRead> get_condition_set_descendants(proj_id, env_id, condition_set_id, page, per_page)
Get Condition Set Descendants

Gets all descendants (children, children of children, and so on)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_id** | **String** | Either the unique id of the condition set, or the URL-friendly key of the condition set (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ConditionSetRead>**](ConditionSetRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_condition_sets

> models::ResponseListConditionSetsV2SchemaProjIdEnvIdConditionSetsGet list_condition_sets(proj_id, env_id, search, r#type, include_total_count, page, per_page)
List Condition Sets

Lists all condition sets matching a filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**search** | Option<**String**> | Text search for the condition sets name or key |  |
**r#type** | Option<[**models::ConditionSetType**](.md)> | if provided, will return only the condition sets of the specified type. e.g: only user sets. |  |
**include_total_count** | Option<**bool**> | Include total count in response |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::ResponseListConditionSetsV2SchemaProjIdEnvIdConditionSetsGet**](Response_List_Condition_Sets_V2_Schema__Proj_Id___Env_Id__Condition_Sets_Get.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_condition_set

> models::ConditionSetRead update_condition_set(proj_id, env_id, condition_set_id, condition_set_update)
Update Condition Set

Partially updates a condition set. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**condition_set_id** | **String** | Either the unique id of the condition set, or the URL-friendly key of the condition set (i.e: the \"slug\"). | [required] |
**condition_set_update** | [**ConditionSetUpdate**](ConditionSetUpdate.md) |  | [required] |

### Return type

[**models::ConditionSetRead**](ConditionSetRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

