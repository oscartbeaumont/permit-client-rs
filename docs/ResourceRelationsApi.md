# \ResourceRelationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource_relation**](ResourceRelationsApi.md#create_resource_relation) | **POST** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/relations | Create Resource Relation
[**delete_resource_relation**](ResourceRelationsApi.md#delete_resource_relation) | **DELETE** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/relations/{relation_id} | Delete Resource Relation
[**get_resource_relation**](ResourceRelationsApi.md#get_resource_relation) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/relations/{relation_id} | Get Resource Relation
[**list_resource_relations**](ResourceRelationsApi.md#list_resource_relations) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/relations | List Resource Relations



## create_resource_relation

> models::RelationRead create_resource_relation(proj_id, env_id, resource_id, relation_create)
Create Resource Relation

Creates a resource relation to another resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**relation_create** | [**RelationCreate**](RelationCreate.md) |  | [required] |

### Return type

[**models::RelationRead**](RelationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_relation

> delete_resource_relation(proj_id, env_id, resource_id, relation_id)
Delete Resource Relation

Deletes a resource relation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**relation_id** | **String** | Either the unique id of the relation, or the URL-friendly key of the relation (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_relation

> models::RelationRead get_resource_relation(proj_id, env_id, resource_id, relation_id)
Get Resource Relation

Get a resource relation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**relation_id** | **String** | Either the unique id of the relation, or the URL-friendly key of the relation (i.e: the \"slug\"). | [required] |

### Return type

[**models::RelationRead**](RelationRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_relations

> models::PaginatedResultRelationRead list_resource_relations(proj_id, env_id, resource_id, page, per_page)
List Resource Relations

List relations on a given resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**models::PaginatedResultRelationRead**](PaginatedResult_RelationRead_.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

