# \ResourceAttributesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource_attribute**](ResourceAttributesApi.md#create_resource_attribute) | **POST** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/attributes | Create Resource Attribute
[**delete_resource_attribute**](ResourceAttributesApi.md#delete_resource_attribute) | **DELETE** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/attributes/{attribute_id} | Delete Resource Attribute
[**get_resource_attribute**](ResourceAttributesApi.md#get_resource_attribute) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/attributes/{attribute_id} | Get Resource Attribute
[**list_resource_attributes**](ResourceAttributesApi.md#list_resource_attributes) | **GET** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/attributes | List Resource Attributes
[**update_resource_attribute**](ResourceAttributesApi.md#update_resource_attribute) | **PATCH** /v2/schema/{proj_id}/{env_id}/resources/{resource_id}/attributes/{attribute_id} | Update Resource Attribute



## create_resource_attribute

> models::ResourceAttributeRead create_resource_attribute(proj_id, env_id, resource_id, resource_attribute_create)
Create Resource Attribute

Creates a new attribute as part of the resource definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**resource_attribute_create** | [**ResourceAttributeCreate**](ResourceAttributeCreate.md) |  | [required] |

### Return type

[**models::ResourceAttributeRead**](ResourceAttributeRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_attribute

> delete_resource_attribute(proj_id, env_id, resource_id, attribute_id, page, per_page)
Delete Resource Attribute

Deletes the attribute and all its related data.  Note: If the attribute is used by policies, removing it will cause the attribute to evaluate as `undefined`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**attribute_id** | **String** | Either the unique id of the attribute, or the URL-friendly key of the attribute (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_attribute

> models::ResourceAttributeRead get_resource_attribute(proj_id, env_id, resource_id, attribute_id)
Get Resource Attribute

Gets a single attribute defined on the resource, if such attribute exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**attribute_id** | **String** | Either the unique id of the attribute, or the URL-friendly key of the attribute (i.e: the \"slug\"). | [required] |

### Return type

[**models::ResourceAttributeRead**](ResourceAttributeRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_attributes

> Vec<models::ResourceAttributeRead> list_resource_attributes(proj_id, env_id, resource_id, page, per_page)
List Resource Attributes

Lists all the attributes defined on the resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ResourceAttributeRead>**](ResourceAttributeRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_resource_attribute

> models::ResourceAttributeRead update_resource_attribute(proj_id, env_id, resource_id, attribute_id, resource_attribute_update)
Update Resource Attribute

Partially updates the attribute defined on a resource. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | **String** | Either the unique id of the resource, or the URL-friendly key of the resource (i.e: the \"slug\"). | [required] |
**attribute_id** | **String** | Either the unique id of the attribute, or the URL-friendly key of the attribute (i.e: the \"slug\"). | [required] |
**resource_attribute_update** | [**ResourceAttributeUpdate**](ResourceAttributeUpdate.md) |  | [required] |

### Return type

[**models::ResourceAttributeRead**](ResourceAttributeRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

