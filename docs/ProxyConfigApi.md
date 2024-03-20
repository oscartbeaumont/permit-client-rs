# \ProxyConfigApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_proxy_config**](ProxyConfigApi.md#create_proxy_config) | **POST** /v2/facts/{proj_id}/{env_id}/proxy_configs | Create Proxy Config
[**delete_proxy_config**](ProxyConfigApi.md#delete_proxy_config) | **DELETE** /v2/facts/{proj_id}/{env_id}/proxy_configs/{proxy_config_id} | Delete Proxy Config
[**get_proxy_config**](ProxyConfigApi.md#get_proxy_config) | **GET** /v2/facts/{proj_id}/{env_id}/proxy_configs/{proxy_config_id} | Get Proxy Config
[**list_proxy_configs**](ProxyConfigApi.md#list_proxy_configs) | **GET** /v2/facts/{proj_id}/{env_id}/proxy_configs | List Proxy Configs
[**update_proxy_config**](ProxyConfigApi.md#update_proxy_config) | **PATCH** /v2/facts/{proj_id}/{env_id}/proxy_configs/{proxy_config_id} | Update Proxy Config



## create_proxy_config

> models::ProxyConfigRead create_proxy_config(proj_id, env_id, proxy_config_create)
Create Proxy Config

Creates a new proxy config inside the Permit.io system.  If the proxy config is already created: will return 200 instead of 201, and will return the existing proxy config object in the response body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**proxy_config_create** | [**ProxyConfigCreate**](ProxyConfigCreate.md) |  | [required] |

### Return type

[**models::ProxyConfigRead**](ProxyConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_proxy_config

> delete_proxy_config(proj_id, env_id, proxy_config_id)
Delete Proxy Config

Deletes the proxy config and all its related data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**proxy_config_id** | **String** | Either the unique id of the proxy config, or the URL-friendly key of the proxy config (i.e: the \"slug\"). | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_proxy_config

> models::ProxyConfigRead get_proxy_config(proj_id, env_id, proxy_config_id)
Get Proxy Config

Gets a proxy config, if such proxy config exists. Otherwise returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**proxy_config_id** | **String** | Either the unique id of the proxy config, or the URL-friendly key of the proxy config (i.e: the \"slug\"). | [required] |

### Return type

[**models::ProxyConfigRead**](ProxyConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_proxy_configs

> Vec<models::ProxyConfigRead> list_proxy_configs(proj_id, env_id, page, per_page)
List Proxy Configs

Lists all the proxy configs defined within an environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::ProxyConfigRead>**](ProxyConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_proxy_config

> models::ProxyConfigRead update_proxy_config(proj_id, env_id, proxy_config_id, proxy_config_update)
Update Proxy Config

Partially updates the proxy config definition. Fields that will be provided will be completely overwritten.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**proxy_config_id** | **String** | Either the unique id of the proxy config, or the URL-friendly key of the proxy config (i.e: the \"slug\"). | [required] |
**proxy_config_update** | [**ProxyConfigUpdate**](ProxyConfigUpdate.md) |  | [required] |

### Return type

[**models::ProxyConfigRead**](ProxyConfigRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

