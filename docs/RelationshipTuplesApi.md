# \RelationshipTuplesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_relationship_tuples**](RelationshipTuplesApi.md#bulk_create_relationship_tuples) | **POST** /v2/facts/{proj_id}/{env_id}/relationship_tuples/bulk | Bulk create relationship tuples(EAP)
[**bulk_delete_relationship_tuples**](RelationshipTuplesApi.md#bulk_delete_relationship_tuples) | **DELETE** /v2/facts/{proj_id}/{env_id}/relationship_tuples/bulk | Bulk Delete Relationship Tuples
[**create_relationship_tuple**](RelationshipTuplesApi.md#create_relationship_tuple) | **POST** /v2/facts/{proj_id}/{env_id}/relationship_tuples | Create Relationship Tuple
[**delete_relationship_tuple**](RelationshipTuplesApi.md#delete_relationship_tuple) | **DELETE** /v2/facts/{proj_id}/{env_id}/relationship_tuples | Delete Relationship Tuple
[**list_relationship_tuples**](RelationshipTuplesApi.md#list_relationship_tuples) | **GET** /v2/facts/{proj_id}/{env_id}/relationship_tuples | List Relationship Tuples



## bulk_create_relationship_tuples

> serde_json::Value bulk_create_relationship_tuples(proj_id, env_id, relationship_tuple_create_bulk_operation)
Bulk create relationship tuples(EAP)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**relationship_tuple_create_bulk_operation** | [**RelationshipTupleCreateBulkOperation**](RelationshipTupleCreateBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_relationship_tuples

> serde_json::Value bulk_delete_relationship_tuples(proj_id, env_id, relationship_tuple_delete_bulk_operation)
Bulk Delete Relationship Tuples

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**relationship_tuple_delete_bulk_operation** | [**RelationshipTupleDeleteBulkOperation**](RelationshipTupleDeleteBulkOperation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_relationship_tuple

> models::RelationshipTupleRead create_relationship_tuple(proj_id, env_id, relationship_tuple_create)
Create Relationship Tuple

Create a relationship between two resource instances using a relation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**relationship_tuple_create** | [**RelationshipTupleCreate**](RelationshipTupleCreate.md) |  | [required] |

### Return type

[**models::RelationshipTupleRead**](RelationshipTupleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_relationship_tuple

> delete_relationship_tuple(proj_id, env_id, relationship_tuple_delete)
Delete Relationship Tuple

Delete a relationship between two resource instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**relationship_tuple_delete** | [**RelationshipTupleDelete**](RelationshipTupleDelete.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_relationship_tuples

> Vec<models::RelationshipTupleRead> list_relationship_tuples(proj_id, env_id, detailed, page, per_page, tenant, subject, relation, object, object_type, subject_type)
List Relationship Tuples

Lists the relationship tuples defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**detailed** | Option<**bool**> | If true, will return the full subject and object resource instances. |  |[default to false]
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]
**tenant** | Option<**String**> | The tenant key or id to filter by |  |
**subject** | Option<**String**> | The subject to filter by, accepts either the resource instance id or resource_type:resource_instance |  |
**relation** | Option<**String**> | The relation id or key to filter by |  |
**object** | Option<**String**> | The object to filter by, accepts either the resource instance id or resource_type:resource_instance |  |
**object_type** | Option<**String**> | The object type to filter by, accepts resource type id or key |  |
**subject_type** | Option<**String**> | The subject type to filter by, accepts resource type id or key |  |

### Return type

[**Vec<models::RelationshipTupleRead>**](RelationshipTupleRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

