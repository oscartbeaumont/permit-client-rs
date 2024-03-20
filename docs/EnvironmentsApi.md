# \EnvironmentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_environment**](EnvironmentsApi.md#copy_environment) | **POST** /v2/projects/{proj_id}/envs/{env_id}/copy | Copy Environment
[**create_environment**](EnvironmentsApi.md#create_environment) | **POST** /v2/projects/{proj_id}/envs | Create Environment
[**delete_environment**](EnvironmentsApi.md#delete_environment) | **DELETE** /v2/projects/{proj_id}/envs/{env_id} | Delete Environment
[**get_environment**](EnvironmentsApi.md#get_environment) | **GET** /v2/projects/{proj_id}/envs/{env_id} | Get Environment
[**list_environments**](EnvironmentsApi.md#list_environments) | **GET** /v2/projects/{proj_id}/envs | List Environments
[**stats_environments**](EnvironmentsApi.md#stats_environments) | **GET** /v2/projects/{proj_id}/envs/{env_id}/stats | Stats Environments
[**test_jwks_by_url**](EnvironmentsApi.md#test_jwks_by_url) | **POST** /v2/projects/{proj_id}/envs/{env_id}/test_jwks | Test Jwks By Url
[**update_environment**](EnvironmentsApi.md#update_environment) | **PATCH** /v2/projects/{proj_id}/envs/{env_id} | Update Environment



## copy_environment

> models::EnvironmentRead copy_environment(proj_id, env_id, environment_copy)
Copy Environment

Copy environment  This endpoint either duplicates an existing environment to a new environment in the same project, or copies from an existing environment to another existing environment.  The `scope` object controls which objects will be copied to the target environment.  To clone to a new environment, the user must have write permissions to the containing project. To clone into an existing environment, the user must have write permissions to the target environment.  Copying environments across projects or organizations is not allowed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**environment_copy** | [**EnvironmentCopy**](EnvironmentCopy.md) |  | [required] |

### Return type

[**models::EnvironmentRead**](EnvironmentRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_environment

> models::EnvironmentRead create_environment(proj_id, environment_create)
Create Environment

Creates a new environment under a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**environment_create** | [**EnvironmentCreate**](EnvironmentCreate.md) |  | [required] |

### Return type

[**models::EnvironmentRead**](EnvironmentRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_environment

> delete_environment(proj_id, env_id)
Delete Environment

Deletes an environment and all its related data.

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


## get_environment

> models::EnvironmentReadWithEmailConfig get_environment(proj_id, env_id)
Get Environment

Gets a single environment matching the given env_id, if such environment exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |

### Return type

[**models::EnvironmentReadWithEmailConfig**](EnvironmentReadWithEmailConfig.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environments

> Vec<models::EnvironmentReadWithEmailConfig> list_environments(proj_id, page, per_page)
List Environments

Lists all the environments under a given project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**page** | Option<**i32**> | Page number of the results to fetch, starting at 1. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). |  |[default to 30]

### Return type

[**Vec<models::EnvironmentReadWithEmailConfig>**](EnvironmentReadWithEmailConfig.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stats_environments

> models::EnvironmentStats stats_environments(proj_id, env_id)
Stats Environments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |

### Return type

[**models::EnvironmentStats**](EnvironmentStats.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_jwks_by_url

> serde_json::Value test_jwks_by_url(proj_id, env_id, url)
Test Jwks By Url

Test Jwks Url.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**url** | Option<**String**> | URL of JWKs to test |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_environment

> models::EnvironmentRead update_environment(proj_id, env_id, environment_update)
Update Environment

Updates the environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proj_id** | **String** | Either the unique id of the project, or the URL-friendly key of the project (i.e: the \"slug\"). | [required] |
**env_id** | **String** | Either the unique id of the environment, or the URL-friendly key of the environment (i.e: the \"slug\"). | [required] |
**environment_update** | [**EnvironmentUpdate**](EnvironmentUpdate.md) |  | [required] |

### Return type

[**models::EnvironmentRead**](EnvironmentRead.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

