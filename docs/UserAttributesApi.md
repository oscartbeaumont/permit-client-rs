# \UserAttributesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_attribute**](UserAttributesApi.md#create_user_attribute) | **POST** /v2/schema/{proj_id}/{env_id}/users/attributes | Create User Attribute
[**delete_user_attribute**](UserAttributesApi.md#delete_user_attribute) | **DELETE** /v2/schema/{proj_id}/{env_id}/users/attributes/{attribute_id} | Delete User Attribute
[**get_user_attribute**](UserAttributesApi.md#get_user_attribute) | **GET** /v2/schema/{proj_id}/{env_id}/users/attributes/{attribute_id} | Get User Attribute
[**list_user_attributes**](UserAttributesApi.md#list_user_attributes) | **GET** /v2/schema/{proj_id}/{env_id}/users/attributes | List User Attributes
[**update_user_attribute**](UserAttributesApi.md#update_user_attribute) | **PATCH** /v2/schema/{proj_id}/{env_id}/users/attributes/{attribute_id} | Update User Attribute



## create_user_attribute

> models::ResourceAttributeRead create_user_attribute(proj_id, env_id, resource_attribute_create, resource_id)
Create User Attribute

Creates a new attribute for the User resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_attribute_create** | [**ResourceAttributeCreate**](ResourceAttributeCreate.md) |  | [required] |
**resource_id** | Option<**String**> |  |  |[default to __user]

### Return type

[**models::ResourceAttributeRead**](ResourceAttributeRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_attribute

> delete_user_attribute(proj_id, env_id, attribute_id, resource_id, page, per_page)
Delete User Attribute

Deletes the attribute and all its related data.  Note: If the attribute is used by policies, removing it will cause the attribute to evaluate as `undefined`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**attribute_id** | **String** | Either the unique id of the attribute, or the URL-friendly key of the attribute (i.e: the \"slug\"). | [required] |
**resource_id** | Option<**String**> |  |  |[default to __user]
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


## get_user_attribute

> models::ResourceAttributeRead get_user_attribute(proj_id, env_id, attribute_id, resource_id)
Get User Attribute

Gets a single attribute defined on the User resource, if such attribute exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**attribute_id** | **String** | Either the unique id of the attribute, or the URL-friendly key of the attribute (i.e: the \"slug\"). | [required] |
**resource_id** | Option<**String**> |  |  |[default to __user]

### Return type

[**models::ResourceAttributeRead**](ResourceAttributeRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_attributes

> Vec<models::ResourceAttributeRead> list_user_attributes(proj_id, env_id, resource_id, page, per_page)
List User Attributes

Lists all the attributes defined on the User resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**resource_id** | Option<**String**> |  |  |[default to __user]
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


## update_user_attribute

> models::ResourceAttributeRead update_user_attribute(proj_id, env_id, attribute_id, resource_attribute_update, resource_id)
Update User Attribute

Partially updates the attribute defined on the User resource. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**attribute_id** | **String** | Either the unique id of the attribute, or the URL-friendly key of the attribute (i.e: the \"slug\"). | [required] |
**resource_attribute_update** | [**ResourceAttributeUpdate**](ResourceAttributeUpdate.md) |  | [required] |
**resource_id** | Option<**String**> |  |  |[default to __user]

### Return type

[**models::ResourceAttributeRead**](ResourceAttributeRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

