# \ScopeConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_scope_config**](ScopeConfigurationsApi.md#get_scope_config) | **GET** /v2/projects/{proj_id}/{env_id}/opal_scope | Get Scope Config
[**reset_scope_config**](ScopeConfigurationsApi.md#reset_scope_config) | **DELETE** /v2/projects/{proj_id}/{env_id}/opal_scope | Reset Scope Config
[**set_scope_config**](ScopeConfigurationsApi.md#set_scope_config) | **PUT** /v2/projects/{proj_id}/{env_id}/opal_scope | Set Scope Config



## get_scope_config

> models::ScopeConfigRead get_scope_config(proj_id, env_id)
Get Scope Config

Returns the custom OPAL Scope config for given environment, if exists. If not custom config is set, return 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |

### Return type

[**models::ScopeConfigRead**](ScopeConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_scope_config

> reset_scope_config(proj_id, env_id)
Reset Scope Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## set_scope_config

> models::ScopeConfigRead set_scope_config(proj_id, env_id, scope_config_set)
Set Scope Config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**scope_config_set** | [**ScopeConfigSet**](ScopeConfigSet.md) |  | [required] |

### Return type

[**models::ScopeConfigRead**](ScopeConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

